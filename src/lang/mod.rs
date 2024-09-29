use std::collections::HashMap;
use serde::Deserialize;
use crate::bot::error::{BotResult, BotError};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Lang {
    messages: HashMap<String, String>,
    fallback: Option<Box<Lang>>,
}

impl Lang {
    pub fn load(lang: &str) -> BotResult<Self> {
        let lang_path = format!("lang/{}.toml", lang);
        let file_content = fs::read_to_string(&lang_path)
            .map_err(|e| BotError::Config(format!("Failed to read language file {}: {}", lang_path, e)))?;
        
        let mut lang: Lang = toml::from_str(&file_content)
            .map_err(|e| BotError::Config(format!("Failed to parse language file {}: {}", lang_path, e)))?;

        if lang != "en" {
            lang.fallback = Some(Box::new(Lang::load("en")?));
        }

        Ok(lang)
    }

    pub fn get(&self, key: &str) -> &str {
        self.messages.get(key).map(String::as_str).unwrap_or_else(|| {
            if let Some(fallback) = &self.fallback {
                fallback.get(key)
            } else {
                key
            }
        })
    }

    pub fn get_with_params(&self, key: &str, params: &[(&str, &str)]) -> String {
        let mut message = self.get(key).to_string();
        for (param, value) in params {
            message = message.replace(&format!("{{{}}}", param), value);
        }
        message
    }

    pub fn has_key(&self, key: &str) -> bool {
        self.messages.contains_key(key) || self.fallback.as_ref().map_or(false, |f| f.has_key(key))
    }

    pub fn list_keys(&self) -> Vec<String> {
        let mut keys: Vec<String> = self.messages.keys().cloned().collect();
        if let Some(fallback) = &self.fallback {
            keys.extend(fallback.list_keys());
        }
        keys.sort();
        keys.dedup();
        keys
    }
}

pub fn load_all_languages(config: &crate::config::Config) -> BotResult<HashMap<String, Lang>> {
    let lang_dir = Path::new("lang");
    let mut languages = HashMap::new();

    for entry in fs::read_dir(lang_dir)
        .map_err(|e| BotError::Config(format!("Failed to read lang directory: {}", e)))? {
        let entry = entry.map_err(|e| BotError::Config(format!("Failed to read directory entry: {}", e)))?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "toml") {
            if let Some(lang_code) = path.file_stem().and_then(|s| s.to_str()) {
                let lang = Lang::load(lang_code)?;
                languages.insert(lang_code.to_string(), lang);
            }
        }
    }

    if !languages.contains_key(&config.bot.default_language) {
        return Err(BotError::Config(format!("Default language '{}' not found", config.bot.default_language)));
    }

    Ok(languages)
}