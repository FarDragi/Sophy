use sea_orm::{ActiveModelTrait, Set};

use crate::database::{get_database, models::users};

pub async fn register_user(id: u64) {
    let user = users::ActiveModel { id: Set(id as i64) };

    let result = user.insert(get_database()).await;
}
