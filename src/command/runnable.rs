use thiserror::Error;

use crate::{paths::Paths, proton::ProtonVersion, steam::SteamData};

#[derive(Debug, Error)]
pub enum RunnableError {
    #[error("No proton found, you can install one with `proton-launch install <version>`")]
    NoProtonAtAll,
    #[error("{} is not installed, you can install it with `proton-launch install {}`", .0, .0.arg_name())]
    SelectedProtonNotInstalled(ProtonVersion),

    #[error("Failed to spawn process: {}", .0)]
    SpawnError(std::io::Error),

    #[error("Generic IO error: {}", .0)]
    IOError(#[from] std::io::Error),
}

pub type RunnableResult<O> = Result<O, RunnableError>;

pub trait Runnable {
    fn run(&self, paths: &Paths, steam_data: &SteamData) -> RunnableResult<()>;
}
