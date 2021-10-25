mod models;

use std::{str::FromStr, thread};

use refinery::config::Config as RefineryConfig;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::sync::OnceCell;

use crate::states::config::Config;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./src/database/scripts");
}

static POOL: OnceCell<Pool<Postgres>> = OnceCell::const_new();

async fn init_database(config: &Config) {
    POOL.get_or_init(|| async {
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&config.database_connection)
            .await
            .expect("Fail create pool database connection")
    })
    .await;

    info!("Database is connected!");
}

async fn migration_database(config: &Config) {
    let mut config =
        RefineryConfig::from_str(&config.database_connection).expect("Fail convert database url");

    let result = thread::spawn(move || embedded::migrations::runner().run(&mut config))
        .join()
        .expect("Fail run refinery");

    match result {
        Ok(report) => {
            let migrates = report.applied_migrations();
            if migrates.is_empty() {
                warn!("No migrate to effectuate");
            } else {
                for migration in migrates {
                    debug!("{}", migration.name());
                }
            }
        }
        Err(err) => error!("{:?}", err),
    }
}

pub async fn bootstrap_database(config: &Config) {
    if config.migrate {
        migration_database(config).await;
    }

    init_database(config).await;
}

pub fn get_database() -> &'static Pool<Postgres> {
    POOL.get().unwrap()
}
