use crate::games::get_game;
use crate::math::get_cm_360;
use crate::parsers::parse_positive_f64;
use anyhow::{Context, Result};
use clap::{Args, value_parser};

#[derive(Args)]
pub struct CmArgs {
    /// The game you're converting from
    #[arg()]
    game: String,

    /// Your in-game sensitivity
    #[arg(value_parser = parse_positive_f64)]
    sensitivity: f64,

    /// your DPI
    #[arg(value_parser = value_parser!(u32).range(1..))]
    dpi: u32,
}

pub fn run(args: CmArgs) -> Result<()> {
    let from_game = get_game(&args.game).context(format!("game '{}' not found", args.game))?;
    let cm_360 = get_cm_360(args.sensitivity, args.dpi, from_game.yaw);

    println!("{:.3}", cm_360);
    Ok(())
}
