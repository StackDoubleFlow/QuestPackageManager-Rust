use std::io::Write;

use crate::data::{config::Config, package::PackageConfig, shared_package::SharedPackageConfig};

pub fn execute_restore_operation() {
    println!("package should be restoring");
    let package = PackageConfig::read();
    let shared_package = SharedPackageConfig::from_package(&package);

    std::fs::create_dir_all("src").expect("Failed to create directory");
    std::fs::create_dir_all("include").expect("Failed to create directory");
    std::fs::create_dir_all(&shared_package.config.shared_dir).expect("Failed to create directory");

    // write the ndk path to a file if available
    let config = Config::read_combine();
    if let Some(ndk_path) = config.ndk_path {
        let mut file = std::fs::File::create("ndkpath.txt").expect("Failed to create ndkpath.txt");
        file.write_all(ndk_path.as_bytes())
            .expect("Failed to write out ndkpath.txt");
    }

    if !std::path::Path::new("mod.json").exists() {
        let mod_json: crate::data::mod_json::ModJson = shared_package.clone().into();
        let mut mod_json_file = std::fs::File::create("mod.json").unwrap();
        mod_json_file
            .write_all(serde_json::to_string_pretty(&mod_json).unwrap().as_bytes())
            .unwrap();
    }

    shared_package.restore();
    shared_package.write();
}
