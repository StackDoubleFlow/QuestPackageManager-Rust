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
    EditExtra(EditExtra)
}

#[derive(Clap, Debug, Clone)]
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
#[clap(setting = AppSettings::ColoredHelp)]
pub struct EditExtra {
    /// Change the branch name in additional data
    #[clap(long="branchName")]
    pub branch_name: Option<String>,

    /// Change the headers only bool in additional data, 0 for false, 1 for true
    #[clap(long="headersOnly")]
    pub headers_only: Option<i8>,

    /// Make the package be statically linked, 0 for false, 1 for true
    #[clap(long="staticLinking")]
    pub static_linking: Option<bool>,

    /// Provide a so link for downloading the regular .so file
    #[clap(long="soLink")]
    pub so_link: Option<String>,

    /// Provide an additional file to add to the extra files list, prepend with - to remove an entry
    #[clap(long="extraFiles")]
    pub extra_files: Option<String>,

    /// Provide a debug so link for downloading the debug .so file
    #[clap(long="debugSoLink")]
    pub debug_so_link: Option<String>,

    /// Provide an overridden name for the .so file
    #[clap(long="overrideSoName")]
    pub override_so_name: Option<String>
}

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct PackageOperationCreateArgs {
    /// The name of the package
    pub name: String,
    /// The version of the package
    pub version: String,
    /// Specify an id, else lowercase will be used
    #[clap(long="id")]
    pub id: Option<String>,
    /// Branch name of a Github repo. Only used when a valid github url is provided
    #[clap(long="branchName")]
    pub branch_name: Option<String>,
    /// Specify that this package is headers only and does not contain a .so or .a file
    #[clap(long="headersOnly")]
    pub headers_only: Option<bool>,
    /// Specify that this package is static linking
    #[clap(long="staticLinking")]
    pub static_linking: Option<bool>,
    /// Specify the download link for a release .so or .a file
    #[clap(long="soLink")]
    pub so_link: Option<String>,
    /// Specify the download link for a debug .so or .a files (usually from the obj folder)
    #[clap(long="debugSoLink")]
    pub debug_so_link: Option<String>,
    /// Override the downloaded .so or .a filename with this name instead.
    #[clap(long="overrideSoName")]
    pub override_so_name: Option<String>
}

pub fn execute_package_operation(operation: Package)
{
    match operation.op {
        PackageOperation::Create(c) => package_create_operation(c),
        PackageOperation::Edit(e) => package_edit_operation(e),
        PackageOperation::EditExtra(ee) => package_edit_extra_operation(ee)
    }
}

