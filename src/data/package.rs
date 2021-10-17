use std::{collections::HashMap, path::PathBuf};

use owo_colors::OwoColorize;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

use crate::data::{
    dependency::{AdditionalDependencyData, Dependency},
    shared_dependency::SharedDependency,
    shared_package::SharedPackageConfig,
};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPackageData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_linking: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub so_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_files: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_so_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_so_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qmod_link: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PackageInfo {
    pub name: String,
    pub id: String,
    pub version: Version,
    pub url: Option<String>,
    pub additional_data: AdditionalPackageData,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PackageConfig {
    pub shared_dir: PathBuf,
    pub dependencies_dir: PathBuf,
    pub info: PackageInfo,
    pub dependencies: Vec<Dependency>,
    pub additional_data: AdditionalDependencyData,
}

impl PackageConfig {
    pub fn write(&self) {
        let file = std::fs::File::create("qpm.json").expect("create failed");
        serde_json::to_writer_pretty(file, &self).expect("Serialization failed");
        println!("Package {} Written!", self.info.id);
    }

    pub fn read() -> PackageConfig {
        let file = std::fs::File::open("qpm.json").expect("Opening qpm.json failed");
        serde_json::from_reader(file).expect("Deserializing package failed")
    }

    pub fn add_dependency(&mut self, dependency: Dependency) {
        let dep = self.get_dependency(&dependency.id);
        match dep {
            Option::Some(_d) => {
                println!(
                    "Not adding dependency {} because it already existed",
                    &dependency.id
                );
            }
            Option::None => {
                self.dependencies.push(dependency);
            }
        }
    }

    pub fn get_dependency(&mut self, id: &str) -> Option<&mut Dependency> {
        for (idx, dependency) in self.dependencies.iter().enumerate() {
            if dependency.id.eq(id) {
                return self.dependencies.get_mut(idx);
            }
        }

        Option::default()
    }

    pub fn remove_dependency(&mut self, id: &str) {
        for (idx, dependency) in self.dependencies.iter().enumerate() {
            if dependency.id.eq(id) {
                println!("removed dependency {}", id);
                self.dependencies.remove(idx);
                return;
            }
        }

        println!("Not removing dependency {} because it did not exist", id);
    }

    pub fn collect(&self) -> HashMap<SharedDependency, SharedPackageConfig> {
        // fd new vector for storing our values
        let mut collected = HashMap::<SharedDependency, SharedPackageConfig>::new();

        // for every dependency defined in our local package (qpm.json)
        for dependency in self.dependencies.iter() {
            dependency.collect(&self.info.id, &mut collected);
        }

        collected
    }

    pub fn collapse(&self) -> HashMap<SharedDependency, SharedPackageConfig> {
        // collect our dependencies first
        let mut collapsed = self.collect();
        let collapsed_clone = collapsed.clone();

        collapsed.retain(|shared_dependency, _shared_package|{
            for pair in collapsed_clone.iter() {
                if pair.0.dependency.id.eq(&shared_dependency.dependency.id) && pair.0.get_hash() != shared_dependency.get_hash() {
                    let req = intersect(pair.0.dependency.version_range.clone(), shared_dependency.dependency.version_range.clone());
                    let match1 = req.matches(&pair.0.version);
                    let match2 = req.matches(&shared_dependency.version);
                    if match1 && match2
                    {
                        // both are good
                        if pair.0.version > shared_dependency.version
                        {
                            // if the first version is larger than the second, then that means this is a bad one, remove second
                            return false;
                        }
                    }
                    else if match1
                    {
                        // just the first is good, remove second
                        return false;
                    }
                    else if match2
                    {
                        // just the second is good, this means do nothing right now, we'll get to the point where it'll be removed
                    }
                    else
                    {
                        // neither is good, this means the config is unusable!
                        println!("Cannot collapse {}, Ranges do not intersect:", shared_dependency.dependency.id.bright_red());
                        println!("Range 1: {} --> {}", &pair.0.dependency.version_range.bright_blue(), &pair.0.version.bright_green());
                        println!("Range 2: {} --> {}", &shared_dependency.dependency.version_range.bright_blue(), &shared_dependency.version.bright_green());
                        println!("Consider running {} and see which packages are using incompatible version ranges", "qpm-rust collect".bright_yellow());
                        std::process::exit(0);
                    }
                }
            }
            true
        });

        collapsed
    }
}

fn intersect(mut lhs: VersionReq, mut rhs: VersionReq) -> VersionReq {
    lhs.comparators.append(&mut rhs.comparators);
    lhs
}
