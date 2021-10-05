use clap::{Clap, AppSettings};
#[allow(non_camel_case_types)]



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
    pub timeout: Option<u32>
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
    match &operation.op {
        ConfigOperation::Cache(c) => execute_cache_config_operation(c),
        ConfigOperation::Symlink(s) => execute_symlink_config_operation(s),
        ConfigOperation::Timeout(t) => execute_timeout_config_operation(t)
    }
}

fn execute_cache_config_operation(operation: &Cache)
{
    match operation.op.clone() {
        CacheOperation::Path(p) => {
            if p.path.is_some()
            {
                // TODO: make it set
                println!("path should've been set to {:#?}", p.path.unwrap());
            }
            else
            {
                // TODO: make it get
                println!("path should've been printed");
            }
        },
    }
}

fn set_symlink_usage(value: bool)
{
    // TODO: make it actually set
    println!("Symlink set: {}", value);
}

fn execute_symlink_config_operation(operation: &Symlink)
{
    // value is given

    match &operation.op {
        Option::Some(s) => {
            match &s {
                SymlinkOperation::Enable => {
                    set_symlink_usage(true);
                },
                SymlinkOperation::Disable => {
                    set_symlink_usage(false);
                }
            }
        }
        Option::None => {
            println!("Symlink usage config should've been printed");
        }
    }
}

fn execute_timeout_config_operation(operation: &Timeout)
{
    match operation.timeout {
        Option::Some(t) => {
            // TODO actually set the value
            println!("Timeout set to {}", t);
        }
        Option::None => {
            // TODO: make it actually read the value from config
            println!("Timeout value should've been printed");
        }
    }
}