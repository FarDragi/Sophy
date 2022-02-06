use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, Duration, Utc};
use tokio::sync::Mutex;

pub struct Levels {
    pub infos: Arc<Mutex<HashMap<u64, Arc<LevelInfo>>>>,
    pub cooldown: Duration,
}

impl Default for Levels {
    fn default() -> Self {
        Self {
            infos: Default::default(),
            cooldown: Duration::minutes(5),
        }
    }
}

pub struct LevelInfo {
    pub last_update: DateTime<Utc>,
}
