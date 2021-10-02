use clap::{Clap, AppSettings};

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Cache {
    /// Clear the cache
    #[clap(subcommand)]
    pub op: CacheOperation
}

#[derive(Clap, Debug, Clone)]
pub enum CacheOperation {
    /// Clear the cache
    Clear
}

pub fn execute_cache_operation(op: CacheOperation)
{
    match op {
        CacheOperation::Clear => clear(),
    }
}

fn clear()
{
    println!("It should clear the cached files now");
}