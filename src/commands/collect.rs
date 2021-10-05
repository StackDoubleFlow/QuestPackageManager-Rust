use crate::data::package;

pub fn execute_collect_operation()
{
    println!("It should collect now");

    let package = package::PackageConfig::read();
    let dependencies = package.collect();

    println!("Collected Dependencies!:\n{:#?}", dependencies);
}