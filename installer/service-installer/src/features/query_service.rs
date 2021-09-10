use crate::features::features::Feature;
use clap::{App, ArgMatches, SubCommand, Arg};
use crate::error::{InstallerResult, InstallerError};
use crate::arguments::Argument;
use crate::features::common;
use crate::features::service_wrapper::WindowsServiceOperatingContext;
use windows_service::service_manager::ServiceManagerAccess;
use windows_service::service::ServiceAccess;
use colored::Colorize;

pub struct QueryServiceFeature {
}

const QUERY_SERVICE_COMMAND:&str = "query";
const SERVICE_NAME_ARGUMENT:&str = "service name";

impl Feature for QueryServiceFeature {
    fn create_argument_parser(&self) -> App {
        SubCommand::with_name(QUERY_SERVICE_COMMAND)
            .arg(
                Arg::with_name(SERVICE_NAME_ARGUMENT)
                    .long("name")
                    .required(true)
                    .multiple(false)
                    .takes_value(true)
            )
    }

    fn create_argument_from_matches(&self, matches: &ArgMatches) -> InstallerResult<Option<Argument>> {
        let sub_command_matches_option = matches.subcommand_matches(QUERY_SERVICE_COMMAND);
        if sub_command_matches_option.is_none() {
            return InstallerResult::Ok(Option::None);
        }

        let sub_command_matches = sub_command_matches_option.unwrap();
        InstallerResult::Ok(Option::Some(Argument {
            action_type: String::from(QUERY_SERVICE_COMMAND),
            executable_path: String::default(),
            service_name: String::from(sub_command_matches.value_of(SERVICE_NAME_ARGUMENT).ok_or(InstallerError::new("Invalid service name."))?),
            display_name: String::default(),
            description: String::default(),
            auto_start: false
        }))
    }

    fn execute_service_feature(&self, argument: &Argument) -> InstallerResult<()> {
        println!("Attempt to query service status for {}", argument.service_name.as_str().cyan());

        let context = WindowsServiceOperatingContext::from(
            &argument.service_name, ServiceManagerAccess::CONNECT, ServiceAccess::QUERY_STATUS)?;

        common::try_query_service_status(&context).map(|_|{})
    }

    fn get_sub_command_name(&self) -> String {
        String::from(QUERY_SERVICE_COMMAND)
    }
}