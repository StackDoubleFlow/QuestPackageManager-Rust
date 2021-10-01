use clap::{Clap};

#[derive(Clap, Debug, Clone)]
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

pub fn ExecuteCacheOperation(op: CacheOperation)
{
    match op {
        CacheOperation::Clear => Clear(),
    }
}

fn Clear()
{

}