use serde::{Deserialize, Serialize};
use crate::data::package;
use crate::data::shared_dependency;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
pub struct SharedPackageConfig {
    pub config: package::PackageConfig,
    pub restoredDependencies: Vec<shared_dependency::RestoredDependency>
}

impl Default for SharedPackageConfig {
    #[inline]
    fn default() -> SharedPackageConfig {
        SharedPackageConfig {
            config: package::PackageConfig::default(),
            restoredDependencies: Vec::<shared_dependency::RestoredDependency>::default(),
        }
    }
}