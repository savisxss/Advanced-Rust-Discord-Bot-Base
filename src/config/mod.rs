use serde::Deserialize;
use crate::utils::error::{BotResult, BotError};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub bot: BotConfig,
    pub database: DatabaseConfig,
    pub discord: DiscordConfig,
}

#[derive(Debug, Deserialize)]
pub struct BotConfig {
    pub name: String,
    pub prefix: String,
    pub owners: Vec<u64>,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Deserialize)]
pub struct DiscordConfig {
    pub token: String,
    pub application_id: u64,
}

impl Config {
    pub fn load() -> BotResult<Self> {
        let config_str = std::fs::read_to_string("config.toml")
            .map_err(|e| BotError::Config(format!("Failed to read config file: {}", e)))?;

        let config: Config = toml::from_str(&config_str)
            .map_err(|e| BotError::Config(format!("Failed to parse config file: {}", e)))?;

        config.validate()?;

        Ok(config)
    }

    fn validate(&self) -> BotResult<()> {
        if self.bot.name.is_empty() {
            return Err(BotError::Config("Bot name cannot be empty".to_string()));
        }
        if self.bot.prefix.is_empty() {
            return Err(BotError::Config("Bot prefix cannot be empty".to_string()));
        }
        if self.discord.token.is_empty() {
            return Err(BotError::Config("Discord token cannot be empty".to_string()));
        }
        if self.discord.application_id == 0 {
            return Err(BotError::Config("Invalid Discord application ID".to_string()));
        }
        Ok(())
    }
}