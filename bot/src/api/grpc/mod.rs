use tonic::transport::Channel;

use crate::error::{AppResult, MapError};

use self::sophy::sophy_client::SophyClient;

pub mod sophy {
    include_proto!("sophy");
}

pub async fn bootstrap_grpc_bot_client() -> AppResult<SophyClient<Channel>> {
    let client = SophyClient::connect("http://localhost:8020")
        .await
        .map_app_err()?;

    Ok(client)
}
