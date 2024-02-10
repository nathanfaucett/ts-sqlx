use std::path::{Path, PathBuf, MAIN_SEPARATOR};

use once_cell::sync::Lazy;
use regex::Regex;
use walkdir::WalkDir;

pub fn pattern_to_regex(pattern: &str) -> Regex {
    let pattern = pattern.replace('!', "");
    let pattern = pattern.replace('.', "\\.");
    let pattern = pattern.replace('*', ".*");
    let pattern = format!("^{}$", pattern);
    Regex::new(&pattern).unwrap()
}

static NODE_MODULES: Lazy<String> =
    Lazy::new(|| format!("{}node_modules{}", MAIN_SEPARATOR, MAIN_SEPARATOR));

pub fn is_valid_path(
    entry_path: &str,
    extension: Option<&str>,
    ignore_regexes: &[(Regex, bool)],
    extensions: &[String],
) -> bool {
    if entry_path.contains(NODE_MODULES.as_str()) {
        return false;
    }
    let should_ignore = ignore_regexes.iter().any(|(re, not)| {
        if *not {
            !re.is_match(entry_path)
        } else {
            re.is_match(entry_path)
        }
    });
    if should_ignore {
        return false;
    }
    let valid_extension = if let Some(ext) = extension {
        extensions.contains(&ext.to_lowercase())
    } else {
        false
    };

    if valid_extension {
        return true;
    } else {
        return false;
    }
}

pub fn scan_folder(
    folder: &Path,
    extensions: &[String],
    ignore_regexes: &[(Regex, bool)],
) -> Vec<PathBuf> {
    let result: Vec<_> = WalkDir::new(Path::new(folder))
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|entry| {
            if let Some(entry_path) = entry.path().as_os_str().to_str() {
                is_valid_path(
                    entry_path,
                    entry.path().extension().and_then(|os| os.to_str()),
                    &ignore_regexes,
                    &extensions,
                )
            } else {
                false
            }
        })
        .map(|entry| entry.path().to_owned())
        .collect();

    result
}
