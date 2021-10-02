use clap::{Clap, AppSettings};

use crate::data::qpm_types;

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
    match &operation.op {
        DependencyOperation::Add(a) => add_dependency(a),
        DependencyOperation::Remove(r) => remove_dependency(r)
    }
}

fn add_dependency(dependency_args: &DependencyOperationAddArgs)
{
    // TODO make it actually add
    let version: String;
    let additional_data: qpm_types::AdditionalDependencyData;
    match &dependency_args.version {
        Option::Some(v) => version = v.clone(),
        Option::None => version = "*".to_string()
    }

    match &dependency_args.additionalData {
        Option::Some(d) => additional_data = serde_json::from_str(d).expect("Deserializing additional data failed"),
        Option::None => additional_data = qpm_types::AdditionalDependencyData::default()
    }

    put_dependency(&dependency_args.id, &version, &additional_data);
}

fn put_dependency(id: &str, version: &str, additional_data: &qpm_types::AdditionalDependencyData)
{
    println!("Adding dependency with id {} and version {}", id, version);
    // TODO make it actually add the dependency
    // TODO make it check already added dependencies

    let mut package = qpm_types::PackageConfig::read();
    let dep = qpm_types::Dependency {id: id.to_string(), versionRange: version.to_string(), additionalData: additional_data.clone()};
    package.add_dependency(dep);
    package.write();
}

fn remove_dependency(dependency_args: &DependencyOperationRemoveArgs)
{
    // TODO make it actually remove
    let mut package = qpm_types::PackageConfig::read();
    package.remove_dependency(&dependency_args.id);
    package.write();
}