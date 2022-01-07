use serenity::{client::Context, model::channel::Message};

use crate::database::functions::{
    user::{create_user, exists_user, CreateUser},
    xp::add_xp,
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
            Ok(_) => debug!("Add xp into user: {}", &user_id),
        }
    }
}
