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
use crate::lang::Lang;

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
        lang: Lang,
    ) -> Self {
        Self {
            bot: Bot::new(config, database, metrics, cache, task_manager, rate_limiter, guild_data, lang),
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

    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        let guild_id = new_member.guild_id;
        let welcome_message = self.bot.guild_data.get(guild_id, "welcome_message")
            .await
            .unwrap_or(Ok(self.bot.get_message("events.member_join")))
            .unwrap_or_else(|_| self.bot.get_message("events.member_join"));

        let welcome_message = welcome_message.replace("{user}", &new_member.user.name);

        if let Err(why) = new_member.user.dm(&ctx.http, |m| m.content(welcome_message)).await {
            log::error!("Error sending welcome message: {:?}", why);
        }

        self.bot.metrics.log_event("member_join").await;
    }

    async fn guild_member_removal(&self, ctx: Context, guild_id: GuildId, user: User, _member_data: Option<Member>) {
        let goodbye_message = self.bot.guild_data.get(guild_id, "goodbye_message")
            .await
            .unwrap_or(Ok(self.bot.get_message("events.member_leave")))
            .unwrap_or_else(|_| self.bot.get_message("events.member_leave"));

        let goodbye_message = goodbye_message.replace("{user}", &user.name);

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