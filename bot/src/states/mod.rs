use self::{grpc::GrpcServices, shard::ShardsLatency};

pub mod grpc;
pub mod shard;

pub struct States {
    pub shards_latency: ShardsLatency,
    pub grpc: GrpcServices,
}
