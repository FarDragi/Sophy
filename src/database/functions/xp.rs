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

pub async fn add_xp(id: &str) -> AppResult<Option<(Option<i64>, Xp)>> {
    let db = get_db();

    let xp = {
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
            RETURNING
                id, level, progress, user_id, update_at
            "#,
            id
        )
        .fetch_optional(&mut tx)
        .await
        .map_err(AppErr::Database)?;

        tx.commit().await.map_err(AppErr::Database)?;

        res
    };

    match xp {
        Some(xp) => {
            let level = xp.level;
            let progress = xp.progress;

            let target_progress = ((((level / 10) + 1) * 25) * level) as i64;

            let level_up = progress >= target_progress;

            if level_up {
                Ok(Some((Some(target_progress), xp)))
            } else {
                Ok(Some((None, xp)))
            }
        }
        None => Ok(None),
    }
}

pub async fn user_level_up(progress: i64, xp: &Xp) -> AppResult<()> {
    let db = get_db();

    {
        let mut tx = db.begin().await.map_err(AppErr::Database)?;

        let new_level = xp.level + 1;
        let new_progress = xp.progress - progress;

        query!(
            r#"
            UPDATE "xp"
            SET
                level = $1,
                progress = $2
            WHERE
                id = $3"#,
            new_level,
            new_progress,
            xp.id
        )
        .execute(&mut tx)
        .await
        .map_err(AppErr::Database)?;

        tx.commit().await.map_err(AppErr::Database)?;
    }

    Ok(())
}
