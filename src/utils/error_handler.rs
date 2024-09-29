use crate::bot::error::BotError;
use serenity::model::channel::Message;
use serenity::prelude::*;
use crate::lang::Lang;

pub async fn handle_error(ctx: &Context, msg: &Message, error: BotError, lang: &Lang) {
    let error_message = match error {
        BotError::Serenity(why) => format!("{}: {:?}", lang.get("errors.discord_api"), why),
        BotError::Database(why) => format!("{}: {:?}", lang.get("errors.database"), why),
        BotError::UnknownCommand(cmd) => format!("{}: {}", lang.get("errors.unknown_command"), cmd),
        BotError::Config(why) => format!("{}: {}", lang.get("errors.configuration"), why),
        BotError::Internal(why) => format!("{}: {}", lang.get("errors.internal"), why),
        BotError::RateLimit(why) => format!("{}: {}", lang.get("errors.rate_limit"), why),
    };

    if let Err(why) = msg.channel_id.say(&ctx.http, &error_message).await {
        log::error!("Error sending error message: {:?}", why);
    }

    log::error!("{}", error_message);
}