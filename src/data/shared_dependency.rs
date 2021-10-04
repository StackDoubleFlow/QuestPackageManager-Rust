use serde::{Serialize, Deserialize};
use super::dependency;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RestoredDependency {
    pub dependency: dependency::Dependency,
    pub version: String
}

impl Default for RestoredDependency {
    #[inline]
    fn default() -> RestoredDependency {
        RestoredDependency {
            dependency: dependency::Dependency::default(),
            version: "".to_string()
        }
    }
}