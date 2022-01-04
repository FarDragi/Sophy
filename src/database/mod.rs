pub mod functions;

use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::sync::OnceCell;

use crate::states::config::Config;

static POOL: OnceCell<Pool<Postgres>> = OnceCell::const_new();

async fn init_db(config: &Config) {
    POOL.get_or_init(|| async {
        PgPoolOptions::new()
            .min_connections(1)
            .max_connections(15)
            .connect_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .connect(&config.database_connection)
            .await
            .expect("Fail connect database pool")
    })
    .await;

    info!("Database is connected!");
}

pub async fn bootstrap_database(config: &Config) {
    init_db(config).await;
}

pub fn get_db() -> &'static Pool<Postgres> {
    POOL.get().unwrap()
}
