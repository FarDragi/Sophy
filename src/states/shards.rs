use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use serenity::prelude::TypeMapKey;

#[derive(Default)]
pub struct Shards {
    pub runners: HashMap<u64, Option<Duration>>,
}

pub struct ShardsKey;

impl TypeMapKey for ShardsKey {
    type Value = Arc<Mutex<Shards>>;
}
