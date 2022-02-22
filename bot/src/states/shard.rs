use std::{collections::HashMap, sync::Arc, time::Duration};

use tokio::sync::Mutex;

pub type ShardsLatency = Arc<Mutex<HashMap<u64, Arc<Duration>>>>;
