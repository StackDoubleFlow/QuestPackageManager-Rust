use std::io::{Read, Write};

use clap::{AppSettings, Clap};
use owo_colors::OwoColorize;
use remove_dir_all::remove_dir_contents;
use walkdir::WalkDir;

use crate::data::{config::Config, package::PackageConfig};

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
    /// Fixes some dependencies that use technically wrong include paths
    LegacyFix,
}

pub fn execute_cache_operation(operation: Cache) {
    match operation.op {
        CacheOperation::Clear => clear(),
        CacheOperation::List => list(),
        CacheOperation::Path => path(),
        CacheOperation::LegacyFix => legacy_fix(),
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

fn legacy_fix() {
    for entry in WalkDir::new(Config::read_combine().cache.unwrap())
        .min_depth(2)
        .max_depth(2)
    {
        let path = entry.unwrap().into_path().join("src");
        let package = PackageConfig::read_path(path.join("qpm.json"));

        let shared_path = path.join(package.shared_dir);

        for entry in WalkDir::new(shared_path) {
            let entry_path = entry.unwrap().into_path();
            if entry_path.is_file() {
                let mut file = match std::fs::File::open(&entry_path) {
                    Ok(o) => o,
                    Err(e) => panic!(
                        "Opening file {} to read failed: {}",
                        entry_path.display().bright_yellow(),
                        e
                    ),
                };

                let mut buf: String = "".to_string();
                match file.read_to_string(&mut buf) {
                    Ok(_) => {}
                    Err(_e) => {
                        #[cfg(debug_assertions)]
                        println!(
                            "reading file {} to string failed: {}",
                            entry_path.display().bright_yellow(),
                            _e
                        );
                        continue;
                    }
                };
                fs_extra::file::remove(&entry_path).unwrap_or_else(|_| {
                    panic!(
                        "removing file {} failed",
                        entry_path.display().bright_yellow()
                    )
                });
                let mut file = std::fs::File::create(&entry_path).unwrap_or_else(|_| {
                    panic!(
                        "opening file {} to write failed",
                        entry_path.display().bright_yellow()
                    )
                });
                file.write_all(
                    buf.replace(
                        "#include \"extern/beatsaber-hook/",
                        "#include \"beatsaber-hook/",
                    )
                    .as_bytes(),
                )
                .unwrap_or_else(|_| {
                    panic!(
                        "writing file {} failed",
                        entry_path.display().bright_yellow()
                    )
                });
            }
        }
    }
}
