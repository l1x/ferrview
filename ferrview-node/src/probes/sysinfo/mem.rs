use ferrview_common::ProbeDataPoint;
use sysinfo::System;
use tracing::info;

use crate::utils::timestamp::get_utc_timestamp;

pub fn probe_memory(sys: &System, node_id: &str) -> Vec<ProbeDataPoint> {
    info!("Starting memory probe");

    let mut data_points = Vec::new();
    let timestamp = get_utc_timestamp();

    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let available_memory = sys.available_memory();
    let total_swap = sys.total_swap();
    let used_swap = sys.used_swap();

    // Adding total memory
    data_points.push(ProbeDataPoint {
        node_id: node_id.to_string(),
        timestamp: timestamp.clone(),
        probe_type: "sysinfo".to_string(),
        probe_name: "memory_total_bytes".to_string(),
        probe_value: total_memory.to_string(),
    });

    // Adding used memory
    data_points.push(ProbeDataPoint {
        node_id: node_id.to_string(),
        timestamp: timestamp.clone(),
        probe_type: "sysinfo".to_string(),
        probe_name: "memory_used_bytes".to_string(),
        probe_value: used_memory.to_string(),
    });

    // Adding available memory
    data_points.push(ProbeDataPoint {
        node_id: node_id.to_string(),
        timestamp: timestamp.clone(),
        probe_type: "sysinfo".to_string(),
        probe_name: "memory_available_bytes".to_string(),
        probe_value: available_memory.to_string(),
    });

    // Adding total swap
    data_points.push(ProbeDataPoint {
        node_id: node_id.to_string(),
        timestamp: timestamp.clone(),
        probe_type: "sysinfo".to_string(),
        probe_name: "swap_total_bytes".to_string(),
        probe_value: total_swap.to_string(),
    });

    // Adding used swap
    data_points.push(ProbeDataPoint {
        node_id: node_id.to_string(),
        timestamp: timestamp.clone(),
        probe_type: "sysinfo".to_string(),
        probe_name: "swap_used_bytes".to_string(),
        probe_value: used_swap.to_string(),
    });

    data_points
}
