use dotenv::var;

pub struct BotConfig {
    pub token: String,
    pub application_id: u64,
}

impl BotConfig {
    pub fn default() -> Self {
        let token = var("SOPHY_TOKEN").expect("Token not found");
        let application_id = var("SOPHY_APPLICATION_ID").expect("Application id not found");

        let application_id = application_id
            .parse::<u64>()
            .expect("Fail convert application id");

        Self {
            token,
            application_id,
        }
    }
}
