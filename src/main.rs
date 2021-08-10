use serde::{Serialize, Deserialize};

fn main() {
    let mut cfg = Config {cache_path: "test".to_string(), timeout: 3 };
    let ser = serde_json::to_string(&cfg).unwrap();
    println!("serialize = {}", ser);
    cfg = serde_json::from_str(&ser).unwrap();
    println!("deser = {:?}", cfg);
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub cache_path: String,
    pub timeout: u32
}

