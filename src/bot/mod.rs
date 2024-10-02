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
use crate::plugins::PluginManager;
use crate::security::SecurityManager;
use crate::telemetry::TelemetryManager;
use crate::backup::BackupManager;

pub struct Bot {
    pub config: Arc<Config>,
    pub database: Arc<Database>,
    pub metrics: Arc<Metrics>,
    pub cache: Arc<Cache<String, String>>,
    pub task_manager: Arc<TaskManager>,
    pub rate_limiter: Arc<RateLimiter>,
    pub guild_data: Arc<GuildData>,
    pub lang: Arc<Lang>,
    pub plugin_manager: Arc<PluginManager>,
    pub security_manager: Arc<SecurityManager>,
    pub telemetry_manager: Arc<TelemetryManager>,
    pub backup_manager: Arc<BackupManager>,
}

impl Bot {
    pub fn new(
        config: Arc<Config>,
        database: Arc<Database>,
        metrics: Arc<Metrics>,
        cache: Arc<Cache<String, String>>,
        task_manager: Arc<TaskManager>,
        rate_limiter: Arc<RateLimiter>,
        guild_data: Arc<GuildData>,
        lang: Arc<Lang>,
        plugin_manager: Arc<PluginManager>,
        security_manager: Arc<SecurityManager>,
        telemetry_manager: Arc<TelemetryManager>,
        backup_manager: Arc<BackupManager>,
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
            plugin_manager,
            security_manager,
            telemetry_manager,
            backup_manager,
        }
    }

    pub async fn handle_interaction(&self, ctx: Context, interaction: Interaction) -> BotResult<()> {
        match interaction {
            Interaction::ApplicationCommand(command) => {
                if !self.security_manager.check_permissions(&command, &ctx).await? {
                    return Err(BotError::MissingPermissions);
                }

                if !self.rate_limiter.check("command", command.user.id.0).await {
                    command
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|message| 
                                    message.content(self.lang.get("errors.rate_limit"))
                                )
                        })
                        .await?;
                    return Ok(());
                }

                self.metrics.increment_command(&command.data.name).await;
                self.telemetry_manager.log_command(&command.data.name).await?;

                let content = match command.data.name.as_str() {
                    "ping" => commands::ping::run(&self.lang),
                    "help" => commands::help::run(&self.config, &self.lang),
                    _ => {
                        if let Some(plugin_command) = self.plugin_manager.get_command(&command.data.name) {
                            plugin_command.run(self, &ctx, &command).await?
                        } else {
                            return Err(BotError::UnknownCommand(command.data.name.clone()));
                        }
                    }
                }?;

                command
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| message.content(content))
                    })
                    .await?;
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
                .create_application_command(|command| commands::help::register(command));
            
            self.plugin_manager.register_commands(commands);
            commands
        })
        .await?;

        log::info!("Registered slash commands: {:#?}", commands);

        self.rate_limiter.add_limit("command", 5, std::time::Duration::from_secs(10));

        self.start_periodic_tasks(ctx.clone());

        self.telemetry_manager.log_event("bot_ready").await?;

        Ok(())
    }

    fn start_periodic_tasks(&self, ctx: Context) {
        let metrics = self.metrics.clone();
        let task_manager = self.task_manager.clone();
        let telemetry_manager = self.telemetry_manager.clone();
        
        task_manager.spawn("metrics_reporter", async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(300)).await;
                let guild_count = ctx.cache.guild_count();
                metrics.set_gauge("connected_guilds", guild_count as f64).await;
                telemetry_manager.log_metric("connected_guilds", guild_count as f64).await.unwrap_or_else(|e| log::error!("Failed to log metric: {:?}", e));
            }
        }).await.expect("Failed to spawn metrics reporter task");
    }
}