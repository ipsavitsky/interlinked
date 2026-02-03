use anyhow::Result;
use dotenvy::dotenv;
use std::{env, path::PathBuf};
use tracing::Level;
use url::Url;

#[derive(Clone)]
pub struct Config {
    pub address: String,
    pub url: Url,
    pub store_dir: PathBuf,
    pub log_level: Level,
    pub difficulty: u32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            address: "127.0.0.1:8080".to_string(),
            url: Url::parse("http://localhost:8080").unwrap(),
            store_dir: PathBuf::from("./"),
            log_level: Level::INFO,
            difficulty: 1,
        }
    }
}

impl Config {
    pub fn parse() -> Result<Config> {
        dotenv().ok();
        let mut config = Config::default();

        if let Ok(address) = env::var("INTERLINKED_ADDRESS") {
            config.address = address;
        }

        if let Ok(url) = env::var("INTERLINKED_URL") {
            config.url = Url::parse(&url).expect("Failed to parse URL");
        }

        if let Ok(store_dir) = env::var("INTERLINKED_STORE_DIR") {
            config.store_dir = PathBuf::from(store_dir);
        }

        if let Ok(log_level) = env::var("INTERLINKED_LOG_LEVEL") {
            config.log_level = log_level.parse().expect("Failed to parse log level");
        }

        if let Ok(difficulty) = env::var("INTERLINKED_DIFFICULTY") {
            config.difficulty = difficulty.parse().expect("Failed to parse difficulty");
        }

        Ok(config)
    }
}
