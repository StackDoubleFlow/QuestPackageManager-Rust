use owo_colors::OwoColorize;

use crate::data::package;

pub fn execute_collapse_operation() {
    let package = package::PackageConfig::read();
    let dependencies = package.collapse();

    println!("Collapsing package {}", package.info.id.bright_purple());

    for (dep, config) in dependencies.iter() {
        println!(
            "{}: ({}) --> {} (config: {}, {} restored dependencies)",
            &dep.dependency.id.bright_red(),
            &dep.dependency.version_range.bright_blue(),
            &dep.version.bright_green(),
            config.config.info.version.green(),
            config.restored_dependencies.len().to_string().yellow()
        );
        for shared_dep in config.restored_dependencies.iter() {
            println!(
                " - {}: ({}) --> {}",
                &shared_dep.dependency.id,
                &shared_dep.dependency.version_range,
                &shared_dep.version
            );
        }
    }
}
