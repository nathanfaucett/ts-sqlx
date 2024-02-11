use anyhow::Result;
use clap::Parser;
use ts_sqlx::{opt::Opt, run::run};

// cargo invokes this binary as `cargo-sqlx sqlx <args>`
// so the parser below is defined with that in mind
#[derive(Parser, Debug)]
#[clap(bin_name = "cargo")]
enum Cli {
  Sqlx(Opt),
}

fn main() -> Result<()> {
  dotenvy::dotenv().ok();
  let Cli::Sqlx(opt) = Cli::parse();
  run(opt)?;
  Ok(())
}
