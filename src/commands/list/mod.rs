use clap::{AppSettings, Clap};

mod extra_properties;
mod packages;
mod versions;
pub type Package = versions::Package;

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

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct ListOperation {
    /// What you want to list
    #[clap(subcommand)]
    pub op: ListOption,
}

pub fn execute_list_operation(operation: ListOperation) {
    match operation.op {
        ListOption::ExtraProperties => extra_properties::execute_extra_properties_list(),
        ListOption::Packages => packages::execute_packages_list(),
        ListOption::Versions(p) => versions::execute_versions_list(p),
    }
}
