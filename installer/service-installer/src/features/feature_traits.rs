use clap::{ArgMatches};
use crate::arguments::Argument;
use crate::error::InstallerResult;

pub trait Feature {
    fn create_argument_parser(&self) -> clap::App;
    fn create_argument_from_matches(&self, matches:&ArgMatches) -> InstallerResult<Option<Argument>>;
    fn execute_service_feature(&self, argument:&Argument) -> InstallerResult<()>;
    fn get_sub_command_name(&self) -> String;
}