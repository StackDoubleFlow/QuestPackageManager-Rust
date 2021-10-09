use serde::{Deserialize, Serialize};
use crate::data::package;
use crate::data::shared_dependency::SharedDependency;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct SharedPackageConfig {
    pub config: package::PackageConfig,
    pub restored_dependencies: Vec<SharedDependency>
}

#[allow(dead_code)]
impl SharedPackageConfig {
    pub fn read() -> SharedPackageConfig
    {
        let mut file = std::fs::File::open("qpm.shared.json").expect("Opening qpm.shared.json failed");
        let mut qpm_package = String::new();
        file.read_to_string(&mut qpm_package).expect("Reading data failed");

        serde_json::from_str::<SharedPackageConfig>(&qpm_package).expect("Deserializing package failed")
    }

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

    pub fn publish(&self)
    {
        for dependency in self.config.dependencies.iter()
        {
            match dependency.get_shared_package() {
                Option::Some(_s) => {},
                Option::None => {
                    println!("dependency {} was not available on qpackages in the given version range", &dependency.id);
                    println!("make sure {} exists for this dependency", &dependency.version_range);
                    std::process::exit(0);
                }
            };
        }
    }
}