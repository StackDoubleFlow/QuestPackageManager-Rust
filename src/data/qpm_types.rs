use serde::{Serialize, Deserialize};
use std::io::Write;
use std::io::Read;

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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPackageData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branchName: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headersOnly: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staticLinking: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soLink: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extraFiles: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debugSoLink: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrideSoName: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
            extraFiles: Option::default(),
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
    pub url: Option<String>,
    pub additionalData: AdditionalPackageData
}

impl Default for PackageInfo {
    #[inline]
    fn default() -> PackageInfo {
        PackageInfo {
            name: String::default(),
            id: String::default(),
            version: String::default(),
            url: Option::default(),
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
    pub additionalData: AdditionalDependencyData
}

impl PackageConfig {
    pub fn write(&self)
    {
        let qpm_package = serde_json::to_string_pretty(&self).expect("Serialization failed");

        let mut file = std::fs::File::create("qpm.json").expect("create failed");
        file.write_all(qpm_package.as_bytes()).expect("write failed");
        println!("Package {} Written!", self.info.id);
    }

    pub fn read() -> PackageConfig 
    {
        let mut file = std::fs::File::open("qpm.json").expect("Opening qpm.json failed");
        let mut qpm_package = String::new();
        file.read_to_string(&mut qpm_package).expect("Reading data failed");

        serde_json::from_str::<PackageConfig>(&qpm_package).expect("Deserializing package failed")
    }

    pub fn add_dependency(&mut self, dependency: Dependency)
    {
        let dep = self.get_dependency(&dependency.id);
        match dep {
            Option::Some(_d) => {println!("Not adding dependency {} because it already existed", &dependency.id); },
            Option::None => {
                self.dependencies.insert(self.dependencies.len(), dependency);
            }
        }
        
    }

    pub fn get_dependency(&mut self, id: &str) -> Option<&mut Dependency>
    {
        let mut idx = 0;
        for dependency in &self.dependencies
        {
            if dependency.id.eq(id)
            {
                break;
            }
            idx += 1;
        }

        self.dependencies.get_mut(idx)
    }

    pub fn remove_dependency(&mut self, id: &str)
    {
        let mut idx = 0;
        
        for dependency in &self.dependencies
        {
            if dependency.id.eq(id)
            {
                break;
            }
            idx += 1;
        }
        if idx.eq(&self.dependencies.len())
        {
            println!("Not removing dependency {} because it did not exist", id);            
        }
        else
        {
            println!("removed dependency {}", id);            
            self.dependencies.remove(idx);
        }
    }
}
impl Default for PackageConfig {
    #[inline]
    fn default() -> PackageConfig {
        PackageConfig {
            sharedDir: "shared".to_string(),
            dependenciesDir: "extern".to_string(),
            info: PackageInfo::default(),
            dependencies: Vec::<Dependency>::default(),
            additionalData: AdditionalDependencyData::default()
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