use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serenity::model::id::GuildId;
use crate::bot::error::BotResult;

pub struct GuildData {
    data: Arc<RwLock<HashMap<GuildId, HashMap<String, String>>>>,
}

impl GuildData {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn set(&self, guild_id: GuildId, key: &str, value: &str) -> BotResult<()> {
        let mut data = self.data.write().await;
        let guild_data = data.entry(guild_id).or_insert_with(HashMap::new);
        guild_data.insert(key.to_string(), value.to_string());
        Ok(())
    }

    pub async fn get(&self, guild_id: GuildId, key: &str) -> BotResult<Option<String>> {
        let data = self.data.read().await;
        Ok(data.get(&guild_id)
            .and_then(|guild_data| guild_data.get(key))
            .cloned())
    }

    pub async fn remove(&self, guild_id: GuildId, key: &str) -> BotResult<()> {
        let mut data = self.data.write().await;
        if let Some(guild_data) = data.get_mut(&guild_id) {
            guild_data.remove(key);
        }
        Ok(())
    }

    pub async fn get_all(&self, guild_id: GuildId) -> BotResult<Option<HashMap<String, String>>> {
        let data = self.data.read().await;
        Ok(data.get(&guild_id).cloned())
    }

    pub async fn clear(&self, guild_id: GuildId) -> BotResult<()> {
        let mut data = self.data.write().await;
        data.remove(&guild_id);
        Ok(())
    }

    pub async fn has_key(&self, guild_id: GuildId, key: &str) -> BotResult<bool> {
        let data = self.data.read().await;
        Ok(data.get(&guild_id)
            .map(|guild_data| guild_data.contains_key(key))
            .unwrap_or(false))
    }
}