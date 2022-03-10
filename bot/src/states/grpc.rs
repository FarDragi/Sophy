use tokio::sync::Mutex;
use tonic::transport::Channel;

use crate::api::grpc::sophy::sophy_client::SophyClient;

pub struct GrpcServices {
    pub bot: Mutex<SophyClient<Channel>>,
}
