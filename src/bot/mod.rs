pub mod error;
pub mod handler;

use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use crate::config::Config;
use crate::database::Database;
use crate::commands;

pub struct Bot {
    config: Config,
    database: Database,
}

impl Bot {
    pub fn new(config: Config, database: Database) -> Self {
        Self { config, database }
    }

    pub async fn handle_interaction(&self, ctx: Context, interaction: Interaction) -> Result<(), error::BotError> {
        match interaction {
            Interaction::ApplicationCommand(command) => {
                let content = match command.data.name.as_str() {
                    "ping" => commands::ping::run(&command.data.options),
                    "help" => commands::help::run(&self.config, &command.data.options),
                    _ => Err(error::BotError::UnknownCommand(command.data.name.clone())),
                }?;

                command
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| message.content(content))
                    })
                    .await
                    .map_err(|e| error::BotError::Serenity(e))?;
            }
            _ => {}
        }
        Ok(())
    }

    pub async fn handle_ready(&self, ctx: Context, ready: Ready) -> Result<(), error::BotError> {
        log::info!("{} is connected!", ready.user.name);

        let guild_id = GuildId(self.config.guild_id);

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::ping::register(command))
                .create_application_command(|command| commands::help::register(command))
        })
        .await
        .map_err(|e| error::BotError::Serenity(e))?;

        log::info!("Registered slash commands: {:#?}", commands);

        Ok(())
    }
}