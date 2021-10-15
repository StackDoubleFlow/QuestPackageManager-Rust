use clap::{Clap, AppSettings};
use owo_colors::*;

use crate::data::dependency;
use crate::data::package::{PackageConfig};
use semver::VersionReq;

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
#[clap(setting = AppSettings::ColoredHelp)]
pub struct DependencyOperationAddArgs {
    /// Id of the dependency as listed on qpackages
    pub id: String,

    /// optional version of the dependency that you want to add
    #[clap(short, long)]
    pub version: Option<VersionReq>,

    /// Additional data for the dependency (as a valid json object)
    #[clap(long)]
    pub additional_data: Option<String>
}

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct DependencyOperationRemoveArgs {
    /// Id of the dependency as listed on qpackages
    pub id: String,
}

pub fn execute_dependency_operation(operation: Dependency)
{
    match operation.op {
        DependencyOperation::Add(a) => add_dependency(a),
        DependencyOperation::Remove(r) => remove_dependency(r)
    }
}

fn add_dependency(dependency_args: DependencyOperationAddArgs)
{
    // TODO make it actually add
    let version: VersionReq;
    let additional_data: dependency::AdditionalDependencyData;
    match dependency_args.version {
        Option::Some(v) => version = v,
        Option::None => version = VersionReq::STAR
    }

    match &dependency_args.additional_data {
        Option::Some(d) => additional_data = serde_json::from_str(d).expect("Deserializing additional data failed"),
        Option::None => additional_data = dependency::AdditionalDependencyData::default()
    }

    put_dependency(&dependency_args.id, version, &additional_data);
}

fn put_dependency(id: &str, version: VersionReq, additional_data: &dependency::AdditionalDependencyData)
{
    println!("Adding dependency with id {} and version {}", id.bright_red(), version.bright_blue());
    // TODO make it actually add the dependency
    // TODO make it check already added dependencies

    let mut package = crate::data::package::PackageConfig::read();
    let dep = dependency::Dependency {id: id.to_string(), version_range: version, additional_data: additional_data.clone()};
    package.add_dependency(dep);
    package.write();
}

fn remove_dependency(dependency_args: DependencyOperationRemoveArgs)
{
    // TODO make it actually remove
    let mut package = PackageConfig::read();
    package.remove_dependency(&dependency_args.id);
    package.write();
}