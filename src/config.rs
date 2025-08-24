use std::env;
use std::sync::Once;
use dotenvy::dotenv;

pub struct Config {
    pub api_key: String,
    pub port: u16,
}

static mut CONFIG: Option<Config> = None;
static INIT: Once = Once::new();

pub fn get_config() -> &'static Config {
    INIT.call_once(|| {
        dotenv().ok();

        let config = Config {
            api_key: env::var("API_KEY").expect("API_KEY must be set"),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8000".to_string())
                .parse()
                .expect("PORT must be a number"),
        };

        unsafe { CONFIG = Some(config) };
    });

    unsafe { CONFIG.as_ref().expect("Config should be initialized") }
}
