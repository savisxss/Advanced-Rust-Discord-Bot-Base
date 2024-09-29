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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    env_logger::init();

    let config = Config::load().expect("Failed to load configuration");

    let database = Database::new(&config.database_url).await?;

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler::new(config.clone(), database.clone()))
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<Config>(Arc::new(config));
        data.insert::<Database>(database);
    }

    if let Err(why) = client.start().await {
        log::error!("Client error: {:?}", why);
    }

    Ok(())
}