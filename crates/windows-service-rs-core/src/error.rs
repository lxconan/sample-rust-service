use std::fmt;
use std::fmt::{Debug, Formatter};

pub struct ServiceError { pub message: String }

impl ServiceError {
    pub fn new<M: Into<String>>(message:M) -> ServiceError {
        ServiceError { message: message.into() }
    }

    pub fn with<T: Debug>(error: T, message: &str) -> ServiceError {
        ServiceError { message: format!("{} -> {:?}", message, error), }
    }
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Debug for ServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ServiceError {
    fn description(&self) -> &str {
        &self.message
    }
}

pub type ServiceResult<T> = Result<T, ServiceError>;