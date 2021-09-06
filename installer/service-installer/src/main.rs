use clap::{Arg, App, ArgMatches, SubCommand, AppSettings};
use colored::Colorize;
use path_absolutize::Absolutize;
use crate::error::{InstallerError, InstallerResult};

mod diagnostic;
mod action_type;
mod error;

fn main() -> Result<(), InstallerError> {
    let argument = match_arguments().or_else(print_error_and_forward)?;
    let service_path = get_service_path(&argument).or_else(print_error_and_forward)?;

    println!("Will perform action: {} with the following arguments:", argument.action_type.as_str().cyan());
    println!("  Service name: {}", argument.service_name.as_str().cyan());
    println!("  Service executable path: {}", service_path.as_str().cyan());

    InstallerResult::Ok(())
}

fn print_error_and_forward<T>(error: InstallerError) -> Result<T, InstallerError> {
    diagnostic::print_error(&error.message); Result::Err(error)
}

fn get_service_path(argument: &Argument) -> Result<String, InstallerError> {
    let absolute_path_result = std::path::Path::new(&argument.executable_path).absolutize();
    let absolute_path = absolute_path_result.map_err(|e| { InstallerError::with(e,"Fail to get absolute path. ", 2) })?;
    if absolute_path.exists() {
        return Result::Ok(String::from(absolute_path.to_str().unwrap()));
    }

    let error_message = format!("The path {} does not exist.", &argument.executable_path);
    return Result::Err(InstallerError::new(error_message, 2));
}

struct Argument {
    action_type: String,
    executable_path: String,
    service_name: String
}

fn match_arguments() -> Result<Argument, InstallerError> {
    const ACTION_TYPE_ARGUMENT:&str = "action type";
    const SERVICE_PATH_ARGUMENT:&str = "service executable path";
    const SERVICE_NAME_ARGUMENT:&str = "service name";

    let matches: ArgMatches = App::new("Windows Service Installer")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(
            SubCommand::with_name(action_type::INSTALL_WINDOWS_SERVICE)
                .about("Install windows service on local machine.")
                .arg(
                    Arg::with_name(SERVICE_PATH_ARGUMENT)
                        .long("bin")
                        .required(true)
                        .multiple(false)
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name(SERVICE_NAME_ARGUMENT)
                        .long("name")
                        .required(true)
                        .multiple(false)
                        .takes_value(true)
                )
        )
        .get_matches();

    match matches.subcommand() {
        (action_type::INSTALL_WINDOWS_SERVICE, Some(sub_command_matches)) => {
            Result::Ok(Argument {
                action_type: String::from(action_type::INSTALL_WINDOWS_SERVICE),
                executable_path: String::from(sub_command_matches.value_of(SERVICE_PATH_ARGUMENT).ok_or(InstallerError::new("Invalid service path argument.", 1))?),
                service_name: String::from(sub_command_matches.value_of(SERVICE_NAME_ARGUMENT).ok_or(InstallerError::new("Invalid service name.", 1))?)
            })
        },
        _ => Result::Err(InstallerError::new("Not supported command.", 1))
    }
}