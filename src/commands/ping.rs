use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::*;
use crate::commands::Command;
use crate::bot::error::BotResult;
use crate::lang::Lang;

pub struct Ping;

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

    fn run(&self, ctx: &Context, _command: &ApplicationCommandInteraction, lang: &Lang) -> BotResult<String> {
        let latency = ctx.cache.current_user().unwrap().id.created_at().timestamp_millis() as u64;
        Ok(lang.get("commands.ping_response").replace("{latency}", &latency.to_string()))
    }
}