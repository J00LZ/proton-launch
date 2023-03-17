use std::path::PathBuf;

use clap::Args;

use crate::{paths::Paths, proton::ProtonVersion, steam::SteamData};

use super::{Runnable, RunnableError, RunnableResult};

#[derive(Args, Debug, Clone)]
pub struct Run {
    /// Path to the game exe to run
    exe: PathBuf,

    /// Optional save name to use
    /// If not specified, the game exe without the extension will be used
    #[clap(short, long)]
    save_name: Option<String>,

    /// Optional proton version to use
    #[clap(short, long)]
    proton: Option<ProtonVersion>,
}

impl Runnable for Run {
    fn run(&self, paths: &Paths, steam_data: &SteamData) -> RunnableResult<()> {
        let selected_proton = self.proton.filter(|p| p.is_installed(steam_data));
        if let Some(handpicked) = self.proton {
            if selected_proton.is_none() {
                return Err(RunnableError::SelectedProtonNotInstalled(handpicked));
            }
        }
        let selected_proton = selected_proton.or_else(|| ProtonVersion::best_installed(steam_data));

        if let Some(selected) = selected_proton {
            let save_name = self
                .save_name
                .as_deref()
                .unwrap_or_else(|| self.exe.file_stem().unwrap().to_str().unwrap());
            let proton_path = selected.get_path(steam_data).expect("You somehow managed to delete the selected proton version while running this command");
            let proton_command = proton_path.join("proton");
            println!("Launching {} with {}", self.exe.display(), selected);

            let compat_dir = paths.compat_dir(save_name);

            let mut command = std::process::Command::new(proton_command);
            command.env("STEAM_COMPAT_CLIENT_INSTALL_PATH", &steam_data.path);
            command.env("STEAM_COMPAT_DATA_PATH", compat_dir);
            command.current_dir(paths.run_dir(save_name));
            command.arg("run");
            command.arg(&self.exe);

            let res = command.spawn().map_err(RunnableError::SpawnError)?.wait()?;
            println!("Exited with status {}", res);
            Ok(())
        } else {
            Err(RunnableError::NoProtonAtAll)
        }
    }
}
