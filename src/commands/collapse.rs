use owo_colors::OwoColorize;

use crate::data::package;

pub fn execute_collapse_operation() {
    let package = package::PackageConfig::read();
    let resolved = package.resolve();
    for shared_package in resolved {
        println!(
            "{}: ({}) --> {} ({} restored dependencies)",
            &shared_package.config.info.id.bright_red(),
            /*&dep.dependency.version_range.bright_blue(),*/ "?".bright_blue(),
            &shared_package.config.info.version.bright_green(),
            shared_package
                .restored_dependencies
                .len()
                .to_string()
                .yellow()
        );

        for shared_dep in shared_package.restored_dependencies.iter() {
            println!(
                " - {}: ({}) --> {}",
                &shared_dep.dependency.id,
                &shared_dep.dependency.version_range,
                &shared_dep.version
            );
        }
    }
}
