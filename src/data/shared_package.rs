use serde::{Deserialize, Serialize};
use crate::data::package;
use crate::data::shared_dependency::SharedDependency;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SharedPackageConfig {
    pub config: package::PackageConfig,
    pub restored_dependencies: Vec<SharedDependency>
}

impl SharedPackageConfig {
    #[allow(dead_code)]
    pub fn read() -> SharedPackageConfig
    {
        let mut file = std::fs::File::open("qpm.shared.json").expect("Opening qpm.shared.json failed");
        let mut qpm_package = String::new();
        file.read_to_string(&mut qpm_package).expect("Reading data failed");

        serde_json::from_str::<SharedPackageConfig>(&qpm_package).expect("Deserializing package failed")
    }

    #[allow(dead_code)]
    pub fn write(&self)
    {
        let qpm_package = serde_json::to_string_pretty(&self).expect("Serialization failed");

        let mut file = std::fs::File::create("qpm.shared.json").expect("create failed");
        file.write_all(qpm_package.as_bytes()).expect("write failed");
        println!("Package {} Written!", self.config.info.id);
    }

    pub fn collect(&mut self) -> Vec<SharedDependency>
    {
        let mut deps =  Vec::<SharedDependency>::new();
        deps.append(&mut self.restored_dependencies);
        for dependency in &self.restored_dependencies
        {
            let mut their_shared = dependency.get_shared_package();
            deps.append(&mut their_shared.collect());
        }

        deps
    }
}
impl Default for SharedPackageConfig {
    #[inline]
    fn default() -> SharedPackageConfig {
        SharedPackageConfig {
            config: package::PackageConfig::default(),
            restored_dependencies: Vec::<SharedDependency>::default(),
        }
    }
}