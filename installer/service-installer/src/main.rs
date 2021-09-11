use colored::Colorize;

use crate::error::{InstallerError, InstallerResult};
use crate::features::features::FeatureFactory;
use crate::arguments::Argument;
use clap::{AppSettings, App, ArgMatches};

mod arguments;
mod error;
mod features;

fn main() -> Result<(), InstallerError> {
    let factory = FeatureFactory::new();

    let argument = match_arguments(&factory)?;
    println!("Will perform action: {} with the following arguments:", argument.action_type.as_str().cyan());
    println!("Service name:                           {}", argument.service_name.as_str().cyan());

    execute_feature(&factory, &argument)
}

fn execute_feature(feature_factory:&FeatureFactory, argument:&Argument) -> InstallerResult<()> {
    let features = feature_factory.get_features();
    for feature in features {
        if argument.action_type == feature.get_sub_command_name() {
            feature.execute_service_feature(argument)?;
        }
    }

    InstallerResult::Ok(())
}

fn match_arguments(feature_factory:&FeatureFactory) -> Result<Argument, error::InstallerError> {
    let mut app = App::new("Windows Service Installer").setting(AppSettings::SubcommandRequired);

    for feature in feature_factory.get_features() {
        app = app.subcommand(feature.create_argument_parser());
    }

    let matches: ArgMatches = app.get_matches();

    for feature in feature_factory.get_features() {
        let sub_command_name = feature.get_sub_command_name();
        let sub_command_matches_option = matches.subcommand_matches(&sub_command_name);
        if sub_command_matches_option.is_none() {
            continue;
        }

        let feature_args = feature.create_argument_from_matches(sub_command_matches_option.unwrap())?;
        if feature_args.is_some() {
            return Result::Ok(feature_args.unwrap());
        }
    }

    Result::Err(error::InstallerError::new("Not supported command."))
}