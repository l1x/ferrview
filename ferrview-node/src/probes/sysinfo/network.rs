use ferrview_common::ProbeDataPoint;
use sysinfo::Networks;
use tracing::info;

use crate::utils::timestamp::get_utc_timestamp;

pub fn probe_networks(networks: &mut Networks, node_id: &str) -> Vec<ProbeDataPoint> {
    info!("Starting network probe");

    let mut data_points = Vec::new();
    let timestamp = get_utc_timestamp();

    // Refresh network information
    networks.refresh(true);
    let network_count = networks.len();
    info!("Detected {} network interface(s)", network_count);

    // Add network interface count metric
    data_points.push(ProbeDataPoint {
        node_id: node_id.to_string(),
        timestamp: timestamp.clone(),
        probe_type: "sysinfo".to_string(),
        probe_name: "network_interface_count".to_string(),
        probe_value: network_count.to_string(),
    });

    // Collect metrics for each network interface
    for (idx, (interface_name, data)) in networks.iter().enumerate() {
        let name = interface_name.to_string();
        info!("Processing network interface {}: {}", idx, name);

        // Interface name
        data_points.push(ProbeDataPoint {
            node_id: node_id.to_string(),
            timestamp: timestamp.clone(),
            probe_type: "sysinfo".to_string(),
            probe_name: format!("network_interface_{}_name", idx),
            probe_value: name,
        });

        // Total received bytes
        let received = data.total_received();
        data_points.push(ProbeDataPoint {
            node_id: node_id.to_string(),
            timestamp: timestamp.clone(),
            probe_type: "sysinfo".to_string(),
            probe_name: format!("network_interface_{}_total_received_bytes", idx),
            probe_value: received.to_string(),
        });

        // Total transmitted bytes
        let transmitted = data.total_transmitted();
        data_points.push(ProbeDataPoint {
            node_id: node_id.to_string(),
            timestamp: timestamp.clone(),
            probe_type: "sysinfo".to_string(),
            probe_name: format!("network_interface_{}_total_transmitted_bytes", idx),
            probe_value: transmitted.to_string(),
        });

        // Received packets
        let packets_received = data.packets_received();
        data_points.push(ProbeDataPoint {
            node_id: node_id.to_string(),
            timestamp: timestamp.clone(),
            probe_type: "sysinfo".to_string(),
            probe_name: format!("network_interface_{}_packets_received", idx),
            probe_value: packets_received.to_string(),
        });

        // Transmitted packets
        let packets_transmitted = data.packets_transmitted();
        data_points.push(ProbeDataPoint {
            node_id: node_id.to_string(),
            timestamp: timestamp.clone(),
            probe_type: "sysinfo".to_string(),
            probe_name: format!("network_interface_{}_packets_transmitted", idx),
            probe_value: packets_transmitted.to_string(),
        });

        // Errors on receive
        let errors_on_received = data.errors_on_received();
        data_points.push(ProbeDataPoint {
            node_id: node_id.to_string(),
            timestamp: timestamp.clone(),
            probe_type: "sysinfo".to_string(),
            probe_name: format!("network_interface_{}_errors_on_received", idx),
            probe_value: errors_on_received.to_string(),
        });

        // Errors on transmit
        let errors_on_transmitted = data.errors_on_transmitted();
        data_points.push(ProbeDataPoint {
            node_id: node_id.to_string(),
            timestamp: timestamp.clone(),
            probe_type: "sysinfo".to_string(),
            probe_name: format!("network_interface_{}_errors_on_transmitted", idx),
            probe_value: errors_on_transmitted.to_string(),
        });
    }

    info!("Collected {} network metrics", data_points.len());
    data_points
}
