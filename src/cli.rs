use clap::Parser;
use emcs_lib::mc_versions::MCVersions;
use log::LevelFilter;

#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Toggle gui
    #[arg(short, long)]
    pub nogui: bool,

    /// Set log level. Valid options are: Off, Error, Warn, Info, Debug, Trace. Optional with Warn as default.
    #[arg(short, long)]
    pub log_level: Option<LevelFilter>,

    /// Set desired Minecraft version. Valid options are 1.12.2, 1.13, 1.13.1 and up. Optional with None as default.
    #[arg(short, long)]
    pub mc_version: Option<MCVersions>,
}
