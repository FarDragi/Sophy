use bot::bootstrap_bot;
use config::bootstrap_config;
use logs::bootstrap_logs;

mod bot;
mod commands;
mod config;
mod constants;
mod error;
mod events;
mod logs;
mod states;
mod utils;

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
