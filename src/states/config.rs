use std::sync::Arc;

use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};
use serenity::{model::id::GuildId, prelude::TypeMapKey};

#[derive(Deserialize, Default, Serialize, Debug)]
pub struct Config {
    pub token: String,
    pub application_id: u64,
    pub owner_guild: GuildId,
    pub update_commands: bool,
    pub database_connection: String,
    pub migrate: bool,
}

impl Config {
    pub fn figment() -> Self {
        Figment::from(Serialized::defaults(Self::default()))
            .merge(Env::prefixed("SOPHY_"))
            .merge(Toml::file("Sophy.toml").nested())
            .extract()
            .expect("Fail get config")
    }
}
pub struct ConfigKey;

impl TypeMapKey for ConfigKey {
    type Value = Arc<Config>;
}
