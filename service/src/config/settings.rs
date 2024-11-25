use std::fs;
use serde::Deserialize;
use toml;

#[derive(Debug, Deserialize)]
pub struct MongoConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
    pub collection: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub mongo: MongoConfig,
}

pub fn get_settings_from_file(file_path: &str) -> Settings {
    let config_content = fs::read_to_string(file_path).expect("Failed to read config file");
    toml::from_str(&config_content).expect("Failed to parse config file")
}
