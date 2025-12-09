// helioscope-collector/src/charts/mod.rs

/// Chart rendering module for generating SVG visualizations
/// of time-series metrics data.
pub mod renderer;
pub mod types;

pub use renderer::SvgRenderer;
pub use types::{ChartData, TimeSeries, TimeSeriesChart};
