use clap::{Subcommand, Args};
use owo_colors::OwoColorize;

use crate::data::config::Config as AppConfig;

#[derive(Subcommand, Debug, Clone)]
pub enum SymlinkOperation {
    /// Enable symlink usage
    Enable,
    /// Disable symlink usage
    Disable,
}

#[derive(Args, Debug, Clone)]

pub struct Symlink {
    #[clap(subcommand)]
    pub op: Option<SymlinkOperation>,
}

pub fn execute_symlink_config_operation(config: &mut AppConfig, operation: Symlink) -> bool {
    // value is given
    if let Some(symlink) = operation.op {
        match symlink {
            SymlinkOperation::Enable => {
                set_symlink_usage(config, true);
            }
            SymlinkOperation::Disable => {
                set_symlink_usage(config, false);
            }
        }
        return true;
    } else if let Some(symlink) = config.symlink.as_ref() {
        println!(
            "Current configured symlink usage is set to: {}",
            symlink.bright_yellow()
        );
    } else {
        println!("Symlink usage is not configured!");
    }

    false
}

fn set_symlink_usage(config: &mut AppConfig, value: bool) {
    println!("Set symlink usage to {}", value.bright_yellow());
    config.symlink = Some(value);
}
