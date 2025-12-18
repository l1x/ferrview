// src/http/handlers/charts.rs

use hyper::StatusCode;
use tracing::{debug, error};

use crate::charts::{ChartData, SvgRenderer, TimeSeries, TimeSeriesChart};
use crate::http::response::{self, BoxBody};
use crate::http::ui::helpers;
use crate::store::date_range::DateRange;
use crate::store::date_range_reader::DateRangeReader;

pub async fn handle_cpu_chart(
    node_id: &str,
    range: &DateRange,
    reader: &DateRangeReader,
) -> (StatusCode, BoxBody) {
    debug!(
        "Generating CPU chart for node {} ({} to {})",
        node_id, range.start_date, range.end_date
    );

    let metrics = match reader
        .query_node_metrics(node_id, "cpu_core_%_usage_percent", range)
        .await
    {
        Ok(m) => m,
        Err(e) => {
            error!("Failed to query CPU metrics: {}", e);
            return response::svg_error("Query failed");
        }
    };

    if metrics.is_empty() {
        return response::svg_error("No CPU data available");
    }

    let series_map = helpers::group_metrics_by_index(&metrics, "cpu_core_");

    if series_map.is_empty() {
        return response::svg_error("No CPU data found");
    }

    let mut chart_data = ChartData::new(format!(
        "CPU Usage - Node {}",
        helpers::shorten_uuid(node_id)
    ))
    .with_labels("Time", "Usage (%)");

    for (name, points) in series_map {
        let mut series = TimeSeries::new(name).with_unit("%");
        for (timestamp, value) in points {
            series.add_point(timestamp, value);
        }
        chart_data.add_series(series);
    }

    render_chart(&chart_data)
}

pub async fn handle_memory_chart(
    node_id: &str,
    range: &DateRange,
    reader: &DateRangeReader,
) -> (StatusCode, BoxBody) {
    debug!(
        "Generating memory chart for node {} ({} to {})",
        node_id, range.start_date, range.end_date
    );

    let used_metrics = match reader
        .query_node_metrics(node_id, "memory_used_bytes", range)
        .await
    {
        Ok(m) => m,
        Err(e) => {
            error!("Failed to query used memory: {}", e);
            return response::svg_error("Query failed");
        }
    };

    let total_metrics = match reader
        .query_node_metrics(node_id, "memory_total_bytes", range)
        .await
    {
        Ok(m) => m,
        Err(e) => {
            error!("Failed to query total memory: {}", e);
            return response::svg_error("Query failed");
        }
    };

    if used_metrics.is_empty() {
        return response::svg_error("No memory data available");
    }

    let mut chart_data = ChartData::new(format!(
        "Memory Usage - Node {}",
        helpers::shorten_uuid(node_id)
    ))
    .with_labels("Time", "Memory (GB)");

    // Used memory series
    let mut used_series = TimeSeries::new("Used Memory").with_unit("GB");
    for metric in &used_metrics {
        if let (Ok(timestamp), Ok(value)) = (
            helpers::parse_timestamp(&metric.timestamp),
            metric.probe_value.parse::<f64>(),
        ) {
            used_series.add_point(timestamp, value / 1_073_741_824.0);
        }
    }
    chart_data.add_series(used_series);

    // Total memory series (if available)
    if !total_metrics.is_empty() {
        let mut total_series = TimeSeries::new("Total Memory").with_unit("GB");
        for metric in &total_metrics {
            if let (Ok(timestamp), Ok(value)) = (
                helpers::parse_timestamp(&metric.timestamp),
                metric.probe_value.parse::<f64>(),
            ) {
                total_series.add_point(timestamp, value / 1_073_741_824.0);
            }
        }
        chart_data.add_series(total_series);
    }

    render_chart(&chart_data)
}

pub async fn handle_temperature_chart(
    node_id: &str,
    range: &DateRange,
    reader: &DateRangeReader,
) -> (StatusCode, BoxBody) {
    debug!(
        "Generating temperature chart for node {} ({} to {})",
        node_id, range.start_date, range.end_date
    );

    let metrics = match reader
        .query_node_metrics(node_id, "temperature_sensor_%_celsius", range)
        .await
    {
        Ok(m) => m,
        Err(e) => {
            error!("Failed to query temperature metrics: {}", e);
            return response::svg_error("Query failed");
        }
    };

    // Filter to only include actual temperature readings, exclude max and critical thresholds
    let filtered_metrics: Vec<_> = metrics
        .into_iter()
        .filter(|m| {
            !m.probe_name.contains("_max_celsius") && !m.probe_name.contains("_critical_celsius")
        })
        .collect();

    if filtered_metrics.is_empty() {
        return response::svg_error("No temperature data available");
    }

    let series_map = helpers::group_metrics_by_index(&filtered_metrics, "temperature_sensor_");

    if series_map.is_empty() {
        return response::svg_error("No temperature data found");
    }

    let mut chart_data = ChartData::new(format!(
        "Temperature - Node {}",
        helpers::shorten_uuid(node_id)
    ))
    .with_labels("Time", "Temperature (°C)");

    for (name, points) in series_map {
        let mut series = TimeSeries::new(name).with_unit("°C");
        for (timestamp, value) in points {
            series.add_point(timestamp, value);
        }
        chart_data.add_series(series);
    }

    render_chart(&chart_data)
}

