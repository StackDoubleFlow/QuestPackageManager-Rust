use serde::{Serialize, Deserialize};
use crate::data::dependency::{Dependency, AdditionalDependencyData};
use std::io::{Write, Read};

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
            Option::Some(_d) => { println!("Not adding dependency {} because it already existed", &dependency.id); },
            Option::None => {
                self.dependencies.insert(self.dependencies.len(), dependency);
            }
        }
        
    }

    pub fn get_dependency(&mut self, id: &str) -> Option<&mut Dependency>
    {
        for (idx, dependency) in self.dependencies.iter().enumerate()
        {
            if dependency.id.eq(id)
            {
                return self.dependencies.get_mut(idx);
            }
        }
        
        Option::default()
    }

    pub fn remove_dependency(&mut self, id: &str)
    {
        for (idx, dependency) in self.dependencies.iter().enumerate()
        {
            if dependency.id.eq(id)
            {
                println!("removed dependency {}", id);            
                self.dependencies.remove(idx);
                return;
            }
        }

        println!("Not removing dependency {} because it did not exist", id);
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