use serde::Deserialize;
use crate::bot::error::{BotResult, BotError};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub bot: BotConfig,
    pub database: DatabaseConfig,
    pub discord: DiscordConfig,
    pub telemetry: TelemetryConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BotConfig {
    pub name: String,
    pub owners: Vec<u64>,
    pub default_language: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DiscordConfig {
    pub token: String,
    pub application_id: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TelemetryConfig {
    pub enabled: bool,
    pub log_file: String,
    pub batch_size: usize,
}

impl Config {
    pub fn load() -> BotResult<Self> {
        let config_path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config.toml".to_string());
        let config_str = fs::read_to_string(&config_path)
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
        if self.discord.token.is_empty() {
            return Err(BotError::Config("Discord token cannot be empty".to_string()));
        }
        if self.discord.application_id == 0 {
            return Err(BotError::Config("Invalid Discord application ID".to_string()));
        }
        Ok(())
    }

    pub fn get_owner_ids(&self) -> &[u64] {
        &self.bot.owners
    }

    pub fn is_owner(&self, user_id: u64) -> bool {
        self.bot.owners.contains(&user_id)
    }
}

pub fn load_env() -> BotResult<()> {
    dotenv::from_path(Path::new(".env"))
        .map_err(|e| BotError::Config(format!("Failed to load .env file: {}", e)))?;
    Ok(())
}