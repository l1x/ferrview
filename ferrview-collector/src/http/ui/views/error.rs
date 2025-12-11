// src/http/ui/views/error.rs

use askama::Template;

use crate::http::ui::templates::ErrorTemplate;

pub fn render(title: &str, message: &str) -> String {
    let template = ErrorTemplate {
        title,
        message,
        version: env!("CARGO_PKG_VERSION"),
    };

    template.render().unwrap_or_else(|e| {
        tracing::error!("Failed to render error template: {}", e);
        format!("Template error: {}", e)
    })
}
