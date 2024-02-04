use anyhow::{anyhow, Result};
use notify::{
    event::AccessKind, recommended_watcher, Error, Event, EventKind, RecursiveMode, Watcher,
};
use regex::Regex;
use std::{
    collections::HashSet,
    env::current_dir,
    fs::{create_dir_all, read_dir, remove_file, write},
    path::{Path, MAIN_SEPARATOR},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

#[cfg(feature = "completions")]
use crate::completions;
use crate::{
    opt::{Command, ConnectOpts, Opt},
    parse_source::parse_source,
    scan_folder::{is_valid_path, pattern_to_regex, scan_folder},
    ts::{get_foss_driver_for_database_url, ts_calls_to_string},
};

pub fn run(opt: Opt) -> Result<()> {
    match opt.command {
        Command::Run { connect_opts } => {
            run_command(&connect_opts)?;
        }
        Command::Watch { connect_opts } => {
            run_command(&connect_opts)?;
            watch_command(connect_opts)?;
        }
        #[cfg(feature = "completions")]
        Command::Completions { shell } => completions::run(shell),
    }
    Ok(())
}

pub fn run_command(connect_opts: &ConnectOpts) -> Result<()> {
    let database_url = &connect_opts.database_url;
    let cwd = current_dir()?;
    let folder = cwd.join(&connect_opts.folder);
    let out_dir = if let Some(out) = &connect_opts.out {
        cwd.join(out)
    } else {
        folder.join(".ts-sqlx")
    };
    run_for_folder(
        database_url,
        &folder,
        &out_dir,
        &vec![
            "ts".to_owned(),
            "tsx".to_owned(),
            "js".to_owned(),
            "jsx".to_owned(),
            "svelte".to_owned(),
            "vue".to_owned(),
        ],
        &vec!["*.d.ts".to_owned()],
    )?;
    Ok(())
}

pub fn watch_command(connect_opts: ConnectOpts) -> Result<()> {
    let database_url = connect_opts.database_url;
    let cwd = current_dir()?;
    let folder = cwd.join(connect_opts.folder);
    let out_dir = if let Some(out) = connect_opts.out {
        cwd.join(out)
    } else {
        folder.join(".ts-sqlx")
    };
    let ignore_regexes = vec!["*.d.ts".to_owned()]
        .iter()
        .map(|ip| (pattern_to_regex(ip), ip.starts_with("!")))
        .collect::<Vec<_>>();

    let watcher_folder = folder.clone();
    let mut watcher = recommended_watcher(move |res: Result<Event, Error>| match res {
        Ok(event) => match event.kind {
            EventKind::Access(access) => match access {
                AccessKind::Close(_) => {
                    for path in event.paths {
                        match run_for_file(
                            &database_url,
                            &path,
                            &watcher_folder,
                            &out_dir,
                            &vec![
                                "ts".to_owned(),
                                "tsx".to_owned(),
                                "js".to_owned(),
                                "jsx".to_owned(),
                                "svelte".to_owned(),
                                "vue".to_owned(),
                            ],
                            &ignore_regexes,
                        ) {
                            Ok(_) => {}
                            Err(e) => println!("{:?}", e),
                        }
                    }
                }
                _ => {}
            },
            EventKind::Any => {}
            EventKind::Create(_) => {}
            EventKind::Modify(_) => {}
            EventKind::Remove(_) => {}
            EventKind::Other => {}
        },
        Err(e) => println!("{:?}", e),
    })?;
    watcher.watch(&folder, RecursiveMode::Recursive)?;

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        println!("exiting...");
    })
    .expect("Error setting Ctrl-C handler");

    println!("ctrl+c to exit");
    while running.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}

pub fn run_for_folder(
    database_url: &str,
    folder: &Path,
    out_dir: &Path,
    extensions: &[String],
    ignore_patterns: &[String],
) -> Result<()> {
    let files = scan_folder(folder, extensions, ignore_patterns);
    if files.is_empty() {
        return Ok(());
    }

    let out_dir_path = Path::new(out_dir);
    create_dir_all(out_dir_path)?;

    let mut current_files: HashSet<String> = HashSet::new();
    for result in read_dir(out_dir_path)? {
        let entry = result?;
        if let Some(filename) = entry.file_name().to_str() {
            if filename.ends_with(".d.ts") {
                current_files.insert(filename.to_owned());
            }
        }
    }

    let driver = get_foss_driver_for_database_url(database_url)?;

    for file in files {
        let filename: String = format!(
            "{}.d.ts",
            file.strip_prefix(folder)?
                .to_str()
                .ok_or(anyhow!("invalid file {:?}", file))?
                .replace(MAIN_SEPARATOR, "_")
        );
        let sqlxs = parse_source(&file)?;
        if sqlxs.is_empty() {
            continue;
        }
        let mut ts_calls = Vec::with_capacity(sqlxs.capacity());

        for sqlx in sqlxs {
            ts_calls.push(driver.to_ts_call(&sqlx, database_url)?);
        }

        current_files.remove(&filename);
        write(out_dir_path.join(filename), ts_calls_to_string(&ts_calls))?;
    }
    for file in current_files {
        remove_file(out_dir_path.join(file))?;
    }

    Ok(())
}

pub fn run_for_file(
    database_url: &str,
    file: &Path,
    folder: &Path,
    out_dir: &Path,
    extensions: &[String],
    ignore_regexes: &[(Regex, bool)],
) -> Result<()> {
    if !is_valid_path(
        file.to_str().ok_or(anyhow!("invalid file {:?}", file))?,
        file.extension().and_then(|e| e.to_str()),
        ignore_regexes,
        extensions,
    ) {
        return Ok(());
    }

    let out_dir_path = Path::new(out_dir);
    create_dir_all(out_dir_path)?;

    let filename: String = format!(
        "{}.d.ts",
        file.strip_prefix(folder)?
            .to_str()
            .ok_or(anyhow!("invalid file {:?}", file))?
            .replace(MAIN_SEPARATOR, "_")
    );

    let sqlxs = parse_source(&file.into())?;
    if sqlxs.is_empty() {
        let _ = remove_file(out_dir_path.join(filename));
        return Ok(());
    }

    let driver = get_foss_driver_for_database_url(database_url)?;
    let mut ts_calls = Vec::with_capacity(sqlxs.capacity());

    for sqlx in sqlxs {
        ts_calls.push(driver.to_ts_call(&sqlx, database_url)?);
    }

    write(out_dir_path.join(filename), ts_calls_to_string(&ts_calls))?;

    Ok(())
}
