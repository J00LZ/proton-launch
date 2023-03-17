use std::{fmt::Display, path::PathBuf};

use clap::ValueEnum;

use crate::steam::SteamData;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProtonVersion {
    Proton37Beta,
    Proton37,
    Proton316Beta,
    Proton316,
    Proton42,
    Proton411,
    Proton50,
    Proton513,
    Proton63,
    Proton70,
    ProtonNext,
    ProtonExperimental,
}

impl ValueEnum for ProtonVersion {
    fn value_variants<'a>() -> &'a [Self] {
        use ProtonVersion::*;
        &[
            ProtonExperimental,
            ProtonNext,
            Proton70,
            Proton63,
            Proton513,
            Proton50,
            Proton411,
            Proton42,
            Proton316,
            Proton316Beta,
            Proton37,
            Proton37Beta,
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(clap::builder::PossibleValue::new(self.arg_name()))
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
            ProtonVersion::ProtonExperimental => write!(f, "Proton Experimental"),
            ProtonVersion::ProtonNext => write!(f, "Proton Next"),
        }
    }
}

impl ProtonVersion {
    pub fn get_appid(&self) -> u64 {
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
            ProtonVersion::ProtonExperimental => 1493710,
            ProtonVersion::ProtonNext => 2230260,
        }
    }

    pub fn all() -> Vec<Self> {
        use ProtonVersion::*;
        vec![
            ProtonExperimental,
            ProtonNext,
            Proton70,
            Proton63,
            Proton513,
            Proton50,
            Proton411,
            Proton42,
            Proton316,
            Proton316Beta,
            Proton37,
            Proton37Beta,
        ]
    }

    pub fn install_url(&self) -> String {
        format!("steam://install/{}", self.get_appid())
    }

    pub fn uninstall_url(&self) -> String {
        format!("steam://uninstall/{}", self.get_appid())
    }

    pub fn best_installed(steam: &SteamData) -> Option<Self> {
        Self::all().into_iter().find(|p| p.is_installed(steam))
    }

    pub fn is_installed(&self, steam: &SteamData) -> bool {
        steam.has_app(self.get_appid())
    }

    pub fn arg_name(&self) -> String {
        format!("{:?}", self).to_lowercase().replace("proton", "")
    }

    pub fn get_path(&self, steam_data: &SteamData) -> Option<PathBuf> {
        steam_data.get_app_dir(self.get_appid())
    }
}
