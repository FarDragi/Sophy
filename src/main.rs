mod commands;
mod config;
mod database;
mod handler;

use std::{env::args, sync::Arc};

use commands::config_commands;
use config::{Config, ConfigKey};
use database::bootstrap_database;
use dotenv::dotenv;
use handler::DefaultHandler;
use serenity::Client;

#[macro_use]
extern crate serenity;

#[macro_use]
extern crate lazy_static;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut config = Config::default();

    for arg in args() {
        if arg.as_str() == "update-commands" {
            println!("Update global commands");
            config.update_commands();
        }
    }

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

    if let Err(err) = client.start().await {
        println!("{:?}", &err);
    }
}
