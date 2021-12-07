use std::path::Path;

use clap::{AppSettings, Clap};
use owo_colors::OwoColorize;
use semver::Version;

use crate::data::{
    dependency::{AdditionalDependencyData, Dependency},
    package::{AdditionalPackageData, PackageConfig, PackageInfo},
    shared_package::SharedPackageConfig,
};
#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Package {
    #[clap(subcommand)]
    pub op: PackageOperation,
}

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub enum PackageOperation {
    /// Create a package
    Create(PackageOperationCreateArgs),
    /// Edit various properties of the package
    Edit(Edit),
    /// Edit extra supported properties of the package
    EditExtra(EditExtra),
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
    pub version: Option<Version>,
}

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct EditExtra {
    /// Change the headers only bool in additional data, 0 for false, 1 for true
    #[clap(long = "headersOnly")]
    pub headers_only: Option<i8>,

    /// Make the package be statically linked, 0 for false, 1 for true
    #[clap(long = "staticLinking")]
    pub static_linking: Option<i8>,

    /// Provide a so link for downloading the regular .so file
    #[clap(long = "soLink")]
    pub so_link: Option<String>,

    /// Provide a debug so link for downloading the debug .so file
    #[clap(long = "debugSoLink")]
    pub debug_so_link: Option<String>,

    /// Provide an overridden name for the .so file
    #[clap(long = "overrideSoName")]
    pub override_so_name: Option<String>,

    /// Provide a link to the mod
    #[clap(long = "modLink")]
    pub mod_link: Option<String>,

    /// Change the branch name in additional data
    #[clap(long = "branchName")]
    pub branch_name: Option<String>,

    /// Provide an additional file to add to the extra files list, prepend with - to remove an entry
    #[clap(long = "extraFiles")]
    pub extra_files: Option<String>,
}

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct PackageOperationCreateArgs {
    /// The name of the package
    pub name: String,
    /// The version of the package
    pub version: Version,
    /// Specify an id, else lowercase will be used
    #[clap(long = "id")]
    pub id: Option<String>,
    /// Branch name of a Github repo. Only used when a valid github url is provided
    #[clap(long = "branchName")]
    pub branch_name: Option<String>,
    /// Specify that this package is headers only and does not contain a .so or .a file
    #[clap(long = "headersOnly")]
    pub headers_only: Option<bool>,
    /// Specify that this package is static linking
    #[clap(long = "staticLinking")]
    pub static_linking: Option<bool>,
    /// Specify the download link for a release .so or .a file
    #[clap(long = "soLink")]
    pub so_link: Option<String>,
    /// Specify the download link for a debug .so or .a files (usually from the obj folder)
    #[clap(long = "debugSoLink")]
    pub debug_so_link: Option<String>,
    /// Override the downloaded .so or .a filename with this name instead.
    #[clap(long = "overrideSoName")]
    pub override_so_name: Option<String>,
}

pub fn execute_package_operation(operation: Package) {
    match operation.op {
        PackageOperation::Create(c) => package_create_operation(c),
        PackageOperation::Edit(e) => package_edit_operation(e),
        PackageOperation::EditExtra(ee) => package_edit_extra_operation(ee),
    }
}

fn package_create_operation(create_parameters: PackageOperationCreateArgs) {
    if PackageConfig::check() {
        println!(
            "{}",
            "Package already existed, not creating a new package!".bright_red()
        );
        println!("Did you try to make a package in the same directory as another, or did you not use a clean folder?");
        return;
    }

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
        Option::None => id = create_parameters.name.to_lowercase(),
    }

    let package_info = PackageInfo {
        id,
        name: create_parameters.name,
        url: None,
        version: create_parameters.version,
        additional_data,
    };

    let package = PackageConfig {
        info: package_info,
        shared_dir: Path::new("shared").to_owned(),
        dependencies_dir: Path::new("extern").to_owned(),
        dependencies: Vec::<Dependency>::default(),
        additional_data: AdditionalDependencyData::default(),
    };

    package.write();
}

fn package_edit_operation(edit_parameters: Edit) {
    let mut package = PackageConfig::read();
    let mut any_changed = false;
    if let Some(id) = edit_parameters.id {
        package_set_id(&mut package, id);
        any_changed = true;
    }
    if let Some(name) = edit_parameters.name {
        package_set_name(&mut package, name);
        any_changed = true;
    }
    if let Some(url) = edit_parameters.url {
        package_set_url(&mut package, url);
        any_changed = true;
    }
    if let Some(version) = edit_parameters.version {
        package_set_version(&mut package, version);
        any_changed = true;
    }

    if any_changed {
        package.write();
        let mut shared_package = SharedPackageConfig::read();
        shared_package.config = package;
        shared_package.write();

        // TODO: Edit qpm defines.cmake
    }
}

