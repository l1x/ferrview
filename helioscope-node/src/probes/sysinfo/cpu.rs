use helioscope_common::ProbeDataPoint;
use sysinfo::System;
use tracing::info;

use crate::utils::timestamp::get_utc_timestamp;

pub fn probe_cpu(sys: &System, node_id: &str) -> Vec<ProbeDataPoint> {
    info!("Starting CPU probe");

    let core_count = sys.cpus().len();
    info!("Detected {} CPU cores", core_count);

    let mut data_points = Vec::new();
    let timestamp = get_utc_timestamp();

    // Adding core count
    data_points.push(ProbeDataPoint {
        node_id: node_id.to_string(),
        timestamp: timestamp.clone(),
        probe_type: "sysinfo".to_string(),
        probe_name: "cpu_core_count".to_string(),
        probe_value: core_count.to_string(),
    });

    // Add per-core metrics
    for (idx, cpu) in sys.cpus().iter().enumerate() {
        // Frequency
        data_points.push(ProbeDataPoint {
            node_id: node_id.to_string(),
            timestamp: timestamp.clone(),
            probe_type: "sysinfo".to_string(),
            probe_name: format!("cpu_core_{}_frequency_mhz", idx),
            probe_value: cpu.frequency().to_string(),
        });

        data_points.push(ProbeDataPoint {
            node_id: node_id.to_string(),
            timestamp: timestamp.clone(),
            probe_type: "sysinfo".to_string(),
            probe_name: format!("cpu_core_{}_usage_percent", idx),
            probe_value: cpu.cpu_usage().to_string(),
        });
    }

    data_points
}
