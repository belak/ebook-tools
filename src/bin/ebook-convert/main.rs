use anyhow::Result;
use clap::Parser;

mod cli;

fn main() -> Result<()> {
    env_logger::init();

    let cli = cli::Cli::parse();
    cli.execute()
}
