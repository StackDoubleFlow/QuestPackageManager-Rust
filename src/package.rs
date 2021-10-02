use clap::{AppSettings, Clap};

use super::qpm_types;

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
    Create,
    /// Edit various properties of the package
    Edit,
    /// Edit extra supported properties of the package
    EditExtra
}

pub fn execute_package_operation(operation: Package)
{
    match operation.op.clone() {
        PackageOperation::Create => 
        PackageOperation::Edit => 
        PackageOperation::EditExtra => 
    }
}

fn package_create_operation()
{

}

fn package_edit_operation()
{

}

fn package_edit_extra_operation()
{
    
}