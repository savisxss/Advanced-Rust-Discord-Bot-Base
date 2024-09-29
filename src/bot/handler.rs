use serenity::async_trait;
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::guild::{Guild, Member};
use serenity::model::id::GuildId;
use serenity::model::user::User;
use serenity::prelude::*;
use std::sync::Arc;

use crate::bot::Bot;
use crate::config::Config;
use crate::database::Database;
use crate::utils::metrics::Metrics;
use crate::utils::cache::Cache;
use crate::utils::task_manager::TaskManager;
use crate::utils::rate_limiter::RateLimiter;
use crate::utils::guild_data::GuildData;

pub struct Handler {
    bot: Bot,
}

impl Handler {
    pub fn new(
        config: Config,
        database: Database,
        metrics: Arc<Metrics>,
        cache: Arc<Cache<String, String>>,
        task_manager: Arc<TaskManager>,
        rate_limiter: Arc<RateLimiter>,
        guild_data: Arc<GuildData>,
    ) -> Self {
        Self {
            bot: Bot::new(config, database, metrics, cache, task_manager, rate_limiter, guild_data),
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            if !self.bot.rate_limiter.check("command", command.user.id.0).await {
                let _ = command.create_interaction_response(&ctx.http, |response| {
                    response.interaction_response_data(|message| 
                        message.content("You're using commands too quickly. Please slow down.")
                    )
                }).await;
                return;
            }

            self.bot.metrics.increment_command(&command.data.name).await;

            let cache_key = format!("last_command:{}", command.user.id);
            self.bot.cache.set(cache_key.clone(), command.data.name.clone()).await;

            if let Err(why) = self.bot.handle_command(&ctx, &command).await {
                log::error!("Error handling command: {:?}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        log::info!("{} is connected!", ready.user.name);
        
        if let Err(why) = self.bot.register_commands(&ctx).await {
            log::error!("Error registering slash commands: {:?}", why);
        }

        let ctx_clone = ctx.clone();
        self.bot.task_manager.spawn("status_update", async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(60)).await;
                ctx_clone.set_activity(Activity::playing("with slash commands")).await;
            }
        }).await.expect("Failed to spawn status update task");
    }

    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        let guild_id = new_member.guild_id;
        let welcome_message = self.bot.guild_data.get(guild_id, "welcome_message")
            .await
            .unwrap_or(Ok("Welcome to the server!".to_string()))
            .unwrap_or("Welcome to the server!".to_string());

        if let Err(why) = new_member.user.dm(&ctx.http, |m| m.content(welcome_message)).await {
            log::error!("Error sending welcome message: {:?}", why);
        }

        self.bot.metrics.log_event("member_join").await;
    }

    async fn guild_member_removal(&self, ctx: Context, guild_id: GuildId, user: User, _member_data: Option<Member>) {
        let goodbye_message = self.bot.guild_data.get(guild_id, "goodbye_message")
            .await
            .unwrap_or(Ok("Goodbye! We hope to see you again.".to_string()))
            .unwrap_or("Goodbye! We hope to see you again.".to_string());

        if let Err(why) = user.dm(&ctx.http, |m| m.content(goodbye_message)).await {
            log::error!("Error sending goodbye message: {:?}", why);
        }

        self.bot.metrics.log_event("member_leave").await;
    }

    async fn guild_create(&self, _ctx: Context, guild: Guild, is_new: bool) {
        if is_new {
            log::info!("Joined new guild: {}", guild.name);
            self.bot.metrics.log_event("guild_join").await;
        }
    }

    async fn guild_delete(&self, _ctx: Context, incomplete: GuildId, _full: Option<Guild>) {
        log::info!("Left guild: {}", incomplete);
        self.bot.metrics.log_event("guild_leave").await;
    }

    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        log::info!("Cache is ready!");
        let guild_count = ctx.cache.guild_count();
        self.bot.metrics.set_gauge("connected_guilds", guild_count as f64).await;
    }
}