fn package_set_id(package: &mut PackageConfig, id: String) {
    println!("Setting package id: {}", id);
    package.info.id = id;
}

fn package_set_name(package: &mut PackageConfig, name: String) {
    println!("Setting package name: {}", name);
    package.info.name = name;
}

fn package_set_url(package: &mut PackageConfig, url: String) {
    println!("Setting package url: {}", url);
    package.info.url = Option::Some(url);
}

fn package_set_version(package: &mut PackageConfig, version: Version) {
    println!("Setting package version: {}", version);
    package.info.version = version;
}

fn package_edit_extra_operation(edit_parameters: EditExtra) {
    let mut package = PackageConfig::read();
    let mut any_changed = false;
    if let Some(branch_name) = edit_parameters.branch_name {
        package_edit_extra_branch_name(&mut package, branch_name);
        any_changed = true;
    }
    if let Some(headers_only) = edit_parameters.headers_only {
        package_edit_extra_headers_only(&mut package, headers_only);
        any_changed = true;
    }
    if let Some(static_linking) = edit_parameters.static_linking {
        package_edit_extra_static_linking(&mut package, static_linking);
        any_changed = true;
    }
    if let Some(so_link) = edit_parameters.so_link {
        package_edit_extra_so_link(&mut package, so_link);
        any_changed = true;
    }
    if let Some(extra_files) = edit_parameters.extra_files {
        package_edit_extra_extra_files(&mut package, extra_files);
        any_changed = true;
    }
    if let Some(debug_so_link) = edit_parameters.debug_so_link {
        package_edit_extra_debug_so_link(&mut package, debug_so_link);
        any_changed = true;
    }
    if let Some(mod_link) = edit_parameters.mod_link {
        package_edit_extra_mod_link(&mut package, mod_link);
        any_changed = true;
    }
    if let Some(override_so_name) = edit_parameters.override_so_name {
        package_edit_extra_override_so_name(&mut package, override_so_name);
        any_changed = true;
    }

    if any_changed {
        package.write();
        let mut shared_package = SharedPackageConfig::read();
        shared_package.config = package;
        shared_package.write();

        // TODO: Edit qpm defines.cmake
    }
}

pub fn package_edit_extra_branch_name(package: &mut PackageConfig, branch_name: String) {
    println!("Setting branch name: {:#?}", branch_name);
    package.info.additional_data.branch_name = Some(branch_name);
}

pub fn package_edit_extra_headers_only(package: &mut PackageConfig, headers_only: i8) {
    println!("Setting headers_only: {:#?}", headers_only);
    package.info.additional_data.headers_only = Some(headers_only != 0);
}

pub fn package_edit_extra_static_linking(package: &mut PackageConfig, static_linking: i8) {
    println!("Setting static_linking: {:#?}", static_linking);
    package.info.additional_data.static_linking = Some(static_linking != 0);
}

pub fn package_edit_extra_so_link(package: &mut PackageConfig, so_link: String) {
    println!("Setting so_link: {:#?}", so_link);
    package.info.additional_data.so_link = Some(so_link);
}

pub fn package_edit_extra_mod_link(package: &mut PackageConfig, mod_link: String) {
    println!("Setting mod_link: {:#?}", mod_link);
    package.info.additional_data.mod_link = Some(mod_link);
}

pub fn package_edit_extra_extra_files(package: &mut PackageConfig, extra_file: String) {
    println!("Setting extra_file: {}", extra_file);
    match extra_file.chars().next().unwrap() {
        '-' => {
            // remove
            package_edit_extra_remove_extra_files(package, extra_file[1..].to_string());
        }
        _ => {
            // add
            package_edit_extra_add_extra_files(package, extra_file);
        }
    }
}

pub fn package_edit_extra_remove_extra_files(package: &mut PackageConfig, extra_file: String) {
    if let Some(extra_files) = &mut package.info.additional_data.extra_files {
        if let Some(idx) = extra_files.iter().position(|f| f == &extra_file) {
            extra_files.remove(idx);
        }
    }
}

pub fn package_edit_extra_add_extra_files(package: &mut PackageConfig, extra_file: String) {
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

pub fn package_edit_extra_debug_so_link(package: &mut PackageConfig, debug_so_link: String) {
    println!("Setting debug_so_link: {:#?}", debug_so_link);
    package.info.additional_data.debug_so_link = Some(debug_so_link);
}

pub fn package_edit_extra_override_so_name(package: &mut PackageConfig, override_so_name: String) {
    println!("Setting override_so_name: {:#?}", override_so_name);
    package.info.additional_data.override_so_name = Some(override_so_name);
}
