// helioscope-collector/src/charts/types.rs

use std::fmt;

/// Represents a single data point in a time-series
#[derive(Debug, Clone)]
pub struct MetricPoint {
    /// Unix timestamp or index
    pub timestamp: i64,
    /// Metric value
    pub value: f64,
}

impl MetricPoint {
    pub fn new(timestamp: i64, value: f64) -> Self {
        Self { timestamp, value }
    }
}

/// A complete time-series for a single metric
#[derive(Debug, Clone)]
pub struct TimeSeries {
    /// Name of the metric (e.g., "cpu_usage", "memory_percent")
    pub name: String,
    /// Data points in chronological order
    pub points: Vec<MetricPoint>,
    /// Optional unit (e.g., "%", "MB", "°C")
    pub unit: Option<String>,
}

impl TimeSeries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            points: Vec::new(),
            unit: None,
        }
    }

    pub fn with_unit(mut self, unit: impl Into<String>) -> Self {
        self.unit = Some(unit.into());
        self
    }

    pub fn add_point(&mut self, timestamp: i64, value: f64) {
        self.points.push(MetricPoint::new(timestamp, value));
    }

    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }
}

/// Chart configuration and data container
#[derive(Debug, Clone)]
pub struct ChartData {
    /// Chart title
    pub title: String,
    /// Series to plot
    pub series: Vec<TimeSeries>,
    /// X-axis label
    pub x_label: String,
    /// Y-axis label
    pub y_label: String,
}

impl ChartData {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            series: Vec::new(),
            x_label: String::from("Time"),
            y_label: String::from("Value"),
        }
    }

    pub fn with_labels(mut self, x_label: impl Into<String>, y_label: impl Into<String>) -> Self {
        self.x_label = x_label.into();
        self.y_label = y_label.into();
        self
    }

    pub fn add_series(&mut self, series: TimeSeries) {
        self.series.push(series);
    }

    pub fn is_empty(&self) -> bool {
        self.series.is_empty() || self.series.iter().all(|s| s.is_empty())
    }
}

/// Chart rendering configuration
#[derive(Debug, Clone)]
pub struct TimeSeriesChart {
    /// Width in pixels
    pub width: u32,
    /// Height in pixels
    pub height: u32,
    /// Show grid lines
    pub show_grid: bool,
    /// Show legend
    pub show_legend: bool,
}

impl Default for TimeSeriesChart {
    fn default() -> Self {
        Self {
            width: 800,
            height: 400,
            show_grid: true,
            show_legend: true,
        }
    }
}

impl TimeSeriesChart {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            ..Default::default()
        }
    }
}

impl fmt::Display for MetricPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {:.2})", self.timestamp, self.value)
    }
}
