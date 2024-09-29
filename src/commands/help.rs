use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::*;
use crate::commands::{Command, CommandHandler};
use crate::config::Config;
use crate::bot::error::BotResult;
use crate::lang::Lang;

pub struct Help;

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

    fn run(&self, _ctx: &Context, _command: &ApplicationCommandInteraction, lang: &Lang) -> BotResult<String> {
        let handler = CommandHandler::new();
        let mut help_text = lang.get("commands.help_title").to_string() + "\n\n";
        help_text += &lang.get("commands.help_description") + "\n\n";

        for cmd in handler.get_commands() {
            help_text += &format!("/{} - {}\n", cmd.name(), cmd.description());
        }

        Ok(help_text)
    }
}