use serenity::async_trait;
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use crate::bot::Bot;
use crate::config::Config;
use crate::database::Database;

pub struct Handler {
    bot: Bot,
}

impl Handler {
    pub fn new(config: Config, database: Database) -> Self {
        Self {
            bot: Bot::new(config, database),
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Err(why) = self.bot.handle_interaction(ctx, interaction).await {
            log::error!("Error handling interaction: {:?}", why);
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        if let Err(why) = self.bot.handle_ready(ctx, ready).await {
            log::error!("Error handling ready event: {:?}", why);
        }
    }
}