use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::sync::Arc;

use crate::config::Config;
use crate::database::Database;
use crate::commands;
use crate::utils::metrics::Metrics;
use crate::utils::cache::Cache;
use crate::utils::task_manager::TaskManager;
use crate::utils::rate_limiter::RateLimiter;
use crate::utils::guild_data::GuildData;
use crate::lang::Lang;
use crate::error::{BotError, BotResult};

pub struct Bot {
    config: Config,
    database: Database,
    metrics: Arc<Metrics>,
    cache: Arc<Cache<String, String>>,
    task_manager: Arc<TaskManager>,
    rate_limiter: Arc<RateLimiter>,
    guild_data: Arc<GuildData>,
    lang: Lang,
}

impl Bot {
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
            config,
            database,
            metrics,
            cache,
            task_manager,
            rate_limiter,
            guild_data,
            lang,
        }
    }

    pub fn get_message(&self, key: &str) -> String {
        self.lang.get(key).to_string()
    }

    pub async fn handle_interaction(&self, ctx: Context, interaction: Interaction) -> BotResult<()> {
        match interaction {
            Interaction::ApplicationCommand(command) => {
                if !self.rate_limiter.check("command", command.user.id.0).await {
                    command
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|message| 
                                    message.content(self.get_message("errors.rate_limit"))
                                )
                        })
                        .await
                        .map_err(|e| BotError::Serenity(e))?;
                    return Ok(());
                }

                self.metrics.increment_command(&command.data.name).await;

                let cache_key = format!("last_command:{}", command.user.id);
                self.cache.set(cache_key.clone(), command.data.name.clone()).await;

                let content = match command.data.name.as_str() {
                    "ping" => commands::ping::run(&self.lang),
                    "help" => commands::help::run(&self.config, &self.lang),
                    _ => Err(BotError::UnknownCommand(command.data.name.clone())),
                }?;

                command
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| message.content(content))
                    })
                    .await
                    .map_err(|e| BotError::Serenity(e))?;
            }
            _ => {}
        }
        Ok(())
    }

    pub async fn handle_ready(&self, ctx: Context, ready: Ready) -> BotResult<()> {
        log::info!("{} is connected!", ready.user.name);

        let guild_id = GuildId(self.config.guild_id);

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::ping::register(command))
                .create_application_command(|command| commands::help::register(command))
        })
        .await
        .map_err(|e| BotError::Serenity(e))?;

        log::info!("Registered slash commands: {:#?}", commands);

        self.rate_limiter.add_limit("command", 5, std::time::Duration::from_secs(10));

        self.start_periodic_tasks(ctx.clone());

        Ok(())
    }

    fn start_periodic_tasks(&self, ctx: Context) {
        let metrics = self.metrics.clone();
        self.task_manager.spawn("metrics_reporter", async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(300)).await;
                let guild_count = ctx.cache.guild_count();
                metrics.set_gauge("connected_guilds", guild_count as f64).await;
            }
        }).await.expect("Failed to spawn metrics reporter task");
    }
}