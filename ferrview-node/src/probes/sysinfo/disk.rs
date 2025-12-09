use ferrview_common::ProbeDataPoint;
use sysinfo::Disks;
use tracing::info;

use crate::utils::timestamp::get_utc_timestamp;

pub fn probe_disks(disks: &mut Disks, node_id: &str) -> Vec<ProbeDataPoint> {
    info!("Starting disk probe");

    let mut data_points = Vec::new();
    let timestamp = get_utc_timestamp();

    // Refresh disk information
    disks.refresh(false);
    let disk_count = disks.len();
    info!("Detected {} disk(s)", disk_count);

    // Add disk count metric
    data_points.push(ProbeDataPoint {
        node_id: node_id.to_string(),
        timestamp: timestamp.clone(),
        probe_type: "sysinfo".to_string(),
        probe_name: "disk_count".to_string(),
        probe_value: disk_count.to_string(),
    });

    // Collect metrics for each disk
    for (idx, disk) in disks.iter().enumerate() {
        let disk_name = disk.name().to_string_lossy().into_owned();
        info!("Processing disk {}: {}", idx, disk_name);

        // Disk name
        data_points.push(ProbeDataPoint {
            node_id: node_id.to_string(),
            timestamp: timestamp.clone(),
            probe_type: "sysinfo".to_string(),
            probe_name: format!("disk_{}_name", idx),
            probe_value: disk_name,
        });

        // Total space
        let total_space = disk.total_space();
        data_points.push(ProbeDataPoint {
            node_id: node_id.to_string(),
            timestamp: timestamp.clone(),
            probe_type: "sysinfo".to_string(),
            probe_name: format!("disk_{}_total_bytes", idx),
            probe_value: total_space.to_string(),
        });

        // Available space
        let available_space = disk.available_space();
        data_points.push(ProbeDataPoint {
            node_id: node_id.to_string(),
            timestamp: timestamp.clone(),
            probe_type: "sysinfo".to_string(),
            probe_name: format!("disk_{}_available_bytes", idx),
            probe_value: available_space.to_string(),
        });

        // Calculate and add usage percentage
        if total_space > 0 {
            let usage_percent = ((total_space - available_space) as f64 / total_space as f64) * 100.0;
            data_points.push(ProbeDataPoint {
                node_id: node_id.to_string(),
                timestamp: timestamp.clone(),
                probe_type: "sysinfo".to_string(),
                probe_name: format!("disk_{}_usage_percent", idx),
                probe_value: format!("{:.2}", usage_percent),
            });
        }

        // Filesystem type
        let fs_type = disk.file_system().to_string_lossy().into_owned();
        data_points.push(ProbeDataPoint {
            node_id: node_id.to_string(),
            timestamp: timestamp.clone(),
            probe_type: "sysinfo".to_string(),
            probe_name: format!("disk_{}_filesystem_type", idx),
            probe_value: fs_type,
        });

        // Mount point
        let mount_point = disk.mount_point().to_string_lossy().into_owned();
        data_points.push(ProbeDataPoint {
            node_id: node_id.to_string(),
            timestamp: timestamp.clone(),
            probe_type: "sysinfo".to_string(),
            probe_name: format!("disk_{}_mount_point", idx),
            probe_value: mount_point,
        });
    }

    info!("Collected {} disk metrics", data_points.len());
    data_points
}