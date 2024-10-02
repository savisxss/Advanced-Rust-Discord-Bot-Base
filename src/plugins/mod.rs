use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use async_trait::async_trait;
use serenity::builder::CreateApplicationCommands;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::*;
use crate::bot::Bot;
use crate::bot::error::BotResult;

#[async_trait]
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn commands(&self) -> Vec<Box<dyn PluginCommand>>;
    async fn on_load(&self, bot: &Bot) -> BotResult<()>;
    async fn on_unload(&self, bot: &Bot) -> BotResult<()>;
}

#[async_trait]
pub trait PluginCommand: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn register(&self, command: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands;
    async fn run(&self, bot: &Bot, ctx: &Context, command: &ApplicationCommandInteraction) -> BotResult<String>;
}

pub struct PluginManager {
    plugins: RwLock<HashMap<String, Box<dyn Plugin>>>,
    commands: RwLock<HashMap<String, Box<dyn PluginCommand>>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: RwLock::new(HashMap::new()),
            commands: RwLock::new(HashMap::new()),
        }
    }

    pub async fn load_plugin(&self, bot: &Bot, plugin: Box<dyn Plugin>) -> BotResult<()> {
        let plugin_name = plugin.name().to_string();
        plugin.on_load(bot).await?;

        let mut plugins = self.plugins.write().await;
        let mut commands = self.commands.write().await;

        for command in plugin.commands() {
            commands.insert(command.name().to_string(), command);
        }

        plugins.insert(plugin_name, plugin);
        Ok(())
    }

    pub async fn unload_plugin(&self, bot: &Bot, plugin_name: &str) -> BotResult<()> {
        let mut plugins = self.plugins.write().await;
        let mut commands = self.commands.write().await;

        if let Some(plugin) = plugins.remove(plugin_name) {
            plugin.on_unload(bot).await?;
            for command in plugin.commands() {
                commands.remove(command.name());
            }
        }

        Ok(())
    }

    pub async fn get_command(&self, name: &str) -> Option<Box<dyn PluginCommand>> {
        let commands = self.commands.read().await;
        commands.get(name).cloned()
    }

    pub fn register_commands(&self, commands: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
        let commands_lock = self.commands.blocking_read();
        for command in commands_lock.values() {
            command.register(commands);
        }
        commands
    }

    pub async fn get_plugins(&self) -> Vec<String> {
        let plugins = self.plugins.read().await;
        plugins.keys().cloned().collect()
    }

    pub async fn get_commands(&self) -> Vec<Box<dyn PluginCommand>> {
        let commands = self.commands.read().await;
        commands.values().cloned().collect()
    }
}