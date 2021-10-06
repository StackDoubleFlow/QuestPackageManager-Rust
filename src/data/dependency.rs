use serde::{Serialize, Deserialize};
use crate::data::shared_package::SharedPackageConfig;
use crate::data::shared_dependency::SharedDependency;

use crate::data::qpackages;
use semver::{Version, VersionReq};
use std::collections::HashMap;
use std::process::exit;

#[derive(Serialize, Deserialize, Clone, Debug, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Dependency {
    pub id: String,
    pub version_range: String,
    pub additional_data: AdditionalDependencyData
}

impl Default for Dependency {
    #[inline]
    fn default() -> Dependency {
        Dependency {
            id: "".to_string(),
            version_range: "".to_string(),
            additional_data: AdditionalDependencyData::default()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash, Eq, PartialEq)]
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

    /// Specify the style to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,

    /// Whether or not the dependency is private and should be used in restore
    #[serde(skip_serializing_if = "Option::is_none", rename(serialize = "private", deserialize = "private"))]
    pub is_private: Option<bool>
}

impl Default for AdditionalDependencyData {
    #[inline]
    fn default() -> AdditionalDependencyData {
        AdditionalDependencyData {
            branch_name: Option::default(),
            extra_files: Option::default(),
            local_path: Option::default(),
            use_release: Option::default(),
            style: Option::default(),
            is_private: Option::default()
        }
    }
}

#[allow(dead_code)]
impl Dependency {
    pub fn get_shared_package(&self) -> Option<SharedPackageConfig>
    {
        let versions = qpackages::get_versions(&self.id, "*", 0);
        let pred = self.version_range.clone().replace('<', ", <");

        match VersionReq::parse(&pred) {
            Ok(req) => {
                for v in versions.iter()
                {
                    let ver = Version::parse(&v.version).expect("Parsing found version failed");

                    if req.matches(&ver)
                    {
                        return Option::Some(qpackages::get_shared_package(&self.id, &v.version));
                    } 
                }

            }
            Err(error) => {
                println!("Failed to parse range for dependency {}: {}", &self.id, &pred);
                panic!("error: {}", error);
            }
        }


        Option::None
    }

    pub fn collect(&self, collected: &mut HashMap<SharedDependency, SharedPackageConfig>)
    {
        let mut shared_package: SharedPackageConfig;
        match self.get_shared_package() {
            Option::Some(s) => { shared_package = s; },
            Option::None => {
                println!("Could not find config for {}", &self.id);
                exit(0);
            }
        }

        let pred = self.version_range.clone().replace('<', ", <");
        let dep_version = VersionReq::parse(&pred).expect("Parsing version range failed");
        let versions = qpackages::get_versions(&self.id, "*",0 );

        for v in versions
        {
            let ver = Version::parse(&v.version).expect("parsing version value failed");
            if dep_version.matches(&ver)
            {
                // found matching version
                let mut new_shared = SharedDependency {
                    dependency: self.clone(),
                    version: v.version
                };
                
                // does our shared package have an override so name?
                let override_so_name_opt = shared_package.config.info.additional_data.override_so_name.clone();
                match override_so_name_opt {
                    // if it does
                    Some(override_so_name) => {
                        // make a pair thats opt
                        let mut pair_opt = Option::<(SharedDependency, SharedPackageConfig)>::None;
                        // try to find a package with hte same override name
                        for (dep, shared_config) in collected.iter_mut()
                        {
                            if let Some(override_so_name_other) = &shared_config.config.info.additional_data.override_so_name {
                                if override_so_name.eq(override_so_name_other) 
                                {
                                    pair_opt = Option::Some((dep.clone(), shared_config.clone()));
                                    break;
                                }
                            }
                        }
                        
                        // if it was found
                        if let Some (pair) = pair_opt {
                            // is the new packages' version greater than our current? if so, replace the current version
                            if Version::parse(&shared_package.config.info.version) > Version::parse(&pair.1.config.info.version)
                            {
                                collected.remove(&pair.0);
                                new_shared.version = shared_package.config.info.version.clone();
                                collected.insert(new_shared, shared_package.clone());
                            }
                        }
                        else if !collected.contains_key(&new_shared)
                        {
                            let mut gotten = false;
                            for (dep, _shared_config) in collected.iter() 
                            {
                                // if we find a package that's the exact same
                                if new_shared.dependency.id.to_lowercase().eq(&dep.dependency.id.to_lowercase()) && new_shared.version.eq(&dep.version)
                                {
                                    gotten = true;
                                    break;
                                }
                            }
                            // if not found
                            if !gotten
                            {
                                // add to list
                                collected.insert(new_shared, shared_package.clone());
                            }
                        }
                    },
                    // if it doesn't have an override
                    None => {
                        // if it contains the key
                        if !collected.contains_key(&new_shared) {
                            let mut gotten = false;
                            for (dep, _shared_config) in collected.iter() {
                                // if we find a package that's the exact same
                                if new_shared.dependency.id.to_lowercase().eq(&dep.dependency.id.to_lowercase()) && new_shared.version.eq(&dep.version) {
                                    gotten = true;
                                    break;
                                }
                            }
                            // if not found
                            if !gotten {
                                // add to list
                                collected.insert(new_shared, shared_package.clone());
                            }
                        }
                    }
                }
                
                shared_package.restored_dependencies.retain(|inner_d|{
                    if let Some(is_private) = inner_d.dependency.additional_data.is_private {
                        return !is_private;
                    }
                    true
                });

                // for each dependency of the shared config that was still found
                for inner_d in shared_package.restored_dependencies.iter_mut() {
                    
                    inner_d.dependency.collect(collected);
                }
                break;
            }
        }
    }
}