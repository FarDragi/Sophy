use self::shard::ShardsLatency;

pub mod shard;

#[derive(Default)]
pub struct States {
    pub shards_latency: ShardsLatency,
}
