use clap::{ArgMatches};
use crate::arguments::Argument;
use crate::error::InstallerResult;
use crate::features::install_service::InstallServiceFeature;
use crate::features::uninstall_service::UninstallServiceFeature;
use crate::features::query_service::QueryServiceFeature;
use crate::features::start_service::StartServiceFeature;
use crate::features::stop_service::StopServiceFeature;

pub trait Feature {
    fn create_argument_parser(&self) -> clap::App;
    fn create_argument_from_matches(&self, sub_command_matches:&ArgMatches) -> InstallerResult<Option<Argument>>;
    fn execute_service_feature(&self, argument:&Argument) -> InstallerResult<()>;
    fn get_sub_command_name(&self) -> String;
}

pub struct FeatureFactory {
    features: Vec<Box<dyn Feature>>
}

impl FeatureFactory {
    pub fn new() -> FeatureFactory {
        FeatureFactory{
            features: vec![
                Box::new(InstallServiceFeature{}),
                Box::new(UninstallServiceFeature{}),
                Box::new(QueryServiceFeature{}),
                Box::new(StartServiceFeature{}),
                Box::new(StopServiceFeature)
            ]
        }
    }

    pub fn get_features(&self) -> &Vec<Box<dyn Feature>> {
        &self.features
    }
}