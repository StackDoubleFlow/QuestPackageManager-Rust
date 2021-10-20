use clap::{AppSettings, Clap};
use owo_colors::OwoColorize;

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct ListOperation {
    /// What you want to list
    #[clap(subcommand)]
    pub op: ListOption,
}

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Package {
    pub package: String,
    #[clap(short, long)]
    pub latest: bool,
}

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub enum ListOption {
    /// List the extra properties that are supported
    ExtraProperties,
    /// List the available packages on qpackages.com
    Packages,
    /// List the versions for a specific package
    Versions(Package),
}

pub fn execute_list_operation(operation: ListOperation) {
    match operation.op {
        ListOption::ExtraProperties => execute_extra_properties_list(),
        ListOption::Packages => execute_packages_list(),
        ListOption::Versions(p) => execute_versions_list(p),
    }
}

fn execute_extra_properties_list() {
    println!("TODO print all extra properties");
}

fn execute_packages_list() {
    let ids = crate::data::qpackages::get_packages();
    if !ids.is_empty() {
        println!(
            "Found {} packages on qpackages.com",
            ids.len().bright_yellow()
        );
        let mut idx = 0;
        for id in ids.iter() {
            println!("{}", id);
            idx += 1;
            if idx % 5 == 0 {
                println!();
                idx = 0;
            }
        }
    } else {
        println!("qpackages.com returned 0 packages, is something wrong?");
    }
}

fn execute_versions_list(package: Package) {
    let versions = crate::data::qpackages::get_versions(&package.package);
    if package.latest {
        println!(
            "The latest version for package {} is {}",
            package.package.bright_red(),
            versions
                .get(0)
                .expect("Getting first version failed!")
                .version
                .to_string()
                .bright_green()
        );
    } else if !versions.is_empty() {
        println!(
            "Package {} has {} versions on qpackages.com:",
            package.package.bright_red(),
            versions.len().bright_yellow()
        );
        for package_version in versions.iter() {
            println!(" - {}", package_version.version.to_string().bright_green());
        }
    } else {
        println!(
            "Package {} either did not exist or has no versions on qpackages.com",
            package.package.bright_red()
        );
    }
}
