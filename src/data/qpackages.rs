use serde::{Serialize, Deserialize};
use crate::data::shared_package::SharedPackageConfig;

static API_URL: &str = "https://qpackages.com";

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
pub struct PackageVersion {
    pub id: String,
    pub version: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SearchRequest {
    pub versions: Vec<PackageVersion>
}

/// Requests the appriopriate package info from qpackage.com
pub fn get_versions(id: &str, req: &str, limit: i32) -> Vec<PackageVersion>
{
    let url = format!("{}/{}/?req={}&limit={}", &API_URL, &id, &req, &limit);
    let response = ureq::get(&url).call().expect("Request to qpackages.com failed").into_string().expect("Into string failed");

    let response_val = format!("{{ \"versions\": {}}}", response);

    //println!("response to be deserialized: {}", response_val);
    let search_request = serde_json::from_str::<SearchRequest>(&response_val).expect("Deserialize failed!"); 
    search_request.versions
}

pub fn get_shared_package(id: &str, ver: &str) -> SharedPackageConfig
{
    let url = format!("{}/{}/{}", &API_URL, &id, &ver);
    let response = ureq::get(&url).call().expect("Request to qpackages.com failed").into_string().expect("Into string failed");

    serde_json::from_str::<SharedPackageConfig>(&response).expect("Deserialize from string failed!")
}