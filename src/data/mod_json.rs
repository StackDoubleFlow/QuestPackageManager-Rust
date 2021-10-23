use std::io::{Read, Write};

use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModJson {
    /// The Questpatcher version this mod.json was made for
    #[serde(rename(serialize = "_QPVersion", deserialize = "_QPVersion"))]
    pub schema_version: String,
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
    pub version: Version,
    /// id of the package the mod is for, ex. com.beatgaems.beatsaber
    pub package_id: String,
    /// Version of the package, ex. 1.1.0
    pub package_version: String,
    /// description for the mod
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// optional cover image filename
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_image: Option<String>,
    /// list of downloadable dependencies
    pub dependencies: Vec<ModDependency>,
    /// list of files that go in the package's mod folder
    pub mod_files: Vec<String>,
    /// list of files that go in the package's libs folder
    pub library_files: Vec<String>,
    /// list of
    pub file_copies: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ModDependency {
    /// the version requirement for this dependency
    #[serde(deserialize_with = "cursed_semver_parser::deserialize")]
    pub version_range: VersionReq,
    /// the id of this dependency
    pub id: String,
    /// the download link for this dependency, must satisfy id and version range!
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mod_link: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct FileCopy {
    /// name of the file in the qmod
    pub name: String,
    /// place where to put it (full path)
    pub destination: String,
}

impl ModJson {
    pub fn _from_package() -> ModJson {
        todo!()
    }

    pub fn _read() -> ModJson {
        let mut file = std::fs::File::open("mod.json").expect("Opening mod.json failed");
        let mut json = String::new();
        file.read_to_string(&mut json).expect("Reading data failed");

        serde_json::from_str::<ModJson>(&json).expect("Deserializing package failed")
    }

    pub fn write(&self) {
        let json = serde_json::to_string_pretty(&self).expect("Serialization failed");

        let mut file = std::fs::File::create("mod.json").expect("create failed");
        file.write_all(json.as_bytes()).expect("write failed");
        println!("Mod json {} Written!", self.id);
    }
}

impl From<crate::data::dependency::Dependency> for ModDependency {
    fn from(dep: crate::data::dependency::Dependency) -> Self {
        Self {
            id: dep.id,
            version_range: dep.version_range,
            mod_link: dep.additional_data.mod_link,
        }
    }
}

impl From<crate::data::shared_dependency::SharedDependency> for ModDependency {
    fn from(dep: crate::data::shared_dependency::SharedDependency) -> Self {
        dep.dependency.into()
    }
}
