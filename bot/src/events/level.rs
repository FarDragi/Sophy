use poise::serenity_prelude::{Context, CreateEmbed, Message};
use tonic::Request;

use crate::{
    api::grpc::sophy::GlobalXpRequest,
    constants::colors,
    error::{AppResult, MapError},
    states::States,
    utils::{message::IsBotMessage, user::GetUserNick},
};

const LEVELS: [i64; 200] = get_levels();

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

fn is_level_up(level: usize, progress: i64) -> Option<(i32, i64)> {
    let progress_target = LEVELS[level];
    if progress >= progress_target {
        Some((level as i32 + 1, progress - progress_target))
    } else {
        None
    }
}

const fn calc_level(level: i64) -> i64 {
    let progress_multiplier = ((level - 1) / 5 + 1) * 20;
    let level_multiplier = ((level - 1) % 5) + 1;
    let base = {
        let mut result = 0;
        let mut i = 0;
        let i_target = ((level - 1) / 5) + 1;

        while i < i_target {
            result += 100 * i;
            i += 1;
        }
        result
    };

    (progress_multiplier * level_multiplier) + base
}

const fn get_levels() -> [i64; 200] {
    let mut levels = [0; 200];

    let mut i = 0;
    while i < 200 {
        levels[i] = calc_level((i + 1) as i64);
        i += 1;
    }

    levels
}
