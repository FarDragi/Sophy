use uuid::Uuid;

use crate::{
    database::get_db,
    error::{AppErr, AppResult},
};

#[derive(TypedBuilder)]
pub struct User {
    pub id: String,
    pub name: String,
    #[builder(default, setter(strip_option))]
    pub xp_id: Option<Uuid>,
}

pub async fn exists_user(id: &str) -> AppResult<bool> {
    let db = get_db();

    let rec = query!(
        r#"SELECT EXISTS (SELECT 1 FROM "user" WHERE id = $1) as "exists!""#,
        id
    )
    .fetch_one(db)
    .await
    .map_err(AppErr::Database)?;

    Ok(rec.exists)
}

pub struct CreateUser {
    pub id: String,
    pub name: String,
}

pub async fn create_user(user: CreateUser) -> AppResult<bool> {
    let db = get_db();

    query!(
        r#"INSERT INTO "user" (id, name) VALUES ($1, $2)"#,
        &user.id,
        user.name
    )
    .execute(db)
    .await
    .map_err(AppErr::Database)?;

    let xp_id = Uuid::new_v4();

    query!(
        r#"INSERT INTO "xp" (id, user_id) VALUES ($1, $2)"#,
        xp_id,
        &user.id
    )
    .execute(db)
    .await
    .map_err(AppErr::Database)?;

    Ok(true)
}
