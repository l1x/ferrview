use argh::FromArgs;
use sysinfo::System;
use tracing::{debug, info};
use tracing_subscriber::{EnvFilter, fmt::time::UtcTime};

use crate::{
    config::Config,
    probes::sysinfo::{cpu, mem, temp},
    utils::timestamp::get_utc_formatter,
};

mod config;
mod probes;
mod utils;

fn default_config_file() -> String {
    String::from("helioscope-node.toml")
}

#[derive(FromArgs, Debug)]
#[argh(description = "Helioscope metrics collection node")]
#[argh(help_triggers("-h", "--help", "help"))]
pub struct Argz {
    /// config file location
    #[argh(option, default = "default_config_file()")]
    config_file: String,
}

fn main() {
    // Initialize tracing
    let timer = UtcTime::new(get_utc_formatter());
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_timer(timer)
        .init();

    info!("Starting helioscope");

    let argz: Argz = argh::from_env();
    debug!("Args: {:?}", argz);
    info!("Config file is read from: {}", argz.config_file);

    let config = Config::load(&argz.config_file).expect("Failed to load helioscope.toml");

    debug!("Config: {:?}", config);

    let mut sys = System::new_all();
    sys.refresh_all();

    if config.probes.sysinfo.cpu {
        let cpu_data = cpu::probe_cpu(&sys, &config.node_id);
        debug!("{:?}", cpu_data);
    }

    if config.probes.sysinfo.memory {
        let mem_data = mem::probe_memory(&sys, &config.node_id);
        debug!("{:?}", mem_data);
    }

    if config.probes.sysinfo.temperature {
        let temp_data = temp::probe_temperature(&config.node_id);
        debug!("{:?}", temp_data);
    }

    info!("Helioscope complete");
}
