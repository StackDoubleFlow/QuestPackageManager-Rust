use std::io::{Read, BufReader};

use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

use crate::data::{
    dependency::Dependency, shared_dependency::SharedDependency,
    shared_package::SharedPackageConfig,
};

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

pub struct PreProcessingData {
    pub version: String,
    pub mod_id: String,
}

impl ModJson {
    pub fn read_parse(preprocess_data: &PreProcessingData) -> Self {
        let mut file = std::fs::File::open("mod.template.json").expect("Opening mod.json failed");

        // Get data
        let mut json = String::new();
        file.read_to_string(&mut json).expect("Reading data failed");

        // Pre process
        let processsed = Self::preprocess(json, preprocess_data);

        
        serde_json::from_str(&processsed).expect("Deserializing package failed")
    }

    fn preprocess(s: String, preprocess_data: &PreProcessingData) -> String {
        s
        .replace("${version}", &preprocess_data.version)
        .replace("{mod_id}", &preprocess_data.mod_id)
    }

    pub fn read_template() -> ModJson {
        let file = std::fs::File::open("mod.template.json").expect("Opening mod.json failed");
        let reader = BufReader::new(file);


        serde_json::from_reader(reader).expect("Deserializing package failed")
    }

    pub fn write_template(&self) {
        let file = std::fs::File::create("mod.template.json").expect("create failed");
        serde_json::to_writer_pretty(file, self).expect("Write failed");
    }
    
    pub fn write_result(&self) {
        let file = std::fs::File::create("mod.json").expect("create failed");
        serde_json::to_writer_pretty(file, self).expect("Write failed");
    }
}

impl From<SharedPackageConfig> for ModJson {
    fn from(mut shared_package: SharedPackageConfig) -> Self {
        shared_package
            .restored_dependencies
            // keep if header only is false, or if not defined
            .retain(|dep| !dep.dependency.additional_data.headers_only.unwrap_or(false));

        // actual direct lib deps
        let mut libs = shared_package
            .restored_dependencies
            .iter()
            .map(|dep| dep.get_so_name())
            .collect::<Vec<String>>();

        libs.retain(|lib| !lib.contains("modloader")); // todo: How to blacklist dependencies such as coremods?

        // downloadable mods links n stuff
        let mods: Vec<ModDependency> = shared_package
            .restored_dependencies
            .iter()
            // Removes any dependency without a qmod link
            .filter(|dep| dep.dependency.additional_data.mod_link.is_some())
            .map(|dep| dep.clone().into())
            .collect();

        // Only keep libs that aren't downloadable
        libs.retain(|lib| !mods.iter().any(|dep| lib.contains(&dep.id)));

        Self {
            schema_version: "0.1.2".to_string(),
            name: shared_package.config.info.name.clone(),
            id: shared_package.config.info.id.clone(),
            author: Default::default(),
            porter: None,
            version: shared_package.config.info.version.clone(),
            package_id: "com.beatgames.beatsaber".to_string(),
            package_version: "*".to_string(),
            description: None,
            cover_image: None,
            dependencies: mods,
            mod_files: vec![shared_package.config.get_so_name()],
            library_files: libs,
            file_copies: Default::default(),
        }
    }
}

impl From<Dependency> for ModDependency {
    fn from(dep: Dependency) -> Self {
        Self {
            id: dep.id,
            version_range: dep.version_range,
            mod_link: dep.additional_data.mod_link,
        }
    }
}

impl From<SharedDependency> for ModDependency {
    fn from(dep: SharedDependency) -> Self {
        dep.dependency.into()
    }
}
