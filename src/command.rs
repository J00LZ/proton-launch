use clap::Subcommand;

use crate::{paths::Paths, steam::SteamData};

mod runnable;
pub use runnable::*;

pub mod backup;
pub mod info;
pub mod install;
pub mod move_compat;
pub mod restore;
pub mod run;
pub mod uninstall;
pub mod desktop_entry;

#[derive(Subcommand)]
pub enum ProtonCommand {
    /// Run a game with proton
    Run(run::Run),

    /// Create a compat folder for a game
    MoveCompat(move_compat::MoveCompat),

    /// Get newly added files in the compat folder
    Backup(backup::Backup),

    /// Restore files from a backup
    Restore(restore::Restore),

    /// Install a proton version, will do nothing if it's already installed
    /// (or well that's what Steam seems to do)
    Install(install::Install),

    /// Uninstall a proton version, will do nothing if it's not installed
    /// (or well that's what Steam seems to do)
    Uninstall(uninstall::Uninstall),

    /// Get info about a proton version, or all versions if no version is specified
    Info(info::Info),
}

impl Runnable for ProtonCommand {
    fn run(&self, paths: &Paths, steam_data: &SteamData) -> RunnableResult<()> {
        match self {
            ProtonCommand::Run(r) => r.run(paths, steam_data),
            ProtonCommand::MoveCompat(m) => m.run(paths, steam_data),
            ProtonCommand::Backup(b) => b.run(paths, steam_data),
            ProtonCommand::Restore(r) => r.run(paths, steam_data),
            ProtonCommand::Install(i) => i.run(paths, steam_data),
            ProtonCommand::Uninstall(u) => u.run(paths, steam_data),
            ProtonCommand::Info(i) => i.run(paths, steam_data),
        }
    }
}
