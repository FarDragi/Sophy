use poise::serenity_prelude::Message;

use crate::{
    database::functions::xp::{add_xp, level_up},
    error::AppResult,
    states::States,
    utils::message::IsBotMessage,
};

const LEVELS: [i64; 200] = get_levels();

pub async fn level_module_run(message: &Message, states: &States) -> AppResult<()> {
    if message.is_bot_message() {
        return Ok(());
    }

    let user_id = &message.author.id;

    let db = states.get_database();
    let result = add_xp(db, user_id.0, 1).await?;

    if let Some(xp) = result {
        if let Some((new_level, new_progress)) = is_level_up(xp.level as usize, xp.progress) {
            if level_up(db, user_id.0, new_level, new_progress).await? {
                // enviar mensagem de level up
            }
        }
    }

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
