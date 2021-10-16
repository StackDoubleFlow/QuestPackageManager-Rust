use std::{collections::HashMap, process::exit};

use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

use crate::data::{
    package::AdditionalPackageData, qpackages, shared_dependency::SharedDependency,
    shared_package::SharedPackageConfig,
};

#[derive(Serialize, Deserialize, Clone, Debug, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Dependency {
    pub id: String,
    #[serde(deserialize_with = "cursed_semver_parser::deserialize")]
    pub version_range: VersionReq,
    pub additional_data: AdditionalDependencyData,
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash, Eq, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalDependencyData {
    /// Branch name of a Github repo. Only used when a valid github url is provided
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,

    /// Specify any additional files to be downloaded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_files: Option<Vec<String>>,

    /// Copy a dependency from a location that is local to this root path instead of from a remote url
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_path: Option<String>,

    /// Specify if a dependency should download a release .so or .a file. Default to false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_release: Option<bool>,

    /// Whether or not the dependency is private and should be used in restore
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename(serialize = "private", deserialize = "private")
    )]
    pub is_private: Option<bool>,

    /// Qmod link to make a qmod downloadable in the mod.json
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qmod_link: Option<String>,
}

impl AdditionalDependencyData {
    pub fn merge(&mut self, other: AdditionalDependencyData) {
        if self.branch_name.is_none() {
            if let Some(other_branch_name) = &other.branch_name {
                self.branch_name = Some(other_branch_name.clone());
            }
        }

        if let (Some(extra_files), Some(other_extra_files)) =
            (&mut self.extra_files, &other.extra_files)
        {
            extra_files.append(&mut other_extra_files.clone());
        } else if self.extra_files.is_none() {
            if let Some(other_extra_files) = &other.extra_files {
                self.extra_files = Some(other_extra_files.clone());
            }
        }

        if self.local_path.is_none() {
            if let Some(other_local_path) = &other.local_path {
                self.local_path = Some(other_local_path.clone());
            }
        }

        if let (Some(is_private), Some(other_is_private)) = (&self.is_private, &other.is_private) {
            self.is_private = Some(*is_private || *other_is_private);
        } else if self.is_private.is_none() {
            if let Some(other_is_private) = &other.is_private {
                self.is_private = Some(*other_is_private);
            }
        }
    }

    pub fn merge_package(&mut self, other: AdditionalPackageData) {
        if let Some(static_linking) = other.static_linking {
            self.use_release = Some(static_linking);
        }

        if self.qmod_link.is_none() {
            self.qmod_link = other.qmod_link;
        }
    }
}

#[allow(dead_code)]
impl Dependency {
    pub fn get_shared_package(&self) -> Option<SharedPackageConfig> {
        let versions = qpackages::get_versions(&self.id, "*", 0);
        for v in versions.iter() {
            let ver = Version::parse(&v.version).expect("Parsing found version failed");

            if self.version_range.matches(&ver) {
                return Option::Some(qpackages::get_shared_package(&self.id, &v.version));
            }
        }

        Option::None
    }

    pub fn collect(
        &self,
        this_id: &str,
        collected: &mut HashMap<SharedDependency, SharedPackageConfig>,
    ) {
        if self.id.to_lowercase().eq(&this_id.to_lowercase()) {
            return;
        }

        let mut shared_package: SharedPackageConfig;
        match self.get_shared_package() {
            Option::Some(s) => {
                shared_package = s;
            }
            Option::None => {
                println!("Could not find config for {}", &self.id);
                exit(0);
            }
        }

        shared_package.restored_dependencies.retain(|dep| {
            if let Some(is_private) = dep.dependency.additional_data.is_private {
                !is_private
            } else {
                true
            }
        });

        // make a shared dependency out of this dependency
        let mut to_add = SharedDependency {
            dependency: self.clone(),
            version: shared_package.config.info.version.clone(),
        };

        if to_add.dependency.additional_data.qmod_link.is_none() {
            to_add.dependency.additional_data.qmod_link =
                shared_package.config.info.additional_data.qmod_link.clone();
        }

        println!("{:#?}", self.additional_data.extra_files);
        collected.insert(to_add.clone(), shared_package);
        // collect for this shared dep
        to_add.collect(this_id, collected);
    }
}
