use serde::{Serialize, Deserialize};
use crate::data::shared_package::SharedPackageConfig;
use std::collections::HashMap;
use lazy_static::lazy_static; // 1.4.0
use std::sync::Mutex;

static API_URL: &str = "https://qpackages.com";

// https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton
lazy_static! {
    static ref VERSIONS_CACHE: Mutex<HashMap<String, Vec<PackageVersion>>> = Mutex::new(HashMap::new());
}

lazy_static! {
    static ref SHARED_PACKAGE_CACHE: Mutex<HashMap<String, SharedPackageConfig>> = Mutex::new(HashMap::new());
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
pub struct PackageVersion {
    pub id: String,
    pub version: String
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
pub struct SearchRequest {
    pub versions: Vec<PackageVersion>
}

/// Requests the appriopriate package info from qpackage.com
pub fn get_versions(id: &str, req: &str, limit: i32) -> Vec<PackageVersion>
{
    let url = format!("{}/{}/?req={}&limit={}", &API_URL, &id, &req, &limit);

    if let Some(entry) = VERSIONS_CACHE.lock().unwrap().get(&url) {
        return entry.clone();
    }

    let response = ureq::get(&url).call().expect("Request to qpackages.com failed").into_string().expect("Into string failed");

    let response_val = format!("{{ \"versions\": {}}}", response);

    //println!("response to be deserialized: {}", response_val);
    let search_request = serde_json::from_str::<SearchRequest>(&response_val).expect("Deserialize failed!"); 

    VERSIONS_CACHE.lock().unwrap().insert(url, search_request.versions.clone()); 
    search_request.versions
}

pub fn get_shared_package(id: &str, ver: &str) -> SharedPackageConfig
{
    let url = format!("{}/{}/{}", &API_URL, &id, &ver);
    if let Some (entry) = SHARED_PACKAGE_CACHE.lock().unwrap().get(&url) {
        return entry.clone();
    }

    let response = ureq::get(&url).call().expect("Request to qpackages.com failed").into_string().expect("Into string failed");

    let shared_package = serde_json::from_str::<SharedPackageConfig>(&response).expect("Deserialize from string failed!");

    SHARED_PACKAGE_CACHE.lock().unwrap().insert(url, shared_package.clone());
    shared_package
}