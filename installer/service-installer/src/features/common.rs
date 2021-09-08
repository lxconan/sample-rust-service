use crate::error::{InstallerError, InstallerResult};
use crate::diagnostic::print_error;

pub fn print_error_and_forward<T>(error: InstallerError) -> InstallerResult<T> {
    print_error(&error.message); Result::Err(error)
}