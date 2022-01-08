use crate::{
    database::get_db,
    error::{AppErr, AppResult},
};

pub async fn exists_guild(id: &str) -> AppResult<bool> {
    let db = get_db();

    let rec = query!(
        r#"SELECT EXISTS (SELECT 1 FROM "guild" WHERE id = $1) as "exists!""#,
        id
    )
    .fetch_one(db)
    .await
    .map_err(AppErr::Database)?;

    Ok(rec.exists)
}

pub struct CreateGuild {
    pub id: String,
    pub name: String,
}

pub async fn create_guild(guild: CreateGuild) -> AppResult<bool> {
    let db = get_db();

    query!(
        r#"INSERT INTO "guild" (id, name) VALUES ($1, $2)"#,
        &guild.id,
        guild.name
    )
    .execute(db)
    .await
    .map_err(AppErr::Database)?;

    Ok(true)
}
