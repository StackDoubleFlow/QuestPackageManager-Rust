use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    io::{Cursor, Read, Write},
    path::{Path, PathBuf},
};

use duct::cmd;
use fs_extra::dir::copy as copy_directory;
use owo_colors::OwoColorize;
use semver::Version;
use serde::{Deserialize, Serialize};
use zip::ZipArchive;

use crate::data::{
    config::{get_keyring, Config},
    dependency::Dependency,
    package::PackageConfig,
    qpackages,
    shared_package::SharedPackageConfig,
};

#[derive(Serialize, Deserialize, Clone, Debug, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SharedDependency {
    pub dependency: Dependency,
    pub version: Version,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubReleaseAsset {
    pub url: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubReleaseData {
    pub assets: Vec<GithubReleaseAsset>,
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

    pub fn cache(&self) {
        // TODO: This method is cringe and needs to be redone
        // check if current version already cached
        // if not, git clone (using token?)
        // else return
        // make sure to keep cache location in mind (settings)
        let config = Config::read_combine();
        println!(
            "Checking cache for dependency {} {}",
            self.dependency.id.bright_red(),
            self.version.bright_green()
        );
        let path = config
            .cache
            .unwrap()
            .join(&self.dependency.id)
            .join(self.version.to_string())
            .join("src");
        let path_data = Path::new(&path);

        if !path_data.exists() {
            std::fs::create_dir_all(&path).expect("Failed to create directory");
            let shared_package = self.dependency.get_shared_package().unwrap();
            let mut url: String;
            if let Some(url_val) = shared_package.config.info.url {
                // url is of format https://github.com/USER/REPO
                url = url_val;
            } else {
                println!("Url for package {} was not set, contact the person who uploaded it so they can fix that!", &shared_package.config.info.id.bright_yellow());
                std::process::exit(0);
            }

            // didn't find it -> download it!
            if let Some(gitidx) = url.find("github.com") {
                if let Ok(token) = get_keyring().get_password() {
                    // had token, use it!
                    url.insert_str(gitidx, &format!("{}@", &token));
                } else {
                    // token not available, try to cache without using
                    println!("No github token found, private repos will not restore!");
                }

                println!("Cloning git repo...");
                // git clone
                if let Some(branch) = shared_package.config.info.additional_data.branch_name {
                    cmd!(
                        "git",
                        "clone",
                        format!("{}.git", url),
                        &path,
                        "--branch",
                        branch,
                        "--depth",
                        "1",
                        "--recurse-submodules",
                        "--shallow-submodules",
                        "--quiet"
                    )
                    .stdout_capture()
                    .stderr_capture()
                    .run()
                    .expect("running git clone failed!");
                } else {
                    println!("No branch name found, cloning default branch");
                    cmd!(
                        "git",
                        "clone",
                        format!("{}.git", url),
                        &path,
                        "--depth",
                        "1",
                        "--recurse-submodules",
                        "--shallow-submodules",
                        "--quiet"
                    )
                    .stdout_capture()
                    .stderr_capture()
                    .run()
                    .expect("running git clone failed!");
                }

                println!("Downloading .so file...");
                // header only not defined, or it's false
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
                    // TODO: Download both .so files since users can decide to use release version in qpm.json
                    // get download link to use
                    let mut so_download: String;
                    // if debug link defined, use that to link against
                    if let Some(debug_so_link) =
                        shared_package.config.info.additional_data.debug_so_link
                    {
                        so_download = debug_so_link;
                    // if that didn't exist, get the normal .so
                    } else if let Some(so_link) = shared_package.config.info.additional_data.so_link
                    {
                        so_download = so_link;
                    } else {
                        println!("Package was not header only, but did not specify a (debug) so link, this must be a mistake, please contact the author of the package!");
                        return;
                    }

                    if let Some(gitidx_so) = so_download.find("github.com") {
                        let filename = if let Some(override_name) =
                            shared_package.config.info.additional_data.override_so_name
                        {
                            override_name
                        } else {
                            format!(
                                "lib{}_{}.so",
                                self.dependency.id,
                                self.version.to_string().replace('.', "_")
                            )
                        };

                        // TODO: Improve the way it gets the token url so it doesn't smell as much
                        // github url, probably release
                        if let Ok(token) = get_keyring().get_password() {
                            // had token, use it!
                            // download url for a private thing: still need to get asset id!
                            // from this: "https://github.com/Gorilla-Tag-Modding-Group/MonkeCodegen/releases/download/v0.9.1/libmonkecodegen.so"
                            // to this: "https://$TOKEN@api.github.com/repos/$USER/$REPO/releases/assets/$ASSET_ID" -o $FILENAME
                            let mut asset_data_link = so_download.clone();

                            asset_data_link.insert_str(gitidx_so, &format!("{}@api.", &token));
                            //https://$TOKEN@api.github.com/$USER/$REPO/releases/download/$TAG/$FILENAME
                            asset_data_link =
                                asset_data_link.replace("github.com/", "github.com/repos/");
                            //https://$TOKEN@api.github.com/repos/$USER/$REPO/releases/download/$TAG/$FILENAME
                            asset_data_link = asset_data_link.replace("/download/", "/tags/");
                            //https://$TOKEN@api.github.com/repos/$USER/$REPO/releases/tags/$TAG/$FILENAME
                            let last_slash = asset_data_link.rfind('/').unwrap();
                            asset_data_link = asset_data_link[..last_slash].to_string();
                            //https://$TOKEN@api.github.com/repos/$USER/$REPO/releases/tags/$TAG

                            let data = ureq::get(&asset_data_link)
                                .call()
                                .unwrap()
                                .into_json::<GithubReleaseData>()
                                .unwrap();

                            for asset in data.assets.iter() {
                                if asset.name == filename {
                                    // this is the correct asset!
                                    so_download = asset.url.replace(
                                        "api.github.com",
                                        &format!("{}@api.github.com", token),
                                    );
                                }
                            }
                        }

                        let mut buffer = Vec::new();
                        ureq::get(&so_download).set("Accept", "application/octet-stream").call().expect("Failed to download release artifact, make sure the link is correct, or if it's private configure a github token").into_reader().read_to_end(&mut buffer).unwrap();

                        let lib_path = path.parent().unwrap().join("libs");

                        std::fs::create_dir_all(&lib_path).expect("Could not create libs folder");

                        let mut file = std::fs::File::create(lib_path.join(filename))
                            .expect("Failed to create lib file");
                        file.write_all(&buffer)
                            .expect("Failed to write out .so file");
                    } else {
                        // not a git url, just straight download for .so
                        // TODO: Actually Implement
                    }
                }
            } else {
                // this is a header only download
                // not a git url, just straight download
                // it was not a github url, probably a zipped file download
                println!("Url was not a github url, assuming it's a zipped file download...");
                let mut buffer = Cursor::new(Vec::new());
                ureq::get(&url)
                    .call()
                    .unwrap()
                    .into_reader()
                    .read_to_end(buffer.get_mut())
                    .unwrap();
                ZipArchive::new(buffer).unwrap().extract(&path).unwrap();
            }
        } else {
            println!(
                "Path {} existed! no need to cache...",
                &path.display().bright_yellow()
            );
            // found it, do nothing!
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
