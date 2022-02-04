use figment::{
    providers::{Env, Format, Toml},
    Figment,
};

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Config {
    pub token: String,
}

impl Config {
    pub fn figment() -> Figment {
        Figment::new()
            .merge(Env::prefixed("SOPHY_"))
            .merge(Toml::file("Sophy.toml").nested())
    }
}

pub fn bootstrap_config() -> Config {
    Config::figment().extract().expect("Fail get config")
}
