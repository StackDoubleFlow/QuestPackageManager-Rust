use clap::{Clap};

#[derive(Clap, Debug, Clone)]
pub struct Config {
    /// The operation to execute
    #[clap(subcommand)]
    pub op: ConfigOperation
}

#[derive(Clap, Debug, Clone)]
pub struct CacheSetPathOperation {
    pub path: Option<String>,
}

#[derive(Clap, Debug, Clone)]
pub struct Cache {
    #[clap(subcommand)]
    pub op: CacheOperation
}

#[derive(Clap, Debug, Clone)]
pub enum CacheOperation {
    /// Gets or sets the path to place the QPM Cache
    Path(CacheSetPathOperation),
}

#[derive(Clap, Debug, Clone)]
pub enum ConfigOperation {
    /// Get or set the cache path
    Cache(Cache),
    /// Enable or disable symlink usage
    Symlink,
    /// Get or set the timeout for web requests
    Timeout
}

pub fn ExecuteConfigOperation(operation: Config)
{
    match operation.op.clone() {
        ConfigOperation::Cache(c) => { ExecuteCacheConfigOperation(c); },
        ConfigOperation::Symlink => {},
        ConfigOperation::Timeout => {}
    }
}

fn ExecuteCacheConfigOperation(operation: Cache)
{
    match operation.op.clone() {
        CacheOperation::Path(p) => {
            if p.path.is_some()
            {
                // TODO make it set
                println!("path should've been set to {:#?}", p.path.unwrap());
            }
            else
            {
                // TODO make it get
                println!("path should've been printed");
            }
        },
    }
}