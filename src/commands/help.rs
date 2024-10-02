use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::*;
use crate::commands::Command;
use crate::bot::Bot;
use crate::bot::error::BotResult;

pub struct Help;

#[async_trait]
impl Command for Help {
    fn name(&self) -> String {
        "help".to_string()
    }

    fn description(&self) -> String {
        "Shows a list of available commands".to_string()
    }

    fn register(&self, command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name(self.name())
            .description(self.description())
    }

    async fn run(&self, bot: &Bot, _ctx: &Context, _command: &ApplicationCommandInteraction) -> BotResult<String> {
        let mut help_text = bot.lang.get("commands.help_title").to_string() + "\n\n";
        help_text += &bot.lang.get("commands.help_description") + "\n\n";

        for cmd in bot.plugin_manager.get_commands() {
            help_text += &format!("/{} - {}\n", cmd.name(), cmd.description());
        }

        bot.telemetry_manager.log_event("help_command_used").await?;
        Ok(help_text)
    }
}