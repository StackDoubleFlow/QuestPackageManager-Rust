use crate::data::package::{PackageConfig};

pub fn execute_publish_operation()
{
    let package = PackageConfig::read();
    if package.info.url.is_none()
    {
        println!("Package without url can not publish!");
        return;
    }
    println!("package should now be published");
}