pub mod embed_builder;
pub mod error_handler;
pub mod metrics;
pub mod cache;
pub mod task_manager;
pub mod rate_limiter;
pub mod guild_data;
pub mod logger;

use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;
use crate::bot::error::BotResult;

pub async fn send_embed(ctx: &Context, channel_id: ChannelId, embed: &CreateEmbed) -> BotResult<Message> {
    channel_id.send_message(&ctx.http, |m| {
        m.set_embed(embed.clone())
    }).await.map_err(|e| e.into())
}

pub fn log_to_console(message: &str) {
    println!("[LOG] {}", message);
}

pub async fn update_presence(ctx: &Context, status: &str) -> BotResult<()> {
    ctx.set_activity(Activity::playing(status)).await;
    Ok(())
}

pub fn format_duration(duration: std::time::Duration) -> String {
    let seconds = duration.as_secs();
    let minutes = seconds / 60;
    let hours = minutes / 60;
    let days = hours / 24;

    if days > 0 {
        format!("{}d {}h {}m {}s", days, hours % 24, minutes % 60, seconds % 60)
    } else if hours > 0 {
        format!("{}h {}m {}s", hours, minutes % 60, seconds % 60)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds % 60)
    } else {
        format!("{}s", seconds)
    }
}