use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub content_root: String,
    pub secret: String
}

impl Config {
    pub fn load_unsafe() -> Self {
        let contents = fs::read_to_string("./config.json").unwrap();
        serde_json::from_str(&contents).unwrap()
    }
}