use serde::Deserialize;
use std::fs;
use toml;

#[derive(Clone, Deserialize)]
pub struct Config {
    pub guild_id: u64,
    pub database_url: String,
    pub log_channel: u64,
    pub roles: Roles,
    pub channels: Channels,
}

#[derive(Clone, Deserialize)]
pub struct Roles {
    pub admin: u64,
    pub moderator: u64,
    pub member: u64,
}

#[derive(Clone, Deserialize)]
pub struct Channels {
    pub welcome: u64,
    pub general: u64,
    pub logs: u64,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string("config.toml")?;
        let config: Config = toml::from_str(&config_str)?;
        Ok(config)
    }
}