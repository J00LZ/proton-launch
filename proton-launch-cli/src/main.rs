use std::path::PathBuf;

use clap::Parser;

use proton_launch::command::{ProtonCommand, Runnable};
use proton_launch::paths::Paths;

use proton_launch::steam::SteamData;

#[derive(Parser)]
#[command(name = "proton-launch")]
#[command(author = "Julius de Jeu")]
pub struct ProtonLaunch {
    #[command(subcommand)]
    command: ProtonCommand,

    #[command(flatten)]
    paths: Paths,

    /// Path to the steam install folder.
    /// If not specified, will try to find it in the default steam locations.
    /// (It has to contain a steamapps folder)
    #[arg(long, short)]
    steam_path: Option<PathBuf>,

    /// Use local compat folder instead of the global one
    /// This is useful if you want to keep the game files locally
    #[arg(long, short, default_value_t)]
    local: bool,
}

fn main() {
    let pl = ProtonLaunch::parse();
    let paths = &pl.paths;
    let steam_data = pl
        .steam_path
        .map_or_else(SteamData::new, SteamData::new_with_path)
        .unwrap();

    let command = &pl.command;
    let res = command.run(paths, &steam_data);
    if let Err(e) = res {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
