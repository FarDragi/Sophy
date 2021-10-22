mod cli;
mod commands;
mod config;
mod database;
mod handler;
mod logs;
pub mod states;
pub mod utils;

use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use clap::Clap;
use cli::Args;
use commands::config_commands;
use config::{Config, ConfigKey};
use database::bootstrap_database;
use handler::DefaultHandler;
use logs::bootstrap_logger;
use serenity::Client;
use states::shards::{Shards, ShardsKey};
use tokio::time::sleep;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serenity;

#[tokio::main]
async fn main() {
    bootstrap_logger();

    let mut config = Config::default();
    let args = Args::parse();
    args.aplay_configs(&mut config);

    bootstrap_database(&config).await;

    let mut client = Client::builder(&config.token)
        .event_handler(DefaultHandler)
        .application_id(config.application_id)
        .await
        .expect("Fail create client");

    let config = Arc::new(config);

    {
        let mut data = client.data.write().await;
        data.insert::<ConfigKey>(config.clone());
    }

    {
        config_commands(&config, &client.cache_and_http.http).await;
    }

    {
        let shard_manager = client.shard_manager.clone();

        let shards = Arc::new(Mutex::new(Shards::default()));

        {
            let mut data = client.data.write().await;
            data.insert::<ShardsKey>(shards.clone());
        }

        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(30)).await;

                {
                    let shard_manager_lock = shard_manager.lock().await;
                    let runners = shard_manager_lock.runners.lock().await;

                    let mut shards_lock = shards.lock().unwrap();

                    for (id, runner) in runners.iter() {
                        debug!(
                            "Shard [{}] is {} with a latency of {:?}",
                            id, runner.stage, runner.latency
                        );

                        shards_lock.runners.insert(id.0, runner.latency);
                    }
                }
            }
        });
    }

    if let Err(err) = client.start_autosharded().await {
        error!("{:?}", &err);
    }
}
