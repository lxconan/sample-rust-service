use crate::features::install_service::InstallServiceFeature;
use crate::features::uninstall_service::UninstallServiceFeature;
use crate::features::feature_traits::Feature;

pub struct FeatureFactory {
    features: Vec<Box<dyn Feature>>
}

impl FeatureFactory {
    pub fn new() -> FeatureFactory {
        FeatureFactory{
            features: vec![
                Box::new(InstallServiceFeature{}),
                Box::new(UninstallServiceFeature{})
            ]
        }
    }

    pub fn get_features(&self) -> &Vec<Box<dyn Feature>> {
        &self.features
    }
}