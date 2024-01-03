use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Config {
    pub routes: Vec<RouteConfig>,
    pub services: std::collections::HashMap<String, ServiceConfig>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RouteConfig {
    pub path: String,
    pub service: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ServiceConfig {
    pub url: String,
}

pub fn load_config() -> Result<Config, toml::de::Error> {
    let config_content = include_str!("../solidflare.toml");
    toml::from_str(config_content)
}