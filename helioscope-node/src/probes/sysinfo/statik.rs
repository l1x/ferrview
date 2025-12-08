use helioscope_common::ProbeDataPoint;
use sysinfo::System;
use tracing::info;

use crate::utils::timestamp::get_utc_timestamp;

pub fn probe_static_info(node_id: &str) -> Vec<ProbeDataPoint> {
    info!("Collecting static system information");

    let timestamp = get_utc_timestamp();

    let data_points = vec![
        ProbeDataPoint {
            node_id: node_id.to_string(),
            timestamp: timestamp.clone(),
            probe_type: "sysinfo".to_string(),
            probe_name: "system_cpu_arch".to_string(),
            probe_value: System::cpu_arch(),
        },
        ProbeDataPoint {
            node_id: node_id.to_string(),
            timestamp: timestamp.clone(),
            probe_type: "sysinfo".to_string(),
            probe_name: "system_os_name".to_string(),
            probe_value: System::name().unwrap_or_default(),
        },
        ProbeDataPoint {
            node_id: node_id.to_string(),
            timestamp: timestamp.clone(),
            probe_type: "sysinfo".to_string(),
            probe_name: "system_kernel_version".to_string(),
            probe_value: System::kernel_version().unwrap_or_default(),
        },
        ProbeDataPoint {
            node_id: node_id.to_string(),
            timestamp: timestamp.clone(),
            probe_type: "sysinfo".to_string(),
            probe_name: "system_os_version".to_string(),
            probe_value: System::os_version().unwrap_or_default(),
        },
        ProbeDataPoint {
            node_id: node_id.to_string(),
            timestamp: timestamp.clone(),
            probe_type: "sysinfo".to_string(),
            probe_name: "system_hostname".to_string(),
            probe_value: System::host_name().unwrap_or_default(),
        },
    ];

    info!("Collected {} static system metrics", data_points.len());
    data_points
}
