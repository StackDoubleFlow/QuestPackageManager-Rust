use clap::{AppSettings, Clap};
use owo_colors::OwoColorize;
use remove_dir_all::remove_dir_contents;
use walkdir::WalkDir;

use crate::data::config::Config;

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Cache {
    /// Clear the cache
    #[clap(subcommand)]
    pub op: CacheOperation,
}

#[derive(Clap, Debug, Clone)]
pub enum CacheOperation {
    /// Clear the cache
    Clear,
    /// Lists versions for each cached package
    List,
    /// Shows you the current cache path
    Path,
}

pub fn execute_cache_operation(operation: Cache) {
    match operation.op {
        CacheOperation::Clear => clear(),
        CacheOperation::List => list(),
        CacheOperation::Path => path(),
    }
}

fn clear() {
    let config = Config::read_combine();
    let path = config.cache.unwrap();
    remove_dir_contents(path).expect("Failed to remove cached folders");
}

fn path() {
    let config = Config::read_combine();
    println!(
        "Config path is: {}",
        config.cache.unwrap().display().bright_yellow()
    );
}

fn list() {
    let config = Config::read_combine();
    let path = config.cache.unwrap();

    for dir in WalkDir::new(&path).max_depth(2).min_depth(1) {
        let unwrapped = dir.unwrap();
        if unwrapped.depth() == 1 {
            println!(
                "package {}:",
                unwrapped.file_name().to_string_lossy().bright_red()
            );
        } else {
            println!(
                " - {}",
                unwrapped.file_name().to_string_lossy().bright_green()
            );
        }
    }
}
