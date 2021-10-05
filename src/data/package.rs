use serde::{Serialize, Deserialize};
use crate::data::dependency::{Dependency, AdditionalDependencyData};
use std::io::{Write, Read};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPackageData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_linking: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub so_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_files: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_so_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_so_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub styles: Option<Vec<PackageStyle>>
}

impl Default for AdditionalPackageData {
    #[inline]
    fn default() -> AdditionalPackageData {
        AdditionalPackageData {
            branch_name: Option::default(),
            headers_only: Option::default(),
            static_linking: Option::default(),
            so_link: Option::default(),
            extra_files: Option::default(),
            debug_so_link: Option::default(),
            override_so_name: Option::default(),
            styles: Option::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PackageStyle {
    pub name: String,
    pub so_link: String,
    pub debug_so_link: String
}

impl Default for PackageStyle {
    #[inline]
    fn default() -> PackageStyle {
        PackageStyle {
            name: String::default(),
            so_link: String::default(),
            debug_so_link: String::default()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PackageInfo {
    pub name: String,
    pub id: String,
    pub version: String,
    pub url: Option<String>,
    pub additional_data: AdditionalPackageData
}

impl Default for PackageInfo {
    #[inline]
    fn default() -> PackageInfo {
        PackageInfo {
            name: String::default(),
            id: String::default(),
            version: String::default(),
            url: Option::default(),
            additional_data: AdditionalPackageData::default()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PackageConfig {
    pub shared_dir: String,
    pub dependencies_dir: String,
    pub info: PackageInfo,
    pub dependencies: Vec<Dependency>,
    pub additional_data: AdditionalDependencyData
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
            shared_dir: "shared".to_string(),
            dependencies_dir: "extern".to_string(),
            info: PackageInfo::default(),
            dependencies: Vec::<Dependency>::default(),
            additional_data: AdditionalDependencyData::default()
        }
    }
}