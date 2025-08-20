use crate::games::get_game;
use crate::math::{convert_sensitivity, convert_sensitivity_dpi};
use crate::parsers::parse_positive_f64;
use anyhow::{Result, anyhow};
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

pub fn execute(args: ConvertArgs) -> Result<()> {
    let from_game = get_game(&args.from_game)
        .ok_or_else(|| anyhow!("Source game '{}' not found", &args.from_game))?;

    let to_game = get_game(&args.to_game)
        .ok_or_else(|| anyhow!("Target game '{}' not found", &args.to_game))?;

    let converted_sensitivity = match (args.from_dpi, args.to_dpi) {
        (Some(from_dpi), Some(to_dpi)) => convert_sensitivity_dpi(
            args.sensitivity,
            from_game.yaw,
            to_game.yaw,
            from_dpi,
            to_dpi,
        ),
        _ => convert_sensitivity(args.sensitivity, from_game.yaw, to_game.yaw),
    };

    println!("{:.3}", converted_sensitivity);

    Ok(())
}
