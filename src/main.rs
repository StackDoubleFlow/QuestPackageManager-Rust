use serde::{Serialize, Deserialize};
//use std::fs::{read_to_string};
use clap::{App, load_yaml};

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub cache_path: String,
    pub timeout: u32
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Dependency {
    pub id: String,
    pub versionRange: String,
    #[serde(default)]
    pub additionalData: serde_json::Value
}

impl Default for Dependency {
    #[inline]
    fn default() -> Dependency {
        Dependency {
            id: "".to_string(),
            versionRange: "".to_string(),
            additionalData: serde_json::Value::default()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PackageInfo {
    pub name: String,
    pub id: String,
    pub version: String,
    pub url: String,
    #[serde(default)]
    pub additionalData: serde_json::Value
}

impl Default for PackageInfo {
    #[inline]
    fn default() -> PackageInfo {
        PackageInfo {
            name: "".to_string(),
            id: "".to_string(),
            version: "".to_string(),
            url: "".to_string(),
            additionalData: serde_json::Value::default()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PackageConfig {
    pub sharedDir: String,
    pub dependenciesDir: String,
    pub info: PackageInfo,
    pub dependencies: Vec<Dependency>,
    #[serde(default)]
    pub additionalData: serde_json::Value
}

impl Default for PackageConfig {
    #[inline]
    fn default() -> PackageConfig {
        PackageConfig {
            sharedDir: "shared".to_string(),
            dependenciesDir: "extern".to_string(),
            info: PackageInfo::default(),
            dependencies: Vec::<Dependency>::default(),
            additionalData: serde_json::Value::default()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RestoredDependency {
    pub dependency: Dependency,
    pub version: String
}

impl Default for RestoredDependency {
    #[inline]
    fn default() -> RestoredDependency {
        RestoredDependency {
            dependency: Dependency::default(),
            version: "".to_string()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SharedPackageConfig {
    pub config: PackageConfig,
    pub restoredDependencies: Vec<RestoredDependency>
}

impl Default for SharedPackageConfig {
    #[inline]
    fn default() -> SharedPackageConfig {
        SharedPackageConfig {
            config: PackageConfig::default(),
            restoredDependencies: Vec::<RestoredDependency>::default(),
        }
    }
}