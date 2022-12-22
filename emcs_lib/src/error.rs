use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InstallServerError {
    #[error("IO fail.")]
    IO {
        #[from]
        source: io::Error,
    },

    #[error("Downloading server failed.")]
    ServerDownload {
        #[from]
        source: reqwest::Error,
    },
}
