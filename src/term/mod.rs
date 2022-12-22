use anyhow::{bail, Ok};

use crate::cli::Cli;

use anyhow::Context;
use emcs_lib::{install_mc_server, mc_versions::MCVersions, options::Options};
use inquire::{Confirm, Select, Text};
use strum::IntoEnumIterator;

pub fn term(cli: Cli) -> anyhow::Result<()> {
    if cli.yes {
        if cli.eula_accepted {
            let options = Options::new(
                cli.mc_version.with_context(|| "No mcversion specified.")?,
                cli.server_dir
                    .clone()
                    .with_context(|| "No server directory specified.")?,
            );
            install_mc_server(options)?;
        } else {
            bail!("Minecraft EULA not accepted")
        }
    }

    let options = check_exists_choices(cli)?;

    install_mc_server(options)?;

    Ok(())
}

fn check_exists_choices(cli: Cli) -> anyhow::Result<Options> {
    if !cli.eula_accepted {
        if !Confirm::new("Do you accept the Minecraft EULA?")
            .with_help_message("You can find the EULA at https://www.minecraft.net/en-us/eula")
            .with_default(false)
            .prompt()
            .with_context(|| "Failed to get acceptance of MC EULA.")?
        {
            bail!("Minecraft EULA not accepted")
        }
    }

    #[allow(unused_assignments)]
    let mut mcver = MCVersions::V1_12_2;
    if let None = cli.mc_version {
        mcver = Select::new("MCVER", MCVersions::iter().collect::<Vec<_>>())
            .prompt()
            .with_context(|| "Failed to get MCVER")?;
    } else {
        mcver = cli.mc_version.with_context(|| "No mcversion specified.")?;
    }

    #[allow(unused_assignments)]
    let mut dir = String::from("");
    if let None = cli.server_dir.clone() {
        dir = Text::new("Where do you want your server to live")
            .prompt()
            .with_context(|| "Failed to get SERVERDIR")?;
    } else {
        dir = cli.server_dir.with_context(|| "No serverdir specified.")?;
    }

    Ok(Options::new(mcver, dir))
}
