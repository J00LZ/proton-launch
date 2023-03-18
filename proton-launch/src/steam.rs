use std::{fs::read_to_string, path::PathBuf};

use keyvalues_parser::Vdf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum SteamDataError {
    #[error("Could not locate Steam installation directory")]
    NoSteamDir,
    #[error("Could not read libraryfolders.vdf")]
    NoLibraryFolders,
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("VDF parse error: {0}")]
    KVParser(#[from] Box<keyvalues_parser::error::Error>),
}

type SteamResult<T> = Result<T, SteamDataError>;

pub struct SteamData {
    library_folders: LibraryFolders,
    pub path: PathBuf,
}

impl SteamData {
    pub fn new() -> SteamResult<Self> {
        let dir = Self::locate()?;
        Self::new_with_path(dir)
    }

    pub fn new_with_path(path: PathBuf) -> SteamResult<Self> {
        let mut sd = Self {
            library_folders: LibraryFolders::EMPTY,
            path,
        };
        sd.init_library_paths()?;

        Ok(sd)
    }

    fn init_library_paths(&mut self) -> SteamResult<()> {
        let library_folders_path = self.path.join("steamapps/libraryfolders.vdf");
        if library_folders_path.is_file() {
            let content = read_to_string(library_folders_path)?;
            self.library_folders = LibraryFolders::from_vdf(&content)?;
            Ok(())
        } else {
            Err(SteamDataError::NoLibraryFolders)
        }
    }

    /// Locates the Steam installation directory on the filesystem and initializes a `SteamDir` (Linux)
    ///
    /// Returns `None` if no Steam installation can be located.
    fn locate() -> SteamResult<PathBuf> {
        let home_dir = dirs::home_dir().ok_or(SteamDataError::NoSteamDir)?;

        // Steam's installation location is pretty easy to find on Linux, too, thanks to the symlink in $USER

        // Check for Flatpak steam install
        let steam_flatpak_path = home_dir.join(".var/app/com.valvesoftware.Steam");
        if steam_flatpak_path.is_dir() {
            let steam_flatpak_install_path = steam_flatpak_path.join(".steam/steam");
            if steam_flatpak_install_path.is_dir() {
                return Ok(steam_flatpak_install_path);
            }
        }

        // Check for Standard steam install
        let standard_path = home_dir.join(".steam/steam");
        if standard_path.is_dir() {
            return Ok(standard_path);
        }

        Err(SteamDataError::NoSteamDir)
    }

    pub fn has_app(&self, app_id: u64) -> bool {
        self.library_folders.has_app(app_id)
    }

    pub fn get_app_dir(&self, app_id: u64) -> Option<PathBuf> {
        self.library_folders.get_app_dir(app_id)
    }
}

#[derive(Debug, Clone)]
pub struct LibraryFolders(Vec<LibraryFolder>);

impl LibraryFolders {
    const EMPTY: Self = Self(Vec::new());

    pub fn from_vdf(vdf: &str) -> SteamResult<Self> {
        let vdf = Vdf::parse(vdf).map_err(Box::new)?.value;
        let obj = vdf.get_obj().ok_or(SteamDataError::NoLibraryFolders)?;

        let folders: Vec<_> = obj
            .iter()
            .filter(|(key, values)| key.parse::<u32>().is_ok() && values.len() == 1)
            .filter_map(|(_, values)| {
                let lfo = values.get(0)?.get_obj()?;
                let library_folder_string = lfo.get("path")?.get(0)?.get_str()?.to_string();
                let apps = lfo
                    .get("apps")?
                    .iter()
                    .flat_map(|v| v.get_obj())
                    .flat_map(|o| o.keys())
                    .filter_map(|k| k.parse::<u64>().ok())
                    .collect::<Vec<_>>();
                let library_folder = PathBuf::from(library_folder_string).join("steamapps");
                Some(LibraryFolder {
                    path: library_folder,
                    apps,
                })
            })
            .collect();

        Ok(Self(folders))
    }

    pub fn has_app(&self, appid: u64) -> bool {
        self.0.iter().any(|lf| lf.has_game(appid))
    }

    pub fn get_app_dir(&self, app_id: u64) -> Option<PathBuf> {
        let library = self.0.iter().find(|lf| lf.has_game(app_id))?;
        let manifest_location = library.path.join(format!("appmanifest_{}.acf", app_id));
        if manifest_location.is_file() {
            let manifest = read_to_string(manifest_location).ok()?;
            let vdf = Vdf::parse(&manifest).ok()?;
            let obj = vdf.value.get_obj()?;
            let install_dir = obj.get("installdir")?.get(0)?.get_str()?;
            return Some(library.path.join("common").join(install_dir));
        }

        None
    }
}

#[derive(Debug, Clone)]
pub struct LibraryFolder {
    path: PathBuf,
    apps: Vec<u64>,
}

impl LibraryFolder {
    fn has_game(&self, appid: u64) -> bool {
        self.apps.contains(&appid)
    }
}
