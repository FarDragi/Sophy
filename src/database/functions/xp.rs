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
}

pub async fn add_xp(id: &str) -> AppResult<()> {
    let db = get_db();

    let res = {
        let mut tx = db.begin().await.map_err(AppErr::Database)?;

        query_as!(
            Xp,
            r#"UPDATE "xp" SET progress = progress + 1 WHERE user_id = $1 RETURNING *"#,
            id
        )
        .fetch_one(&mut tx)
        .await
        .map_err(AppErr::Database)?
    };

    Ok(())
}
