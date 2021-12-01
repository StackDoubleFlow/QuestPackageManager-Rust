use std::io::Write;

use crate::data::{
    config::Config, mod_json::ModJson, package::PackageConfig, shared_package::SharedPackageConfig,
};
pub fn execute_restore_operation() {
    println!("package should be restoring");
    let package = PackageConfig::read();
    let shared_package = SharedPackageConfig::from_package(&package);

    // create used dirs
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

    shared_package.restore();
    shared_package.write();

    // make mod.json if it doesn't exist
    let mod_json: ModJson = shared_package.into();
    if !std::path::Path::new("mod.json").exists() {
        mod_json.write();
    } else {
        // Update mod.json from current shared_package, pretty sure it's done but could be bad
        let mut existing_json = ModJson::read();

        existing_json.mod_files = mod_json.mod_files;
        existing_json.dependencies = mod_json.dependencies;
        existing_json.library_files = mod_json.library_files;
        existing_json.id = mod_json.id;
        existing_json.version = mod_json.version;

        existing_json.write();
    }
}
