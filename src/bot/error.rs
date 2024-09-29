use thiserror::Error;
use serenity::prelude::SerenityError;

#[derive(Error, Debug)]
pub enum BotError {
    #[error("Serenity error: {0}")]
    Serenity(#[from] SerenityError),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Command error: {0}")]
    Command(String),

    #[error("Interaction error: {0}")]
    Interaction(String),

    #[error("Rate limit error: {0}")]
    RateLimit(String),

    #[error("API error: {0}")]
    Api(String),

    #[error("Cache error: {0}")]
    Cache(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type BotResult<T> = Result<T, BotError>;

impl From<std::io::Error> for BotError {
    fn from(err: std::io::Error) -> Self {
        BotError::Internal(err.to_string())
    }
}

impl From<toml::de::Error> for BotError {
    fn from(err: toml::de::Error) -> Self {
        BotError::Config(err.to_string())
    }
}