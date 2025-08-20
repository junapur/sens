use crate::games::GAMES;
use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct ListArgs {
    /// Also show aliases
    #[arg(short, long)]
    aliases: bool,
}

pub fn execute(args: ListArgs) -> Result<()> {
    for game in GAMES {
        if args.aliases && !game.aliases.is_empty() {
            println!("{} ({})", game.title, game.aliases.join(", "));
        } else {
            println!("{}", game.title);
        }
    }

    Ok(())
}
