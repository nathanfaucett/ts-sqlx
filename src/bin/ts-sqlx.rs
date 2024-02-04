use anyhow::Result;
use clap::Parser;
use ts_sqlx::{opt::Opt, run::run};

fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    run(Opt::parse())?;
    Ok(())
}
