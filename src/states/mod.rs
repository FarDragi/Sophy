use std::sync::Arc;

use sqlx::{Pool, Postgres};

use self::{level::Levels, shard::ShardsLatency};

pub mod level;
pub mod shard;

pub struct States {
    pub shards_latency: ShardsLatency,
    pub levels: Arc<Levels>,
    pub database: Pool<Postgres>,
}

impl States {
    pub fn get_database(&self) -> &Pool<Postgres> {
        &self.database
    }
}
