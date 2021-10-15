use crate::data::shared_package::SharedPackageConfig;
use crate::data::package::PackageConfig;
use crate::data::android_mk::AndroidMk;
pub fn execute_restore_operation()
{
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