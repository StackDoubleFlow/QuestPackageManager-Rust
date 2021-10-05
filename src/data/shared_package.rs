use serde::{Deserialize, Serialize};
use crate::data::package;
use crate::data::shared_dependency;

#[derive(Serialize, Deserialize, Clone, Debug)]

#[serde(rename_all = "camelCase")]
pub struct SharedPackageConfig {
    pub config: package::PackageConfig,
    pub restored_dependencies: Vec<shared_dependency::RestoredDependency>
}

impl Default for SharedPackageConfig {
    #[inline]
    fn default() -> SharedPackageConfig {
        SharedPackageConfig {
            config: package::PackageConfig::default(),
            restored_dependencies: Vec::<shared_dependency::RestoredDependency>::default(),
        }
    }
}