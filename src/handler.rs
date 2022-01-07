use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{channel::Message, interactions::Interaction, prelude::Ready},
};

use crate::{commands::run_command, modules::xp::xp_module_run};

pub struct DefaultHandler;

#[async_trait]
impl EventHandler for DefaultHandler {
    async fn message(&self, ctx: Context, new_message: Message) {
        message_handler(ctx, new_message).await;
    }

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

async fn message_handler(ctx: Context, message: Message) {
    let user = &message.author;

    if user.bot {
        return;
    }

    xp_module_run(&ctx, &message).await;
}
