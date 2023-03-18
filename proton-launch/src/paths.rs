use std::{fmt::Display, ops::Deref, path::PathBuf, str::FromStr};

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

impl DataDir {
    pub fn compat_dir(&self, app_id: &str) -> PathBuf {
        let compat_dir = self.0.join("compat").join(app_id);
        std::fs::create_dir_all(&compat_dir).unwrap();
        compat_dir
    }

    pub fn run_dir(&self, app_id: &str) -> PathBuf {
        let run_dir = self.0.join("run").join(app_id);
        std::fs::create_dir_all(&run_dir).unwrap();
        run_dir
    }

    pub fn icon_path(&self, app_id: &str) -> PathBuf {
        let icons_dir = self.0.join("icons");
        std::fs::create_dir_all(&icons_dir).unwrap();
        icons_dir.join(format!("{}.png", app_id))
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
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "commandline", derive(clap::Args))]
pub struct Paths {
    /// The directory to store the `compat` folders in
    #[cfg_attr(feature = "commandline", arg(short, long, default_value_t))]
    data_dir: DataDir,
    /// The directory to store the `proton-launch` config in.
    /// This is both Global and Game specific config
    #[cfg_attr(feature = "commandline", arg(short, long, default_value_t))]
    config_dir: ConfigDir,
}

impl Paths {
    pub fn compat_dir(&self, app_id: &str) -> PathBuf {
        self.data_dir.compat_dir(app_id)
    }

    pub fn run_dir(&self, app_id: &str) -> PathBuf {
        self.data_dir.run_dir(app_id)
    }

    pub fn icon_path(&self, app_id: &str) -> PathBuf {
        self.data_dir.icon_path(app_id)
    }

    pub fn application_entry(&self, app_id: &str) -> PathBuf {
        let mut path = dirs::data_dir().unwrap().join("applications");
        std::fs::create_dir_all(&path).unwrap();
        path.push(format!("proton-{}.desktop", app_id));
        path
    }
}
