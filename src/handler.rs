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
    async fn message(&self, _ctx: Context, new_message: Message) {
        let user = new_message.author;

        if !exists_user(user.id.to_string()).await.unwrap() {
            let result = create_user(&CreateUser {
                id: user.id.0.to_string(),
                name: user.tag(),
            })
            .await;

            if result.is_err() {
                error!("Fail create user: {}", user.id.0);
            }

            debug!("Create user: {}", user.id.0);
        }
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
