#![feature(is_symlink, once_cell)]
#![allow(dead_code)]

use std::{
    io::{Read, Write},
    str::FromStr,
};

use clap::{AppSettings, Clap};
use serde::{Deserialize, Serialize};

mod commands;
mod data;
mod resolver;
mod utils;

/// QPM is a command line tool that allows modmakers to
/// easily download dependencies for interacting with a game or other mods
#[derive(Clap, Debug)]
#[clap(version = "0.1.0", author = "RedBrumbler & Sc2ad")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: MainCommand,
}

#[derive(Clap, Debug, Clone)]
enum MainCommand {
    /// Cache control
    Cache(commands::cache::Cache),
    /// Clear all resolved dependencies by clearing the lock file
    Clear,
    /// Collect and collapse dependencies and print them to console
    Collapse,
    /// Config control
    Config(commands::config::Config),
    /// Dependency control
    Dependency(commands::dependency::Dependency),
    /// Package control
    Package(commands::package::Package),
    /// List all properties that are currently supported by QPM
    List(commands::list::ListOperation),
    /// Publish package
    Publish,
    /// Restore and resolve all dependencies from the package
    Restore,
    /// Qmod control
    Qmod(commands::qmod::Qmod),
    /// Test parsing and writing of cmake files
    TestCMakeParse,
}

fn main() {
    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    match (Opts::parse() as Opts).subcmd {
        MainCommand::Cache(c) => commands::cache::execute_cache_operation(c),
        MainCommand::Clear => commands::clear::execute_clear_operation(),
        MainCommand::Collapse => commands::collapse::execute_collapse_operation(),
        MainCommand::Config(c) => commands::config::execute_config_operation(c),
        MainCommand::Dependency(d) => commands::dependency::execute_dependency_operation(d),
        MainCommand::Package(p) => commands::package::execute_package_operation(p),
        MainCommand::List(l) => commands::list::execute_list_operation(l),
        MainCommand::Publish => commands::publish::execute_publish_operation(),
        MainCommand::Restore => commands::restore::execute_restore_operation(),
        MainCommand::Qmod(q) => commands::qmod::execute_qmod_operation(q),
        MainCommand::TestCMakeParse => test_cmake_parse(),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub cache_path: String,
    pub timeout: u32,
}

fn test_cmake_parse() {
    let mut file = std::fs::File::open("CMakeLists.txt").expect("Opening qpm.json failed");
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let cmake_list = crate::data::c_make::CMakeList::from_str(&buf).unwrap();

    std::fs::File::create("CMakeLists.txt")
        .unwrap()
        .write_all(cmake_list.to_string().as_bytes())
        .expect("Failed to write out file");
}
