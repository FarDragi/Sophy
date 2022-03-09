use tonic::transport::Channel;

use crate::error::{AppResult, MapError};

use self::bot_client::bot_client::BotClient;

pub mod bot_client {
    include_proto!("bot");
}

pub async fn bootstrap_grpc_bot_client() -> AppResult<BotClient<Channel>> {
    let client = BotClient::connect("http://localhost:8020")
        .await
        .map_app_err()?;

    Ok(client)
}
