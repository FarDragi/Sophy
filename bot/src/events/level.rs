use poise::serenity_prelude::{Context, CreateEmbed, Message};
use tonic::Request;

use crate::{
    api::grpc::sophy::GlobalXpRequest,
    constants::colors,
    error::{AppResult, MapError},
    states::States,
    utils::{message::IsBotMessage, user::GetUserNick},
};

pub async fn level_module_run(ctx: &Context, message: &Message, states: &States) -> AppResult<()> {
    if message.is_bot_message() {
        return Ok(());
    }

    let user_id = &message.author.id;

    let request = Request::new(GlobalXpRequest {
        discord_id: user_id.to_string(),
        token: "".to_owned(),
    });

    let result = states
        .grpc
        .bot
        .lock()
        .await
        .add_user_global_xp(request)
        .await;

    if let Ok(response) = result {
        let level = response.into_inner();
        if level.level_up {
            send_level_up(ctx, message, level.new_level).await?;
        }
    }

    Ok(())
}

async fn send_level_up(ctx: &Context, message: &Message, new_level: i32) -> AppResult<()> {
    let mut embed = CreateEmbed::default();
    embed
        .title(format!(
            "{} level up to {}!",
            message.get_user_nick(ctx).await,
            new_level
        ))
        .color(colors::PURPLE);

    message
        .channel_id
        .send_message(ctx, |x| x.set_embed(embed))
        .await
        .map_app_err()?;

    Ok(())
}
