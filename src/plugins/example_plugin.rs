use async_trait::async_trait;
use serenity::builder::CreateApplicationCommands;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::*;
use crate::bot::Bot;
use crate::bot::error::BotResult;
use crate::plugins::{Plugin, PluginCommand};

pub struct ExamplePlugin;

#[async_trait]
impl Plugin for ExamplePlugin {
    fn name(&self) -> &str {
        "example"
    }

    fn description(&self) -> &str {
        "An example plugin"
    }

    fn commands(&self) -> Vec<Box<dyn PluginCommand>> {
        vec![Box::new(ExampleCommand)]
    }

    async fn on_load(&self, _bot: &Bot) -> BotResult<()> {
        println!("Example plugin loaded!");
        Ok(())
    }

    async fn on_unload(&self, _bot: &Bot) -> BotResult<()> {
        println!("Example plugin unloaded!");
        Ok(())
    }
}

struct ExampleCommand;

#[async_trait]
impl PluginCommand for ExampleCommand {
    fn name(&self) -> &str {
        "example"
    }

    fn description(&self) -> &str {
        "An example command"
    }

    fn register(&self, command: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
        command.create_application_command(|command| {
            command.name(self.name()).description(self.description())
        })
    }

    async fn run(&self, bot: &Bot, _ctx: &Context, _command: &ApplicationCommandInteraction) -> BotResult<String> {
        bot.telemetry_manager.log_event("example_command_used").await?;
        Ok("This is an example command from a plugin!".to_string())
    }
}