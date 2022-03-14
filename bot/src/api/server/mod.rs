use tonic::transport::Channel;

use crate::{
    config::Config,
    error::{AppResult, MapError},
};

use self::sophy::sophy_client::SophyClient;

pub mod sophy {
    include_proto!("sophy");
}

pub async fn bootstrap_server_sophy_client(config: &Config) -> AppResult<SophyClient<Channel>> {
    let client = SophyClient::connect(config.grpc_url.clone())
        .await
        .map_app_err()?;

    Ok(client)
}
