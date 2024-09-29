use dotenv::dotenv;
use serenity::prelude::*;
use std::env;
use std::sync::Arc;

mod bot;
mod commands;
mod config;
mod database;
mod lang;
mod utils;

use bot::handler::Handler;
use config::Config;
use database::Database;
use utils::logger;
use utils::metrics::Metrics;
use utils::cache::Cache;
use utils::task_manager::TaskManager;
use utils::rate_limiter::RateLimiter;
use utils::guild_data::GuildData;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    logger::init();

    let config = Config::load()?;
    let database = Database::new(&config.database.url).await?;
    let metrics = Arc::new(Metrics::new());
    let cache = Arc::new(Cache::new(std::time::Duration::from_secs(300)));
    let task_manager = Arc::new(TaskManager::new());
    let rate_limiter = Arc::new(RateLimiter::new());
    let guild_data = Arc::new(GuildData::new());

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILDS;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler::new(
            config.clone(),
            database.clone(),
            metrics.clone(),
            cache.clone(),
            task_manager.clone(),
            rate_limiter.clone(),
            guild_data.clone(),
        ))
        .await
        .expect("Err creating client");

    log::info!("Starting bot...");
    if let Err(why) = client.start().await {
        log::error!("Client error: {:?}", why);
    }

    Ok(())
}