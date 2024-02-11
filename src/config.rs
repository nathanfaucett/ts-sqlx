use std::{
  env::{self, current_dir},
  fs::File,
  io::BufReader,
  path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};
use hashbrown::HashMap;
use regex::Regex;
use serde::Deserialize;
use url::Url;

use crate::{
  scan_folder::pattern_to_regex,
  ts::{get_foss_driver_for_database_url, QueryToTSDriver},
};

#[derive(Deserialize, Default)]
pub struct Config {
  pub src: Option<String>,
  pub dest: Option<String>,
  pub extensions: Option<Vec<String>>,
  pub ignore_patterns: Option<Vec<String>>,
  pub databases: HashMap<String, String>,
  #[serde(skip, default)]
  pub config_path: Option<PathBuf>,
}

impl Config {
  pub fn from_path<P>(path: P) -> Result<Self>
  where
    P: AsRef<Path>,
  {
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let mut config: Self = serde_json::from_reader(reader)?;
    config.config_path = path.as_ref().to_path_buf().parent().map(Path::to_path_buf);
    Ok(config)
  }
  pub fn from_env() -> Result<Self> {
    Self::from_path(current_dir()?.join(".ts-sqlx.json"))
  }
}

impl TryInto<RuntimeConfig> for Config {
  type Error = anyhow::Error;

  fn try_into(self) -> Result<RuntimeConfig, Self::Error> {
    let cwd = if let Some(config_path) = self.config_path {
      config_path
    } else {
      current_dir()?
    };
    let src = if let Some(s) = self.src {
      cwd.join(s)
    } else {
      cwd.clone()
    };
    let dest = if let Some(d) = self.dest {
      cwd.join(d)
    } else {
      src.join(".ts-sqlx")
    };

    let mut databases = HashMap::with_capacity(self.databases.capacity());
    for (name, url) in self.databases {
      databases.insert(name, url.parse()?);
    }

    Ok(RuntimeConfig {
      src,
      dest,
      extensions: self.extensions.unwrap_or_else(|| {
        vec![
          "ts".to_owned(),
          "tsx".to_owned(),
          "js".to_owned(),
          "jsx".to_owned(),
        ]
      }),
      ignore_regexes: self
        .ignore_patterns
        .unwrap_or_else(|| vec!["*.d.ts".to_owned()])
        .into_iter()
        .map(|ip| (pattern_to_regex(&ip), ip.starts_with("!")))
        .collect::<Vec<_>>(),
      databases,
    })
  }
}

#[derive(Debug, Clone)]
pub struct RuntimeConfig {
  pub src: PathBuf,
  pub dest: PathBuf,
  pub extensions: Vec<String>,
  pub ignore_regexes: Vec<(Regex, bool)>,
  pub databases: HashMap<String, Url>,
}

impl RuntimeConfig {
  pub fn from_path<P>(path: P) -> Result<Self>
  where
    P: AsRef<Path>,
  {
    Config::from_path(path)?.try_into()
  }
  pub fn from_env() -> Result<Self> {
    Config::from_env()?.try_into()
  }

  pub fn get_driver(&self, name: Option<&str>) -> anyhow::Result<(String, Url, &QueryToTSDriver)> {
    let database = name.unwrap_or("default");

    let database_url = if let Some(database_url) = self.databases.get(database) {
      Some(database_url.clone())
    } else if let Some(env_database_url) = env::var(if database != "default" {
      format!("{}_DATABASE_URL", database.to_uppercase())
    } else {
      "DATABASE_URL".to_owned()
    })
    .ok()
    {
      Some(env_database_url.parse()?)
    } else {
      None
    };

    if let Some(database_url) = database_url {
      let driver = get_foss_driver_for_database_url(&database_url)?;
      Ok((database.to_owned(), database_url, driver))
    } else {
      Err(anyhow!("database {:?} not found", database))
    }
  }
}
