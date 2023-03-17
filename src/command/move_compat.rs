use std::path::{PathBuf, Path};

use clap::{Args, ValueEnum};

use crate::{paths::Paths, steam::SteamData};

use super::{Runnable, RunnableResult};

#[derive(Debug, Clone, ValueEnum)]
enum MoveDirection {
    /// Move the compat folder from the global save folder to a local folder named `compat`
    GlobalToLocal,
    /// Move the compat folder from the local `compat` folder to the global save folder
    LocalToGlobal,
}

#[derive(Args, Debug, Clone)]
pub struct MoveCompat {
    /// Direction to move the compat folder
    direction: MoveDirection,

    /// Path to the game exe
    exe: PathBuf,

    /// Optional save name to use
    /// If not specified, the game exe without the extension will be used
    #[clap(short, long)]
    save_name: Option<String>,
}

impl Runnable for MoveCompat {
    fn run(&self, paths: &Paths, _steam_data: &SteamData) -> RunnableResult<()> {
        let save_name = self
            .save_name
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or_else(|| self.exe.file_stem().unwrap().to_str().unwrap());
        let global_compat_dir = paths.data_dir.join(save_name);
        let local_compat_dir = self.exe.parent().unwrap().join("compat");
        println!("global exists: {}", global_compat_dir.exists());
        println!("local exists: {}", local_compat_dir.exists());
        match self.direction {
            MoveDirection::GlobalToLocal => {
                println!(
                    "Moving compat folder from {} to {}",
                    global_compat_dir.display(),
                    local_compat_dir.display()
                );
                copy_file_tree(&global_compat_dir, &local_compat_dir).unwrap();
            }
            MoveDirection::LocalToGlobal => {
                println!(
                    "Moving compat folder from {} to {}",
                    local_compat_dir.display(),
                    global_compat_dir.display()
                );
                copy_file_tree(&local_compat_dir, &global_compat_dir).unwrap();
            }
        };
        Ok(())
    }
}

fn copy_file_tree(source: &Path, dest: &Path) -> Result<(), std::io::Error> {
    std::fs::create_dir_all(&dest)?;
    for entry in walkdir::WalkDir::new(&source) {
        let entry = entry?;
        let path = entry.path().strip_prefix(&source).unwrap();
        if entry.file_type().is_dir() {
            std::fs::create_dir_all(&dest.join(path))?;
        } else if entry.file_type().is_file() {
            std::fs::copy(&entry.path(), &dest.join(path))?;
        }
    }

    Ok(())
}
