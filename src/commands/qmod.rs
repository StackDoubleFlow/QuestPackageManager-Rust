use clap::{Clap, AppSettings};

use crate::data::mod_json::{ModJson, ModDependency};

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Qmod {
    #[clap(subcommand)]
    pub op: QmodOperation
}

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
#[allow(non_snake_case)]
pub struct CreateQmodJsonOperationArgs {
    /// The Questpatcher version this mod.json was made for
    #[clap(long="qpversion")]
    pub _QPVersion: Option<String>,
    /// Name of the mod
    #[clap(long)]
    pub name: String,
    /// ID of the mod
    #[clap(long)]
    pub id: String,
    /// Author of the mod
    #[clap(long)]
    pub author: String,
    /// Optional slot for if you ported a mod
    #[clap(long)]
    pub porter: Option<String>,
    /// Mod version
    #[clap(long)]
    pub version: String,
    /// id of the package the mod is for, ex. com.beatgaems.beatsaber
    #[clap(long="packageID")]
    pub packageId: String,
    /// Version of the package, ex. 1.1.0
    #[clap(long="packageversion")]
    pub packageVersion: String,
    /// description for the mod
    #[clap(long)]
    pub description: Option<String>,
    /// optional cover image filename
    #[clap(long="coverimage")]
    pub coverImage: Option<String>,
}

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub enum QmodOperation {
    Create(CreateQmodJsonOperationArgs),
    Build,
    Edit,

}

pub fn execute_qmod_operation(operation: Qmod)
{
    match operation.op {
        QmodOperation::Create(q) => execute_qmod_create(q),
        QmodOperation::Build => execute_qmod_build(),
        QmodOperation::Edit => execute_qmod_edit(),
    }
}

fn execute_qmod_create(create_parameters: CreateQmodJsonOperationArgs)
{
    let _QPVersion: String;
    match create_parameters._QPVersion {
        Option::Some(s) => _QPVersion = s,
        Option::None => _QPVersion = "0.1.1".to_string()
    }
    let modJson = ModJson {
        _QPVersion: _QPVersion,
        name: create_parameters.name,
        id: create_parameters.id,
        author: create_parameters.author,
        porter: create_parameters.porter,
        version: create_parameters.version,
        packageId: create_parameters.packageId,
        packageVersion: create_parameters.packageVersion,
        description: create_parameters.description,
        coverImage: create_parameters.coverImage,
        ..Default::default()
    };

    modJson.write();
}

fn execute_qmod_build()
{

}

fn execute_qmod_edit()
{

}