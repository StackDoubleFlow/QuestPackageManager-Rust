use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    io::{Cursor, Read, Write},
    path::{Path, PathBuf},
};

use fs_extra::dir::copy as copy_directory;
use owo_colors::OwoColorize;
use semver::Version;
use serde::{Deserialize, Serialize};
use zip::ZipArchive;

use crate::{
    data::{
        config::Config,
        dependency::Dependency,
        package::{PackageConfig, SharedPackageConfig},
        qpackages,
    },
    utils::git,
};

#[derive(Serialize, Deserialize, Clone, Debug, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SharedDependency {
    pub dependency: Dependency,
    pub version: Version,
}

impl SharedDependency {
    pub fn get_shared_package(&self) -> SharedPackageConfig {
        qpackages::get_shared_package(&self.dependency.id, &self.version)
    }

    pub fn get_so_name(&self) -> String {
        self.dependency
            .additional_data
            .override_so_name
            .clone()
            .unwrap_or(format!(
                "lib{}_{}.{}",
                self.dependency.id,
                self.version.to_string().replace('.', "_"),
                if self
                    .dependency
                    .additional_data
                    .static_linking
                    .unwrap_or(false)
                {
                    "a"
                } else {
                    "so"
                },
            ))
    }

    pub fn get_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }

    pub fn cache2electricboogaloo(&self) {
        // Check if already cached
        // if true, don't download repo / header files
        // else cache to tmp folder in package id folder @ cache path
        //          git repo -> git clone w/ or without github token
        //          not git repo (no github.com) -> assume it's a zip
        //          !! HANDLE SUBFOLDER FROM TMP, OR IF NO SUBFOLDER JUST RENAME TMP TO SRC !!
        //          -- now we have the header files --
        // Check if .so files are downloaded, if not:
        // Download release .so and possibly debug .so to libs folder, if from github use token if available
        // Now it should be cached!

        println!(
            "Checking cache for dependency {} {}",
            self.dependency.id.bright_red(),
            self.version.bright_green()
        );
        let config = Config::read_combine();
        let base_path = config
            .cache
            .unwrap()
            .join(&self.dependency.id)
            .join(self.version.to_string());

        let src_path = base_path.join("src");
        let lib_path = base_path.join("lib");
        let tmp_path = base_path.join("tmp");

        let shared_package = self.dependency.get_shared_package().unwrap();

        let so_path = lib_path.join(shared_package.config.get_so_name());
        let debug_so_path = lib_path.join(format!("debug_{}", shared_package.config.get_so_name()));

        // Downloads the repo / zip file into src folder w/ subfolder taken into account
        if !src_path.exists() {
            // src did not exist, this means that we need to download the repo/zip file from packageconfig.info.url
            std::fs::create_dir_all(&src_path.parent().unwrap())
                .expect("Failed to create lib path");
            let url = shared_package.config.info.url.unwrap();
            if url.contains("github.com") {
                // github url!
                git::clone(
                    url,
                    shared_package.config.info.additional_data.branch_name,
                    &tmp_path,
                );
            } else {
                // not a github url, assume it's a zip
                let mut buffer = Cursor::new(Vec::new());
                ureq::get(&url)
                    .call()
                    .unwrap()
                    .into_reader()
                    .read_to_end(buffer.get_mut())
                    .unwrap();
                // Extract to tmp folder
                ZipArchive::new(buffer).unwrap().extract(&tmp_path).unwrap();
            }
            // the only way the above if else would break is if someone put a link to a zip file on github in the url slot
            // if you are reading this and think of doing that so I have to fix this, fuck you

            let from_path =
                if let Some(sub_folder) = shared_package.config.info.additional_data.sub_folder {
                    // the package exists in a subfolder of the downloaded thing, just move the subfolder to src
                    tmp_path.join(sub_folder)
                } else {
                    // the downloaded thing IS the package, just rename the folder to src
                    tmp_path.clone()
                };
            if from_path.exists() {
                println!(
                    "from: {}\nto: {}",
                    from_path.display().bright_yellow(),
                    src_path.display().bright_yellow()
                );

                //if from_path == tmp_path {
                //    std::fs::rename(from_path, src_path);
                //} else {
                //    let mut options = fs_extra::dir::CopyOptions::new();
                //    options.overwrite = true;
                //    options.copy_inside = true;
                //    options.content_only = true;
                //    copy_directory(&from, &to, &options).expect("Failed to copy directory!");
                //}

                //std::fs::create_dir_all(&src_path)
                //    .expect("Failed to create destination src directory");
                //let options = fs_extra::dir::CopyOptions::new();
                std::fs::rename(&from_path, src_path).expect("Failed to move folder");
            } else {
                panic!("Failed to restore folder for this dependency\nif you have a token configured check if it's still valid\nIf it is, check if you can manually reach the repo");
            }

            // clear up tmp folder
            if tmp_path.exists() {
                std::fs::remove_dir_all(tmp_path).expect("Failed to remove tmp folder");
            }
        }

        if !lib_path.exists() {
            std::fs::create_dir_all(&lib_path).expect("Failed to create lib path");
            // libs didn't exist or the release object didn't exist, we need to download from packageconfig.info.additional_data.so_link and packageconfig.info.additional_data.debug_so_link
            if !so_path.exists() {
                if let Some(so_link) = shared_package.config.info.additional_data.so_link {
                    // so_link existed, download
                    if so_link.contains("github.com") {
                        // github url!
                        git::get_release(so_link, &so_path);
                    } else {
                        // other dl link, assume it's a raw lib file download
                        let mut buffer = Cursor::new(Vec::new());
                        ureq::get(&so_link)
                            .call()
                            .unwrap()
                            .into_reader()
                            .read_to_end(buffer.get_mut())
                            .unwrap();
                        let mut file =
                            std::fs::File::create(so_path).expect("create so file failed");
                        file.write_all(&*buffer.into_inner())
                            .expect("Failed to write out downloaded bytes");
                    }
                }
            }

            if !debug_so_path.exists() {
                if let Some(debug_so_link) =
                    shared_package.config.info.additional_data.debug_so_link
                {
                    // debug_so_link existed, download
                    if debug_so_link.contains("github.com") {
                        // github url!
                        git::get_release(debug_so_link, &debug_so_path);
                    } else {
                        // other dl link, assume it's a raw lib file download
                        let mut buffer = Cursor::new(Vec::new());
                        ureq::get(&debug_so_link)
                            .call()
                            .unwrap()
                            .into_reader()
                            .read_to_end(buffer.get_mut())
                            .unwrap();
                        let mut file =
                            std::fs::File::create(debug_so_path).expect("create so file failed");
                        file.write_all(&*buffer.into_inner())
                            .expect("Failed to write out downloaded bytes");
                    }
                }
            }
        }
    }

    pub fn restore_from_cache(&self, also_lib: bool) {
        // restore from cached files, give error on fail (nonexistent?)
        if Config::read_combine().symlink.unwrap_or(false) {
            self.restore_from_cache_symlink(also_lib);
        } else {
            self.restore_from_cache_copy(also_lib);
        }
    }

    pub fn collect_to_copy(&self, also_lib: bool) -> Vec<(PathBuf, PathBuf)> {
        // TODO: Look into improving the way it gets all the things to copy
        let config = Config::read_combine();
        let package = PackageConfig::read();
        let shared_package = self.get_shared_package();

        let base_path = config
            .cache
            .unwrap()
            .join(&self.dependency.id)
            .join(self.version.to_string());
        let src_path = base_path.join("src");
        let libs_path = base_path.join("libs");
        let dependencies_path = Path::new(&package.dependencies_dir);
        std::fs::create_dir_all(dependencies_path).unwrap();
        let dependencies_path = dependencies_path.canonicalize().unwrap().join("includes");
        let local_path = dependencies_path.join(&self.dependency.id);
        let mut to_copy = Vec::new();
        if also_lib {
            let so_name: String;
            if let Some(override_so_name) =
                shared_package.config.info.additional_data.override_so_name
            {
                so_name = override_so_name;
            } else {
                so_name = format!(
                    "lib{}_{}.so",
                    self.dependency.id,
                    self.version.to_string().replace('.', "_")
                );
            }

            // if not headers only, copy over .so file
            if shared_package
                .config
                .info
                .additional_data
                .headers_only
                .is_none()
                || !shared_package
                    .config
                    .info
                    .additional_data
                    .headers_only
                    .unwrap()
            {
                let lib_so_path = libs_path.join(&so_name);
                let local_so_path = Path::new(&package.dependencies_dir)
                    .canonicalize()
                    .unwrap()
                    .join("libs")
                    .join(&so_name);
                // from to
                to_copy.push((lib_so_path, local_so_path));
            }
        }
        // copy  shared / include over
        let cache_shared_path = src_path.join(&shared_package.config.shared_dir);
        let shared_path = local_path.join(&shared_package.config.shared_dir);
        to_copy.push((cache_shared_path, shared_path));

        if let Some(extra_files) = &self.dependency.additional_data.extra_files {
            for entry in extra_files.iter() {
                let cache_entry_path = src_path.join(entry);
                let entry_path = local_path.join(entry);
                to_copy.push((cache_entry_path, entry_path));
            }
        }

        to_copy
    }

    pub fn restore_from_cache_symlink(&self, also_lib: bool) {
        let to_copy = self.collect_to_copy(also_lib);
        // sort out issues with the symlinking, stuff is being symlinked weirdly
        for (from, to) in to_copy.iter() {
            // make sure to parent dir exists!
            std::fs::create_dir_all(to.parent().unwrap()).ok();
            if let Err(e) = symlink::symlink_auto(&from, &to) {
                #[cfg(windows)]
                println!("Failed to create symlink: {}\nfalling back to copy, did the link already exist, or did you not enable windows dev mode?\nTo disable this warning (and default to copy), use the command {}", e.bright_red(), "qpm config symlink disable".bright_yellow());
                #[cfg(not(windows))]
                println!("Failed to create symlink: {}\nfalling back to copy, did the link already exist?\nTo disable this warning (and default to copy), use the command {}", e.bright_red(), "qpm config symlink disable".bright_yellow());

                if from.is_dir() {
                    let mut options = fs_extra::dir::CopyOptions::new();
                    options.overwrite = true;
                    options.copy_inside = true;
                    options.content_only = true;
                    copy_directory(&from, &to, &options).expect("Failed to copy directory!");
                } else if from.is_file() {
                    // we can get the parent beccause this is a file path
                    std::fs::create_dir_all(&to.parent().unwrap())
                        .expect("Failed to create containing directory");
                    std::fs::copy(&from, &to).expect("Failed to copy file!");
                }
            }
        }
    }

    pub fn restore_from_cache_copy(&self, also_lib: bool) {
        // get the files to copy
        let to_copy = self.collect_to_copy(also_lib);
        for (from_str, to_str) in to_copy.iter() {
            let from = Path::new(&from_str);
            let to = Path::new(&to_str);
            // if dir, make sure it exists
            if from.is_dir() {
                std::fs::create_dir_all(&to).expect("Failed to create destination folder");
                let mut options = fs_extra::dir::CopyOptions::new();
                options.overwrite = true;
                options.copy_inside = true;
                options.content_only = true;
                // copy it over
                copy_directory(&from, &to, &options).expect("Failed to copy directory!");
            } else if from.is_file() {
                // if it's a file, copy that over instead
                std::fs::copy(&from, &to).expect("Failed to copy file!");
            }
        }
    }
}
