use bot::bootstrap_bot;
use config::bootstrap_config;
use database::bootstrap_database;
use logs::bootstrap_logs;

mod bot;
pub mod commands;
pub mod config;
pub mod database;
pub mod error;
pub mod events;
pub mod logs;
pub mod states;
pub mod utils;

#[macro_use]
extern crate poise;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate log;

#[macro_use]
extern crate sqlx;

#[macro_use]
extern crate typed_builder;

#[tokio::main]
async fn main() {
    bootstrap_logs();
    let config = bootstrap_config();
    let db_pool = bootstrap_database(&config).await;
    bootstrap_bot(&config, db_pool).await;
}
