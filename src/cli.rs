use crate::commands::{cm, convert, list};
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// List supported games
    List(list::ListArgs),

    /// Get your cm/360°
    Cm(cm::CmArgs),

    /// Convert your sensitivity from one game to another
    Convert(convert::ConvertArgs),
}

impl Cli {
    pub fn run(self) -> Result<()> {
        match self.command {
            Command::List(args) => list::run(args),
            Command::Cm(args) => cm::run(args),
            Command::Convert(args) => convert::run(args),
        }
    }
}
