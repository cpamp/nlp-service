use std::env;
use std::sync::{OnceLock};
use dotenvy::dotenv;

pub struct Config {
    pub api_key: String,
    pub port: u16,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn get_config() -> &'static Config {
    CONFIG.get_or_init(|| {
        dotenv().ok();

        Config {
            api_key: env::var("API_KEY").expect("API_KEY must be set"),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8000".to_string())
                .parse()
                .expect("PORT must be a number"),
        }
    });

    CONFIG.get().expect("Config should be initialized")
}
