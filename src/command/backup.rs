use std::{
    collections::HashSet,
    fs::File,
    path::{Path, PathBuf},
};

use clap::Args;
use zip::write::FileOptions;

use crate::{paths::Paths, steam::SteamData};

use super::{Runnable, RunnableResult};

#[derive(Args, Debug, Clone)]
pub struct Backup {
    /// Path to the game exe
    exe: PathBuf,

    /// Optional save name to use
    /// If not specified, the game exe without the extension will be used
    #[clap(short, long)]
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
        let mut zip = zip::ZipWriter::new(f);
        zip.set_comment(format!("Made with proton-launch {}", env!("CARGO_PKG_VERSION")).as_str());
        for f in r {
            let path = f
                .strip_prefix(&global_compat_dir)
                .unwrap()
                .to_string_lossy();
            let meta = f.metadata().unwrap();
            let last_modified = meta.modified().unwrap();
            let offset_datetime = time::OffsetDateTime::from(last_modified);
            let options =
                FileOptions::default().last_modified_time(offset_datetime.try_into().unwrap());
            zip.start_file(path, options).unwrap();
            let mut file = File::open(f).unwrap();
            std::io::copy(&mut file, &mut zip).unwrap();
        }
        zip.finish().unwrap();
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
