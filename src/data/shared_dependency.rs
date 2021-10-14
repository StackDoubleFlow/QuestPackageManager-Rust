use serde::{Serialize, Deserialize};
use symlink::*;
use fs_extra::dir::copy as copy_directory;
use crate::data::{
    package::PackageConfig,
    shared_package::SharedPackageConfig,
    dependency::Dependency,
    qpackages,
    config::{Config, get_keyring}
};

use std::{
    collections::{HashMap, hash_map::DefaultHasher},
    hash::{Hash, Hasher},
    io::{Cursor, Read, Write},
};

use semver::{Version};
use owo_colors::*;
use duct::cmd;
use zip::ZipArchive;

#[derive(Serialize, Deserialize, Clone, Debug, Hash, Eq, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SharedDependency {
    pub dependency: Dependency,
    pub version: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubReleaseAsset {
    pub url: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubReleaseData {
    pub assets: Vec<GithubReleaseAsset>
}
#[allow(dead_code)]
impl SharedDependency {
    pub fn get_shared_package(&self) -> SharedPackageConfig
    {
        qpackages::get_shared_package(&self.dependency.id, &self.version)
    }

    pub fn collect(&self, this_id: &str, collected: &mut HashMap<SharedDependency, SharedPackageConfig>)
    {
        if this_id.eq(&self.dependency.id)
        {
            return;
        }

        if !collected.contains_key(self)
        {
            let mut found = false;

            // some need to be removed and imo this is the easiest way to do it because borrowing is a bitch and I hate it
            collected.retain(|shared_dependency, _| {
                // if the passed thing is the same ID as self
                if shared_dependency.dependency.id.eq(&self.dependency.id)
                {
                    // if it has an override so name
                    if let Some(override_so_name) = shared_dependency.get_shared_package().config.info.additional_data.override_so_name {
                        // if self has an override so name
                        if let Some(self_override_so_name) = self.get_shared_package().config.info.additional_data.override_so_name {
                            // if they are the same
                            if override_so_name.to_lowercase().eq(&self_override_so_name.to_lowercase())
                            {
                                // if self version is higher, remove it (and we add it back later)
                                if Version::parse(&self.version).unwrap() > Version::parse(&shared_dependency.version).unwrap()
                                {
                                    return false;
                                }
                                else
                                {
                                    // we found a good dep, we don't need to add this one again
                                    found = true;
                                }
                            }
                        }
                    }
                    // if id == id (checked before) & version is exact same, don't add it again
                    else if self.version.eq(&shared_dependency.version)
                    {
                        found = true;
                    }
                }
                
                true
            });

            // if we didn't find a valid version
            if !found
            {
                // get shared package to add to the hashmap
                let mut shared_package = self.get_shared_package();
                
                // remove private deps
                shared_package.restored_dependencies.retain(|restored_dependency| {
                    if let Some(is_private) = restored_dependency.dependency.additional_data.is_private {
                        return !is_private;
                    }
        
                    true
                });
                
                // insert into hashmap
                collected.insert(self.clone(), shared_package.clone());
                
                // collect all the shared deps
                for shared_dependency in shared_package.restored_dependencies.iter()
                {
                    shared_dependency.collect(&shared_package.config.info.id, collected);
                }
            }
        }
    }

    pub fn get_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }

    pub fn cache(&self)
    {
        // check if current version already cached
        // if not, git clone (using token?)
        // else return
        // make sure to keep cache location in mind (settings)
        let config = Config::read_combine();
        println!("Checking cache for dependency {} {}", self.dependency.id.bright_red(), self.version.bright_green());
        let path = format!("{}/{}/{}/src", config.cache.unwrap(), self.dependency.id, self.version);
        let path_data = std::path::Path::new(&path);

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
                if let Some (branch) = shared_package.config.info.additional_data.branch_name {
                    cmd!("git", "clone", format!("{}.git", url), &path, "--branch", branch, "--depth", "1", "--recurse-submodules", "--shallow-submodules", "--quiet").stdout_capture().stderr_capture().run().expect("running git clone failed!");
                } else {
                    println!("No branch name found, cloning default branch");
                    cmd!("git", "clone", format!("{}.git", url), &path, "--depth", "1", "--recurse-submodules", "--shallow-submodules", "--quiet").stdout_capture().stderr_capture().run().expect("running git clone failed!");
                }

                println!("Downloading .so file...");
                // header only not defined, or it's false
                if shared_package.config.info.additional_data.headers_only.is_none() || !shared_package.config.info.additional_data.headers_only.unwrap() {
                    // get download link to use
                    let mut so_download: String;
                    // if debug link defined, use that to link against
                    if let Some(debug_so_link) = shared_package.config.info.additional_data.debug_so_link {
                        so_download = debug_so_link;
                    // if that didn't exist, get the normal .so
                    } else if let Some(so_link) = shared_package.config.info.additional_data.so_link {
                        so_download = so_link;
                    } else {
                        println!("Package was not header only, but did not specify a (debug) so link, this must be a mistake, please contact the author of the package!");
                        return;
                    }

                    if let Some(gitidx_so) = so_download.find("github.com") {
                        
                        let filename: String;
                        if let Some(override_name) = shared_package.config.info.additional_data.override_so_name {
                            filename = override_name;
                        } else {
                            filename = format!("lib{}_{}.so", self.dependency.id, self.version.replace('.', "_"));
                        }

                        // github url, probably release
                        if let Ok(token) = get_keyring().get_password() {
                            // had token, use it!
                            // download url for a private thing: still need to get asset id!
                            // from this: "https://github.com/Gorilla-Tag-Modding-Group/MonkeCodegen/releases/download/v0.9.1/libmonkecodegen.so"
                            // to this: "https://$TOKEN@api.github.com/repos/$USER/$REPO/releases/assets/$ASSET_ID" -o $FILENAME
                            let mut asset_data_link = so_download.clone();
                            
                            asset_data_link.insert_str(gitidx_so, &format!("{}@api.", &token));
                            //https://$TOKEN@api.github.com/$USER/$REPO/releases/download/$TAG/$FILENAME
                            asset_data_link = asset_data_link.replace("github.com/", "github.com/repos/");
                            //https://$TOKEN@api.github.com/repos/$USER/$REPO/releases/download/$TAG/$FILENAME
                            asset_data_link = asset_data_link.replace("/download/", "/tags/");
                            //https://$TOKEN@api.github.com/repos/$USER/$REPO/releases/tags/$TAG/$FILENAME
                            let last_slash = asset_data_link.rfind('/').unwrap();
                            asset_data_link = asset_data_link[..last_slash].to_string();
                            //https://$TOKEN@api.github.com/repos/$USER/$REPO/releases/tags/$TAG

                            let data = ureq::get(&asset_data_link).call().unwrap().into_json::<GithubReleaseData>().unwrap();

                            for asset in data.assets.iter() {
                                if asset.name == filename {
                                    // this is the correct asset!
                                    so_download = asset.url.replace("api.github.com", &format!("{}@api.github.com", token));
                                }
                            }
                        } else {
                            // token not available, try to cache without using
                            println!("No github token found, private releases will not download!");
                        }
                        let mut buffer = Vec::new();
                        ureq::get(&so_download).set("Accept", "application/octet-stream").call().unwrap().into_reader().read_to_end(&mut buffer).unwrap();

                        let lib_path = path.replace("/src", "/libs");
                        
                        std::fs::create_dir_all(&lib_path).expect("Could not create libs folder");

                        let mut file = std::fs::File::create(&format!("{}/{}", lib_path, filename)).expect("Failed to create lib file");
                        file.write_all(&buffer).expect("Failed to write out .so file");
                    } else {
                        // not a git url, just straight download
                    }
                }
            } else {
                // it was not a github url, probably a zipped file download
                println!("Url was not a github url, assuming it's a zipped file download...");
                let mut buffer = Cursor::new(Vec::new());
                ureq::get(&url).call().unwrap().into_reader().read_to_end(buffer.get_mut()).unwrap();
                ZipArchive::new(buffer).unwrap().extract(&path).unwrap();
            }
        } else {
            println!("Path {} existed! no need to cache...", &path.bright_yellow());
            // found it, do nothing!
        }
    }

    pub fn restore_from_cache(&self)
    {
        // restore from cached files, give error on fail (nonexistent?)
        // make sure to check the symlink setting (can we even do that in rust ?)
        // also keep cache location in mind

        if Config::read_combine().symlink.unwrap() {
            self.restore_from_cache_symlink();
        } else {
            self.restore_from_cache_copy();
        }
    }

    pub fn collect_to_copy(&self) -> Vec<(String, String)>
    {
        let config = Config::read_combine();
        let package = PackageConfig::read();
        let shared_package = self.get_shared_package();
        let base_path = format!("{}/{}/{}", config.cache.unwrap(), self.dependency.id, self.version);
        let src_path = format!("{}/src", &base_path);
        let libs_path = format!("{}/libs", &base_path);
        let local_path = format!("{}/{}", &package.dependencies_dir, self.dependency.id);

        let so_name: String;
        if let Some(override_so_name) = shared_package.config.info.additional_data.override_so_name {
            so_name = override_so_name;
        } else {
            so_name = format!("lib{}_{}.so", self.dependency.id, self.version.replace('.', "_"));
        }

        let mut to_copy = Vec::new();
        // if not headers only, copy over .so file
        if shared_package.config.info.additional_data.headers_only.is_none() || !shared_package.config.info.additional_data.headers_only.unwrap() {
            let lib_so_path = format!("{}/{}", &libs_path, &so_name);
            let local_so_path = format!("{}/{}", &package.dependencies_dir, &so_name);
            // from to
            to_copy.push((lib_so_path, local_so_path));
        }
        
        // copy  shared / include over
        let cache_shared_path = format!("{}/{}", src_path, shared_package.config.shared_dir);
        let shared_path = format!("{}/{}", local_path, shared_package.config.shared_dir);
        to_copy.push((cache_shared_path, shared_path));

        if let Some(extra_files) = &self.dependency.additional_data.extra_files {
            for entry in extra_files.iter() {
                let cache_entry_path = format!("{}/{}", src_path, entry);
                let entry_path = format!("{}/{}", local_path, entry);
                to_copy.push((cache_entry_path, entry_path));
            }
        }

        to_copy
    }

    pub fn restore_from_cache_symlink(&self)
    {
        let to_copy = self.collect_to_copy();
        // sort out issues with the symlinking, stuff is being symlinked weirdly
        for (from_str, to_str) in to_copy.iter() {
            let from = std::path::Path::new(&from_str);
            let to = std::path::Path::new(&to_str);

            if let Err(e) = symlink_auto(&from, &to) {
                println!("Failed to create symlink: {}\nfalling back to copy, did the link already exist, or are you not running qpm as adminstrator?", e.bright_red());
                if from.is_dir() {
                std::fs::create_dir_all(&to).expect("Failed to create destination folder");
                let mut options = fs_extra::dir::CopyOptions::new();
                    options.overwrite = true;
                    options.copy_inside = true;
                    options.content_only = true;
                    copy_directory(&from, &to, &options).expect("Failed to copy directory!");
                } else if from.is_file() {
                    std::fs::copy(&from, &to).expect("Failed to copy file!");
                }
            }
        }
    }

    pub fn restore_from_cache_copy(&self)
    {
        // get the files to copy
        let to_copy = self.collect_to_copy();
        for (from_str, to_str) in to_copy.iter() {
            let from = std::path::Path::new(&from_str);
            let to = std::path::Path::new(&to_str);
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