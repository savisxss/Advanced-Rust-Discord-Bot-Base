use std::fs;
use toml;
use crate::config::Config;
use crate::bot::error::BotError;

pub struct ConfigLoader;

impl ConfigLoader {
    pub fn load() -> Result<Config, BotError> {
        let config_str = fs::read_to_string("config.toml")
            .map_err(|e| BotError::Config(format!("Failed to read config file: {}", e)))?;

        let config: Config = toml::from_str(&config_str)
            .map_err(|e| BotError::Config(format!("Failed to parse config file: {}", e)))?;

        Ok(config)
    }

    pub fn load_env() -> Result<(), BotError> {
        dotenv::dotenv()
            .map_err(|e| BotError::Config(format!("Failed to load .env file: {}", e)))?;

        Ok(())
    }
}