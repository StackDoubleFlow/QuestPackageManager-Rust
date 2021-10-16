use crate::data::{
    android_mk::AndroidMk, package::PackageConfig, shared_package::SharedPackageConfig,
};

pub fn execute_restore_operation() {
    println!("package should be restoring");
    let package = PackageConfig::read();
    let shared_package = SharedPackageConfig::from_package(package);

    shared_package.restore();

    let android_mk = AndroidMk::read();
    // TODO: Edit android.mk here!
    // TODO: Do we switch to cmake now ?

    android_mk.write();
    shared_package.write();
}
