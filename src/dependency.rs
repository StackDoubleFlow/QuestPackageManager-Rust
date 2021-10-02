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

    /// Branch name of a Github repo. Only used when a valid github url is provided
    #[clap(long)]
    pub branchName: Option<String>,

    /// Specify any additional files to be downloaded
    #[clap(long)]
    pub extraFiles: Option<Vec<String>>,

    /// Copy a dependency from a location that is local to this root path instead of from a remote url
    #[clap(long)]
    pub localPath: Option<String>,

    /// Specify if a dependency should download a release .so or .a file. Default to false
    #[clap(long)]
    pub useRelease: Option<bool>,

    /// Specify the style to use
    #[clap(long)]
    pub style: Option<String>
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

fn get_vec(op: Option<Vec<String>>) -> Vec<String>
{
    if op.is_some()
    {
        return op.unwrap();
    }
    else
    {
        return Vec::default();
    }

}

fn add_dependency(dependency_args: DependencyOperationAddArgs)
{
    // TODO make it actually add

    match dependency_args.version.clone() {
        Option::Some(v) => put_dependency(dependency_args.id, v, qpm_types::AdditionalDependencyData { 
            branchName: dependency_args.branchName,
            extraFiles: get_vec(dependency_args.extraFiles),
            localPath: dependency_args.localPath,
            useRelease: dependency_args.useRelease,
            style: dependency_args.style
        }),
        Option::None => put_dependency(dependency_args.id, "*".to_string(), qpm_types::AdditionalDependencyData { 
            branchName: dependency_args.branchName,
            extraFiles: get_vec(dependency_args.extraFiles),
            localPath: dependency_args.localPath,
            useRelease: dependency_args.useRelease,
            style: dependency_args.style
        }),
    }
}

fn put_dependency(id: String, version: String, additional_data: qpm_types::AdditionalDependencyData)
{
    println!("Adding dependency with id {} and verison {}", id, version);
    // TODO make it actually add the dependency
    // TODO make it check already added dependencies

    let dep = qpm_types::Dependency {id, versionRange: version, additionalData: additional_data, ..Default::default() };
}

fn remove_dependency(dependency_args: DependencyOperationRemoveArgs)
{
    // TODO make it actually remove
    println!("Removing dependency with id {}", dependency_args.id);
}