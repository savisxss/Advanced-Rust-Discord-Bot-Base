use crate::bot::error::BotError;
use serenity::model::channel::Message;
use serenity::prelude::*;

pub async fn handle_error(ctx: &Context, msg: &Message, error: BotError) {
    let error_message = match error {
        BotError::Serenity(why) => format!("Discord API Error: {:?}", why),
        BotError::Database(why) => format!("Database Error: {:?}", why),
        BotError::UnknownCommand(cmd) => format!("Unknown command: {}", cmd),
        BotError::Config(why) => format!("Configuration Error: {}", why),
        BotError::Internal(why) => format!("Internal Error: {}", why),
    };

    if let Err(why) = msg.channel_id.say(&ctx.http, &error_message).await {
        println!("Error sending error message: {:?}", why);
    }

    log::error!("{}", error_message);
}