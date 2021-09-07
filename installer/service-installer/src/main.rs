use colored::Colorize;
use path_absolutize::Absolutize;
use crate::error::{InstallerError, InstallerResult};

mod diagnostic;
mod arguments;
mod error;
mod installer;
mod service_wrapper;

fn main() -> Result<(), InstallerError> {
    let argument = arguments::match_arguments().or_else(print_error_and_forward)?;
    println!("Will perform action: {} with the following arguments:", argument.action_type.as_str().cyan());
    println!("  Service name: {}", argument.service_name.as_str().cyan());

    if &argument.action_type == arguments::INSTALL_WINDOWS_SERVICE {
        let service_path = get_service_path(&argument).or_else(print_error_and_forward)?;
        println!("  Service executable path: {}", service_path.as_str().cyan());

        installer::install_windows_service(
            &argument.service_name,
            &argument.display_name,
            &argument.description,
            argument.auto_start,
            &service_path
        ).or_else(print_error_and_forward)?;
    } else if &argument.action_type == arguments::UNINSTALL_WINDOWS_SERVICE {
        installer::uninstall_windows_service(&argument.service_name).or_else(print_error_and_forward)?;
    }

    InstallerResult::Ok(())
}

fn print_error_and_forward<T>(error: InstallerError) -> Result<T, InstallerError> {
    diagnostic::print_error(&error.message); Result::Err(error)
}

fn get_service_path(argument: &arguments::Argument) -> Result<String, InstallerError> {
    let absolute_path_result = std::path::Path::new(&argument.executable_path).absolutize();
    let absolute_path = absolute_path_result.map_err(|e| { InstallerError::with(e,"Fail to get absolute path. ") })?;
    if absolute_path.exists() {
        return Result::Ok(String::from(absolute_path.to_str().unwrap()));
    }

    let error_message = format!("The path {} does not exist.", &argument.executable_path);
    return Result::Err(InstallerError::new(error_message));
}
