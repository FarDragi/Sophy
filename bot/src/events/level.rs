use poise::serenity_prelude::{Context, CreateEmbed, Message};
use tonic::{Code, Request};

use crate::{
    api::server::sophy::{GlobalXpRequest, GuildConfigRequest},
    constants::colors,
    error::{AppResult, MapError},
    format::ExtraArgs,
    states::States,
    utils::MessageExtension,
};

pub async fn level_module_run(ctx: &Context, message: &Message, states: &States) -> AppResult<()> {
    if message.is_bot_message() || !message.is_guild() {
        return Ok(());
    }

    let user_id = &message.author.id;

    let request = Request::new(GlobalXpRequest {
        discord_id: user_id.to_string(),
        token: "".to_owned(),
    });

    let result = states
        .grpc
        .sophy
        .lock()
        .await
        .add_user_global_xp(request)
        .await;

    match result {
        Ok(response) => {
            let level = response.into_inner();
            if level.level_up {
                send_level_up(ctx, message, level.new_level, states).await?;
            }
        }
        Err(status) => {
            if status.code() == Code::Aborted {
                debug!("{} is in cooldown", message.author.tag());
            }
        }
    }

    Ok(())
}

async fn send_level_up(
    ctx: &Context,
    message: &Message,
    new_level: i32,
    states: &States,
) -> AppResult<()> {
    let guild_config = states
        .grpc
        .sophy
        .lock()
        .await
        .get_guild_config(GuildConfigRequest {
            token: "".to_owned(),
            discord_id: message.guild_id.unwrap().to_string(),
        })
        .await
        .map_app_err()?
        .into_inner();

    let mut args = ExtraArgs::default();
    args.add_arg("level".to_owned(), new_level.to_string());

    let mut embed = CreateEmbed::default();
    embed
        .title(
            args.format_args(ctx, message, &guild_config.level_up_format)
                .await,
        )
        .color(colors::PURPLE);

    message
        .channel_id
        .send_message(ctx, |x| x.set_embed(embed))
        .await
        .map_app_err()?;

    Ok(())
}
