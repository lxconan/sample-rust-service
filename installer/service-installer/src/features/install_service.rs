use crate::features::features::Feature;
use clap::{App, SubCommand, Arg, ArgMatches};
use crate::arguments::Argument;
use crate::error::{InstallerResult, InstallerError};
use crate::features::service_wrapper::WindowsServiceOperatingContext;
use colored::Colorize;
use path_absolutize::Absolutize;

pub struct InstallServiceFeature {
}

const COMMAND_NAME: &str = "create";
const SERVICE_NAME_KEY:&str = "service name";
const DISPLAY_NAME_KEY:&str = "display name";
const DESCRIPTION_KEY:&str = "description";
const AUTO_START_SWITCH_KEY:&str = "auto start";
const SERVICE_PATH_KEY:&str = "service executable path";

impl Feature for InstallServiceFeature {
    fn create_argument_parser(&self) -> App {
        SubCommand::with_name(COMMAND_NAME)
            .about("Install windows service on local machine.")
            .arg(
                Arg::with_name(SERVICE_NAME_KEY)
                    .long("name")
                    .required(true)
                    .multiple(false)
                    .takes_value(true)
            )
            .arg(
                Arg::with_name(DISPLAY_NAME_KEY)
                    .long("disp")
                    .required(true)
                    .multiple(false)
                    .takes_value(true)
            )
            .arg(
                Arg::with_name(DESCRIPTION_KEY)
                    .long("desc")
                    .required(true)
                    .multiple(false)
                    .takes_value(true)
            )
            .arg(
                Arg::with_name(AUTO_START_SWITCH_KEY)
                    .long("auto")
                    .required(false)
                    .multiple(false)
                    .takes_value(false)
            )
            .arg(
                Arg::with_name(SERVICE_PATH_KEY)
                    .long("bin")
                    .required(true)
                    .multiple(false)
                    .takes_value(true)
            )
    }
    fn create_argument_from_matches(&self, sub_command_matches: &ArgMatches) -> InstallerResult<Option<Argument>> {
        return InstallerResult::Ok(Option::Some(Argument {
            action_type: String::from(COMMAND_NAME),
            executable_path: String::from(sub_command_matches.value_of(SERVICE_PATH_KEY).ok_or(InstallerError::new("Invalid service path argument."))?),
            service_name: String::from(sub_command_matches.value_of(SERVICE_NAME_KEY).ok_or(InstallerError::new("Invalid service name."))?),
            display_name: String::from(sub_command_matches.value_of(DISPLAY_NAME_KEY).ok_or(InstallerError::new("Invalid display name."))?),
            description: String::from(sub_command_matches.value_of(DESCRIPTION_KEY).ok_or(InstallerError::new("Invalid description."))?),
            auto_start: sub_command_matches.is_present(AUTO_START_SWITCH_KEY)
        }));
    }
    fn execute_service_feature(&self, argument:&Argument) -> InstallerResult<()> {
        println!("Attempt to install windows service: {}", argument.service_name.as_str().cyan());

        let service_path = get_service_path(&argument)?;
        println!("  Service executable path: {}", service_path.as_str().cyan());

        print!("Creating windows service ...");
        WindowsServiceOperatingContext::create_windows_service(
            &argument.service_name,
            &argument.display_name,
            &argument.description,
            argument.auto_start,
            &service_path)?;
        println!("{}", "Done".green());
        Ok(())
    }

    fn get_sub_command_name(&self) -> String {
        return String::from(COMMAND_NAME);
    }
}

fn get_service_path(argument: &Argument) -> Result<String, InstallerError> {
    let absolute_path_result = std::path::Path::new(&argument.executable_path).absolutize();
    let absolute_path = absolute_path_result.map_err(|e| { InstallerError::with(e,"Fail to get absolute path. ") })?;
    if absolute_path.exists() {
        return Result::Ok(String::from(absolute_path.to_str().unwrap()));
    }

    let error_message = format!("The path {} does not exist.", &argument.executable_path);
    return Result::Err(InstallerError::new(error_message));
}