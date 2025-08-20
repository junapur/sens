use crate::games::get_game;
use crate::math::convert_sensitivity;
use crate::parsers::parse_positive_f64;
use anyhow::{Result, anyhow};
use clap::Args;

#[derive(Args)]
pub struct ConvertArgs {
    /// Your in-game sensitivity
    #[arg(value_parser = parse_positive_f64)]
    sensitivity: f64,

    /// The game you're converting from
    #[arg()]
    from: String,

    /// The game you're converting from
    #[arg()]
    to: String,
}

pub fn execute(args: ConvertArgs) -> Result<()> {
    let from_game =
        get_game(&args.from).ok_or_else(|| anyhow!("Source game '{}' not found", args.from))?;

    let to_game =
        get_game(&args.to).ok_or_else(|| anyhow!("Target game '{}' not found", args.to))?;

    let converted_sensitivity = convert_sensitivity(args.sensitivity, from_game.yaw, to_game.yaw);
    println!("{:.3}", converted_sensitivity);

    Ok(())
}
