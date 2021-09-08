use colored::Colorize;
use crate::error::{InstallerError, InstallerResult};
use crate::features::install_service::InstallServiceFeature;
use crate::features::feature_traits::Feature;

mod diagnostic;
mod arguments;
mod error;
mod installer;
mod service_wrapper;
mod features;

fn main() -> Result<(), InstallerError> {
    let argument = arguments::match_arguments().or_else(print_error_and_forward)?;
    println!("Will perform action: {} with the following arguments:", argument.action_type.as_str().cyan());
    println!("  Service name: {}", argument.service_name.as_str().cyan());

    let install_feature = InstallServiceFeature {};

    if argument.action_type == install_feature.get_sub_command_name() {
        install_feature.execute_service_feature(&argument)?;
    } else if &argument.action_type == arguments::UNINSTALL_WINDOWS_SERVICE {
        installer::uninstall_windows_service(&argument.service_name).or_else(print_error_and_forward)?;
    }

    InstallerResult::Ok(())
}

fn print_error_and_forward<T>(error: InstallerError) -> Result<T, InstallerError> {
    diagnostic::print_error(&error.message); Result::Err(error)
}
