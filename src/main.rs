mod cli;
use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();
    init_logger(&cli);

    log::info!("Starting app.");
    log::trace!("Hello World!");
    log::trace!("a trace example");
    log::debug!("deboogging");
    log::info!("such information");
    log::warn!("o_O");
    log::error!("boom");
    log::info!("{:#?}", cli);
    log::trace!("2 + 2 = 4? = {}", emcs_lib::add(2, 2) == 4);

    if cli.nogui {
        no_gui(cli);
    } else {
        gui(cli);
    }
}

fn no_gui(cli: Cli) {}
fn gui(cli: Cli) {}

fn init_logger(cli: &Cli) {
    use simplelog::*;
    use std::fs::File;
    let log_file = match home::home_dir() {
        Some(home_dir) => home_dir.join(".emcs.log"),
        None => {
            CombinedLogger::init(vec![TermLogger::new(
                LevelFilter::Trace,
                Config::default(),
                TerminalMode::Mixed,
                ColorChoice::Auto,
            )])
            .unwrap();

            log::error!("Failed to get home directory for logging!");
            std::process::exit(1)
        }
    };
    let log_level = if let Some(l) = cli.log_level {
        l
    } else {
        LevelFilter::Warn
    };
    CombinedLogger::init(vec![
        TermLogger::new(
            log_level,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Trace,
            Config::default(),
            File::create(log_file).unwrap(),
        ),
    ])
    .unwrap();
}
