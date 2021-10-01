use serde::{Serialize, Deserialize};
use std::fs::{read_to_string};

fn main() {
    let mut cfg = Config {cache_path: "test".to_string(), timeout: 3 };
    let ser = serde_json::to_string(&cfg).unwrap();
    println!("serialize = {}", ser);
    cfg = serde_json::from_str(&ser).unwrap();
    println!("deser = {:?}", cfg);

    let qpm_json = read_to_string("./qpm.json").unwrap();
    
    let qpm_cfg = serde_json::from_str::<PackageConfig>(&qpm_json).unwrap();
    println!("qpm = {:?}", qpm_cfg);
    println!("");

    let qpm_shared_json = read_to_string("./qpm.shared.json").unwrap();
    let qpm_shared_cfg = serde_json::from_str::<SharedPackageConfig>(&qpm_shared_json).unwrap();
    println!("shared qpm = {:?}", qpm_shared_cfg);
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub cache_path: String,
    pub timeout: u32
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Dependency {
    pub id: String,
    #[serde(alias = "versionRange")]
    pub version_range: String,
    #[serde(default)]
    #[serde(alias = "additionalData")]
    pub additional_data: serde_json::Value
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PackageInfo {
    pub name: String,
    pub id: String,
    pub version: String,
    pub url: String,
    #[serde(default)]
    #[serde(alias = "additionalData")]
    pub additional_data: serde_json::Value
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PackageConfig {
    #[serde(alias = "sharedDir")]
    pub shared_dir: String,
    #[serde(alias = "dependenciesDir")]
    pub dependencies_dir: String,
    pub info: PackageInfo,
    pub dependencies: Vec<Dependency>,
    #[serde(default)]
    #[serde(alias = "additionalData")]
    pub additional_data: serde_json::Value
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RestoredDependency {
    pub dependency: Dependency,
    pub version: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SharedPackageConfig {
    pub config: PackageConfig,
    #[serde(alias = "restoredDependencies")]
    pub restored_dependencies: Vec<RestoredDependency>
}