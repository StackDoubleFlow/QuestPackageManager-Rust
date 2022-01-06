use clap::{Args};
use owo_colors::OwoColorize;

use crate::data::config::Config as AppConfig;

#[derive(Args, Debug, Clone)]
pub struct NDKPath {
    /// The path to set for the ndk path
    pub ndk_path: Option<String>,
}

pub fn execute_ndk_config_operation(config: &mut AppConfig, operation: NDKPath) -> bool {
    if let Some(path) = operation.ndk_path {
        println!("Set ndk path to {}!", path.bright_yellow());
        config.ndk_path = Some(path);
        true
    } else if let Some(path) = &config.ndk_path {
        println!("Current configured ndk path is: {}", path.bright_yellow());
        false
    } else {
        println!("No ndk path was configured!");
        false
    }
}
