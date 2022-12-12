use clap::Parser;
use log::LevelFilter;

#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Toggle gui
    #[arg(short, long)]
    pub nogui: bool,

    /// Set log level. Valid options are: Off, Error, Warn, Info, Debug, Trace. Defaults to Warn.
    #[arg(short, long)]
    pub log_level: Option<LevelFilter>,
}
