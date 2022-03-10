use poise::serenity_prelude::{Context, Message};

#[async_trait]
pub trait MessageExtension {
    fn is_bot_message(&self) -> bool;
    fn is_guild(&self) -> bool;
    async fn get_user_nick(&self, ctx: &Context) -> String;
}

#[async_trait]
impl MessageExtension for Message {
    fn is_bot_message(&self) -> bool {
        self.author.bot
    }

    fn is_guild(&self) -> bool {
        self.guild_id.is_some()
    }

    async fn get_user_nick(&self, ctx: &Context) -> String {
        self.author_nick(ctx)
            .await
            .unwrap_or_else(|| self.author.name.to_owned())
    }
}
