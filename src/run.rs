use anyhow::{anyhow, Result};
use notify::{
    event::AccessKind, recommended_watcher, Error, Event, EventKind, RecursiveMode, Watcher,
};
use std::{
    collections::HashSet,
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
    config::RuntimeConfig,
    opt::{Command, Opt},
    parse_source::parse_source,
    scan_folder::{is_valid_path, scan_folder},
    ts::ts_calls_to_string,
};

pub fn run(opt: Opt) -> Result<()> {
    match opt.command {
        Command::Run { config_opts } => {
            let config: RuntimeConfig = config_opts.try_into()?;
            run_command(&config)?;
        }
        Command::Watch { config_opts } => {
            let config: RuntimeConfig = config_opts.try_into()?;
            run_command(&config)?;
            watch_command(&config)?;
        }
        #[cfg(feature = "completions")]
        Command::Completions { shell } => completions::run(shell),
    }
    Ok(())
}

pub fn run_command(config: &RuntimeConfig) -> Result<()> {
    run_for_folder(config)?;
    Ok(())
}

pub fn watch_command(config: &RuntimeConfig) -> Result<()> {
    let watcher_config = config.clone();
    let mut watcher = recommended_watcher(move |res: Result<Event, Error>| match res {
        Ok(event) => match event.kind {
            EventKind::Access(access) => match access {
                AccessKind::Close(_) => {
                    for path in event.paths {
                        match run_for_file(&path, &watcher_config) {
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
    watcher.watch(&config.src, RecursiveMode::Recursive)?;

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

pub fn run_for_folder(config: &RuntimeConfig) -> Result<()> {
    let files = scan_folder(&config.src, &config.extensions, &config.ignore_regexes);
    if files.is_empty() {
        return Ok(());
    }

    let mut current_files: HashSet<String> = HashSet::new();
    if config.dest.exists() {
        for result in read_dir(&config.dest)? {
            let entry = result?;
            if let Some(filename) = entry.file_name().to_str() {
                if filename.ends_with(".d.ts") {
                    current_files.insert(filename.to_owned());
                }
            }
        }
    }
    create_dir_all(&config.dest)?;

    for file in files {
        let filename: String = format!(
            "{}.d.ts",
            file.strip_prefix(&config.src)?
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
            let (database, database_url, driver) =
                config.get_driver(sqlx.database.as_ref().map(String::as_str))?;
            ts_calls.push(driver.to_ts_call(&sqlx.query, &database, &database_url)?);
        }

        current_files.remove(&filename);
        write(config.dest.join(filename), ts_calls_to_string(&ts_calls))?;
    }
    for file in current_files {
        remove_file(config.dest.join(file))?;
    }

    Ok(())
}

pub fn run_for_file(file: &Path, config: &RuntimeConfig) -> Result<()> {
    if !is_valid_path(
        file.to_str().ok_or(anyhow!("invalid file {:?}", file))?,
        file.extension().and_then(|e| e.to_str()),
        &config.ignore_regexes,
        &config.extensions,
    ) {
        return Ok(());
    }

    create_dir_all(&config.dest)?;

    let filename: String = format!(
        "{}.d.ts",
        file.strip_prefix(&config.src)?
            .to_str()
            .ok_or(anyhow!("invalid file {:?}", file))?
            .replace(MAIN_SEPARATOR, "_")
    );

    let sqlxs = parse_source(&file.into())?;
    if sqlxs.is_empty() {
        let _ = remove_file(config.dest.join(filename));
        return Ok(());
    }

    let mut ts_calls = Vec::with_capacity(sqlxs.capacity());

    for sqlx in sqlxs {
        let (database, database_url, driver) =
            config.get_driver(sqlx.database.as_ref().map(String::as_str))?;
        ts_calls.push(driver.to_ts_call(&sqlx.query, &database, &database_url)?);
    }

    write(config.dest.join(filename), ts_calls_to_string(&ts_calls))?;

    Ok(())
}
