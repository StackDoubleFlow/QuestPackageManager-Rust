use std::{collections::HashMap, lazy::SyncLazy as Lazy, sync::Mutex};

use serde::{Deserialize, Serialize};

use crate::data::shared_package::SharedPackageConfig;

static API_URL: &str = "https://qpackages.com";

// https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton

static VERSIONS_CACHE: Lazy<Mutex<HashMap<String, Vec<PackageVersion>>>> =
    Lazy::new(Default::default);
static SHARED_PACKAGE_CACHE: Lazy<Mutex<HashMap<String, SharedPackageConfig>>> =
    Lazy::new(Default::default);

#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
pub struct PackageVersion {
    pub id: String,
    pub version: String,
}

/// Requests the appriopriate package info from qpackage.com
pub fn get_versions(id: &str, req: &str, limit: i32) -> Vec<PackageVersion> {
    let url = format!("{}/{}/?req={}&limit={}", &API_URL, &id, &req, &limit);

    if let Some(entry) = VERSIONS_CACHE.lock().unwrap().get(&url) {
        return entry.clone();
    }

    let versions = ureq::get(&url)
        .call()
        .expect("Request to qpackages.com failed")
        .into_json::<Vec<PackageVersion>>()
        .expect("Into json failed");

    VERSIONS_CACHE.lock().unwrap().insert(url, versions.clone());
    versions
}

pub fn get_shared_package(id: &str, ver: &str) -> SharedPackageConfig {
    let url = format!("{}/{}/{}", &API_URL, &id, &ver);
    if let Some(entry) = SHARED_PACKAGE_CACHE.lock().unwrap().get(&url) {
        return entry.clone();
    }

    let shared_package = ureq::get(&url)
        .call()
        .expect("Request to qpackages.com failed")
        .into_json::<SharedPackageConfig>()
        .expect("Into json failed");

    SHARED_PACKAGE_CACHE
        .lock()
        .unwrap()
        .insert(url, shared_package.clone());
    shared_package
}
