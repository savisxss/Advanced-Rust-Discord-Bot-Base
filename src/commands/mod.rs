use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::ApplicationCommandInteraction;

pub mod ping;
pub mod help;

pub trait Command {
    fn name(&self) -> String;
    fn register(&self, command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand;
    fn run(&self, command: &ApplicationCommandInteraction) -> String;
}

pub struct CommandHandler {
    commands: Vec<Box<dyn Command>>,
}

impl CommandHandler {
    pub fn new() -> Self {
        Self {
            commands: vec![
                Box::new(ping::Ping),
                Box::new(help::Help),
            ],
        }
    }

    pub fn get_commands(&self) -> &Vec<Box<dyn Command>> {
        &self.commands
    }

    pub fn find_command(&self, name: &str) -> Option<&Box<dyn Command>> {
        self.commands.iter().find(|cmd| cmd.name() == name)
    }

    pub fn register_commands(&self, ctx: &Context, guild_id: GuildId) -> Result<(), SerenityError> {
        let commands = guild_id.set_application_commands(&ctx.http, |commands| {
            for command in &self.commands {
                commands.create_application_command(|cmd| command.register(cmd));
            }
            commands
        })?;

        println!("Registered the following commands: {:?}", commands);
        Ok(())
    }
}