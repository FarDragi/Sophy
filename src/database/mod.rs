use rbatis::rbatis::Rbatis;

use crate::config::Config;

lazy_static! {
    static ref DATABASE: Rbatis = Rbatis::new();
}

async fn init_database(config: &Config) {
    DATABASE
        .link(&config.database_connection)
        .await
        .expect("Fail create database connection");
}

pub async fn bootstrap_database(config: &Config) {
    init_database(config).await;
}

pub fn get_database() -> &'static Rbatis {
    &DATABASE
}
