use clap::{AppSettings, Clap};
use semver::Version;

use crate::data::mod_json::ModJson;

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Qmod {
    #[clap(subcommand)]
    pub op: QmodOperation,
}

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct CreateQmodJsonOperationArgs {
    /// The schema version this mod was made for
    #[clap(long = "qpversion")]
    pub schema_version: Option<String>,
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
    pub version: Version,
    /// id of the package the mod is for, ex. com.beatgaems.beatsaber
    #[clap(long = "packageID")]
    pub package_id: String,
    /// Version of the package, ex. 1.1.0
    #[clap(long = "packageversion")]
    pub package_version: String,
    /// description for the mod
    #[clap(long)]
    pub description: Option<String>,
    /// optional cover image filename
    #[clap(long = "coverimage")]
    pub cover_image: Option<String>,
}

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
#[allow(clippy::large_enum_variant)]
pub enum QmodOperation {
    Create(CreateQmodJsonOperationArgs),
    Build,
    Edit,
}

pub fn execute_qmod_operation(operation: Qmod) {
    match operation.op {
        QmodOperation::Create(q) => execute_qmod_create_operation(q),
        QmodOperation::Build => execute_qmod_build_operation(),
        QmodOperation::Edit => execute_qmod_edit_operation(),
    }
}

fn execute_qmod_create_operation(create_parameters: CreateQmodJsonOperationArgs) {
    let schema_version: String;
    match create_parameters.schema_version {
        Option::Some(s) => schema_version = s,
        Option::None => schema_version = "0.1.1".to_string(),
    }
    let json = ModJson {
        schema_version,
        name: create_parameters.name,
        id: create_parameters.id,
        author: create_parameters.author,
        porter: create_parameters.porter,
        version: create_parameters.version,
        package_id: create_parameters.package_id,
        package_version: create_parameters.package_version,
        description: create_parameters.description,
        cover_image: create_parameters.cover_image,
        dependencies: Default::default(),
        mod_files: Default::default(),
        library_files: Default::default(),
        file_copies: Default::default(),
    };

    json.write();
}

fn execute_qmod_build_operation() {
    // TODO: Make it work (probably executing cmake --build ./ or something like that)
}

fn execute_qmod_edit_operation() {
    // TODO: Make it actually edit qmod stuff like mod files and other things
}
