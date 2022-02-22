use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::config::Config;

pub mod functions;

pub async fn bootstrap_database(config: &Config) -> Pool<Postgres> {
    PgPoolOptions::new()
        .min_connections(1)
        .max_connections(10)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .connect(&config.database_url)
        .await
        .expect("Fail start database")
}
