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
    VersionError,
    DirectoryError,
    ServerDownloadError,
    ServerRunError,
    EulaModError,
}

impl fmt::Display for ServerCreationError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerCreationError::VersionError => todo!(),
            ServerCreationError::DirectoryError => todo!(),
            ServerCreationError::ServerDownloadError => todo!(),
            ServerCreationError::ServerRunError => todo!(),
            ServerCreationError::EulaModError => todo!(),
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
pub struct ServerCreateFilesRunError;

impl fmt::Display for ServerCreateFilesRunError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Failed to run server.")
    }
}

impl Context for ServerCreateFilesRunError {}

#[derive(Debug)]
pub enum EulaModError {
    FileOpenError,
    FileReadError,
    FileCreateError,
    FileWríteError,
}

impl fmt::Display for EulaModError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EulaModError::FileOpenError => fmt.write_str("Could not open file."),
            EulaModError::FileReadError => fmt.write_str("Could not read from file."),
            EulaModError::FileCreateError => fmt.write_str("Could not create file."),
            EulaModError::FileWríteError => fmt.write_str("Could not write to file."),
        }
    }
}

impl Context for EulaModError {}

#[derive(Debug)]
pub struct InstallError;

impl fmt::Display for InstallError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Installation failed.")
    }
}

impl Context for InstallError {}
