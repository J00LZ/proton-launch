use std::{fmt::Display, path::PathBuf};

use clap::{builder::PossibleValue, Parser, Subcommand, ValueEnum};
use steamlocate::SteamDir;
use xdg::BaseDirectories;

#[derive(Debug, Clone)]
struct Paths {
    data_dir: PathBuf,
    config_dir: PathBuf,
}

impl Default for Paths {
    fn default() -> Self {
        let basedirs = BaseDirectories::new().unwrap();
        let data_dir = basedirs.create_data_directory("proton-launch").unwrap();
        let config_dir = basedirs.create_config_directory("proton-launch").unwrap();
        Self {
            data_dir,
            config_dir,
        }
    }
}

#[derive(Parser)]
#[command(name = "Proton-Launch")]
#[command(author = "Julius de Jeu")]
struct ProtonLaunch {
    #[command(subcommand)]
    command: ProtonCommand,
}

#[derive(Subcommand)]
enum ProtonCommand {
    /// Run a game with proton
    Run {
        /// Path to the game exe to run
        game: PathBuf,

        /// Optional save name to use
        /// If not specified, the game exe without the extension will be used
        #[clap(short, long)]
        save_name: Option<String>,

        /// Optional proton version to use
        #[clap(short, long)]
        proton: Option<ProtonVersion>,
    },
}

fn main() {
    let pl = ProtonLaunch::parse();

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum ProtonVersion {
    Proton37,
    Proton37Beta,
    Proton316,
    Proton316Beta,
    Proton42,
    Proton411,
    Proton50,
    Proton513,
    Proton63,
    Proton70,
    ProtonBattlEyeRuntime,
    ProtonEasyAntiCheatRuntime,
    ProtonExperimental,
}

impl ValueEnum for ProtonVersion {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            ProtonVersion::Proton37,
            ProtonVersion::Proton37Beta,
            ProtonVersion::Proton316,
            ProtonVersion::Proton316Beta,
            ProtonVersion::Proton42,
            ProtonVersion::Proton411,
            ProtonVersion::Proton50,
            ProtonVersion::Proton513,
            ProtonVersion::Proton63,
            ProtonVersion::Proton70,
            ProtonVersion::ProtonBattlEyeRuntime,
            ProtonVersion::ProtonEasyAntiCheatRuntime,
            ProtonVersion::ProtonExperimental,
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        let n = format!("{:?}", self);
        // hide all but installed proton versions
        let v = PossibleValue::new(n.to_lowercase()).hide(!self.is_installed().unwrap_or_default());
        Some(v)
    }
}

impl Display for ProtonVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtonVersion::Proton37 => write!(f, "Proton 3.7"),
            ProtonVersion::Proton37Beta => write!(f, "Proton 3.7 Beta"),
            ProtonVersion::Proton316 => write!(f, "Proton 3.16"),
            ProtonVersion::Proton316Beta => write!(f, "Proton 3.16 Beta"),
            ProtonVersion::Proton42 => write!(f, "Proton 4.2"),
            ProtonVersion::Proton411 => write!(f, "Proton 4.11"),
            ProtonVersion::Proton50 => write!(f, "Proton 5.0"),
            ProtonVersion::Proton513 => write!(f, "Proton 5.13"),
            ProtonVersion::Proton63 => write!(f, "Proton 6.3"),
            ProtonVersion::Proton70 => write!(f, "Proton 7.0"),
            ProtonVersion::ProtonBattlEyeRuntime => write!(f, "Proton BattlEye Runtime"),
            ProtonVersion::ProtonEasyAntiCheatRuntime => write!(f, "Proton EasyAntiCheat Runtime"),
            ProtonVersion::ProtonExperimental => write!(f, "Proton Experimental"),
        }
    }
}

impl ProtonVersion {
    fn get_appid(&self) -> u32 {
        match self {
            ProtonVersion::Proton37 => 858280,
            ProtonVersion::Proton37Beta => 930400,
            ProtonVersion::Proton316 => 961940,
            ProtonVersion::Proton316Beta => 996510,
            ProtonVersion::Proton42 => 1054830,
            ProtonVersion::Proton411 => 1113280,
            ProtonVersion::Proton50 => 1245040,
            ProtonVersion::Proton513 => 1420170,
            ProtonVersion::Proton63 => 1580130,
            ProtonVersion::Proton70 => 1887720,
            ProtonVersion::ProtonBattlEyeRuntime => 1161040,
            ProtonVersion::ProtonEasyAntiCheatRuntime => 1826330,
            ProtonVersion::ProtonExperimental => 1493710,
        }
    }

    fn all_versions() -> Vec<ProtonVersion> {
        vec![
            ProtonVersion::ProtonBattlEyeRuntime,
            ProtonVersion::ProtonEasyAntiCheatRuntime,
            ProtonVersion::Proton37,
            ProtonVersion::Proton37Beta,
            ProtonVersion::Proton316,
            ProtonVersion::Proton316Beta,
            ProtonVersion::Proton42,
            ProtonVersion::Proton411,
            ProtonVersion::Proton50,
            ProtonVersion::Proton513,
            ProtonVersion::Proton63,
            ProtonVersion::Proton70,
            ProtonVersion::ProtonExperimental,
        ]
    }

    fn install_url(&self) -> String {
        format!("steam://install/{}", self.get_appid())
    }

    fn is_installed(&self) -> Option<bool> {
        let mut steam = SteamDir::locate()?;
        let appid = self.get_appid();
        if let Some(app) = steam.app(&appid) {
            Some(app.path.exists())
        } else {
            Some(false)
        }
    }

    fn first_available() -> Option<Self> {
        let mut all = ProtonVersion::all_versions();
        all.reverse();
        all.into_iter()
            .find(|v| v.is_installed().unwrap_or_default())
    }
}
