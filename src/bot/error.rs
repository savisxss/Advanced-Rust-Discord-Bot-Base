use thiserror::Error;

#[derive(Error, Debug)]
pub enum BotError {
    #[error("Serenity error: {0}")]
    Serenity(#[from] serenity::Error),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Unknown command: {0}")]
    UnknownCommand(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Internal error: {0}")]
    Internal(String),
}