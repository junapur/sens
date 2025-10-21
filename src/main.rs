mod cli;
mod commands;
mod games;
mod math;
mod parsers;

use anyhow::Result;
use clap::Parser;
use cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.run()?;

    Ok(())
}
