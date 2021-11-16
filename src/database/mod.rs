pub mod functions;
pub mod schemas;

use std::{process::exit, str::FromStr, time::Duration};

use refinery::config::Config as RefineryConfig;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tokio::{sync::OnceCell, task};

use crate::states::config::Config;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./src/database/scripts");
}

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

async fn migration_database(config: &Config) {
    let mut config =
        RefineryConfig::from_str(&config.database_connection).expect("Fail convert database url");

    let result = task::spawn_blocking(move || embedded::migrations::runner().run(&mut config))
        .await
        .expect("Fail run refinery");

    match result {
        Ok(report) => {
            let migrates = report.applied_migrations();
            if migrates.is_empty() {
                warn!("No migrate to effectuate");
            } else {
                for migration in migrates {
                    debug!(
                        "Aplay {}{}__{}",
                        migration.prefix(),
                        migration.version(),
                        migration.name()
                    );
                }
            }
        }
        Err(err) => error!("{:?}", err),
    }

    exit(0)
}

pub async fn bootstrap_database(config: &Config) {
    if config.migrate {
        migration_database(config).await;
    }

    init_database(config).await;
}

pub fn get_database() -> &'static DatabaseConnection {
    DATABASE.get().unwrap()
}
