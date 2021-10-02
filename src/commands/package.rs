use clap::{AppSettings, Clap};

use crate::data::qpm_types;

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Package {
    #[clap(subcommand)]
    pub op: PackageOperation
}

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub enum PackageOperation {
    /// Create a package
    Create(PackageOperationCreateArgs),
    /// Edit various properties of the package
    Edit(Edit),
    /// Edit extra supported properties of the package
    EditExtra
}

#[derive(Clap, Debug, Clone)]
#[allow(non_snake_case)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Edit {
    ///Edit the id property of the package
    #[clap(long)]
    pub id: Option<String>,
    ///Edit the name property of the package
    #[clap(long)]
    pub name: Option<String>,
    ///Edit the url property of the package
    #[clap(long)]
    pub url: Option<String>,
    ///Edit the version property of the package
    #[clap(long)]
    pub version: Option<String>
}

#[derive(Clap, Debug, Clone)]
#[allow(non_snake_case)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct PackageOperationCreateArgs {
    ///Edit the id property of the package
    pub id: String,
    ///Edit the name property of the package
    pub version: String,
    /// Branch name of a Github repo. Only used when a valid github url is provided
    #[clap(long)]
    pub branchName: Option<String>,
    /// Specify that this package is headers only and does not contain a .so or .a file
    #[clap(long)]
    pub headersOnly: Option<bool>,
    /// Specify that this package is static linking
    #[clap(long)]
    pub staticLinking: Option<bool>,
    /// Specify the download link for a release .so or .a file
    #[clap(long)]
    pub soLink: Option<String>,
    /// Specify the download link for a debug .so or .a files (usually from the obj folder)
    #[clap(long)]
    pub debugSoLink: Option<String>,
    /// Override the downloaded .so or .a filename with this name instead.
    #[clap(long)]
    pub overrideSoName: Option<String>
}

pub fn execute_package_operation(operation: Package)
{
    match operation.op {
        PackageOperation::Create(c) => package_create_operation(c),
        PackageOperation::Edit(e) => package_edit_operation(e),
        PackageOperation::EditExtra => package_edit_extra_operation()
    }
}

fn package_create_operation(create_parameters: PackageOperationCreateArgs)
{
    let additional_data = qpm_types::AdditionalPackageData {
        branchName: create_parameters.branchName,
        headersOnly: create_parameters.headersOnly,
        staticLinking: create_parameters.staticLinking,
        soLink: create_parameters.soLink,
        debugSoLink: create_parameters.debugSoLink,
        overrideSoName: create_parameters.overrideSoName,
        ..Default::default()
    };

    let package_info = qpm_types::PackageInfo {
        id: create_parameters.id.clone(),
        name: create_parameters.id,
        version: create_parameters.version,
        additionalData: additional_data,
        ..Default::default()
    };

    let package = qpm_types::PackageConfig {
        info: package_info,
        ..Default::default()
    };

    package.write();
}

fn package_edit_operation(edit_parameters: Edit)
{
    if edit_parameters.id.is_some() { package_set_id(edit_parameters.id.unwrap()); }
    if edit_parameters.name.is_some() { package_set_name(edit_parameters.name.unwrap()); }
    if edit_parameters.url.is_some() { package_set_url(edit_parameters.url.unwrap()); }
    if edit_parameters.version.is_some() { package_set_version(edit_parameters.version.unwrap()); }
}

fn package_set_id(id: String)
{
    println!("Setting package id: {}", id);
}

fn package_set_name(name: String)
{
    println!("Setting package name: {}", name);
}

fn package_set_url(url: String)
{
    println!("Setting package url: {}", url);
}

fn package_set_version(version: String)
{
    println!("Setting package version: {}", version);
}

fn package_edit_extra_operation()
{

}