use color_eyre::Result;
use dotenv::dotenv;
use eyre::WrapErr;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: i32,
    pub static_dir: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        dotenv().ok();
        let cfg = config::Config::builder()
            .add_source(config::Environment::default())
            .build()?;

        cfg.try_into()
            .context("loading configuration from environment")
    }
}

impl From<config::Config> for AppConfig {
    fn from(c: config::Config) -> Self {
        Self {
            host: c
                .get_string("HOST")
                .unwrap_or_else(|_| "localhost".to_string()),
            port: c.get::<i32>("PORT").unwrap_or_else(|_| 3000),
            static_dir: c
                .get_string("STATIC_DIR")
                .unwrap_or_else(|_| "calculator-ui/dist".to_string()),
        }
    }
}
