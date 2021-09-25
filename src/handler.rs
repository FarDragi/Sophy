use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::prelude::Ready,
};

pub struct DefaultHandler;

#[async_trait]
impl EventHandler for DefaultHandler {
    async fn ready(&self, _ctx: Context, data_about_bot: Ready) {
        println!("{} is Connected!", data_about_bot.user.tag());
    }
}
