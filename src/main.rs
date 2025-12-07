use serde::Deserialize;
use std::fs;
use sysinfo::System;
use time::macros::format_description;
use tracing::{debug, info};
use tracing_subscriber;
use tracing_subscriber::fmt::time::UtcTime;

use crate::probes::sysinfo::{cpu, mem, temp};

mod probes;

#[derive(Debug, Deserialize)]
pub struct ProbesConfig {
    pub sysinfo: SysinfoProbes,
    // Future: other probe sources
    // pub something_else: SomethingElseProbes,
}

#[derive(Debug, Deserialize)]
pub struct SysinfoProbes {
    pub cpu: bool,
    pub memory: bool,
    pub disk: bool,
    pub network: bool,
    pub temperature: bool,
    pub static_info: bool,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub probes: ProbesConfig,
}

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
        .with_max_level(tracing::Level::INFO)
        .with_timer(timer)
        .init();

    info!("Starting helioscope");

    let config = Config::load("helioscope.toml").expect("Failed to load helioscope.toml");

    debug!("{:?}", config);

    let mut sys = System::new_all();
    sys.refresh_all();

    if config.probes.sysinfo.cpu {
        cpu::probe_cpu(&sys);
    }

    if config.probes.sysinfo.memory {
        mem::probe_memory(&sys);
    }

    if config.probes.sysinfo.temperature {
        temp::probe_temperature();
    }

    info!("Helioscope complete");
}

// use sysinfo::{Components, CpuRefreshKind, Disks, Networks, RefreshKind, System};

// fn main() {
//     println!("=> system static:");

//     // Display system information:
//     println!("System arch:             {:?}", System::cpu_arch());
//     println!("System name:             {:?}", System::name());
//     println!("System kernel version:   {:?}", System::kernel_version());
//     println!("System OS version:       {:?}", System::os_version());
//     println!("System host name:        {:?}", System::host_name());

//     let mut sys = System::new_all();
//     sys.refresh_all();
//     println!("=> system dynamic:");

//     // let s =
//     //     System::new_with_specifics(RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()));

//     for cpu in sys.cpus() {
//         println!("Core {} : {} MHz", cpu.name(), cpu.frequency(),);
//     }

//     // RAM and swap information:
//     println!("total memory: {} bytes", sys.total_memory());
//     println!("used memory : {} bytes", sys.used_memory());
//     println!("total swap  : {} bytes", sys.total_swap());
//     println!("used swap   : {} bytes", sys.used_swap());

//     // Number of CPUs:
//     println!("NB CPUs: {}", sys.cpus().len());

//     // We display all disks' information:
//     println!("=> disks:");
//     let disks = Disks::new_with_refreshed_list();
//     for disk in &disks {
//         println!("{disk:?}");
//     }

//     // Network interfaces name, total data received and total data transmitted:
//     let networks = Networks::new_with_refreshed_list();
//     println!("=> networks:");
//     for (interface_name, data) in &networks {
//         println!(
//             "{interface_name}: {} B (down) / {} B (up)",
//             data.total_received(),
//             data.total_transmitted(),
//         );
//     }

//     // Components temperature:
//     let components = Components::new_with_refreshed_list();
//     println!("=> components:");
//     for component in &components {
//         println!("{component:?}");
//     }
// }
