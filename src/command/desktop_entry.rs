use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

use clap::Args;
use exe::{Buffer, ResourceDirectory, VecPE};
use serde::{Deserialize, Serialize};

use crate::{paths::Paths, steam::SteamData};

use super::{Runnable, RunnableResult};

#[derive(Debug, Clone, Args)]
pub struct MakeDE {
    exe: PathBuf,
    name: String,
    save_name: Option<String>,
}

fn make_icon(exe: &Path, paths: &Paths, name: &str) -> PathBuf {
    let image = VecPE::from_disk_file(exe).unwrap();
    let res = ResourceDirectory::parse(&image).unwrap();
    let groups = res.icon_groups(&image).unwrap();
    let v = groups.values().into_iter().next().unwrap();
    let buf = v.to_icon_buffer(&image).unwrap();
    let img = image::load_from_memory(buf.as_slice()).unwrap();
    let img = img.resize(256, 256, image::imageops::FilterType::Lanczos3);
    let path = paths.icon_path(name);
    img.save(&path).unwrap();
    path
}

impl Runnable for MakeDE {
    fn run(&self, paths: &Paths, _steam_data: &SteamData) -> RunnableResult<()> {
        let save_name = self
            .save_name
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or_else(|| self.exe.file_stem().unwrap().to_str().unwrap());
        let mut de = DesktopEntry::new(self.name.clone());
        de.comment = format!("Run {} with Proton", self.name);
        de.exec = format!(
            "proton-launch run -s {save_name} {} ",
            self.exe.display()
        );
        de.path = paths.run_dir(save_name).display().to_string();
        let icon_path = make_icon(&self.exe, paths, &self.name);
        de.icon = icon_path.display().to_string();

        {
            let mut f = File::create(paths.application_entry(&self.name)).unwrap();
            write!(f, "[Desktop Entry]\n").unwrap();
            let mut s = serde_ini::Serializer::new(serde_ini::Writer::new(
                &mut f,
                serde_ini::LineEnding::Linefeed,
            ));
            de.serialize(&mut s).unwrap();
        }
        let command = Command::new("update-desktop-database")
            .arg(paths.application_entry(&self.name).parent().unwrap())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
        println!("update-desktop-database exited with {}", command);
        Ok(())
    }
}

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DesktopEntry {
    #[serde(rename = "Type")]
    xdg_type: String,
    version: String,
    name: String,
    comment: String,
    exec: String,
    path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    terminal: Option<String>,
    icon: String,
    generic_name: String,
    categories: String,
}

impl DesktopEntry {
    fn new(name: impl ToString) -> Self {
        Self {
            xdg_type: "Application".to_string(),
            version: "1.0".to_string(),
            name: name.to_string(),
            comment: "".to_string(),
            icon: name.to_string(),
            exec: "".to_string(),
            path: "".to_string(),
            terminal: None,
            generic_name: "Game".to_string(),
            categories: "Game".to_string(),
        }
    }
}
