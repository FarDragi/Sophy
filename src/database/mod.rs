pub mod entity;
pub mod functions;

use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tokio::sync::OnceCell;

use crate::states::config::Config;

static DATABASE: OnceCell<DatabaseConnection> = OnceCell::const_new();

async fn init_database(config: &Config) {
    DATABASE
        .get_or_init(|| async {
            let mut opt = ConnectOptions::new(config.database_connection.to_string());
            opt.min_connections(5)
                .max_connections(15)
                .connect_timeout(Duration::from_secs(8))
                .idle_timeout(Duration::from_secs(8));

            Database::connect(opt)
                .await
                .expect("Fail create databse connection")
        })
        .await;

    info!("Database is connected!");
}

pub async fn bootstrap_database(config: &Config) {
    init_database(config).await;
}

pub fn get_database() -> &'static DatabaseConnection {
    DATABASE.get().unwrap()
}
