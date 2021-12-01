use crate::data::shared_package::SharedPackageConfig;

pub fn execute_publish_operation() {
    let package = SharedPackageConfig::read();
    if package.config.info.url.is_none() {
        println!("Package without url can not publish!");
        return;
    }

    for dependency in package.config.dependencies.iter() {
        match dependency.get_shared_package() {
            Option::Some(_s) => {}
            Option::None => {
                println!(
                    "dependency {} was not available on qpackages in the given version range",
                    &dependency.id
                );
                println!(
                    "make sure {} exists for this dependency",
                    &dependency.version_range
                );
                std::process::exit(0);
            }
        };
    }

    package.publish();
    println!("package should now be published");
}
