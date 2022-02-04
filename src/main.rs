use bot::bootstrap_bot;
use config::bootstrap_config;

mod bot;
pub mod commands;
pub mod config;
pub mod error;

#[macro_use]
extern crate poise;

#[macro_use]
extern crate serde;

#[tokio::main]
async fn main() {
    let config = bootstrap_config();
    bootstrap_bot(&config).await;
}
