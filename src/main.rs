use bot::bootstrap_bot;
use config::bootstrap_config;
use logs::bootstrap_logs;

mod bot;
pub mod commands;
pub mod config;
pub mod error;
pub mod logs;

#[macro_use]
extern crate poise;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    bootstrap_logs();
    let config = bootstrap_config();
    bootstrap_bot(&config).await;
}
