use dotenvy::dotenv;
use serde::Deserialize;
use std::env;

use crate::Result;

pub const SERVER_ADDRESS: &str = "SERVER_ADDRESS";
pub const TARGET_URL: &str = "TARGET_URL";

#[derive(Clone, Deserialize)]
pub struct Config {
    pub server_address: String,
    pub target_url: String,
}

impl Config {
    pub fn build() -> Result<Config> {
        dotenv().ok();

        let server_address: String = env::var(SERVER_ADDRESS).expect("SERVER_ADDRESS is not set");
        let target_url: String = env::var(TARGET_URL).expect("TARGET_URL is not set");

        Ok(Config {
            server_address,
            target_url,
        })
    }
}
