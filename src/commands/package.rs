use clap::{AppSettings, Clap};

use crate::data::package::{PackageConfig, PackageInfo, AdditionalPackageData};

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
    #[clap(long="branchName")]
    pub branchName: Option<String>,
    /// Specify that this package is headers only and does not contain a .so or .a file
    #[clap(long="headersOnly")]
    pub headersOnly: Option<bool>,
    /// Specify that this package is static linking
    #[clap(long="staticLinking")]
    pub staticLinking: Option<bool>,
    /// Specify the download link for a release .so or .a file
    #[clap(long="soLink")]
    pub soLink: Option<String>,
    /// Specify the download link for a debug .so or .a files (usually from the obj folder)
    #[clap(long="debugSoLink")]
    pub debugSoLink: Option<String>,
    /// Override the downloaded .so or .a filename with this name instead.
    #[clap(long="overrideSoName")]
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
    let additional_data = AdditionalPackageData {
        branchName: create_parameters.branchName,
        headersOnly: create_parameters.headersOnly,
        staticLinking: create_parameters.staticLinking,
        soLink: create_parameters.soLink,
        debugSoLink: create_parameters.debugSoLink,
        overrideSoName: create_parameters.overrideSoName,
        ..Default::default()
    };

    let package_info = PackageInfo {
        id: create_parameters.id.clone(),
        name: create_parameters.id,
        version: create_parameters.version,
        additionalData: additional_data,
        ..Default::default()
    };

    let package = PackageConfig {
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
    // TODO edit mod.json and android.mk
    let mut package = PackageConfig::read();
    package.info.id = id;
    package.write();
}

fn package_set_name(name: String)
{
    println!("Setting package name: {}", name);
    let mut package = PackageConfig::read();
    package.info.name = name;
    package.write();
}

fn package_set_url(url: String)
{
    println!("Setting package url: {}", url);
    let mut package = PackageConfig::read();
    package.info.url = Option::Some(url);
    package.write();
}

fn package_set_version(version: String)
{
    println!("Setting package version: {}", version);
    // TODO  make it edit the version in mod.json and android.mk
    let mut package = PackageConfig::read();
    package.info.version = version;
    package.write();
}

fn package_edit_extra_operation()
{

}