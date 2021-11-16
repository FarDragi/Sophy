use sea_orm::{ActiveModelTrait, Set};

use crate::{
    database::{get_database, schemas::users},
    error::{AppErr, AppResult},
};

pub type UserModel = users::ActiveModel;

pub struct UserInput {
    pub id: u64,
    pub name: String,
}

pub async fn register_user(input: UserInput) -> AppResult<UserModel> {
    let user = UserModel {
        id: Set(input.id as i64),
        name: Set(input.name),
    };

    let user = user
        .insert(get_database())
        .await
        .map_err(AppErr::Database)?;

    Ok(user)
}
