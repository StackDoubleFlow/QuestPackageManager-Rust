use crate::data::package;
use owo_colors::*;

pub fn execute_collect_operation()
{
    let package = package::PackageConfig::read();
    let dependencies = package.collect();

    println!("Collecting package {}", package.info.id.bright_purple());

    for (dep, config) in dependencies.iter()
    {
        println!("{}: ({}) --> {} (config: {}, {} restored dependencies)", &dep.dependency.id.bright_red(), &dep.dependency.version_range.bright_blue(), &dep.version.bright_green(), config.config.info.version.green(), config.restored_dependencies.len().to_string().yellow());
        for shared_dep in config.restored_dependencies.iter() { println!(" - {}: ({}) --> {}", &shared_dep.dependency.id, &shared_dep.dependency.version_range, &shared_dep.version); }
    }
}