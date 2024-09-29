use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::ApplicationCommandInteraction;
use crate::commands::{Command, CommandHandler};

pub struct Help;

impl Command for Help {
    fn name(&self) -> String {
        "help".to_string()
    }

    fn register(&self, command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name("help")
            .description("Shows a list of available commands")
    }

    fn run(&self, _command: &ApplicationCommandInteraction) -> String {
        let handler = CommandHandler::new();
        let mut help_text = String::from("Available commands:\n");

        for cmd in handler.get_commands() {
            help_text.push_str(&format!("/{} - {}\n", cmd.name(), cmd.register(&mut CreateApplicationCommand::default()).description.clone().unwrap_or_default()));
        }

        help_text
    }
}