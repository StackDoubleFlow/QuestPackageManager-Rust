use serde::{Serialize, Deserialize};
use std::io::{Read, Write};
use dirs;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub symlink: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<String>,
    pub timeout: Option<u64>
}

impl Default for Config {
    #[inline]
    fn default() -> Config {
        Config {
            symlink: Some(false),
            cache: Some(format!("{}\\QPM-Rust\\", dirs::config_dir().unwrap().display())),
            timeout: Some(5000)
        }
    }
}

#[allow(dead_code)]
impl Config {
    /// always gets the global config
    pub fn read() -> Config
    {
        // todo make it use a local qpm settings file
        let path = format!("{}\\QPM-Rust\\qpm.settings.json", dirs::config_dir().unwrap().display());
        std::fs::create_dir_all(&format!("{}\\QPM-Rust\\", dirs::config_dir().unwrap().display())).expect("Failed to make config folder");
        
        if let Ok(mut file) = std::fs::File::open(path) {
            // existed
            let mut config_str = String::new();
            file.read_to_string(&mut config_str).expect("Reading data failed");
    
            serde_json::from_str::<Config>(&config_str).expect("Deserializing package failed")
        } else {
            // didn't exist
            Config { .. Default::default() }
        }
    }

    /// combines the values of the global config with whatever is written in a local qpm.settings.json
    pub fn read_combine() -> Config
    {
        let mut config = Config::read();

        // read a local qpm.settings.json to 
        let local_path = "qpm.settings.json";
        if let Ok(mut file) = std::fs::File::open(local_path) {
            let mut config_str = String::new();
            file.read_to_string(&mut config_str).expect("Reading data failed");
            
            let local_config = serde_json::from_str::<Config>(&config_str).expect("Deserializing package failed");

            if local_config.symlink.is_some() { config.symlink = local_config.symlink; }
            if local_config.cache.is_some() { config.cache = local_config.cache; }
            if local_config.timeout.is_some() { config.timeout = local_config.timeout; }
        }

        config
    }

    pub fn write(&self)
    {
        let config = serde_json::to_string_pretty(&self).expect("Serialization failed");

        let path = format!("{}\\QPM-Rust\\qpm.settings.json", dirs::config_dir().unwrap().display());

        let mut file = std::fs::File::create(path).expect("create failed");
        file.write_all(config.as_bytes()).expect("write failed");
        println!("Saved Config!");
    }
}