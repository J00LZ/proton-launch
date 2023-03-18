use std::{
    collections::HashSet,
    fs::File,
    path::{Path, PathBuf},
};

use crate::{paths::Paths, steam::SteamData};

use super::{Runnable, RunnableResult};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "commandline", derive(clap::Args))]
pub struct Backup {
    /// Path to the game exe
    exe: PathBuf,

    /// Optional save name to use
    /// If not specified, the game exe without the extension will be used
    #[cfg_attr(feature = "commandline", clap(short, long))]
    save_name: Option<String>,
}

impl Runnable for Backup {
    fn run(&self, paths: &Paths, _steam_data: &SteamData) -> RunnableResult<()> {
        let save_name = self
            .save_name
            .as_deref()
            .unwrap_or_else(|| self.exe.file_stem().unwrap().to_str().unwrap());
        let global_compat_dir = paths.compat_dir(save_name);
        let r = find_new_files(&global_compat_dir).unwrap();
        let f = File::create(format!("{}.backup", save_name)).unwrap();
        let w = zstd::Encoder::new(f, 3).unwrap().auto_finish();
        let mut t = tar::Builder::new(w);
        for f in r {
            let path = f.strip_prefix(&global_compat_dir).unwrap();
            t.append_path_with_name(&f, path).unwrap();
        }
        t.finish().unwrap();
        Ok(())
    }
}

fn find_new_files(compat_dir: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut v = Vec::new();
    let tracked_files = compat_dir.join("tracked_files");
    let tracked_files = std::fs::read_to_string(tracked_files)?;
    let tracked_files: HashSet<&str> = tracked_files.lines().collect();
    let prefix = compat_dir.join("pfx");

    for entry in walkdir::WalkDir::new(&prefix) {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type().is_file() {
            let path = path.strip_prefix(&prefix).unwrap();
            if !tracked_files.contains(path.to_str().unwrap())
                && path.to_string_lossy().contains("users")
            {
                v.push(entry.path().to_path_buf());
            }
        }
    }

    Ok(v)
}
