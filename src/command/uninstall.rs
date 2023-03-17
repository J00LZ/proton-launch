use clap::Args;

use crate::{paths::Paths, proton::ProtonVersion, steam::SteamData};

use super::{Runnable, RunnableResult};

#[derive(Args, Debug, Clone)]
pub struct Uninstall {
    /// Proton version to uninstall
    version: ProtonVersion,
}

impl Runnable for Uninstall {
    fn run(&self, _paths: &Paths, _steam_data: &SteamData) -> RunnableResult<()> {
        let uninstall_url = self.version.uninstall_url();
        open::that(uninstall_url).unwrap();
        Ok(())
    }
}
