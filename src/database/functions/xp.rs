use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::{
    database::get_db,
    error::{AppErr, AppResult},
};

pub struct Xp {
    pub id: Uuid,
    pub level: i32,
    pub progress: i64,
    pub user_id: String,
    pub update_at: NaiveDateTime,
}

pub async fn add_xp(id: &str) -> AppResult<Option<Xp>> {
    let db = get_db();

    let res = {
        let mut tx = db.begin().await.map_err(AppErr::Database)?;

        let res = query_as!(
            Xp,
            r#"
            UPDATE
                "xp"
            SET
                progress = progress + 1,
                update_at = CURRENT_TIMESTAMP
            WHERE
                user_id = $1 AND
                update_at + '1 minutes' < CURRENT_TIMESTAMP
            RETURNING *
            "#,
            id
        )
        .fetch_optional(&mut tx)
        .await
        .map_err(AppErr::Database)?;

        tx.commit().await.map_err(AppErr::Database)?;

        res
    };

    Ok(res)
}
