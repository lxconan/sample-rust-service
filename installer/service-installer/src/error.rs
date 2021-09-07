use std::fmt;

#[derive(Debug)]
pub struct InstallerError { pub message: String }

impl InstallerError {
    pub fn new<M: Into<String>>(message:M) -> InstallerError {
        InstallerError { message: message.into() }
    }

    pub fn with<T: std::error::Error>(error: T, message: &str) -> InstallerError {
        InstallerError { message: format!("{} -> {:?}", message, error), }
    }
}

impl fmt::Display for InstallerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.message.fmt(f)
    }
}

impl std::error::Error for InstallerError {
    fn description(&self) -> &str {
        &self.message
    }
}

pub type InstallerResult<T> = Result<T, InstallerError>;