use serenity::builder::CreateEmbed;
use serenity::utils::Colour;

pub struct EmbedBuilder;

impl EmbedBuilder {
    pub fn build_simple(title: &str, description: &str, color: Colour) -> CreateEmbed {
        let mut embed = CreateEmbed::default();
        embed.title(title)
            .description(description)
            .color(color);
        embed
    }

    pub fn build_error(title: &str, description: &str) -> CreateEmbed {
        Self::build_simple(title, description, Colour::RED)
    }

    pub fn build_success(title: &str, description: &str) -> CreateEmbed {
        Self::build_simple(title, description, Colour::GREEN)
    }

    pub fn build_info(title: &str, description: &str) -> CreateEmbed {
        Self::build_simple(title, description, Colour::BLUE)
    }
}