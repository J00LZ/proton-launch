use clap::Args;

use crate::{paths::Paths, proton::ProtonVersion, steam::SteamData};

use super::{Runnable, RunnableResult};

#[derive(Args, Debug, Clone)]
pub struct Install {
    /// Proton version to install
    version: ProtonVersion,
}

impl Runnable for Install {
    fn run(&self, _paths: &Paths, _steam_data: &SteamData) -> RunnableResult<()> {
        let install_url = self.version.install_url();
        open::that(install_url).unwrap();
        Ok(())
    }
}
