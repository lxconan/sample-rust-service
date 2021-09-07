use clap::{Arg, App, ArgMatches, SubCommand, AppSettings};
use crate::error;

pub const INSTALL_WINDOWS_SERVICE: &str = "create";
pub const UNINSTALL_WINDOWS_SERVICE: &str = "delete";

pub struct Argument {
    pub action_type: String,
    pub executable_path: String,
    pub service_name: String,
    pub display_name: String,
    pub description: String,
    pub auto_start: bool
}

pub fn match_arguments() -> Result<Argument, error::InstallerError> {
    const SERVICE_NAME_ARGUMENT:&str = "service name";
    const DISPLAY_NAME_ARGUMENT:&str = "display name";
    const DESCRIPTION_ARGUMENT:&str = "description";
    const AUTO_START_SWITCH:&str = "auto start";
    const SERVICE_PATH_ARGUMENT:&str = "service executable path";

    let matches: ArgMatches = App::new("Windows Service Installer")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(
            SubCommand::with_name(INSTALL_WINDOWS_SERVICE)
                .about("Install windows service on local machine.")
                .arg(
                    Arg::with_name(SERVICE_NAME_ARGUMENT)
                        .long("name")
                        .required(true)
                        .multiple(false)
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name(DISPLAY_NAME_ARGUMENT)
                        .long("disp")
                        .required(true)
                        .multiple(false)
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name(DESCRIPTION_ARGUMENT)
                        .long("desc")
                        .required(true)
                        .multiple(false)
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name(AUTO_START_SWITCH)
                        .long("auto")
                        .required(false)
                        .multiple(false)
                        .takes_value(false)
                )
                .arg(
                    Arg::with_name(SERVICE_PATH_ARGUMENT)
                        .long("bin")
                        .required(true)
                        .multiple(false)
                        .takes_value(true)
                )
        )
        .subcommand(
            SubCommand::with_name(UNINSTALL_WINDOWS_SERVICE)
                .about("Uninstall windows service on local machine.")
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
        (INSTALL_WINDOWS_SERVICE, Some(sub_command_matches)) => {
            Result::Ok(Argument {
                action_type: String::from(INSTALL_WINDOWS_SERVICE),
                executable_path: String::from(sub_command_matches.value_of(SERVICE_PATH_ARGUMENT).ok_or(error::InstallerError::new("Invalid service path argument."))?),
                service_name: String::from(sub_command_matches.value_of(SERVICE_NAME_ARGUMENT).ok_or(error::InstallerError::new("Invalid service name."))?),
                display_name: String::from(sub_command_matches.value_of(DISPLAY_NAME_ARGUMENT).ok_or(error::InstallerError::new("Invalid display name."))?),
                description: String::from(sub_command_matches.value_of(DESCRIPTION_ARGUMENT).ok_or(error::InstallerError::new("Invalid description."))?),
                auto_start: sub_command_matches.is_present(AUTO_START_SWITCH)
            })
        },
        (UNINSTALL_WINDOWS_SERVICE, Some(sub_command_matches)) => {
            Result::Ok(Argument {
                action_type: String::from(UNINSTALL_WINDOWS_SERVICE),
                executable_path: String::default(),
                service_name: String::from(sub_command_matches.value_of(SERVICE_NAME_ARGUMENT).ok_or(error::InstallerError::new("Invalid service name."))?),
                display_name: String::default(),
                description: String::default(),
                auto_start: false
            })
        },
        _ => Result::Err(error::InstallerError::new("Not supported command."))
    }
}