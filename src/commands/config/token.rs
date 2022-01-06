use clap::{Args};
use owo_colors::OwoColorize;

use crate::data::config::get_keyring;

#[derive(Args, Debug, Clone)]
pub struct Token {
    pub token: Option<String>,
    #[clap(long)]
    pub delete: bool,
}

pub fn execute_token_config_operation(operation: Token) {
    if operation.delete && get_keyring().get_password().is_ok() {
        get_keyring()
            .delete_password()
            .expect("Removing password failed");
        println!("Deleted github token from config, it will no longer be used");
        return;
    } else if operation.delete {
        println!("There was no github token configured, did not delete it");
        return;
    }

    if let Some(token) = operation.token {
        // write token
        get_keyring()
            .set_password(&token)
            .expect("Storing token failed!");
        println!("Configured a github token! This will now be used in qpm restore");
    } else {
        // read token, possibly unused so prepend with _ to prevent warnings
        if let Ok(_token) = get_keyring().get_password() {
            #[cfg(debug_assertions)]
            println!("Configured github token: {}", _token.bright_yellow());
            #[cfg(not(debug_assertions))]
            println!(
                "In release builds you {} view the configured github token, a token was configured though!",
                "cannot".bright_red()
            );
        } else {
            println!("No token was configured, or getting the token failed!");
        }
    }
}
