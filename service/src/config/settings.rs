use std::env;
use std::fs;

use dotenv::dotenv;
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

pub fn get_settings() -> Settings {
    dotenv().ok();

    // 从环境变量构造 Settings
    let mongo = MongoConfig {
        host: env::var("MONGO_HOST").expect("MONGO_HOST must be set"),
        port: env::var("MONGO_PORT")
            .expect("MONGO_PORT must be set")
            .parse()
            .expect("MONGO_PORT must be a number"),
        username: env::var("MONGO_USERNAME").expect("MONGO_USERNAME must be set"),
        password: env::var("MONGO_PASSWORD").expect("MONGO_PASSWORD must be set"),
        database: env::var("MONGO_DATABASE").expect("MONGO_DATABASE must be set"),
        collection: env::var("MONGO_COLLECTION").expect("MONGO_DATABASE must be set"),
    };

    Settings { mongo }
}

pub fn get_settings_from_file(file_path: &str) -> Settings {
    let config_content = fs::read_to_string(file_path).expect("Failed to read config file");
    toml::from_str(&config_content).expect("Failed to parse config file")
}
