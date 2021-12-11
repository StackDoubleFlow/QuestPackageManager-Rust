use clap::{AppSettings, Clap};
use owo_colors::OwoColorize;

use crate::data::config::Config as AppConfig;

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Timeout {
    pub timeout: Option<u64>,
}

pub fn execute_timeout_config_operation(config: &mut AppConfig, operation: Timeout) -> bool {
    if let Some(timeout) = operation.timeout {
        println!("Set timeout to {}!", timeout.bright_yellow());
        config.timeout = Some(timeout);
        true
    } else if let Some(timeout) = config.timeout {
        println!(
            "Current configured timeout is set to: {}",
            timeout.bright_yellow()
        );
        false
    } else {
        println!("Timeout is not configured!");
        false
    }
}
