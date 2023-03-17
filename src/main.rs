use std::{
    collections::HashSet,
    fs::File,
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand, ValueEnum};

mod paths;
mod proton;
mod steam;

use exe::{Buffer, ResourceDirectory, VecPE};
use paths::Paths;
use proton::ProtonVersion;

use serde::{Deserialize, Serialize};
use xdgkit::desktop_entry::string_xdg_type;
use zip::write::FileOptions;

use crate::steam::SteamData;

#[derive(Parser)]
#[command(name = "proton-launch")]
#[command(author = "Julius de Jeu")]
struct ProtonLaunch {
    #[command(subcommand)]
    command: ProtonCommand,

    #[command(flatten)]
    paths: Paths,

    /// Path to the steam install folder.
    /// If not specified, will try to find it in the default steam locations.
    /// (It has to contain a steamapps folder)
    #[arg(long, short)]
    steam_path: Option<PathBuf>,

    /// Use local compat folder instead of the global one
    /// This is useful if you want to keep the game files locally
    #[arg(long, short, default_value_t)]
    local: bool,
}

#[derive(Debug, Clone, ValueEnum)]
enum MoveDirection {
    /// Move the compat folder from the global save folder to a local folder named `compat`
    GlobalToLocal,
    /// Move the compat folder from the local `compat` folder to the global save folder
    LocalToGlobal,
}

#[derive(Subcommand)]
enum ProtonCommand {
    /// Run a game with proton
    Run {
        /// Path to the game exe to run
        exe: PathBuf,

        /// Optional save name to use
        /// If not specified, the game exe without the extension will be used
        #[clap(short, long)]
        save_name: Option<String>,

        /// Optional proton version to use
        #[clap(short, long)]
        proton: Option<ProtonVersion>,

        /// If specified, will not add .exe to the filename provided
        #[clap(long)]
        raw: bool,
    },

    /// Create a compat folder for a game
    MoveCompat {
        /// Direction to move the compat folder
        direction: MoveDirection,

        /// Path to the game exe
        exe: PathBuf,

        /// Optional save name to use
        /// If not specified, the game exe without the extension will be used
        #[clap(short, long)]
        save_name: Option<String>,
    },

    /// Get newly added files in the compat folder
    Backup {
        /// Path to the game exe
        exe: PathBuf,

        /// Optional save name to use
        /// If not specified, the game exe without the extension will be used
        #[clap(short, long)]
        save_name: Option<String>,
    },

    /// Restore files from a backup
    Restore {
        /// Path to the .backup file
        backup: PathBuf,

        /// Optional save name to use
        /// If not specified, the backup file without the extension will be used
        #[clap(short, long)]
        save_name: Option<String>,
    },

    /// Install a proton version, will do nothing if it's already installed
    /// (or well that's what Steam seems to do)
    Install {
        /// Proton version to install. You probably want one of the first entries in the list
        ///
        version: ProtonVersion,
    },

    /// Uninstall a proton version, will do nothing if it's not installed
    /// (or well that's what Steam seems to do)
    Uninstall {
        /// Proton version to uninstall.  
        version: ProtonVersion,
    },

    /// Get info about a proton version, or all versions if no version is specified
    Info {
        /// Proton version to get info about
        version: Option<ProtonVersion>,
    },

    GetIcon {
        exe: PathBuf,
    },

    DesktopEntry {
        exe: PathBuf,
        name: String,
        save_name: Option<String>,
    },
}

