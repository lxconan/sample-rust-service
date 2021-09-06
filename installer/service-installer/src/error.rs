use std::fmt;

#[derive(Debug)]
pub struct InstallerError {
    pub message: String,
    exit_code: i32
}

impl InstallerError {
    pub fn new<M: Into<String>, C: Into<i32>>(message:M, exit_code:C) -> InstallerError {
        InstallerError {
            message: message.into(),
            exit_code: exit_code.into()
        }
    }

    pub fn with<T: std::error::Error, C: Into<i32>>(error: T, message: &str, exit_code: C) -> InstallerError {
        InstallerError {
            message: format!("{} -> {}", message, error),
            exit_code: exit_code.into()
        }
    }
}

impl Into<i32> for InstallerError {
    fn into(self) -> i32 {
        self.exit_code
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