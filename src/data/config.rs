use serde::{Serialize, Deserialize};
use dirs;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub symlink: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<String>,
    pub timeout: u64
}

impl Default for Config {
    #[inline]
    fn default() -> Config {
        Config {
            symlink: false,
            cache: None,
            timeout: 5000
        }
    }
}
impl Config {
    pub fn read() -> Config
    {
        // todo make it use a local qpm settings file
        let path = format!("{}\\QPM-Rust\\qpm.settings.json", dirs::config_dir().append());

        if let Some(file) = std::fs::File::open(path) {
            // existed
            let mut config = String::new();
            file.read_to_string(&mut config).expect("Reading data failed");
    
            return serde_json::from_str::<Config>(&config).expect("Deserializing package failed");
        } else {
            // didn't exist
            return Config { .. Default::default() };
        }
    }

    pub fn write(&self)
    {
        let config = serde_json::to_string_pretty(&self).expect("Serialization failed");

        let path = format!("{}\\QPM-Rust\\qpm.settings.json", dirs::config_dir().append());

        let mut file = std::fs::File::create(path).expect("create failed");
        file.write_all(config.as_bytes()).expect("write failed");
        println!("Saved Config!");
    }
}