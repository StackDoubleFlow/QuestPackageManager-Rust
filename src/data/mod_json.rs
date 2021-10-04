use serde::{Serialize, Deserialize};
use std::io::{Write, Read};
use crate::data::package::{PackageConfig};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
pub struct ModJson {
    /// The Questpatcher version this mod.json was made for
    pub _QPVersion: String,
    /// Name of the mod
    pub name: String,
    /// ID of the mod
    pub id: String,
    /// Author of the mod
    pub author: String,
    /// Optional slot for if you ported a mod
    #[serde(skip_serializing_if = "Option::is_none")]
    pub porter: Option<String>,
    /// Mod version
    pub version: String,
    /// id of the package the mod is for, ex. com.beatgaems.beatsaber
    pub packageId: String,
    /// Version of the package, ex. 1.1.0
    pub packageVersion: String,
    /// description for the mod
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// optional cover image filename
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverImage: Option<String>,
    /// list of downloadable dependencies
    pub dependencies: Vec<ModDependency>,
    /// list of files that go in the package's mod folder
    pub modFiles: Vec<String>,
    /// list of files that go in the package's libs folder
    pub libraryFiles: Vec<String>,
    /// list of 
    pub fileCopies: Vec<String>,
}

impl Default for ModJson {
    fn default() -> ModJson 
    {
        ModJson {
            _QPVersion: "0.1.1".to_string(),
            name: String::default(),
            id: String::default(),
            author: String::default(),
            porter: Option::default(),
            version: String::default(),
            packageId: String::default(),
            packageVersion: String::default(),
            description: Option::default(),
            coverImage: Option::default(),
            dependencies: Vec::default(),
            modFiles: Vec::default(),
            libraryFiles: Vec::default(),
            fileCopies: Vec::default()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
pub struct ModDependency {
    /// the version requirement for this dependency
    pub versionRange: String,
    /// the id of this dependency
    pub id: String,
    /// the download link for this dependency, must satisfy id and version range!
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qmodLink: Option<String>
}

impl Default for ModDependency {
    fn default() -> ModDependency {
        ModDependency {
            versionRange: String::default(),
            id: String::default(),
            qmodLink: Option::default()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
pub struct FileCopy {
    /// name of the file in the qmod
    pub name: String,
    /// place where to put it (full path)
    pub destination: String
}

impl Default for FileCopy {
    fn default() -> FileCopy {
        FileCopy {
            name: String::default(),
            destination: String::default()
        }
    }
}

impl ModJson {
    pub fn from_package() -> ModJson
    {
        let package = PackageConfig::read();

        ModJson {..Default::default()}
    }

    pub fn read() -> ModJson 
    {
        let mut file = std::fs::File::open("mod.json").expect("Opening mod.json failed");
        let mut modJson = String::new();
        file.read_to_string(&mut modJson).expect("Reading data failed");

        serde_json::from_str::<ModJson>(&modJson).expect("Deserializing package failed")
    }

    pub fn write(&self)
    {
        let json = serde_json::to_string_pretty(&self).expect("Serialization failed");

        let mut file = std::fs::File::create("mod.json").expect("create failed");
        file.write_all(json.as_bytes()).expect("write failed");
        println!("Mod json {} Written!", self.id);
    }
}