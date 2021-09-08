use crate::features::feature_traits::Feature;
use clap::{App, ArgMatches, SubCommand, Arg};
use crate::error::{InstallerResult, InstallerError};
use crate::arguments::Argument;
use crate::features::service_wrapper::WindowsServiceOperatingContext;
use windows_service::service_manager::ServiceManagerAccess;
use windows_service::service::{ServiceAccess, ServiceState};
use crate::features::common::{try_query_service_status, try_wait_for_status, try_stop_and_wait, try_uninstall_service};
use colored::Colorize;

const UNINSTALL_WINDOWS_SERVICE: &str = "delete";
const SERVICE_NAME_ARGUMENT:&str = "service name";

pub struct UninstallServiceFeature {}

impl Feature for UninstallServiceFeature {
    fn create_argument_parser(&self) -> App {
        SubCommand::with_name(UNINSTALL_WINDOWS_SERVICE)
            .about("Uninstall windows service on local machine.")
            .arg(
                Arg::with_name(SERVICE_NAME_ARGUMENT)
                    .long("name")
                    .required(true)
                    .multiple(false)
                    .takes_value(true))
    }

    fn create_argument_from_matches(&self, matches: &ArgMatches) -> InstallerResult<Option<Argument>> {
        let sub_command_matches_option = matches.subcommand_matches(UNINSTALL_WINDOWS_SERVICE);
        if sub_command_matches_option.is_none() {
            return InstallerResult::Ok(Option::None);
        }

        let sub_command_matches = sub_command_matches_option.unwrap();

        InstallerResult::Ok(Option::Some(Argument {
            action_type: String::from(UNINSTALL_WINDOWS_SERVICE),
            executable_path: String::default(),
            service_name: String::from(sub_command_matches.value_of(SERVICE_NAME_ARGUMENT).ok_or(InstallerError::new("Invalid service name."))?),
            display_name: String::default(),
            description: String::default(),
            auto_start: false
        }))
    }

    fn execute_service_feature(&self, argument: &Argument) -> InstallerResult<()> {
        println!("Attempt to uninstall windows service: {}", argument.service_name.as_str().cyan());

        let context = WindowsServiceOperatingContext::from(
            &argument.service_name,
            ServiceManagerAccess::CONNECT,
            ServiceAccess::QUERY_STATUS | ServiceAccess::STOP | ServiceAccess::DELETE)?;

        let service_status = try_query_service_status(&context)?;

        // we need to make sure that the service can be removed after the statement.
        match service_status.current_state {
            ServiceState::Stopped => { /* it can be removed safely */ }
            ServiceState::StartPending => {
                try_wait_for_status(&context, ServiceState::Running)?;
                try_stop_and_wait(&context)?;
            }
            ServiceState::StopPending => {  try_wait_for_status(&context, ServiceState::Stopped)?; },
            ServiceState::Running => { try_stop_and_wait(&context)?; },
            ServiceState::ContinuePending => {
                try_wait_for_status(&context, ServiceState::Running)?;
                try_stop_and_wait(&context)?;
            },
            ServiceState::PausePending => { try_wait_for_status(&context, ServiceState::Paused)?; },
            ServiceState::Paused => { /* we are not sure how to do that */ },
        }

        try_uninstall_service(context)
    }

    fn get_sub_command_name(&self) -> String { String::from(UNINSTALL_WINDOWS_SERVICE) }
}