pub async fn handle_network_chart(
    node_id: &str,
    range: &DateRange,
    reader: &DateRangeReader,
) -> (StatusCode, BoxBody) {
    debug!(
        "Generating network chart for node {} ({} to {})",
        node_id, range.start_date, range.end_date
    );

    // Query received bytes
    let rx_metrics = match reader
        .query_node_metrics(node_id, "network_interface_%_total_received_bytes", range)
        .await
    {
        Ok(m) => m,
        Err(e) => {
            error!("Failed to query network RX metrics: {}", e);
            return response::svg_error("Query failed");
        }
    };

    // Query transmitted bytes
    let tx_metrics = match reader
        .query_node_metrics(node_id, "network_interface_%_total_transmitted_bytes", range)
        .await
    {
        Ok(m) => m,
        Err(e) => {
            error!("Failed to query network TX metrics: {}", e);
            return response::svg_error("Query failed");
        }
    };

    if rx_metrics.is_empty() && tx_metrics.is_empty() {
        return response::svg_error("No network data available");
    }

    let mut chart_data = ChartData::new(format!(
        "Network Traffic - Node {}",
        helpers::shorten_uuid(node_id)
    ))
    .with_labels("Time", "Traffic (GB)");

    // Group by interface index and create series
    let rx_map = helpers::group_metrics_by_index(&rx_metrics, "network_interface_");
    let tx_map = helpers::group_metrics_by_index(&tx_metrics, "network_interface_");

    // Add RX series (convert bytes to GB)
    for (name, points) in rx_map {
        let series_name = format!("{} RX", name.replace("Core", "eth"));
        let mut series = TimeSeries::new(series_name).with_unit("GB");
        for (timestamp, value) in points {
            series.add_point(timestamp, value / 1_073_741_824.0);
        }
        chart_data.add_series(series);
    }

    // Add TX series (convert bytes to GB)
    for (name, points) in tx_map {
        let series_name = format!("{} TX", name.replace("Core", "eth"));
        let mut series = TimeSeries::new(series_name).with_unit("GB");
        for (timestamp, value) in points {
            series.add_point(timestamp, value / 1_073_741_824.0);
        }
        chart_data.add_series(series);
    }

    render_chart(&chart_data)
}

pub async fn handle_disk_chart(
    node_id: &str,
    range: &DateRange,
    reader: &DateRangeReader,
) -> (StatusCode, BoxBody) {
    debug!(
        "Generating disk chart for node {} ({} to {})",
        node_id, range.start_date, range.end_date
    );

    let metrics = match reader
        .query_node_metrics(node_id, "disk_%_usage_percent", range)
        .await
    {
        Ok(m) => m,
        Err(e) => {
            error!("Failed to query disk metrics: {}", e);
            return response::svg_error("Query failed");
        }
    };

    if metrics.is_empty() {
        return response::svg_error("No disk data available");
    }

    let series_map = helpers::group_metrics_by_index(&metrics, "disk_");

    if series_map.is_empty() {
        return response::svg_error("No disk data found");
    }

    let mut chart_data = ChartData::new(format!(
        "Disk Usage - Node {}",
        helpers::shorten_uuid(node_id)
    ))
    .with_labels("Time", "Usage (%)");

    for (name, points) in series_map {
        let series_name = name.replace("Core", "Disk");
        let mut series = TimeSeries::new(series_name).with_unit("%");
        for (timestamp, value) in points {
            series.add_point(timestamp, value);
        }
        chart_data.add_series(series);
    }

    render_chart(&chart_data)
}

pub async fn handle_forks_chart(
    node_id: &str,
    range: &DateRange,
    reader: &DateRangeReader,
) -> (StatusCode, BoxBody) {
    debug!(
        "Generating forks chart for node {} ({} to {})",
        node_id, range.start_date, range.end_date
    );

    let metrics = match reader
        .query_node_metrics(node_id, "forks_total", range)
        .await
    {
        Ok(m) => m,
        Err(e) => {
            error!("Failed to query forks metrics: {}", e);
            return response::svg_error("Query failed");
        }
    };

    if metrics.is_empty() {
        return response::svg_error("No forks data available");
    }

    let mut chart_data = ChartData::new(format!(
        "Process Forks - Node {}",
        helpers::shorten_uuid(node_id)
    ))
    .with_labels("Time", "Total Forks");

    let mut series = TimeSeries::new("Forks (cumulative)").with_unit("");
    for metric in &metrics {
        if let (Ok(timestamp), Ok(value)) = (
            helpers::parse_timestamp(&metric.timestamp),
            metric.probe_value.parse::<f64>(),
        ) {
            series.add_point(timestamp, value);
        }
    }
    chart_data.add_series(series);

    render_chart(&chart_data)
}

fn render_chart(chart_data: &ChartData) -> (StatusCode, BoxBody) {
    let config = TimeSeriesChart::new(1200, 500);
    let renderer = SvgRenderer::new(config);

    match renderer.render_to_string(chart_data) {
        Ok(svg) => response::svg(&svg),
        Err(e) => {
            error!("Failed to render chart: {}", e);
            response::svg_error("Render failed")
        }
    }
}
