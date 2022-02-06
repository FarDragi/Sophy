use std::sync::Arc;

use chrono::Utc;
use poise::serenity_prelude::Message;

use crate::{
    database::functions::xp::add_xp,
    states::{level::LevelInfo, States},
};

pub async fn level_module_run(message: &Message, states: &States) {
    let user_id = &message.author.id;

    let level_info = {
        let infos_lock = states.levels.infos.lock().await;
        infos_lock.get(&user_id.0).cloned()
    };

    if let Some(level_info) = level_info {
        let now = Utc::now();

        if level_info.last_update < (now - states.levels.cooldown) {
            {
                let mut infos_lock = states.levels.infos.lock().await;
                infos_lock.insert(user_id.0, Arc::new(LevelInfo { last_update: now }));
            }

            let db = states.get_database();
            let result = add_xp(db, user_id.0, 1).await;
        }
    } else {
        let db = states.get_database();
        let result = add_xp(db, user_id.0, 1).await;
    }
}
