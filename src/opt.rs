use clap::{Args, Parser};
#[cfg(feature = "completions")]
use clap_complete::Shell;

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
        connect_opts: ConnectOpts,
    },
    Watch {
        #[clap(flatten)]
        connect_opts: ConnectOpts,
    },

    #[cfg(feature = "completions")]
    Completions { shell: Shell },
}

#[derive(Args, Debug)]
pub struct ConnectOpts {
    #[clap(long, short = 'f', default_value = ".")]
    pub folder: String,

    #[clap(long, short = 'o')]
    pub out: Option<String>,

    #[clap(long, short = 'D', env)]
    pub database_url: String,

    #[clap(long, default_value = "10")]
    pub connect_timeout: u64,

    #[cfg(feature = "sqlite")]
    #[clap(long, action = clap::ArgAction::Set, default_value = "true")]
    pub sqlite_create_db_wal: bool,
}
