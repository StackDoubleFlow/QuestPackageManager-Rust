use std::io::{Read, Write};

use semver::VersionReq;
use serde::{Deserialize, Serialize};

use crate::data::{
    dependency::Dependency, package::PackageConfig, shared_dependency::SharedDependency,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SharedPackageConfig {
    pub config: PackageConfig,
    pub restored_dependencies: Vec<SharedDependency>,
}

impl SharedPackageConfig {
    pub fn read() -> SharedPackageConfig {
        let mut file =
            std::fs::File::open("qpm.shared.json").expect("Opening qpm.shared.json failed");
        let mut qpm_package = String::new();
        file.read_to_string(&mut qpm_package)
            .expect("Reading data failed");

        serde_json::from_str::<SharedPackageConfig>(&qpm_package)
            .expect("Deserializing package failed")
    }

    pub fn write(&self) {
        let qpm_package = serde_json::to_string_pretty(&self).expect("Serialization failed");

        let mut file = std::fs::File::create("qpm.shared.json").expect("create failed");
        file.write_all(qpm_package.as_bytes())
            .expect("write failed");
        println!("Package {} Written!", self.config.info.id);
    }

    pub fn publish(&self) {
        for dependency in self.config.dependencies.iter() {
            match dependency.get_shared_package() {
                Option::Some(_s) => {}
                Option::None => {
                    println!(
                        "dependency {} was not available on qpackages in the given version range",
                        &dependency.id
                    );
                    println!(
                        "make sure {} exists for this dependency",
                        &dependency.version_range
                    );
                    std::process::exit(0);
                }
            };
        }
    }

    pub fn from_package(package: &PackageConfig) -> SharedPackageConfig {
        let shared_iter = package.resolve();

        #[allow(unused_mut)]
        let mut shared_package = SharedPackageConfig {
            config: package.clone(),
            restored_dependencies: shared_iter
                .collect::<Vec<SharedPackageConfig>>()
                .iter()
                .map(|cfg| cfg.to_shared_dependency())
                .collect::<Vec<SharedDependency>>(),
        };

        /*
        for dep in shared_package.config.dependencies.iter() {
            let restored_dep = shared_package
                .restored_dependencies
                .iter_mut()
                .find(|el| el.dependency.id == dep.id)
                .unwrap();

            restored_dep
                .dependency
                .additional_data
                .merge(dep.additional_data.clone());
        }
        */

        shared_package
    }

    pub fn to_shared_dependency(&self) -> SharedDependency {
        let result = SharedDependency {
            dependency: Dependency {
                id: self.config.info.id.to_string(),
                version_range: VersionReq::parse(&format!("={}", self.config.info.version))
                    .unwrap(),
                additional_data: Default::default(), //self.config.info.additional_data.to_dependency_data(),
            },
            version: self.config.info.version.clone(),
        };

        result
    }

    pub fn restore(&self) {
        for to_restore in self.restored_dependencies.iter() {
            // if the shared dep is contained within the direct dependencies, link against that, always copy headers!
            to_restore.cache2electricboogaloo();
            to_restore.restore_from_cache(
                self.config
                    .dependencies
                    .iter()
                    .any(|dep| dep.id == to_restore.dependency.id),
            );
        }

        self.write_extern_cmake();
        self.write_define_cmake();

        // TODO: edit mod.json
    }

    pub fn write_extern_cmake(&self) {
        // TODO: add incclude paths from the new qpm stuff
        let mut extern_cmake_file =
            std::fs::File::create("extern.cmake").expect("Failed to create extern cmake file");

        extern_cmake_file
        .write_all("# always added\ntarget_include_directories(${COMPILE_ID} PRIVATE ${EXTERN_DIR}/includes)\ntarget_include_directories(${COMPILE_ID} PRIVATE ${EXTERN_DIR}/includes/libil2cpp/il2cpp/libil2cpp)\n\n# there are different codegens, so dependending on which is used, the id changes\ntarget_include_directories(${COMPILE_ID} PRIVATE ${EXTERN_DIR}/includes/${CODEGEN_ID}/include)\n\n# libs dir -> stores .so or .a files (or symlinked!)\ntarget_link_directories(${COMPILE_ID} PRIVATE ${EXTERN_DIR}/libs)\n\nRECURSE_FILES(so_list ${EXTERN_DIR}/libs/*.so)\nRECURSE_FILES(a_list ${EXTERN_DIR}/libs/*.a)\n\n# every .so or .a that needs to be linked, put here!\n# I don't believe you need to specify if a lib is static or not, poggers!\ntarget_link_libraries(${COMPILE_ID} PRIVATE\n\t${so_list}\n\t${a_list}\n)"
            .as_bytes(),
        )
        .expect("Failed to write out extern cmake file");
    }

    pub fn write_define_cmake(&self) {
        let mut defines_cmake_file = std::fs::File::create("qpm_defines.cmake")
            .expect("Failed to create defines cmake file");

        defines_cmake_file
            .write_all(self.make_defines_string().as_bytes())
            .expect("Failed to write out own define make string");
    }

    pub fn make_defines_string(&self) -> String {
        let mut result = String::new();

        result.push_str("# YOU SHOULD NOT MANUALLY EDIT THIS FILE, QPM WILL VOID ALL CHANGES\n# Version defines, pretty useful\n");
        result.push_str(&format!(
            "set(MOD_VERSION \"{}\")\n",
            self.config.info.version.to_string()
        ));
        result.push_str("# take the mod name and just remove spaces, that will be MOD_ID, if you don't like it change it after the include of this file\n");
        result.push_str(&format!(
            "set(MOD_ID \"{}\")\n\n",
            self.config.info.name.replace(' ', "")
        ));
        result.push_str("# derived from override .so name or just id_version\n");
        result.push_str(&format!(
            "set(COMPILE_ID \"{}\")\n",
            self.config.get_module_id()
        ));
        result.push_str(
            "# derived from whichever codegen package is installed, will default to just codegen\n",
        );
        result.push_str(&format!(
            "set(CODEGEN_ID \"{}\")\n\n",
            if let Some(codegen_dep) = self
                .restored_dependencies
                .iter()
                .find(|dep| dep.dependency.id.contains("codegen"))
            {
                // found a codegen
                &codegen_dep.dependency.id
            } else {
                "codegen"
            }
        ));

        result.push_str("# given from qpm, automatically updated from qpm.json\n");
        result.push_str(&format!(
            "set(EXTERN_DIR_NAME \"{}\")\n",
            self.config.dependencies_dir.display()
        ));
        result.push_str(&format!(
            "set(SHARED_DIR_NAME \"{}\")\n\n",
            self.config.shared_dir.display()
        ));

        result.push_str("# if no target given, use Debug\nif (NOT DEFINED CMAKE_BUILD_TYPE)\n\tset(CMAKE_BUILD_TYPE \"Debug\")\nendif()\n\n");
        result.push_str("# defines used in ninja / cmake ndk builds\nif (NOT DEFINED CMAKE_ANDROID_NDK)\n\tif(DEFINED ENV{ANDROID_NDK_ROOT})\n\t\tset(CMAKE_ANDROID_NDK ENV{ANDROID_NDK_ROOT})\n\telse()\n\t\tfile (STRINGS \"ndkpath.txt\" CMAKE_ANDROID_NDK)\n\tendif()\nendif()\nstring(REPLACE \"\\\\\" \"/\" CMAKE_ANDROID_NDK ${CMAKE_ANDROID_NDK})\n\nset(ANDROID_PLATFORM 24)\nset(ANDROID_ABI arm64-v8a)\nset(ANDROID_STL c++_static)\n\nset(CMAKE_TOOLCHAIN_FILE ${CMAKE_ANDROID_NDK}/build/cmake/android.toolchain.cmake)\n\n");
        result.push_str("# define used for external data, mostly just the qpm dependencies\nset(EXTERN_DIR ${CMAKE_CURRENT_SOURCE_DIR}/${EXTERN_DIR_NAME})\nset(SHARED_DIR ${CMAKE_CURRENT_SOURCE_DIR}/${SHARED_DIR_NAME})\n\n");
        result.push_str("# get files by filter recursively\nMACRO(RECURSE_FILES return_list filter)\n\tFILE(GLOB_RECURSE new_list ${filter})\n\tSET(file_list \"\")\n\tFOREACH(file_path ${new_list})\n\t\tSET(file_list ${file_list} ${file_path})\n\tENDFOREACH()\n\tLIST(REMOVE_DUPLICATES file_list)\n\tSET(${return_list} ${file_list})\nENDMACRO()");

        result
    }
}
