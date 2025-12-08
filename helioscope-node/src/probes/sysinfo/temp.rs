use helioscope_common::ProbeDataPoint;
use sysinfo::Components;
use tracing::info;

use crate::utils::timestamp::get_utc_timestamp;

pub fn probe_temperature(node_id: &str) -> Vec<ProbeDataPoint> {
    info!("Starting temperature probe");

    let mut data_points = Vec::new();
    let timestamp = get_utc_timestamp();

    let components = Components::new_with_refreshed_list();
    let component_count = components.len();
    info!("Detected {} temperature sensors", component_count);

    // Add sensor count metric
    data_points.push(ProbeDataPoint {
        node_id: node_id.to_string(),
        timestamp: timestamp.clone(),
        probe_type: "sysinfo".to_string(),
        probe_name: "temperature_sensor_count".to_string(),
        probe_value: component_count.to_string(),
    });

    if component_count == 0 {
        info!("No temperature sensors available");
        return data_points;
    }

    for (idx, component) in components.iter().enumerate() {
        let label = component.label();
        let temperature = component.temperature();
        let max_temp = component.max();
        let critical_temp = component.critical();

        // Add  temperature
        if let Some(temp) = temperature {
            data_points.push(ProbeDataPoint {
                node_id: node_id.to_string(),
                timestamp: timestamp.clone(),
                probe_type: "sysinfo".to_string(),
                probe_name: format!("temperature_sensor_{}_celsius", idx),
                probe_value: temp.to_string(),
            });
        }

        // Add sensor label as a separate metric for identification
        data_points.push(ProbeDataPoint {
            node_id: node_id.to_string(),
            timestamp: timestamp.clone(),
            probe_type: "sysinfo".to_string(),
            probe_name: format!("temperature_sensor_{}_label", idx),
            probe_value: label.to_string(),
        });

        // Add max temperature if available
        if let Some(max) = max_temp {
            data_points.push(ProbeDataPoint {
                node_id: node_id.to_string(),
                timestamp: timestamp.clone(),
                probe_type: "sysinfo".to_string(),
                probe_name: format!("temperature_sensor_{}_max_celsius", idx),
                probe_value: max.to_string(),
            });
        }

        // Add critical temperature if available
        if let Some(critical) = critical_temp {
            data_points.push(ProbeDataPoint {
                node_id: node_id.to_string(),
                timestamp: timestamp.clone(),
                probe_type: "sysinfo".to_string(),
                probe_name: format!("temperature_sensor_{}_critical_celsius", idx),
                probe_value: critical.to_string(),
            });
        }
    }

    data_points
}
