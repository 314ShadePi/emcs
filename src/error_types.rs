use error_stack::Context;
use std::fmt;

#[derive(Debug)]
pub enum JavaError {
    NotInstalledError,
}

impl fmt::Display for JavaError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JavaError::NotInstalledError => fmt.write_str("Java not installed."),
        }
    }
}

impl Context for JavaError {}

#[derive(Debug)]
pub enum EulaAcceptError {
    EulaNotAcceptedError,
    UserPromptError,
}

impl fmt::Display for EulaAcceptError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EulaAcceptError::EulaNotAcceptedError => fmt.write_str("EULA not accepted."),
            EulaAcceptError::UserPromptError => fmt.write_str("User prompt failed."),
        }
    }
}

impl Context for EulaAcceptError {}

#[derive(Debug)]
pub enum ServerCreationError {
    ServerDownloadError,
    ServerInitError,
    EulaModError,
}

impl fmt::Display for ServerCreationError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerCreationError::ServerDownloadError => fmt.write_str("Could not download server."),
            ServerCreationError::ServerInitError => fmt.write_str("Could not initialize server."),
            ServerCreationError::EulaModError => fmt.write_str("Failed to modify EULA."),
        }
    }
}

impl Context for ServerCreationError {}

#[derive(Debug)]
pub enum VersionError {
    UserPromptError,
    InvalidVersionError,
}

impl fmt::Display for VersionError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VersionError::UserPromptError => fmt.write_str("User prompt failed."),
            VersionError::InvalidVersionError => fmt.write_str("Invalid version."),
        }
    }
}

impl Context for VersionError {}

#[derive(Debug)]
pub enum DirectoryError {
    UserPromptError,
    EmptyNameError,
    CreationError,
}

impl fmt::Display for DirectoryError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DirectoryError::UserPromptError => fmt.write_str("User prompt failed."),
            DirectoryError::EmptyNameError => fmt.write_str("Empty name."),
            DirectoryError::CreationError => fmt.write_str("Failed to create directory."),
        }
    }
}

impl Context for DirectoryError {}

#[derive(Debug)]
pub enum ServerDownloadError {
    RequestError,
    FileCreationError,
    ResponseBytesError,
    ContentCopyError,
}

impl fmt::Display for ServerDownloadError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerDownloadError::RequestError => fmt.write_str("Failed to get resource."),
            ServerDownloadError::FileCreationError => {
                fmt.write_str("Failed to create server file.")
            }
            ServerDownloadError::ResponseBytesError => {
                fmt.write_str("Failed to get response bytes.")
            }
            ServerDownloadError::ContentCopyError => fmt.write_str("Failed to copy content."),
        }
    }
}

impl Context for ServerDownloadError {}

#[derive(Debug)]
pub struct ServerInitError;

impl fmt::Display for ServerInitError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Failed to run server.")
    }
}

impl Context for ServerInitError {}

#[derive(Debug)]
pub enum EulaModError {
    FileOpenError,
    FileReadError,
    FileCreateError,
    FileWriteError,
}

impl fmt::Display for EulaModError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EulaModError::FileOpenError => fmt.write_str("Could not open file."),
            EulaModError::FileReadError => fmt.write_str("Could not read from file."),
            EulaModError::FileCreateError => fmt.write_str("Could not create file."),
            EulaModError::FileWriteError => fmt.write_str("Could not write to file."),
        }
    }
}

impl Context for EulaModError {}

#[derive(Debug)]
pub enum ServerRunError {
    SpawnError,
    StdoutError,
}

impl fmt::Display for ServerRunError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerRunError::SpawnError => fmt.write_str("Failed to spawn server."),
            ServerRunError::StdoutError => fmt.write_str("Could not capture stdout."),
        }
    }
}

impl Context for ServerRunError {}

#[derive(Debug)]
pub enum StartFileCreationError {
    FileCreateError,
    FileWriteError,
}

impl fmt::Display for StartFileCreationError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StartFileCreationError::FileCreateError => fmt.write_str("Could not create file."),
            StartFileCreationError::FileWriteError => fmt.write_str("Could not write to file."),
        }
    }
}

impl Context for StartFileCreationError {}

#[derive(Debug)]
pub enum InputError {
    UserPromptError,
    ArgError,
    DirectoryError,
}

impl fmt::Display for InputError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputError::UserPromptError => fmt.write_str("Failed to get/verify user input."),
            InputError::ArgError => fmt.write_str("Failed to get/verify CLI args."),
            InputError::DirectoryError => fmt.write_str("Failed to create directory."),
        }
    }
}

impl Context for InputError {}

#[derive(Debug)]
pub struct InstallError;

impl fmt::Display for InstallError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Installation failed.")
    }
}

impl Context for InstallError {}