fn main() {
    let pl = ProtonLaunch::parse();
    let paths = &pl.paths;
    let steam_data = pl
        .steam_path
        .map_or_else(|| SteamData::new(), SteamData::new_with_path)
        .unwrap();

    let command = &pl.command;
    match command {
        ProtonCommand::Run {
            exe,
            save_name,
            proton,
            raw,
        } => {
            let selected_proton = proton.filter(|p| p.is_installed(&steam_data));
            if let Some(handpicked) = proton {
                if !selected_proton.is_some() {
                    println!("Proton version {} is not installed, you can install it with `proton-launch install {}`", handpicked, handpicked.arg_name());
                }
            }
            let selected_proton =
                selected_proton.or_else(|| ProtonVersion::best_installed(&steam_data));

            if let Some(selected) = selected_proton {
                let save_name = save_name.as_ref().map(|s| s.as_str()).unwrap_or_else(|| {
                    if *raw {
                        exe.file_name().unwrap().to_str().unwrap()
                    } else {
                        exe.file_stem().unwrap().to_str().unwrap()
                    }
                });
                let proton_path = selected.get_path(&steam_data).expect("You somehow managed to delete the selected proton version while running this command");
                let proton_command = proton_path.join("proton");
                println!("Launching {} with {}", exe.display(), selected);

                let compat_dir = paths.compat_dir(&save_name);

                let mut command = std::process::Command::new(proton_command);
                command.env("STEAM_COMPAT_CLIENT_INSTALL_PATH", steam_data.path);
                command.env("STEAM_COMPAT_DATA_PATH", compat_dir);
                command.current_dir(paths.run_dir(&save_name));
                command.arg("run");
                command.arg(exe);

                let res = command.spawn().unwrap().wait().unwrap();
                println!("{}", res);
            } else {
                println!(
                    "No proton found, you can install one with `proton-launch install <version>`"
                );
            }
        }
        ProtonCommand::MoveCompat {
            direction,
            exe,
            save_name,
        } => {
            let save_name = save_name
                .as_ref()
                .map(|s| s.as_str())
                .unwrap_or_else(|| exe.file_stem().unwrap().to_str().unwrap());
            let global_compat_dir = paths.compat_dir(save_name);
            let local_compat_dir = exe.parent().unwrap().join("compat");
            println!("global exists: {}", global_compat_dir.exists());
            println!("local exists: {}", local_compat_dir.exists());
            match direction {
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
            }
        }
        ProtonCommand::Backup { exe, save_name } => {
            let save_name = save_name
                .as_ref()
                .map(|s| s.as_str())
                .unwrap_or_else(|| exe.file_stem().unwrap().to_str().unwrap());
            let global_compat_dir = paths.compat_dir(&save_name);
            let r = find_new_files(&global_compat_dir).unwrap();
            let f = File::create(format!("{}.backup", save_name)).unwrap();
            let mut zip = zip::ZipWriter::new(f);
            zip.set_comment(
                format!("Made with proton-launch {}", env!("CARGO_PKG_VERSION")).as_str(),
            );
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
        }
        ProtonCommand::Restore { backup, save_name } => {
            let save_name = save_name
                .as_ref()
                .map(|s| s.as_str())
                .unwrap_or_else(|| backup.file_stem().unwrap().to_str().unwrap());
            let global_compat_dir = paths.compat_dir(save_name);
            let f = File::open(backup).unwrap();
            let mut zip = zip::ZipArchive::new(f).unwrap();
            for i in 0..zip.len() {
                let mut file = zip.by_index(i).unwrap();
                let outpath = global_compat_dir.join(
                    file.enclosed_name()
                        .map(Path::to_path_buf)
                        .unwrap_or(file.mangled_name()),
                );
                if (&*file.name()).ends_with('/') {
                    std::fs::create_dir_all(&outpath).unwrap();
                } else {
                    if let Some(p) = outpath.parent() {
                        if !p.exists() {
                            std::fs::create_dir_all(&p).unwrap();
                        }
                    }
                    let mut outfile = File::create(&outpath).unwrap();
                    std::io::copy(&mut file, &mut outfile).unwrap();
                }
            }
        }
        ProtonCommand::Install { version: proton } => {
            let install_url = proton.install_url();
            open::that(install_url).unwrap();
        }
        ProtonCommand::Uninstall { version: proton } => {
            let uninstall_url = proton.uninstall_url();
            open::that(uninstall_url).unwrap();
        }
        ProtonCommand::Info { version } => {
            let protons = if let Some(version) = version {
                vec![version.clone()]
            } else {
                ProtonVersion::all()
            };
            for p in protons {
                println!("=== {} ===", p);
                println!("Install url: {}", p.install_url());
                println!("Uninstall url: {}", p.uninstall_url());
                println!("App id: {}", p.get_appid());
                let installed = steam_data.has_app(p.get_appid());
                println!("Installed: {}", installed);
                if installed {
                    let path = steam_data.get_app_dir(p.get_appid());
                    println!("Path: {:?}", path);
                }
                println!();
            }
        }
        ProtonCommand::GetIcon { exe } => {
            let image = VecPE::from_disk_file(exe).unwrap();
            let res = ResourceDirectory::parse(&image).unwrap();
            let groups = res.icon_groups(&image).unwrap();
            let v = groups.values().into_iter().next().unwrap();
            let buf = v.to_icon_buffer(&image).unwrap();
            let img = image::load_from_memory(buf.as_slice()).unwrap();
            let img = img.resize(256, 256, image::imageops::FilterType::Lanczos3);
            img.save("icon.png").unwrap();
        }
        ProtonCommand::DesktopEntry {
            exe,
            save_name,
            name,
        } => {
            let save_name = save_name
                .as_ref()
                .map(|s| s.as_str())
                .unwrap_or_else(|| exe.file_stem().unwrap().to_str().unwrap());
            // let mut de = DesktopEntry::default();
            // de.exec = Some(format!("proton-launch run {}", exe.display()));
            // de.name = Some(name.clone());
            // de.comment = Some(format!("Run {} with Proton", name));
            let mut de = DesktopEntry::new(name.clone());
            de.comment = format!("Run {} with Proton", name);
            de.exec = format!("proton-launch run {}", exe.display());
            let de = de_to_string(&de);
            println!("{}", de);

        }
    }
}

fn de_to_string(de: &DesktopEntry) -> String {
    serde_ini::ser::to_string(de).unwrap()
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
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<String>,
}

impl DesktopEntry {
    fn new(name: impl ToString) -> Self {
        Self {
            xdg_type: "Application".to_string(),
            version: "1.0".to_string(),
            name: name.to_string(),
            comment: "".to_string(),
            icon: None,
            exec: "".to_string(),
            path: "".to_string(),
            terminal: None,
        }
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
