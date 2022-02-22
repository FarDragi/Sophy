use sqlx::{Pool, Postgres};

use self::shard::ShardsLatency;

pub mod shard;

pub struct States {
    pub shards_latency: ShardsLatency,
    pub database: Pool<Postgres>,
}

impl States {
    pub fn get_database(&self) -> &Pool<Postgres> {
        &self.database
    }
}
