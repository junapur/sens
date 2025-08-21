use crate::games::{GAMES, Game};
use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct ListArgs {
    /// Also show aliases
    #[arg(short, long)]
    aliases: bool,
}

pub fn execute(args: ListArgs) -> Result<()> {
    let mut sorted_games: Vec<&Game> = GAMES.iter().collect();
    sorted_games.sort_by_key(|game| game.title.to_lowercase());

    for game in sorted_games {
        println!("─ {}", game.title);
        if args.aliases && !game.aliases.is_empty() {
            println!("  └─ Aliases: {} \n", game.aliases.join(", "));
        }
    }

    Ok(())
}
