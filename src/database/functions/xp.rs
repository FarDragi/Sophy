use chrono::Utc;
use sqlx::{Pool, Postgres};

use crate::error::{AppResult, MapDatabaseError};

pub struct XpLevel {
    pub progress: i64,
    pub level: i32,
    pub last_update: i64,
}

pub async fn add_xp(db: &Pool<Postgres>, user_id: u64, count: i64) -> AppResult<Option<XpLevel>> {
    let now = Utc::now().timestamp();

    let result = query_as!(
        XpLevel,
        r#"
        INSERT INTO xp
            (id, last_update)
        VALUES
            ($1, $3)
        ON CONFLICT
            (id)
        DO UPDATE
        SET
            progress = xp.progress + $2,
            last_update = $3
        WHERE
            xp.last_update < $3 - 300
        RETURNING
            progress, level, last_update
        "#,
        user_id.to_string(),
        count,
        now
    )
    .fetch_optional(db)
    .await
    .map_database_error()?;

    Ok(result)
}

pub async fn level_up(
    db: &Pool<Postgres>,
    user_id: u64,
    new_level: i32,
    new_progress: i64,
) -> AppResult<bool> {
    let result = query!(
        r#"
        UPDATE xp
        SET
            level = $1,
            progress = $2
        WHERE
            id = $3
        "#,
        new_level,
        new_progress,
        user_id.to_string()
    )
    .execute(db)
    .await
    .map_database_error()?;

    Ok(result.rows_affected() == 1)
}
