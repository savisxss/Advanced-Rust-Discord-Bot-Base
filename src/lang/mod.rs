use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Lang {
    messages: HashMap<String, String>,
}

impl Lang {
    pub fn load(lang: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file_content = std::fs::read_to_string(format!("lang/{}.toml", lang))?;
        let lang: Lang = toml::from_str(&file_content)?;
        Ok(lang)
    }

    pub fn get(&self, key: &str) -> &str {
        self.messages.get(key).map(String::as_str).unwrap_or(key)
    }
}