pub struct Game {
    pub title: &'static str,
    pub aliases: &'static [&'static str],
    pub yaw: f64,
}

pub fn get_game(query: &str) -> Option<&'static Game> {
    GAMES.iter().find(|game| {
        game.title.eq_ignore_ascii_case(query)
            || game
                .aliases
                .iter()
                .any(|alias| alias.eq_ignore_ascii_case(query))
    })
}

pub const GAMES: [Game; 16] = [
    Game {
        title: "Counter-Strike 2",
        aliases: &["cs", "cs2"],
        yaw: 0.022,
    },
    Game {
        title: "Team Fortress 2",
        aliases: &["tf2"],
        yaw: 0.022,
    },
    Game {
        title: "Quake",
        aliases: &["qk"],
        yaw: 0.022,
    },
    Game {
        title: "Apex Legends",
        aliases: &["al", "apex"],
        yaw: 0.022,
    },
    Game {
        title: "Overwatch 2",
        aliases: &["ow", "ow2", "overwatch"],
        yaw: 0.0066,
    },
    Game {
        title: "Call of Duty",
        aliases: &["cod"],
        yaw: 0.0066,
    },
    Game {
        title: "Destiny 2",
        aliases: &["d2"],
        yaw: 0.0066,
    },
    Game {
        title: "Valorant",
        aliases: &["vl", "val"],
        yaw: 0.06996,
    },
    Game {
        title: "Fortnite",
        aliases: &["fn"],
        yaw: 0.005555,
    },
    Game {
        title: "Rust",
        aliases: &["rs"],
        yaw: 0.1125,
    },
    Game {
        title: "The Finals",
        aliases: &["finals"],
        yaw: 0.001,
    },
    Game {
        title: "Marvel Rivals",
        aliases: &["mr", "rivals"],
        yaw: 0.0175,
    },
    Game {
        title: "Deadlock",
        aliases: &["dl"],
        yaw: 0.044,
    },
    Game {
        title: "Fragpunk",
        aliases: &["fp"],
        yaw: 0.05555,
    },
    Game {
        title: "Delta Force",
        aliases: &["df"],
        yaw: 0.03,
    },
    Game {
        title: "Rainbow Six Siege X",
        aliases: &["r6", "r6s"],
        yaw: 0.00572957795,
    },
];
