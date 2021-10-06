use crate::data::package;

pub fn execute_collect_operation()
{
    println!("It should collect now");

    let package = package::PackageConfig::read();
    let dependencies = package.collect();

    for (dep, config) in dependencies.iter()
    {
        println!("{}: ({}) --> {} (config: {}, {} restored dependencies)", &dep.dependency.id, &dep.dependency.version_range, &dep.version, config.config.info.version, config.restored_dependencies.len());
        
        for shared_dep in config.restored_dependencies.iter()
        {
            println!(" - {}: ({}) --> {}", &shared_dep.dependency.id, &shared_dep.dependency.version_range, &shared_dep.version);
        }
    }
}