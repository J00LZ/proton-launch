use std::{
    fs::File,
    path::{Path, PathBuf},
};

use clap::Args;

use crate::{paths::Paths, steam::SteamData};

use super::{Runnable, RunnableResult};

#[derive(Args, Debug, Clone)]
pub struct Restore {
    /// Path to the backup file
    backup: PathBuf,

    /// Optional save name to use
    /// If not specified, the backup file without the extension will be used
    #[clap(short, long)]
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
        let mut zip = zip::ZipArchive::new(f).unwrap();
        for i in 0..zip.len() {
            let mut file = zip.by_index(i).unwrap();
            let outpath = global_compat_dir.join(
                file.enclosed_name()
                    .map(Path::to_path_buf)
                    .unwrap_or(file.mangled_name()),
            );
            if (*file.name()).ends_with('/') {
                std::fs::create_dir_all(&outpath).unwrap();
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(p).unwrap();
                    }
                }
                let mut outfile = File::create(&outpath).unwrap();
                std::io::copy(&mut file, &mut outfile).unwrap();
            }
        }
        Ok(())
    }
}
