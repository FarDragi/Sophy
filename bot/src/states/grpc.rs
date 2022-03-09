use tokio::sync::Mutex;
use tonic::transport::Channel;

use crate::api::core::bot_client::bot_client::BotClient;

pub struct GrpcServices {
    pub bot: Mutex<BotClient<Channel>>,
}
