use crate::data::{package::PackageConfig, shared_package::SharedPackageConfig};

pub fn execute_restore_operation() {
    println!("package should be restoring");
    let package = PackageConfig::read();
    let shared_package = SharedPackageConfig::from_package(&package);

    std::fs::create_dir_all("src").expect("Failed to create directory");
    std::fs::create_dir_all("include").expect("Failed to create directory");
    std::fs::create_dir_all(&shared_package.config.shared_dir).expect("Failed to create directory");

    shared_package.restore();

    /*
    if let Some(mut android_mk) = AndroidMk::read() {
        android_mk.update_shared_package(&shared_package);
        android_mk.write();
    }
    */

    shared_package.write();
}
