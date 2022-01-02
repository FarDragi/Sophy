use sea_orm::{prelude::Uuid, ActiveModelTrait, EntityTrait, Set};

use crate::{
    database::{
        entity::{user, xp},
        get_database,
    },
    error::{AppErr, AppResult},
};

pub struct CreateUser {
    pub id: String,
    pub name: String,
}

pub async fn create_user(user: &CreateUser) -> AppResult<()> {
    let db = get_database();

    let user_model = user::ActiveModel {
        id: Set(user.id.to_owned()),
        name: Set(user.name.to_owned()),
    };

    user_model.insert(db).await.map_err(AppErr::Database)?;

    let xp_model = xp::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(Some(user.id.to_owned())),
        ..Default::default()
    };

    xp_model.insert(db).await.map_err(AppErr::Database)?;

    Ok(())
}

pub async fn exists_user(id: String) -> AppResult<bool> {
    let db = get_database();

    match user::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(AppErr::Database)?
    {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}
