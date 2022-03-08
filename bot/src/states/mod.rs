use self::shard::ShardsLatency;

pub mod shard;

pub struct States {
    pub shards_latency: ShardsLatency,
}
