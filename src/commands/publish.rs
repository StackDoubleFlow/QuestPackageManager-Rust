use crate::data::shared_package::{SharedPackageConfig};

pub fn execute_publish_operation()
{
    let package = SharedPackageConfig::read();
    if package.config.info.url.is_none()
    {
        println!("Package without url can not publish!");
        return;
    }

    package.publish();
    println!("package should now be published");
}