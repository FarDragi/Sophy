use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{channel::Message, guild::Guild, interactions::Interaction, prelude::Ready},
};

use crate::{
    commands::run_command,
    database::functions::guild::{create_guild, exists_guild, CreateGuild},
    modules::xp::xp_module_run,
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

    async fn guild_create(&self, ctx: Context, guild: Guild) {
        guild_handler(ctx, guild).await;
    }
}

async fn message_handler(ctx: Context, message: Message) {
    let user = &message.author;

    if user.bot {
        return;
    }

    xp_module_run(&ctx, &message).await;
}

async fn guild_handler(_ctx: Context, guild: Guild) {
    let guild_id = guild.id.to_string();

    if !exists_guild(&guild_id).await.unwrap_or(false) {
        let result = create_guild(CreateGuild {
            id: guild_id.to_owned(),
            name: guild.name,
        })
        .await;

        if let Err(err) = result {
            err.log();
            error!("Fail create guild: {}", &guild_id);
        } else {
            debug!("Create guild: {}", &guild_id);
        }
    }
}
