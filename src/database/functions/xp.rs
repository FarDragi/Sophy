use sqlx::{Pool, Postgres};

use crate::error::{AppError, MapDatabaseError};

pub async fn add_xp(db: &Pool<Postgres>, user_id: u64, count: i64) -> Result<bool, AppError> {
    let result = query!(
        r#"
        INSERT INTO xp
            (id)
        VALUES
            ($1)
        ON CONFLICT
            (id)
        DO UPDATE
        SET
            progress = xp.progress + $2
        RETURNING
            progress, level
        "#,
        user_id.to_string(),
        count
    )
    .fetch_one(db)
    .await
    .map_database_error()?;

    Ok(true)
}
