use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
pub struct Dependency {
    pub id: String,
    pub versionRange: String,
    pub additionalData: AdditionalDependencyData
}

impl Default for Dependency {
    #[inline]
    fn default() -> Dependency {
        Dependency {
            id: "".to_string(),
            versionRange: "".to_string(),
            additionalData: AdditionalDependencyData::default()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalDependencyData {
    /// Branch name of a Github repo. Only used when a valid github url is provided
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branchName: Option<String>,

    /// Specify any additional files to be downloaded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extraFiles: Option<Vec<String>>,

    /// Copy a dependency from a location that is local to this root path instead of from a remote url
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localPath: Option<String>,

    /// Specify if a dependency should download a release .so or .a file. Default to false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub useRelease: Option<bool>,

    /// Specify the style to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>
}

impl Default for AdditionalDependencyData {
    #[inline]
    fn default() -> AdditionalDependencyData {
        AdditionalDependencyData {
            branchName: Option::default(),
            extraFiles: Option::default(),
            localPath: Option::default(),
            useRelease: Option::default(),
            style: Option::default(),
        }
    }
}