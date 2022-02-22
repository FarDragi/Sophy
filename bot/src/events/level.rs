use poise::serenity_prelude::{Context, CreateEmbed, Message};

use crate::{
    constants::colors,
    database::functions::xp::{add_xp, level_up},
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

    let db = states.get_database();
    let result = add_xp(db, user_id.0, 1).await?;

    if let Some(xp) = result {
        if let Some((new_level, new_progress)) = is_level_up(xp.level as usize, xp.progress) {
            if level_up(db, user_id.0, new_level, new_progress).await? {
                send_level_up(ctx, message, new_level).await?;
            }
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
    let progres_target = LEVELS[level];
    if progress >= progres_target {
        Some((level as i32 + 1, progress - progres_target))
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