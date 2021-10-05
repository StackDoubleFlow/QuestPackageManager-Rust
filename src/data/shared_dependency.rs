use serde::{Serialize, Deserialize};
use crate::data::dependency::Dependency;
use crate::data::shared_package::SharedPackageConfig;
use crate::data::qpackages;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SharedDependency {
    pub dependency: Dependency,
    pub version: String
}

impl Default for SharedDependency {
    #[inline]
    fn default() -> SharedDependency {
        SharedDependency {
            dependency: Dependency::default(),
            version: "".to_string()
        }
    }
}

impl SharedDependency {
    pub fn get_shared_package(&self) -> SharedPackageConfig
    {
        qpackages::get_shared_package(&self.dependency.id, &self.version)
    }
}