use clap::Args;

use crate::{paths::Paths, proton::ProtonVersion, steam::SteamData};

use super::{Runnable, RunnableResult};

#[derive(Args, Debug, Clone)]
pub struct Info {
    /// Proton version to get info for
    version: Option<ProtonVersion>,
}

impl Runnable for Info {
    fn run(&self, _paths: &Paths, steam_data: &SteamData) -> RunnableResult<()> {
        let protons = if let Some(version) = self.version {
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
        Ok(())
    }
}
