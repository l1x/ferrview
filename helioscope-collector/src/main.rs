use argh::FromArgs;
use serde::Deserialize;
use std::fs;
use time::macros::format_description;
use tracing::{debug, info};
use tracing_subscriber::fmt::time::UtcTime;

fn default_config_file() -> String {
    String::from("helioscope-collector.toml")
}

#[derive(FromArgs, Debug)]
#[argh(description = "A brief description of what your program does.")]
#[argh(help_triggers("-h", "--help", "help"))]
pub struct Argz {
    /// config file location
    #[argh(option, default = "default_config_file()")]
    config_file: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {}

impl Config {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }
}

fn main() {
    // Initialize tracing
    //
    let timer = UtcTime::new(format_description!(
        "[year]-[month padding:zero]-[day padding:zero]T[hour padding:zero]:[minute padding:zero]:[second padding:zero]Z"
    ));

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with_timer(timer)
        .init();

    info!("Starting helioscope");

    let argz: Argz = argh::from_env();
    debug!("Args: {:?}", argz);
    info!("Config file is read from: {}", argz.config_file);

    let config = Config::load("helioscope.toml").expect("Failed to load helioscope.toml");

    debug!("Config: {:?}", config);
}
