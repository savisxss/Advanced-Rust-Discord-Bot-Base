use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use crate::utils::error::BotResult;

pub trait Command {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn register(&self, command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand;
    fn run(&self, ctx: &Context, command: &ApplicationCommandInteraction) -> BotResult<String>;
}

pub trait CommandGroup {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn commands(&self) -> Vec<Box<dyn Command>>;
}

pub struct CommandHandler {
    groups: Vec<Box<dyn CommandGroup>>,
}

impl CommandHandler {
    pub fn new() -> Self {
        Self { groups: Vec::new() }
    }

    pub fn add_group(&mut self, group: Box<dyn CommandGroup>) {
        self.groups.push(group);
    }

    pub fn register_commands(&self, ctx: &Context, guild_id: GuildId) -> BotResult<()> {
        guild_id.set_application_commands(&ctx.http, |commands| {
            for group in &self.groups {
                for command in group.commands() {
                    commands.create_application_command(|cmd| command.register(cmd));
                }
            }
            commands
        })?;

        Ok(())
    }

    pub async fn handle_command(&self, ctx: &Context, command: &ApplicationCommandInteraction) -> BotResult<()> {
        for group in &self.groups {
            for cmd in group.commands() {
                if cmd.name() == command.data.name {
                    let response = cmd.run(ctx, command)?;
                    command.create_interaction_response(&ctx.http, |r| {
                        r.interaction_response_data(|d| d.content(response))
                    }).await?;
                    return Ok(());
                }
            }
        }
        Err(BotError::Command(format!("Unknown command: {}", command.data.name)))
    }
}