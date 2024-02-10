use std::env::current_dir;

use clap::{Args, Parser};
#[cfg(feature = "completions")]
use clap_complete::Shell;

use crate::config::{Config, RuntimeConfig};

#[derive(Parser, Debug)]
#[clap(version, about, author)]
pub struct Opt {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Parser, Debug)]
pub enum Command {
    Run {
        #[clap(flatten)]
        config_opts: ConfigOpts,
    },
    Watch {
        #[clap(flatten)]
        config_opts: ConfigOpts,
    },

    #[cfg(feature = "completions")]
    Completions { shell: Shell },
}

#[derive(Args, Debug, Clone)]
pub struct ConfigOpts {
    #[clap(long, short = 'c')]
    pub config: Option<String>,

    #[clap(long, short = 's')]
    pub src: Option<String>,

    #[clap(long, short = 'd')]
    pub dest: Option<String>,

    #[clap(long, short = 'D', env)]
    pub database_url: Option<String>,

    #[clap(long, default_value = "10")]
    pub connect_timeout: u64,

    #[cfg(feature = "sqlite")]
    #[clap(long, action = clap::ArgAction::Set, default_value = "true")]
    pub sqlite_create_db_wal: bool,
}

impl TryInto<RuntimeConfig> for ConfigOpts {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<RuntimeConfig, Self::Error> {
        let mut config = if let Some(c) = self.config {
            Config::from_path(current_dir()?.join(c))?
        } else {
            Config::default()
        };
        if let Some(s) = self.src {
            config.src.replace(s);
        }
        if let Some(d) = self.dest {
            config.dest.replace(d);
        }
        if let Some(database_url) = self.database_url {
            config.databases.insert("default".to_owned(), database_url);
        }
        config.try_into()
    }
}
