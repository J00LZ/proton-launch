use std::{
    fs::File,
    path::PathBuf,
};

use crate::{paths::Paths, steam::SteamData};

use super::{Runnable, RunnableResult};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "commandline", derive(clap::Args))]
pub struct Restore {
    /// Path to the backup file
    backup: PathBuf,

    /// Optional save name to use
    /// If not specified, the backup file without the extension will be used
    #[cfg_attr(feature = "commandline", clap(short, long))]
    save_name: Option<String>,
}

impl Runnable for Restore {
    fn run(&self, paths: &Paths, _steam_data: &SteamData) -> RunnableResult<()> {
        let save_name = self
            .save_name
            .as_deref()
            .unwrap_or_else(|| self.backup.file_stem().unwrap().to_str().unwrap());
        let global_compat_dir = paths.compat_dir(save_name);
        let f = File::open(&self.backup).unwrap();
        let d = zstd::Decoder::new(f).unwrap();
        let mut archive = tar::Archive::new(d);
        archive.unpack(&global_compat_dir).unwrap();
        Ok(())
    }
}
