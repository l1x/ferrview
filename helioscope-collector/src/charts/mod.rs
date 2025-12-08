// helioscope-collector/src/charts/mod.rs

/// Chart rendering module for generating SVG visualizations
/// of time-series metrics data.
pub mod renderer;
pub mod types;

pub use renderer::{RenderError, SvgRenderer};
pub use types::{ChartData, MetricPoint, TimeSeries, TimeSeriesChart};
