use serenity::model::id::{UserId, GuildId};
use serenity::model::permissions::Permissions;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::bot::error::BotResult;

pub struct SecurityManager {
    command_permissions: Arc<RwLock<HashMap<String, Permissions>>>,
    user_roles: Arc<RwLock<HashMap<(GuildId, UserId), Vec<String>>>>,
    blocked_users: Arc<RwLock<Vec<UserId>>>,
    rate_limits: Arc<RwLock<HashMap<String, (u32, std::time::Duration)>>>,
}

impl SecurityManager {
    pub fn new() -> Self {
        Self {
            command_permissions: Arc::new(RwLock::new(HashMap::new())),
            user_roles: Arc::new(RwLock::new(HashMap::new())),
            blocked_users: Arc::new(RwLock::new(Vec::new())),
            rate_limits: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn set_command_permissions(&self, command_name: &str, permissions: Permissions) {
        let mut command_permissions = self.command_permissions.write().await;
        command_permissions.insert(command_name.to_string(), permissions);
    }

    pub async fn check_permissions(&self, command: &ApplicationCommandInteraction, ctx: &Context) -> BotResult<bool> {
        let command_name = &command.data.name;
        let command_permissions = self.command_permissions.read().await;

        if let Some(required_permissions) = command_permissions.get(command_name) {
            if let Some(member) = &command.member {
                let guild = command.guild_id.unwrap().to_partial_guild(&ctx.http).await?;
                let user_permissions = guild.member_permissions(member.user.id)?;
                
                if !user_permissions.contains(*required_permissions) {
                    return Ok(false);
                }
            } else {
                return Ok(false);
            }
        }

        Ok(true)
    }

    pub async fn set_user_roles(&self, guild_id: GuildId, user_id: UserId, roles: Vec<String>) {
        let mut user_roles = self.user_roles.write().await;
        user_roles.insert((guild_id, user_id), roles);
    }

    pub async fn get_user_roles(&self, guild_id: GuildId, user_id: UserId) -> Vec<String> {
        let user_roles = self.user_roles.read().await;
        user_roles.get(&(guild_id, user_id)).cloned().unwrap_or_default()
    }

    pub async fn has_role(&self, guild_id: GuildId, user_id: UserId, role: &str) -> bool {
        let user_roles = self.get_user_roles(guild_id, user_id).await;
        user_roles.contains(&role.to_string())
    }

    pub async fn block_user(&self, user_id: UserId) {
        let mut blocked_users = self.blocked_users.write().await;
        if !blocked_users.contains(&user_id) {
            blocked_users.push(user_id);
        }
    }

    pub async fn unblock_user(&self, user_id: UserId) {
        let mut blocked_users = self.blocked_users.write().await;
        blocked_users.retain(|&id| id != user_id);
    }

    pub async fn is_user_blocked(&self, user_id: UserId) -> bool {
        let blocked_users = self.blocked_users.read().await;
        blocked_users.contains(&user_id)
    }

    pub async fn set_rate_limit(&self, command_name: &str, limit: u32, duration: std::time::Duration) {
        let mut rate_limits = self.rate_limits.write().await;
        rate_limits.insert(command_name.to_string(), (limit, duration));
    }

    pub async fn check_rate_limit(&self, command_name: &str, user_id: UserId) -> bool {
        true
    }

    pub fn validate_url(&self, url: &str) -> bool {
        url.starts_with("http://") || url.starts_with("https://")
    }

    pub fn escape_markdown(&self, text: &str) -> String {
        text.replace('*', "\\*")
            .replace('_', "\\_")
            .replace('`', "\\`")
            .replace('~', "\\~")
    }

    pub fn sanitize_input(&self, input: &str) -> String {
        input.replace('<', "&lt;").replace('>', "&gt;")
    }
}