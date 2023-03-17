use std::{fmt::Display, ops::Deref, path::PathBuf, str::FromStr};

use clap::Args;
use xdg::BaseDirectories;

#[derive(Debug, Clone)]
pub struct DataDir(PathBuf);

impl FromStr for DataDir {
    type Err = <PathBuf as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(PathBuf::from_str(s)?))
    }
}

impl Default for DataDir {
    fn default() -> Self {
        let basedirs = BaseDirectories::new().unwrap();
        let data_dir = basedirs.create_data_directory("proton-launch").unwrap();
        Self(data_dir)
    }
}

impl Display for DataDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

impl Deref for DataDir {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct ConfigDir(PathBuf);

impl FromStr for ConfigDir {
    type Err = <PathBuf as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(PathBuf::from_str(s)?))
    }
}

impl Default for ConfigDir {
    fn default() -> Self {
        let basedirs = BaseDirectories::new().unwrap();
        let config_dir = basedirs.create_config_directory("proton-launch").unwrap();
        Self(config_dir)
    }
}

impl Display for ConfigDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

impl Deref for ConfigDir {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// The data paths to be used by the application
#[derive(Debug, Clone, Args, Default)]
pub struct Paths {
    /// The directory to store the `compat` folders in
    #[arg(short, long, default_value_t)]
    pub data_dir: DataDir,
    /// The directory to store the `proton-launch` config in.
    /// This is both Global and Game specific config
    #[arg(short, long, default_value_t)]
    pub config_dir: ConfigDir,
}
