use clap::{Clap, AppSettings};
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]

use super::qpm_types;

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Dependency {
    #[clap(subcommand)]
    pub op: DependencyOperation
}

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub enum DependencyOperation {
    /// Add a dependency
    Add(DependencyOperationAddArgs),
    /// Remove a dependency
    Remove(DependencyOperationRemoveArgs)
}

#[derive(Clap, Debug, Clone)]
#[allow(non_snake_case)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct DependencyOperationAddArgs {
    /// Id of the dependency as listed on qpackages
    pub id: String,

    /// optional version of the dependency that you want to add
    #[clap(short, long)]
    pub version: Option<String>,

    /// Additional data for the dependency (as a valid json object)
    #[clap(long)]
    pub additionalData: Option<String>
}

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct DependencyOperationRemoveArgs {
    /// Id of the dependency as listed on qpackages
    pub id: String,
}

pub fn execute_dependency_operation(operation: Dependency)
{
    match operation.op.clone() {
        DependencyOperation::Add(a) => add_dependency(a),
        DependencyOperation::Remove(r) => remove_dependency(r)
    }
}

fn add_dependency(dependency_args: DependencyOperationAddArgs)
{
    // TODO make it actually add
    match &dependency_args.version {
        Option::Some(v) => {
            let additional_data: qpm_types::AdditionalDependencyData;
            match &dependency_args.additionalData {
                Option::Some(a) => { additional_data = serde_json::from_str(a).expect("Deserialize failed"); },
                Option::None => { additional_data = qpm_types::AdditionalDependencyData::default(); }
            }
            put_dependency(dependency_args.id, v.clone(), additional_data);
        },
        Option::None => {
            let additional_data: qpm_types::AdditionalDependencyData;
            match &dependency_args.additionalData {
                Option::Some(a) => { additional_data = serde_json::from_str(a).expect("Deserialize failed"); },
                Option::None => { additional_data = qpm_types::AdditionalDependencyData::default(); }
            }
            put_dependency(dependency_args.id, "*".to_string(), additional_data);
        }
    }
}

fn put_dependency(id: String, version: String, additional_data: qpm_types::AdditionalDependencyData)
{
    println!("Adding dependency with id {} and version {}", id, version);
    // TODO make it actually add the dependency
    // TODO make it check already added dependencies

    let mut package = qpm_types::PackageConfig::read();
    let dep = qpm_types::Dependency {id, versionRange: version, additionalData: additional_data, ..Default::default() };
    
    for dependency in &package.dependencies
    {
        if dependency.id == dep.id
        {
            println!("not adding dependency because it was already contained in the package");
            return;
        }
    }

    package.dependencies.insert(package.dependencies.len(), dep);

    package.write();
}

fn remove_dependency(dependency_args: DependencyOperationRemoveArgs)
{
    // TODO make it actually remove
    let mut package = qpm_types::PackageConfig::read();
    let mut idx = 0;

    for dependency in &package.dependencies
    {
        if dependency.id == dependency_args.id
        {
            break;
        }
        idx += 1;
    }

    if package.dependencies.len() != idx
    {
        package.dependencies.remove(idx);
        package.write();
        println!("Removed dependency {}", dependency_args.id);
    }
    else
    {
        println!("Dependency {} did not exist!", dependency_args.id);
    }
}