pub mod embed_builder;
pub mod error_handler;

use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn send_embed(ctx: &Context, channel_id: ChannelId, embed: &CreateEmbed) -> Result<Message, serenity::Error> {
    channel_id.send_message(&ctx.http, |m| {
        m.set_embed(embed.clone())
    }).await
}

pub fn log_to_console(message: &str) {
    println!("[LOG] {}", message);
}