use clap::{Arg, App, ArgMatches, SubCommand, AppSettings};
use crate::error;
use crate::features::install_service::InstallServiceFeature;
use crate::features::feature_traits::Feature;

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

    let install_service_feature = InstallServiceFeature {};

    let matches: ArgMatches = App::new("Windows Service Installer")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(install_service_feature.create_argument_parser())
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

    let install_service_feature_args = install_service_feature.create_argument_from_matches(&matches)?;
    if install_service_feature_args.is_some() {
        return Result::Ok(install_service_feature_args.unwrap());
    }

    match matches.subcommand() {
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