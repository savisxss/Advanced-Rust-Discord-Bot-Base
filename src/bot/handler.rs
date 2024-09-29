use serenity::async_trait;
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::model::guild::Member;

use crate::bot::Bot;
use crate::config::Config;
use crate::database::Database;

pub struct Handler {
    bot: Bot,
}

impl Handler {
    pub fn new(config: Config, database: Database) -> Self {
        Self {
            bot: Bot::new(config, database),
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Err(why) = self.bot.handle_interaction(ctx, interaction).await {
            log::error!("Error handling interaction: {:?}", why);
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        if let Err(why) = self.bot.handle_ready(ctx, ready).await {
            log::error!("Error handling ready event: {:?}", why);
        }
    }

    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        let config = &self.bot.config;
        let welcome_channel_id = ChannelId(config.channels.welcome);
        let welcome_message = config.messages.welcome.replace("{user}", &new_member.user.name);

        if let Err(why) = welcome_channel_id.say(&ctx.http, welcome_message).await {
            println!("Error sending welcome message: {:?}", why);
        }
    }

    async fn guild_member_removal(&self, ctx: Context, guild_id: GuildId, user: User, _member_data: Option<Member>) {
        let config = &self.bot.config;
        let goodbye_channel_id = ChannelId(config.channels.general);
        let goodbye_message = config.messages.goodbye.replace("{user}", &user.name);

        if let Err(why) = goodbye_channel_id.say(&ctx.http, goodbye_message).await {
            println!("Error sending goodbye message: {:?}", why);
        }
    }
}