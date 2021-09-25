use config::BotConfig;
use dotenv::dotenv;
use handler::DefaultHandler;
use serenity::Client;

mod config;
mod handler;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let bot_config = BotConfig::default();

    let mut client = Client::builder(bot_config.token)
        .event_handler(DefaultHandler)
        .application_id(bot_config.application_id)
        .await
        .expect("Fail create client");

    if let Err(err) = client.start().await {
        println!("{:?}", &err);
    }
}
