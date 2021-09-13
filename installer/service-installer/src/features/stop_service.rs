use crate::features::features::Feature;
use clap::{App, ArgMatches, SubCommand, Arg};
use crate::error::{InstallerResult, InstallerError};
use crate::arguments::Argument;
use colored::Colorize;
use crate::features::service_wrapper::WindowsServiceOperatingContext;
use windows_service::service_manager::ServiceManagerAccess;
use windows_service::service::ServiceAccess;
use crate::features::common::try_stop_and_wait;

pub struct StopServiceFeature;

const COMMAND_NAME:&str = "stop";
const SERVICE_NAME_KEY:&str = "service name";

impl Feature for StopServiceFeature {
    fn create_argument_parser(&self) -> App {
        SubCommand::with_name(COMMAND_NAME)
            .arg(
                Arg::with_name(SERVICE_NAME_KEY)
                    .long("name")
                    .required(true)
                    .multiple(false)
                    .takes_value(true)
            )
    }

    fn create_argument_from_matches(&self, sub_command_matches: &ArgMatches) -> InstallerResult<Option<Argument>> {
        InstallerResult::Ok(Option::Some(Argument {
            action_type: String::from(COMMAND_NAME),
            executable_path: String::default(),
            service_name: String::from(sub_command_matches.value_of(SERVICE_NAME_KEY).ok_or(InstallerError::new("invalid service name"))?),
            description: String::default(),
            display_name: String::default(),
            auto_start: false,
        }))
    }

    fn execute_service_feature(&self, argument: &Argument) -> InstallerResult<()> {
        println!("Attempt to stop service {}", argument.service_name.as_str().cyan());

        let context = WindowsServiceOperatingContext::from(
            &argument.service_name,
            ServiceManagerAccess::CONNECT,
            ServiceAccess::QUERY_STATUS | ServiceAccess::STOP)?;

        try_stop_and_wait(&context)
    }

    fn get_sub_command_name(&self) -> String {
        String::from(COMMAND_NAME)
    }
}