fn package_create_operation(create_parameters: PackageOperationCreateArgs)
{
    let additional_data = AdditionalPackageData {
        branch_name: create_parameters.branch_name,
        headers_only: create_parameters.headers_only,
        static_linking: create_parameters.static_linking,
        so_link: create_parameters.so_link,
        debug_so_link: create_parameters.debug_so_link,
        override_so_name: create_parameters.override_so_name,
        ..Default::default()
    };

    // id is optional so we need to check if it's defined, else use the name to lowercase
    let id: String;
    match create_parameters.id {
        Option::Some(s) => id = s,
        Option::None => id = create_parameters.name.to_lowercase()
    }

    let package_info = PackageInfo {
        id,
        name: create_parameters.name,
        version: create_parameters.version,
        additional_data,
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
    let mut package = PackageConfig::read();
    if edit_parameters.id.is_some() { package_set_id(&mut package, edit_parameters.id.unwrap()); }
    if edit_parameters.name.is_some() { package_set_name(&mut package, edit_parameters.name.unwrap()); }
    if edit_parameters.url.is_some() { package_set_url(&mut package, edit_parameters.url.unwrap()); }
    if edit_parameters.version.is_some() { package_set_version(&mut package, edit_parameters.version.unwrap()); }
    package.write();
}

fn package_set_id(package: &mut PackageConfig, id: String)
{
    println!("Setting package id: {}", id);
    // TODO edit mod.json and android.mk
    package.info.id = id;
}

fn package_set_name(package: &mut PackageConfig, name: String)
{
    println!("Setting package name: {}", name);
    package.info.name = name;
}

fn package_set_url(package: &mut PackageConfig, url: String)
{
    println!("Setting package url: {}", url);
    package.info.url = Option::Some(url);
}

fn package_set_version(package: &mut PackageConfig, version: String)
{
    println!("Setting package version: {}", version);
    // TODO  make it edit the version in mod.json and android.mk
    package.info.version = version;
}

fn package_edit_extra_operation(edit_parameters: EditExtra)
{
    let mut package = PackageConfig::read();
    if edit_parameters.branch_name.is_some() { package_edit_extra_branch_name(&mut package, edit_parameters.branch_name.unwrap()); }
    if edit_parameters.headers_only.is_some() { package_edit_extra_headers_only(&mut package, edit_parameters.headers_only.unwrap()); }
    if edit_parameters.static_linking.is_some() { package_edit_extra_static_linking(&mut package, edit_parameters.static_linking.unwrap()); }
    if edit_parameters.so_link.is_some() { package_edit_extra_so_link(&mut package, edit_parameters.so_link.unwrap()); }
    if edit_parameters.extra_files.is_some() { package_edit_extra_extra_files(&mut package, edit_parameters.extra_files.unwrap()); }
    if edit_parameters.debug_so_link.is_some() { package_edit_extra_debug_so_link(&mut package, edit_parameters.debug_so_link.unwrap()); }
    if edit_parameters.override_so_name.is_some() { package_edit_extra_override_so_name(&mut package, edit_parameters.override_so_name.unwrap()); }
    package.write();
}

pub fn package_edit_extra_branch_name(package: &mut PackageConfig, branch_name: String)
{
    println!("Setting branch name: {:#?}", branch_name);
    package.info.additional_data.branch_name = Some(branch_name);
}

pub fn package_edit_extra_headers_only(package: &mut PackageConfig, headers_only: i8)
{
    println!("Setting headers_only: {:#?}", headers_only);
    package.info.additional_data.headers_only = Some(headers_only != 0);
}

pub fn package_edit_extra_static_linking(package: &mut PackageConfig, static_linking: bool)
{
    println!("Setting static_linking: {:#?}", static_linking);
    package.info.additional_data.static_linking = Some(static_linking);
}

pub fn package_edit_extra_so_link(package: &mut PackageConfig, so_link: String)
{
    println!("Setting so_link: {:#?}", so_link);
    package.info.additional_data.so_link = Some(so_link);
}

pub fn package_edit_extra_extra_files(package: &mut PackageConfig, extra_file: String)
{
    println!("Setting extra_file: {}", extra_file);
    match extra_file.chars().next().unwrap() {
        '-' => {
            // remove
            package_edit_extra_remove_extra_files(package, extra_file[1..].to_string());
        },
        _ => {
            // add
            package_edit_extra_add_extra_files(package, extra_file);
        }
    }
}

pub fn package_edit_extra_remove_extra_files(package: &mut PackageConfig, extra_file: String)
{
    if let Some(extra_files) = &mut package.info.additional_data.extra_files {
        if let Some(idx) = extra_files.iter().position(|f| f == &extra_file) {
            extra_files.remove(idx);
        }
    }
}

pub fn package_edit_extra_add_extra_files(package: &mut PackageConfig, extra_file: String)
{
    if let Some(extra_files) = &mut package.info.additional_data.extra_files {
        if !extra_files.iter().any(|f| f == &extra_file) {
            extra_files.push(extra_file);
        }
    } else {
        let mut extra_files = Vec::default();
        extra_files.push(extra_file);
        package.info.additional_data.extra_files = Some(extra_files);
    }
}

pub fn package_edit_extra_debug_so_link(package: &mut PackageConfig, debug_so_link: String)
{
    println!("Setting debug_so_link: {:#?}", debug_so_link);
    package.info.additional_data.debug_so_link = Some(debug_so_link);
}

pub fn package_edit_extra_override_so_name(package: &mut PackageConfig, override_so_name: String)
{
    println!("Setting override_so_name: {:#?}", override_so_name);
    package.info.additional_data.override_so_name = Some(override_so_name);
}