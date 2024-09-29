use serenity::builder::CreateEmbed;
use serenity::utils::Colour;
use crate::lang::Lang;

pub struct EmbedBuilder<'a> {
    lang: &'a Lang,
}

impl<'a> EmbedBuilder<'a> {
    pub fn new(lang: &'a Lang) -> Self {
        Self { lang }
    }

    pub fn build_simple(&self, title_key: &str, description_key: &str, color: Colour) -> CreateEmbed {
        let mut embed = CreateEmbed::default();
        embed.title(self.lang.get(title_key))
            .description(self.lang.get(description_key))
            .color(color);
        embed
    }

    pub fn build_error(&self, title_key: &str, description_key: &str) -> CreateEmbed {
        self.build_simple(title_key, description_key, Colour::RED)
    }

    pub fn build_success(&self, title_key: &str, description_key: &str) -> CreateEmbed {
        self.build_simple(title_key, description_key, Colour::GREEN)
    }

    pub fn build_info(&self, title_key: &str, description_key: &str) -> CreateEmbed {
        self.build_simple(title_key, description_key, Colour::BLUE)
    }

    pub fn build_custom(&self, f: impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed) -> CreateEmbed {
        let mut embed = CreateEmbed::default();
        f(&mut embed);
        embed
    }

    pub fn add_field(&self, embed: &mut CreateEmbed, name_key: &str, value_key: &str, inline: bool) -> &mut CreateEmbed {
        embed.field(self.lang.get(name_key), self.lang.get(value_key), inline)
    }

    pub fn set_footer(&self, embed: &mut CreateEmbed, text_key: &str, icon_url: Option<&str>) -> &mut CreateEmbed {
        embed.footer(|f| {
            f.text(self.lang.get(text_key));
            if let Some(url) = icon_url {
                f.icon_url(url);
            }
            f
        })
    }
}