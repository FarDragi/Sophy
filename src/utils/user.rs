use poise::serenity_prelude::{Context, Message};

#[async_trait]
pub trait GetUserNick {
    async fn get_user_nick(&self, ctx: &Context) -> String;
}

#[async_trait]
impl GetUserNick for Message {
    async fn get_user_nick(&self, ctx: &Context) -> String {
        self.author_nick(ctx)
            .await
            .unwrap_or_else(|| self.author.name.to_owned())
    }
}
