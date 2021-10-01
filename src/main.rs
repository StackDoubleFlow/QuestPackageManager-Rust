use serde::{Serialize, Deserialize};
//use std::fs::{read_to_string};
use clap::{AppSettings, Clap};

mod cache;
use cache::Cache;

mod clear;
mod collapse;
mod collect;
mod config;

mod dependency;
mod makeqmod;
mod package;
mod propertieslist;
mod publish;
mod restore;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap, Debug)]
#[clap(version = "0.1.0", author = "RedBrumbler & Sc2ad")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// the github token to use for operations
    #[clap(short, long)]
    token: Option<String>,
    #[clap(subcommand)]
    subcmd: MainCommand
}

#[derive(Clap, Debug, Clone)]
enum MainCommand {
    /// Cache control
    Cache(Cache),
    /// Clear all resolved dependencies by clearing the lock file
    Clear,
    /// Collect and collapse dependencies and print them to console
    Collapse,
    /// Collect dependencies and print them to console
    Collect,
    /// Config control
    Config(config::Config),
    /// Dependency control
    Dependency,
    /// Package control
    Package,
    /// List all properties that are currently supported by QPM
    PropertiesList,
    /// Publish package
    Publish,
    /// Restore and resolve all dependencies from the package
    Restore,
    /// Makes the qmod from the files specified
    MakeQmod
}

fn main() {
    let opts: Opts = Opts::parse();
    let token = opts.token.clone();
    if token.is_some()
    {
        println!("using token {}", token.unwrap());

    }

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    match opts.subcmd.clone() {
        MainCommand::Cache(c) => { cache::ExecuteCacheOperation(c.op); },
        MainCommand::Clear => { clear::ExecuteClearOperation(); },
        MainCommand::Collapse => { collapse::ExecuteCollapseOperation(); },
        MainCommand::Collect => { collect::ExecuteCollectOperation(); },
        MainCommand::Config(c) => { config::ExecuteConfigOperation(c); },
        MainCommand::Dependency => { println!("Dependency"); },
        MainCommand::Package => { println!("Package"); },
        MainCommand::PropertiesList => { println!("PropertiesList"); },
        MainCommand::Publish => { println!("Publish"); },
        MainCommand::Restore => { println!("Restore"); },
        MainCommand::MakeQmod => { println!("MakeQmod"); }
    }

    println!("opts: {:#?}", opts);
    // more program logic goes here...
}

/*
fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    println!("{:#?}", matches);
}
*/

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub cache_path: String,
    pub timeout: u32
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Dependency {
    pub id: String,
    pub versionRange: String,
    #[serde(default)]
    pub additionalData: serde_json::Value
}

impl Default for Dependency {
    #[inline]
    fn default() -> Dependency {
        Dependency {
            id: "".to_string(),
            versionRange: "".to_string(),
            additionalData: serde_json::Value::default()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PackageInfo {
    pub name: String,
    pub id: String,
    pub version: String,
    pub url: String,
    #[serde(default)]
    pub additionalData: serde_json::Value
}

impl Default for PackageInfo {
    #[inline]
    fn default() -> PackageInfo {
        PackageInfo {
            name: "".to_string(),
            id: "".to_string(),
            version: "".to_string(),
            url: "".to_string(),
            additionalData: serde_json::Value::default()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PackageConfig {
    pub sharedDir: String,
    pub dependenciesDir: String,
    pub info: PackageInfo,
    pub dependencies: Vec<Dependency>,
    #[serde(default)]
    pub additionalData: serde_json::Value
}

impl Default for PackageConfig {
    #[inline]
    fn default() -> PackageConfig {
        PackageConfig {
            sharedDir: "shared".to_string(),
            dependenciesDir: "extern".to_string(),
            info: PackageInfo::default(),
            dependencies: Vec::<Dependency>::default(),
            additionalData: serde_json::Value::default()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RestoredDependency {
    pub dependency: Dependency,
    pub version: String
}

impl Default for RestoredDependency {
    #[inline]
    fn default() -> RestoredDependency {
        RestoredDependency {
            dependency: Dependency::default(),
            version: "".to_string()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SharedPackageConfig {
    pub config: PackageConfig,
    pub restoredDependencies: Vec<RestoredDependency>
}

impl Default for SharedPackageConfig {
    #[inline]
    fn default() -> SharedPackageConfig {
        SharedPackageConfig {
            config: PackageConfig::default(),
            restoredDependencies: Vec::<RestoredDependency>::default(),
        }
    }
}