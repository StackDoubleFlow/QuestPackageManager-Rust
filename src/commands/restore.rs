use crate::data::shared_package::SharedPackageConfig;
use crate::data::package::PackageConfig;

pub fn execute_restore_operation()
{
    println!("package should be restoring");
    let package = PackageConfig::read();
    let shared_package = SharedPackageConfig {config: package, ..Default::default()};

    shared_package.write();
}