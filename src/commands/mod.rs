use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::*;
use crate::bot::error::BotResult;
use crate::lang::Lang;

pub mod ping;
pub mod help;

pub trait Command {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn register(&self, command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand;
    fn run(&self, ctx: &Context, command: &ApplicationCommandInteraction, lang: &Lang) -> BotResult<String>;
}

pub struct CommandHandler {
    commands: Vec<Box<dyn Command>>,
}

impl CommandHandler {
    pub fn new() -> Self {
        let mut handler = Self { commands: Vec::new() };
        handler.register_commands();
        handler
    }

    fn register_commands(&mut self) {
        self.commands.push(Box::new(ping::Ping));
        self.commands.push(Box::new(help::Help));
    }

    pub fn get_commands(&self) -> &[Box<dyn Command>] {
        &self.commands
    }

    pub async fn handle_command(&self, ctx: &Context, command: &ApplicationCommandInteraction, lang: &Lang) -> BotResult<String> {
        for cmd in &self.commands {
            if cmd.name() == command.data.name {
                return cmd.run(ctx, command, lang);
            }
        }
        Err(crate::bot::error::BotError::UnknownCommand(command.data.name.clone()))
    }

    pub fn register_application_commands(&self, commands: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
        for command in &self.commands {
            commands.create_application_command(|create_command| {
                command.register(create_command)
            });
        }
        commands
    }
}