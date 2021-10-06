use serde::{Serialize, Deserialize};
use crate::data::shared_package::SharedPackageConfig;
use crate::data::qpackages;
use semver::{Version, VersionReq};
#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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

impl Dependency {
    pub fn get_shared_package(&self) -> Option<SharedPackageConfig>
    {
        let versions = qpackages::get_versions(&self.id, "*", 0);

        for v in versions.iter()
        {
            let req = VersionReq::parse(&self.version_range).expect("parsing version range failed");
            let ver = Version::parse(&v.version).expect("Parsing found version failed");

            if req.matches(&ver)
            {
                return Option::Some(qpackages::get_shared_package(&self.id, &v.version));
            }
        }

        Option::None
    }
}