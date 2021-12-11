use std::path::PathBuf;

use clap::{AppSettings, Clap};
use owo_colors::OwoColorize;

use crate::data::config::Config as AppConfig;

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Cache {
    #[clap(subcommand)]
    pub op: CacheOperation,
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
    pub path: Option<PathBuf>,
}

pub fn execute_cache_config_operation(config: &mut AppConfig, operation: Cache) -> bool {
    match operation.op {
        CacheOperation::Path(p) => {
            if let Some(path) = p.path {
                let path_data = path.as_path();
                // if it's relative, that is bad, do not accept!
                if path_data.is_relative() {
                    println!(
                        "Path input {} is relative, this is not allowed! pass in absolute paths!",
                        path.display().bright_yellow()
                    );
                // if it's a path to a file, that's not usable, do not accept!
                } else if path_data.is_file() {
                    println!(
                        "Path input {} is a file, this is not allowed! pass in a folder!",
                        path.display().bright_yellow()
                    );
                } else {
                    // if we can not create the folder, that is bad, do not accept!
                    if let Err(err) = std::fs::create_dir_all(&path) {
                        println!("Creating dir {} failed! does qpm have permission to create that directory?", path.display().bright_yellow());
                        println!("Not setting cache path due to: {}", err.bright_red());
                        return false;
                    }

                    // get temp file path
                    let temp_path = path.join("temp.txt");

                    // check if we have write access
                    if std::fs::File::create(&temp_path).is_ok() {
                        std::fs::remove_file(&temp_path).expect("Couldn't remove created file");
                        println!("Set cache path to {}", path.display().bright_yellow());
                        println!(
                            "\nDon't forget to clean up your old cache location if needed: {}",
                            config.cache.clone().unwrap().display().bright_yellow()
                        );
                        config.cache = Some(path);
                        return true;
                    } else {
                        println!("Failed to set cache path to {}, since opening a test file there was not succesful", path.display().bright_yellow());
                    }
                }
            } else if let Some(path) = config.cache.as_ref() {
                println!(
                    "Current configured cache path is {}",
                    path.display().bright_yellow()
                );
            } else {
                println!("Cache path is not configured!");
            }
        }
    }

    false
}
