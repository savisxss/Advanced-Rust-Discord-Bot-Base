use dotenv::dotenv;
use serenity::prelude::*;
use std::env;
use std::sync::Arc;
use tokio::sync::broadcast;

mod bot;
mod commands;
mod config;
mod database;
mod lang;
mod utils;
mod plugins;
mod security;
mod telemetry;
mod backup;

use bot::handler::Handler;
use config::Config;
use database::Database;
use utils::logger;
use utils::metrics::Metrics;
use utils::cache::Cache;
use utils::task_manager::TaskManager;
use utils::rate_limiter::RateLimiter;
use utils::guild_data::GuildData;
use lang::Lang;
use utils::event_bus::EventBus;
use plugins::PluginManager;
use security::SecurityManager;
use telemetry::TelemetryManager;
use backup::BackupManager;

use crate::plugins::example_plugin::ExamplePlugin;
use crate::telemetry::TelemetryManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    logger::init();

    let config = Arc::new(Config::load()?);
    let database = Arc::new(Database::new(&config.database.url).await?);
    database.run_migrations().await?;
    
    let metrics = Arc::new(Metrics::new());
    let cache = Arc::new(Cache::new(std::time::Duration::from_secs(300)));
    let task_manager = Arc::new(TaskManager::new(5));
    let rate_limiter = Arc::new(RateLimiter::new());
    let guild_data = Arc::new(GuildData::new());
    let lang = Arc::new(Lang::load(&config.bot.default_language)?);

    let (event_sender, _) = broadcast::channel(100);
    let event_bus = Arc::new(EventBus::new(event_sender));

    let plugin_manager = Arc::new(PluginManager::new());
    let security_manager = Arc::new(SecurityManager::new());
    let telemetry_manager = Arc::new(TelemetryManager::new(&config.telemetry)?);
    let backup_manager = Arc::new(BackupManager::new(&config.backup, Arc::clone(&database))?);

    let telemetry_manager = Arc::new(TelemetryManager::new(&config.telemetry));
    telemetry_manager.start_periodic_flush().await;

    plugin_manager.load_plugin(Box::new(ExamplePlugin)).await?;

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILDS;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler::new(
            Arc::clone(&config),
            Arc::clone(&database),
            Arc::clone(&metrics),
            Arc::clone(&cache),
            Arc::clone(&task_manager),
            Arc::clone(&rate_limiter),
            Arc::clone(&guild_data),
            Arc::clone(&lang),
            Arc::clone(&event_bus),
            Arc::clone(&plugin_manager),
            Arc::clone(&security_manager),
            Arc::clone(&telemetry_manager),
            Arc::clone(&backup_manager),
        ))
        .await
        .expect("Err creating client");

    task_manager.spawn("metrics_reporter", {
        let metrics = Arc::clone(&metrics);
        async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(60)).await;
                metrics.report().await;
            }
        }
    }).await?;

    task_manager.spawn("cache_cleaner", {
        let cache = Arc::clone(&cache);
        async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(300)).await;
                cache.cleanup().await;
            }
        }
    }).await?;

    task_manager.spawn("backup_scheduler", {
        let backup_manager = Arc::clone(&backup_manager);
        async move {
            backup_manager.run_scheduled_backups().await;
        }
    }).await?;

    log::info!("Starting bot...");
    telemetry_manager.log_event("bot_start").await?;

    if let Err(why) = client.start().await {
        log::error!("Client error: {:?}", why);
        telemetry_manager.log_error("client_error", &why.to_string()).await?;
    }

    Ok(())
}