mod models;

use std::{str::FromStr, thread};

use rbatis::rbatis::Rbatis;
use refinery::config::Config as RefineryConfig;

use crate::states::config::Config;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./src/database/scripts");
}

lazy_static! {
    static ref DATABASE: Rbatis = Rbatis::new();
}

async fn init_database(config: &Config) {
    DATABASE
        .link(&config.database_connection)
        .await
        .expect("Fail create database connection");
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

pub fn get_database() -> &'static Rbatis {
    &DATABASE
}
