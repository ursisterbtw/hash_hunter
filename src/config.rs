use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub app: App,
    pub search: Search,
    pub performance: Performance,
    pub security: Security,
}

impl Config {
    pub fn load() -> Self {
        let config_content =
            fs::read_to_string("src/config.yml").expect("Failed to read config file");
        serde_yaml::from_str(&config_content).expect("Failed to parse config")
    }
}

#[derive(Debug, Deserialize)]
pub struct App {
    pub name: String,
    pub version: String,
    pub description: String,
    pub warning: String,
}

#[derive(Debug, Deserialize)]
pub struct Search {
    pub patterns: Patterns,
    pub validation: Validation,
}

#[derive(Debug, Deserialize)]
pub struct Patterns {
    pub start: String,
    pub end: String,
    pub regex: String,
}

#[derive(Debug, Deserialize)]
pub struct Validation {
    pub use_checksum: bool,
    pub min_zeros: usize,
    pub verify_addresses: bool,
}

#[derive(Debug, Deserialize)]
pub struct Performance {
    pub step_size: u64,
    pub max_tries: u64,
    pub log_interval_ms: u64,
    pub threads: String,
}

#[derive(Debug, Deserialize)]
pub struct Security {
    pub skip_confirmation: bool,
}
