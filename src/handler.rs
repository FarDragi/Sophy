use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{interactions::Interaction, prelude::Ready},
};

use crate::commands::run_command;

pub struct DefaultHandler;

#[async_trait]
impl EventHandler for DefaultHandler {
    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        info!(
            "{} is connected! Shard [{}]",
            data_about_bot.user.tag(),
            ctx.shard_id
        );
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command_interaction) = interaction {
            run_command(&ctx, &command_interaction).await.ok();
        }
    }
}
