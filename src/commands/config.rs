use clap::{Clap, AppSettings};
use owo_colors::*;
#[allow(non_camel_case_types)]

use crate::data::config::Config as AppConfig;


#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Config {
    /// The operation to execute
    #[clap(subcommand)]
    pub op: ConfigOperation
}

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Cache {
    #[clap(subcommand)]
    pub op: CacheOperation
}

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub enum CacheOperation {
    /// Gets or sets the path to place the QPM Cache
    Path(CacheSetPathOperation),
}

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct CacheSetPathOperation {
    pub path: Option<String>,
}

#[derive(Clap, Debug, Clone)]
pub enum SymlinkOperation {
    /// Enable symlink usage
    Enable,
    /// Disable symlink usage
    Disable
}

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Symlink {
    #[clap(subcommand)]
    pub op: Option<SymlinkOperation>
}

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Timeout {
    pub timeout: Option<u64>
}

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub enum ConfigOperation {
    /// Get or set the cache path
    Cache(Cache),
    /// Enable or disable symlink usage
    Symlink(Symlink),
    /// Get or set the timeout for web requests
    Timeout(Timeout )
}

pub fn execute_config_operation(operation: Config)
{
    let mut config = AppConfig::read();
    match operation.op {
        ConfigOperation::Cache(c) => execute_cache_config_operation(&mut config, c),
        ConfigOperation::Symlink(s) => execute_symlink_config_operation(&mut config, s),
        ConfigOperation::Timeout(t) => execute_timeout_config_operation(&mut config, t)
    }
    config.write();
}

fn execute_cache_config_operation(config: &mut AppConfig, operation: Cache)
{
    match operation.op {
        CacheOperation::Path(p) => {
            if let Some(path) = p.path {
                println!("Set cache path to {}", path.bright_yellow());
                config.cache = Some(path);
            } else {
                println!("Current configured cache path is {}", config.cache.as_ref().unwrap().bright_yellow());
            }
        },
    }
}

fn set_symlink_usage(config: &mut AppConfig, value: bool)
{
    println!("Set symlink usage to {}", value.bright_yellow());
    config.symlink = Some(value);
}

fn execute_symlink_config_operation(config: &mut AppConfig, operation: Symlink)
{
    // value is given
    if let Some(symlink) = operation.op {
        match symlink {
            SymlinkOperation::Enable => {
                set_symlink_usage(config, true);
            },
            SymlinkOperation::Disable => {
                set_symlink_usage(config, false);
            }
        }
    } else {
        println!("Current configured symlink usage is set to: {}", config.symlink.as_ref().unwrap().bright_yellow());
    }
}

fn execute_timeout_config_operation(config: &mut AppConfig, operation: Timeout)
{
    if let Some(timeout) = operation.timeout {
        // TODO actually set the value
        println!("Set timeout to {}!", timeout.bright_yellow());
        config.timeout = Some(timeout);
    } else {
        // TODO: make it actually read the value from config
            println!("Current configured timeout is set to: {}", config.timeout.as_ref().unwrap().bright_yellow());
    }
}