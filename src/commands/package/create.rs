use std::path::Path;

use clap::{AppSettings, Clap};
use owo_colors::OwoColorize;
use semver::Version;

use crate::data::{
    dependency::{AdditionalDependencyData, Dependency},
    package::{AdditionalPackageData, PackageConfig, PackageInfo},
};
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

pub fn package_create_operation(create_parameters: PackageOperationCreateArgs) {
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
    let id = match create_parameters.id {
        Option::Some(s) => s,
        Option::None => create_parameters.name.to_lowercase(),
    };

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
