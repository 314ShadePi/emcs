use clap::Parser;
use emcs_lib::mc_versions::MCVersions;
use log::LevelFilter;

#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Toggle gui
    #[arg(long)]
    pub nogui: bool,

    /// Set log level. Valid options are: Off, Error, Warn, Info, Debug, Trace. Optional with Warn as default.
    #[arg(short, long)]
    pub log_level: Option<LevelFilter>,

    /// Set desired Minecraft version. Valid options are 1.12.2, 1.13, 1.13.1 and up. Optional with ask user as default.
    #[arg(short, long)]
    pub mc_version: Option<MCVersions>,

    /// Accept the Minecraft EULA. Optional with ask user as default.
    #[arg(short, long)]
    pub eula_accepted: bool,

    /// Set the directory where you want to install the server. Optional with ask user as default.
    #[arg(short, long)]
    pub server_dir: Option<String>,

    /// Don't confirm cli choices when in nogui mode. Assumes all values were passed in from cli.
    #[arg(short)]
    pub yes: bool,
}
