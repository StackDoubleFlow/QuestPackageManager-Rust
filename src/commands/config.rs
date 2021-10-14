use clap::{Clap, AppSettings};
use owo_colors::*;

use crate::data::config::Config as AppConfig;
use crate::data::config::get_keyring;

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Config {
    /// The operation to execute
    #[clap(subcommand)]
    pub op: ConfigOperation,
    /// use this flag to edit the local config instead of the global one
    #[clap(short, long)]
    pub local: bool
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
    pub op: Option<SymlinkOperation>,
}

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Timeout {
    pub timeout: Option<u64>,
}

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Token {
    pub token: Option<String>,
    #[clap(long)]
    pub delete: bool,
}
#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub enum ConfigOperation {
    /// Get or set the cache path
    Cache(Cache),
    /// Enable or disable symlink usage
    Symlink(Symlink),
    /// Get or set the timeout for web requests
    Timeout(Timeout),
    /// Get or set the github token used for restore
    Token(Token),
    /// Print the location of the global config
    Location
}

pub fn execute_config_operation(operation: Config)
{
    let mut config: AppConfig;
    if operation.local {
        config = AppConfig::read_local();
    } else {
        config = AppConfig::read();
    }

    let mut changed_any = false;
    match operation.op {
        ConfigOperation::Cache(c) => changed_any = execute_cache_config_operation(&mut config, c),
        ConfigOperation::Symlink(s) => changed_any = execute_symlink_config_operation(&mut config, s),
        ConfigOperation::Timeout(t) => changed_any = execute_timeout_config_operation(&mut config, t),
        ConfigOperation::Token(t) => execute_token_config_operation(t),
        ConfigOperation::Location => println!("Global Config is located at {}", AppConfig::global_config_path().bright_yellow())
    }

    if !changed_any { return; }
    
    if operation.local {
        config.write_local();
    } else {
        config.write();
    }
}

fn execute_cache_config_operation(config: &mut AppConfig, operation: Cache) -> bool
{
    match operation.op {
        CacheOperation::Path(p) => {
            if let Some(path) = p.path {
                // TODO implement check if valid
                let path_data = std::path::Path::new(&path);
                // if it's relative, that is bad, do not accept!
                if path_data.is_relative() {
                    println!("Path input {} is relative, this is not allowed! pass in absolute paths!", path.bright_yellow());
                // if it's a path to a file, that's not usable, do not accept!
                } else if path_data.is_file() {
                    println!("Path input {} is a file, this is not allowed! pass in a folder!", path.bright_yellow());
                } else {
                    // if we can not create the folder, that is bad, do not accept!
                    if let Err(err) = std::fs::create_dir_all(&path) {
                        println!("Creating dir {} failed! does qpm have permission to create that directory?", path.bright_yellow());
                        println!("Not setting cache path due to: {}", err.bright_red());
                        return false;
                    }
                    
                    // get temp file path
                    let temp_path: String;
                    if path.ends_with('/') || path.ends_with('\\') {
                        temp_path = format!("{}test.txt", &path);
                    } else {
                        temp_path = format!("{}\\test.txt", &path);
                    }

                    // check if we have write access
                    if std::fs::File::create(&temp_path).is_ok() {
                        std::fs::remove_file(&temp_path).expect("Couldn't remove created file");
                        println!("Set cache path to {}", path.bright_yellow());
                        config.cache = Some(path);
                        // TODO clean up old cache place ?
                        return true;
                    } else {
                        println!("Failed to set cache path to {}, since opening a test file there was not succesful", path.bright_yellow());
                    }
                }
            } else if let Some(path) = config.cache.as_ref() {
                println!("Current configured cache path is {}", path.bright_yellow());
            } else {
                println!("Cache path is not configured!");
            }
        },
    }

    false
}

fn set_symlink_usage(config: &mut AppConfig, value: bool)
{
    println!("Set symlink usage to {}", value.bright_yellow());
    config.symlink = Some(value);
}

fn execute_symlink_config_operation(config: &mut AppConfig, operation: Symlink) -> bool
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
        return true;
    } else if let Some(symlink) = config.symlink.as_ref() {
        println!("Current configured symlink usage is set to: {}", symlink.bright_yellow());
    } else {
        println!("Symlink usage is not configured!");
    }

    false
}

fn execute_timeout_config_operation(config: &mut AppConfig, operation: Timeout) -> bool
{
    if let Some(timeout) = operation.timeout {
        // TODO actually set the value
        println!("Set timeout to {}!", timeout.bright_yellow());
        config.timeout = Some(timeout);
        return true;
    } else {
        // TODO: make it actually read the value from config
        if let Some(timeout) = config.timeout.as_ref() {
            println!("Current configured timeout is set to: {}", timeout.bright_yellow());
        } else {
            println!("Timeout is not configured!");
        }
    }

    false
}

fn execute_token_config_operation(operation: Token)
{
    if operation.delete && get_keyring().get_password().is_ok() {
        get_keyring().delete_password().expect("Removing password failed");
        println!("Deleted github token from config, it will no longer be used");
        return;
    } else if operation.delete {
        println!("There was no github token configured, did not delete it");
        return;
    }
    
    if let Some(token) = operation.token {
        // write token
        get_keyring().set_password(&token).expect("Storing token failed!");
        println!("Configured a github token! This will now be used in qpm restore");
    } else {
        // read token
        if let Ok(token) = get_keyring().get_password() {
            println!("Configured github token: {}", token.bright_yellow());
        } else {
            println!("No token was configured, or getting the token failed!");
        }
    }
}