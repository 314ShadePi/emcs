use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(super) struct Cli {
    /// Accept EULA (optional)
    #[arg(short, long, value_name = "TRUE|FALSE")]
    pub eula: Option<bool>,

    /// Specify Minecraft version (optional)
    #[arg(short, long, value_name = "VERSION")]
    pub mcversion: Option<String>,

    /// Specify target directory (optional)
    #[arg(short, long, value_name = "DIRECTORY")]
    pub directory: Option<String>,
}
