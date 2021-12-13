use clap::{AppSettings, Clap};
use semver::Version;

use crate::data::mod_json::ModJson;

/// Some properties are not editable through the qmod edit command, these properties are either editable through the package, or not at all
#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct EditQmodJsonOperationArgs {
    /// The schema version this mod was made for
    #[clap(long = "qpversion")]
    pub schema_version: Option<Version>,
    /// Author of the mod
    #[clap(long)]
    pub author: Option<String>,
    /// Optional slot for if you ported a mod
    #[clap(long)]
    pub porter: Option<String>,
    /// id of the package the mod is for, ex. com.beatgaems.beatsaber
    #[clap(long = "packageID")]
    pub package_id: Option<String>,
    /// Version of the package, ex. 1.1.0
    #[clap(long = "packageVersion")]
    pub package_version: Option<String>,
    /// description for the mod
    #[clap(long)]
    pub description: Option<String>,
    /// optional cover image filename
    #[clap(long = "coverimage")]
    pub cover_image: Option<String>,
}

pub fn execute_qmod_edit_operation(edit_parameters: EditQmodJsonOperationArgs) {
    // TODO: Make it actually edit qmod stuff like mod files and other things
    let mut json = ModJson::read(ModJson::get_template_path());

    if let Some(schema_version) = edit_parameters.schema_version {
        json.schema_version = schema_version;
    }
    if let Some(author) = edit_parameters.author {
        json.author = author;
    }
    if let Some(porter) = edit_parameters.porter {
        if porter == "clear" {
            json.porter = None;
        } else {
            json.porter = Some(porter);
        }
    }
    if let Some(package_id) = edit_parameters.package_id {
        json.package_id = package_id;
    }
    if let Some(package_version) = edit_parameters.package_version {
        json.package_version = package_version;
    }
    if let Some(description) = edit_parameters.description {
        if description == "clear" {
            json.description = None;
        } else {
            json.description = Some(description);
        }
    }
    if let Some(cover_image) = edit_parameters.cover_image {
        if cover_image == "clear" {
            json.cover_image = None;
        } else {
            json.cover_image = Some(cover_image);
        }
    }

    json.write(ModJson::get_template_path());
}
