use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
pub struct Dependency {
    pub id: String,
    pub versionRange: String,
    #[serde(default)]
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
    pub branchName: Option<String>,

    /// Specify any additional files to be downloaded
    pub extraFiles: Vec<String>,

    /// Copy a dependency from a location that is local to this root path instead of from a remote url
    pub localPath: Option<String>,

    /// Specify if a dependency should download a release .so or .a file. Default to false
    pub useRelease: Option<bool>,

    /// Specify the style to use
    pub style: Option<String>
}

impl Default for AdditionalDependencyData {
    #[inline]
    fn default() -> AdditionalDependencyData {
        AdditionalDependencyData {
            branchName: Option::default(),
            extraFiles: Vec::default(),
            localPath: Option::default(),
            useRelease: Option::Some(false),
            style: Option::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPackageData {
    pub branchName: Option<String>,
    pub headersOnly: Option<bool>,
    pub staticLinking: Option<bool>,
    pub soLink: Option<String>,
    pub extraFiles: Vec<String>,
    pub debugSoLink: Option<String>,
    pub overrideSoName: Option<String>,
    pub styles: Option<Vec<PackageStyle>>
}

impl Default for AdditionalPackageData {
    #[inline]
    fn default() -> AdditionalPackageData {
        AdditionalPackageData {
            branchName: Option::default(),
            headersOnly: Option::default(),
            staticLinking: Option::default(),
            soLink: Option::default(),
            extraFiles: Vec::default(),
            debugSoLink: Option::default(),
            overrideSoName: Option::default(),
            styles: Option::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
pub struct PackageStyle {
    pub name: String,
    pub soLink: String,
    pub debugSoLink: String
}

impl Default for PackageStyle {
    #[inline]
    fn default() -> PackageStyle {
        PackageStyle {
            name: String::default(),
            soLink: String::default(),
            debugSoLink: String::default()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
pub struct PackageInfo {
    pub name: String,
    pub id: String,
    pub version: String,
    pub url: String,
    pub additionalData: AdditionalPackageData
}

impl Default for PackageInfo {
    #[inline]
    fn default() -> PackageInfo {
        PackageInfo {
            name: String::default(),
            id: String::default(),
            version: String::default(),
            url: String::default(),
            additionalData: AdditionalPackageData::default()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
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
#[allow(non_snake_case)]
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