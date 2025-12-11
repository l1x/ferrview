// src/http/ui/views/home.rs

use askama::Template;

use crate::http::ui::{models::NodeSummary, templates::HomeTemplate};

pub fn render(nodes: &[NodeSummary]) -> String {
    let template = HomeTemplate {
        nodes,
        version: env!("CARGO_PKG_VERSION"),
    };

    template.render().unwrap_or_else(|e| {
        tracing::error!("Failed to render home template: {}", e);
        format!("Template error: {}", e)
    })
}
