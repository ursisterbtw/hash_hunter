use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub version: String,
    pub app: App,
    pub search: Search,
    pub performance: Performance,
    pub output: Output,
    pub docker: Docker,
    pub security: Security,
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
    pub min_zeros: usize, // Changed from u32 to usize
    pub verify_addresses: bool,
}

#[derive(Debug, Deserialize)]
pub struct Performance {
    pub step_size: u64,
    pub max_tries: u64,
    pub log_interval_ms: u64,
    pub threads: String,
    pub resources: Resources,
}

#[derive(Debug, Deserialize)]
pub struct Resources {
    pub cpu_limit: f32,
    pub memory_limit: String,
}

#[derive(Debug, Deserialize)]
pub struct Output {
    pub directory: String,
    pub files: Files,
    pub progress_bar: ProgressBar,
}

#[derive(Debug, Deserialize)]
pub struct Files {
    pub log: String,
    pub success_marker: String,
}

#[derive(Debug, Deserialize)]
pub struct ProgressBar {
    pub template: String,
    pub chars: String,
}

#[derive(Debug, Deserialize)]
pub struct Docker {
    pub base_image: String,
    pub builder_image: String, // Corrected field name from build_image to builder_image
    pub platforms: Vec<String>,
    pub healthcheck: Healthcheck,
    pub volumes: Vec<Volume>,
}

#[derive(Debug, Deserialize)]
pub struct Healthcheck {
    pub interval: String,
    pub timeout: String,
    pub retries: u32,
}

#[derive(Debug, Deserialize)]
pub struct Volume {
    pub source: String,
    pub target: String,
}

#[derive(Debug, Deserialize)]
pub struct Security {
    pub skip_confirmation: bool,
    pub entropy: Entropy,
}

#[derive(Debug, Deserialize)]
pub struct Entropy {
    pub guesses_per_second: f64,
    pub min_bits: u32,
}
