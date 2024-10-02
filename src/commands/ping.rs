use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::*;
use crate::commands::Command;
use crate::bot::Bot;
use crate::bot::error::BotResult;

pub struct Ping;

#[async_trait]
impl Command for Ping {
    fn name(&self) -> String {
        "ping".to_string()
    }

    fn description(&self) -> String {
        "A simple ping command".to_string()
    }

    fn register(&self, command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name(self.name())
            .description(self.description())
    }

    async fn run(&self, bot: &Bot, ctx: &Context, _command: &ApplicationCommandInteraction) -> BotResult<String> {
        let latency = ctx.cache.current_user().unwrap().id.created_at().timestamp_millis() as u64;
        let response = bot.lang.get("commands.ping_response").replace("{latency}", &latency.to_string());
        bot.telemetry_manager.log_event("ping_command_used").await?;
        Ok(response)
    }
}