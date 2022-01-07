use serenity::{client::Context, model::channel::Message};

use crate::database::functions::{
    user::{create_user, exists_user, CreateUser},
    xp::{add_xp, user_level_up},
};

pub async fn xp_module_run(_ctx: &Context, msg: &Message) {
    let user = &msg.author;
    let user_id = msg.author.id.to_string();

    if !exists_user(&user_id).await.unwrap_or(false) {
        let result = create_user(CreateUser {
            id: user_id.to_owned(),
            name: user.tag(),
        })
        .await;

        if let Err(err) = result {
            err.log();
            error!("Fail create user: {}", &user_id);
        } else {
            debug!("Create user: {}", &user_id);
        }
    } else {
        match add_xp(&user_id).await {
            Err(err) => {
                err.log();
            }
            Ok(Some((level_up, xp))) => {
                debug!("Add xp into user: {}", &user_id);

                if let Some(progress) = level_up {
                    if let Err(err) = user_level_up(progress, &xp).await {
                        err.log();
                    } else {
                        debug!("Level up: {}", &user_id);
                    }
                }
            }
            _ => {}
        }
    }
}
