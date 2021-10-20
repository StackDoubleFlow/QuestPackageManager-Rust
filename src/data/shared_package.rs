use std::io::{Read, Write};

use semver::VersionReq;
use serde::{Deserialize, Serialize};

use crate::data::{
    dependency::Dependency, package::PackageConfig, shared_dependency::SharedDependency,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SharedPackageConfig {
    pub config: PackageConfig,
    pub restored_dependencies: Vec<SharedDependency>,
}

impl SharedPackageConfig {
    pub fn read() -> SharedPackageConfig {
        let mut file =
            std::fs::File::open("qpm.shared.json").expect("Opening qpm.shared.json failed");
        let mut qpm_package = String::new();
        file.read_to_string(&mut qpm_package)
            .expect("Reading data failed");

        serde_json::from_str::<SharedPackageConfig>(&qpm_package)
            .expect("Deserializing package failed")
    }

    pub fn write(&self) {
        let qpm_package = serde_json::to_string_pretty(&self).expect("Serialization failed");

        let mut file = std::fs::File::create("qpm.shared.json").expect("create failed");
        file.write_all(qpm_package.as_bytes())
            .expect("write failed");
        println!("Package {} Written!", self.config.info.id);
    }

    pub fn collect(&mut self) -> Vec<SharedDependency> {
        let mut deps = Vec::<SharedDependency>::new();
        deps.append(&mut self.restored_dependencies);
        for dependency in &self.restored_dependencies {
            let mut their_shared = dependency.get_shared_package();
            deps.append(&mut their_shared.collect());
        }

        deps
    }

    pub fn publish(&self) {
        for dependency in self.config.dependencies.iter() {
            match dependency.get_shared_package() {
                Option::Some(_s) => {}
                Option::None => {
                    println!(
                        "dependency {} was not available on qpackages in the given version range",
                        &dependency.id
                    );
                    println!(
                        "make sure {} exists for this dependency",
                        &dependency.version_range
                    );
                    std::process::exit(0);
                }
            };
        }
    }

    pub fn from_package(package: &PackageConfig) -> SharedPackageConfig {
        let shared_iter = package.resolve();

        let mut shared_package = SharedPackageConfig {
            config: package.clone(),
            restored_dependencies: shared_iter
                .collect::<Vec<SharedPackageConfig>>()
                .iter()
                .map(|cfg| cfg.to_shared_dependency())
                .collect::<Vec<SharedDependency>>(),
        };

        for dep in shared_package.config.dependencies.iter() {
            let restored_dep = shared_package
                .restored_dependencies
                .iter_mut()
                .find(|el| el.dependency.id == dep.id)
                .unwrap();

            restored_dep
                .dependency
                .additional_data
                .merge(dep.additional_data.clone());
        }

        shared_package
    }

    pub fn to_shared_dependency(&self) -> SharedDependency {
        let result = SharedDependency {
            dependency: Dependency {
                id: self.config.info.id.to_string(),
                version_range: VersionReq::parse(&format!("={}", self.config.info.version))
                    .unwrap(),
                additional_data: self.config.info.additional_data.to_dependency_data(),
            },
            version: self.config.info.version.clone(),
        };

        result
    }

    pub fn restore(&self) {
        for restore in self.restored_dependencies.iter() {
            // if the shared dep is contained within the direct dependencies, copy them over
            if self
                .config
                .dependencies
                .iter()
                .any(|dep| dep.id == restore.dependency.id)
            {
                restore.cache();
                restore.restore_from_cache();
            }
        }

        // todo edit android_mk
        // todo edit mod.json
    }
}
