// helioscope-collector/src/http/ui.rs

use bytes::Bytes;
use http_body_util::{BodyExt, Full, combinators::BoxBody};
use hyper::{Response, StatusCode};

/// Generate the HTML homepage for the UI
pub async fn handle_ui_home() -> Response<BoxBody<Bytes, hyper::Error>> {
    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Helioscope Metrics Collector</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        body {
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
            line-height: 1.6;
            color: #333;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            padding: 20px;
        }

        .container {
            max-width: 800px;
            width: 100%;
            background: white;
            border-radius: 12px;
            box-shadow: 0 10px 40px rgba(0, 0, 0, 0.1);
            padding: 40px;
        }

        h1 {
            color: #667eea;
            margin-bottom: 10px;
            font-size: 2.5em;
        }

        .subtitle {
            color: #666;
            margin-bottom: 30px;
            font-size: 1.1em;
        }

        .status {
            background: #f0f9ff;
            border-left: 4px solid #0ea5e9;
            padding: 15px 20px;
            margin-bottom: 30px;
            border-radius: 4px;
        }

        .status-title {
            font-weight: 600;
            color: #0ea5e9;
            margin-bottom: 5px;
        }

        .features {
            margin: 30px 0;
        }

        .feature-list {
            list-style: none;
            display: grid;
            gap: 15px;
        }

        .feature-item {
            padding: 15px;
            background: #f8fafc;
            border-radius: 8px;
            border: 1px solid #e2e8f0;
            transition: transform 0.2s, box-shadow 0.2s;
        }

        .feature-item:hover {
            transform: translateY(-2px);
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
        }

        .feature-title {
            font-weight: 600;
            color: #1e293b;
            margin-bottom: 5px;
        }

        .feature-desc {
            color: #64748b;
            font-size: 0.9em;
        }

        .endpoints {
            margin-top: 30px;
            padding: 20px;
            background: #fafafa;
            border-radius: 8px;
        }

        .endpoints h2 {
            color: #333;
            margin-bottom: 15px;
            font-size: 1.3em;
        }

        .endpoint {
            font-family: "Courier New", monospace;
            background: white;
            padding: 10px 15px;
            margin: 8px 0;
            border-radius: 4px;
            border: 1px solid #ddd;
            display: flex;
            align-items: center;
            gap: 10px;
        }

        .method {
            font-weight: bold;
            padding: 2px 8px;
            border-radius: 4px;
            font-size: 0.85em;
        }

        .method-get {
            background: #dcfce7;
            color: #166534;
        }

        .method-post {
            background: #dbeafe;
            color: #1e40af;
        }

        .path {
            color: #475569;
            flex: 1;
        }

        footer {
            margin-top: 40px;
            padding-top: 20px;
            border-top: 1px solid #e2e8f0;
            text-align: center;
            color: #64748b;
            font-size: 0.9em;
        }

        a {
            color: #667eea;
            text-decoration: none;
        }

        a:hover {
            text-decoration: underline;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>🌞 Helioscope</h1>
        <p class="subtitle">Lightweight System Monitoring Collector</p>

        <div class="status">
            <div class="status-title">✓ Collector Active</div>
            <div>Receiving metrics from monitoring nodes</div>
        </div>

        <div class="features">
            <h2>Features</h2>
            <ul class="feature-list">
                <li class="feature-item">
                    <div class="feature-title">📊 Time-Series Storage</div>
                    <div class="feature-desc">SQLite-backed storage for efficient metric collection</div>
                </li>
                <li class="feature-item">
                    <div class="feature-title">🚀 High Performance</div>
                    <div class="feature-desc">Async Rust with Tokio for concurrent data ingestion</div>
                </li>
                <li class="feature-item">
                    <div class="feature-title">📈 Visualization Ready</div>
                    <div class="feature-desc">Server-side chart generation with Plotters</div>
                </li>
            </ul>
        </div>

        <div class="endpoints">
            <h2>API Endpoints</h2>
            <div class="endpoint">
                <span class="method method-get">GET</span>
                <span class="path">/health</span>
            </div>
            <div class="endpoint">
                <span class="method method-post">POST</span>
                <span class="path">/api/v1/probe</span>
            </div>
            <div class="endpoint">
                <span class="method method-get">GET</span>
                <span class="path">/ui</span>
            </div>
        </div>

        <footer>
            <p>Helioscope v0.2.0 | <a href="https://github.com/l1x/helioscope" target="_blank">GitHub</a></p>
            <p>Built with Rust 🦀</p>
        </footer>
    </div>
</body>
</html>"#;

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html; charset=utf-8")
        .body(full_body(html))
        .unwrap()
}

fn full_body(content: &str) -> BoxBody<Bytes, hyper::Error> {
    Full::new(Bytes::from(content.to_string()))
        .map_err(|never| match never {})
        .boxed()
}
