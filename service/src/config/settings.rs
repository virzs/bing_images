use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub uri: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: DatabaseConfig,
}
