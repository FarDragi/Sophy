use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::sync::Mutex;
use tonic::transport::Channel;

use crate::api::server::sophy::sophy_client::SophyClient;

pub type ShardsLatency = Arc<Mutex<HashMap<u64, Arc<Duration>>>>;

pub struct GrpcServices {
    pub sophy: Mutex<SophyClient<Channel>>,
}

pub struct States {
    pub shards_latency: ShardsLatency,
    pub grpc: GrpcServices,
}
