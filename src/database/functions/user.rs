use derive_builder::Builder;
use uuid::Uuid;

use crate::{
    database::get_db,
    error::{AppErr, AppResult},
};

#[derive(Builder)]
pub struct User {
    pub id: String,
    pub name: String,
    #[builder(setter(skip, strip_option))]
    pub xp_id: Option<Uuid>,
}

pub async fn exists_user(id: &str) -> AppResult<bool> {
    let db = get_db();

    let rec = query!(
        r#"SELECT EXISTS (SELECT 1 FROM "User" WHERE id = $1) as "exists!""#,
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

pub async fn create_user(user: CreateUser) -> AppResult<User> {
    let db = get_db();

    let rec_user = query!(
        r#"INSERT INTO "User" (id, name) VALUES ($1, $2) RETURNING id, name"#,
        user.id,
        user.name
    )
    .fetch_one(db)
    .await
    .map_err(AppErr::Database)?;

    let xp_id = Uuid::new_v4();

    query!(
        r#"INSERT INTO "Xp" (id, "userId") VALUES ($1, $2)"#,
        xp_id,
        rec_user.id
    )
    .execute(db)
    .await
    .map_err(AppErr::Database)?;

    Ok(UserBuilder::default()
        .id(rec_user.id)
        .name(rec_user.name)
        .build()
        .map_err(|err| AppErr::Builder(err.to_string()))?)
}
