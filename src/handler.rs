use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{channel::Message, interactions::Interaction, prelude::Ready},
};

use crate::{
    commands::run_command,
    database::functions::user::{create_user, exists_user, CreateUser},
};

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

async fn message_handler(_ctx: Context, message: Message) {
    let user = message.author;

    if user.bot {
        return;
    }

    if !exists_user(&user.id.to_string()).await.unwrap_or(false) {
        let result = create_user(CreateUser {
            id: user.id.0.to_string(),
            name: user.tag(),
        })
        .await;

        if let Err(err) = result {
            err.log();
            error!("Fail create user: {}", user.id.0);
        } else {
            debug!("Create user: {}", user.id.0);
        }
    }
}
