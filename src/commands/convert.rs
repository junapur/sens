use crate::games::get_game;
use crate::math::convert_sensitivity;
use crate::parsers::parse_positive_f64;
use anyhow::{Context, Result};
use clap::{Args, value_parser};

#[derive(Args)]
pub struct ConvertArgs {
    /// The game you're converting from
    #[arg()]
    from_game: String,

    /// The game you're converting to
    #[arg()]
    to_game: String,

    /// Your in-game sensitivity
    #[arg(value_parser = parse_positive_f64)]
    sensitivity: f64,

    /// Your current DPI
    #[arg(requires = "to_dpi", value_parser = value_parser!(u32).range(1..))]
    from_dpi: Option<u32>,

    /// Your target DPI
    #[arg(requires = "from_dpi", value_parser = value_parser!(u32).range(1..))]
    to_dpi: Option<u32>,
}

pub fn run(args: ConvertArgs) -> Result<()> {
    let from_game =
        get_game(&args.from_game).context(format!("source game '{}' not found", args.from_game))?;

    let to_game =
        get_game(&args.to_game).context(format!("target game '{}' not found", args.to_game))?;

    let sensitivity = convert_sensitivity(
        args.sensitivity,
        from_game.yaw,
        to_game.yaw,
        args.from_dpi,
        args.to_dpi,
    );

    println!("{:.3}", sensitivity);
    Ok(())
}
