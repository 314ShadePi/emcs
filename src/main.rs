use std::fs::File;

use simplelog::*;

fn main() {
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
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Trace,
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

    log::info!("Starting app.");
    log::trace!("Hello World!");
    log::trace!("a trace example");
    log::debug!("deboogging");
    log::info!("such information");
    log::warn!("o_O");
    log::error!("boom");
}
