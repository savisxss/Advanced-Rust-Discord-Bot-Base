use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::ApplicationCommandInteraction;
use crate::commands::Command;

pub struct Ping;

impl Command for Ping {
    fn name(&self) -> String {
        "ping".to_string()
    }

    fn register(&self, command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name("ping")
            .description("A simple ping command")
    }

    fn run(&self, _command: &ApplicationCommandInteraction) -> String {
        "Pong!".to_string()
    }
}