use std::sync::Arc;

use dotenv::{dotenv, var};
use serenity::{model::id::GuildId, prelude::TypeMapKey};

pub struct Config {
    pub token: String,
    pub application_id: u64,
    pub owner_guild: GuildId,
    pub update_commands: bool,
    pub database_connection: String,
    pub migrate: bool,
}

impl Default for Config {
    fn default() -> Self {
        dotenv().ok();

        let token = var("SOPHY_TOKEN").expect("Token not found");

        let application_id = {
            var("SOPHY_APPLICATION_ID")
                .expect("Application id not found")
                .parse::<u64>()
                .expect("Fail convert application id")
        };

        let owner_guild = {
            GuildId(
                var("SOPHY_OWNER_GUILD")
                    .expect("Fail get owner server id")
                    .parse::<u64>()
                    .expect("Fail convert owner server id"),
            )
        };

        let database_connection = var("DATABASE_URL").expect("Database connection not found");

        Self {
            token,
            application_id,
            owner_guild,
            update_commands: false,
            database_connection,
            migrate: false,
        }
    }
}

pub struct ConfigKey;

impl TypeMapKey for ConfigKey {
    type Value = Arc<Config>;
}
