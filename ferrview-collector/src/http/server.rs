// src/http/server.rs

use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response};
use hyper_util::rt::TokioIo;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{debug, error, info};

use crate::http::handlers::{api, charts, web};
use crate::http::response::BoxBody;
use crate::store::date_range::DateRange;
use crate::store::date_range_reader::DateRangeReader;
use crate::store::errors::StoreError;
use crate::store::writer::WriterHandle;

pub struct ServerState {
    pub writer: WriterHandle,
    pub date_range_reader: DateRangeReader,
    pub data_dir: String,
}

pub struct HttpServer {
    addr: SocketAddr,
    state: Arc<ServerState>,
}

impl HttpServer {
    pub fn new(
        host: &str,
        port: &str,
        writer: WriterHandle,
        date_range_reader: DateRangeReader,
        data_dir: String,
    ) -> Result<Self, StoreError> {
        let addr: SocketAddr = format!("{}:{}", host, port)
            .parse()
            .map_err(|e| StoreError::InvalidQuery(format!("Invalid address: {}", e)))?;

        let state = Arc::new(ServerState {
            writer,
            date_range_reader,
            data_dir,
        });

        Ok(Self { addr, state })
    }

    pub async fn run(self) -> Result<(), StoreError> {
        let listener = TcpListener::bind(self.addr).await.map_err(StoreError::Io)?;

        info!("HTTP server listening on {}", self.addr);

        loop {
            let (stream, remote_addr) = match listener.accept().await {
                Ok(conn) => conn,
                Err(e) => {
                    error!("Failed to accept connection: {}", e);
                    continue;
                }
            };

            let io = TokioIo::new(stream);
            let state = Arc::clone(&self.state);

            tokio::spawn(async move {
                let service = service_fn(move |req| {
                    let state = Arc::clone(&state);
                    route(req, state, remote_addr)
                });

                if let Err(e) = http1::Builder::new().serve_connection(io, service).await {
                    error!("Error serving connection from {}: {}", remote_addr, e);
                }
            });
        }
    }
}

async fn route(
    req: Request<Incoming>,
    state: Arc<ServerState>,
    remote_addr: SocketAddr,
) -> Result<Response<BoxBody>, hyper::Error> {
    debug!(
        "Request from {}: {} {}",
        remote_addr,
        req.method(),
        req.uri()
    );

    let path = req.uri().path();

    let (status, body, content_type) = match (req.method(), path) {
        // API routes
        (&Method::POST, "/api/v1/probe") => {
            let (s, b) = api::handle_probe(req, &state.writer).await;
            (s, b, "application/json")
        }
        (&Method::GET, "/health") => {
            let (s, b) = api::handle_health().await;
            (s, b, "application/json")
        }

        // Web UI routes
        (&Method::GET, "/ui") => {
            let (s, b) = web::handle_home(&state.data_dir).await;
            (s, b, "text/html; charset=utf-8")
        }
        (&Method::GET, _) if path.starts_with("/ui/node/") => {
            route_node(path, req.uri().query(), &state).await
        }

        // 404
        _ => {
            let (s, b) = api::handle_not_found().await;
            (s, b, "application/json")
        }
    };

    Ok(Response::builder()
        .status(status)
        .header("Content-Type", content_type)
        .body(body)
        .unwrap())
}

async fn route_node(
    path: &str,
    query: Option<&str>,
    state: &ServerState,
) -> (hyper::StatusCode, BoxBody, &'static str) {
    // Path: /ui/node/{node_id} or /ui/node/{node_id}/{chart}.svg
    let parts: Vec<&str> = path.split('/').collect();
    // parts: ["", "ui", "node", "{node_id}", ...]

    if parts.len() < 4 {
        let (s, b) = api::handle_not_found().await;
        return (s, b, "application/json");
    }

    let node_id = parts[3];

    // Chart routes: /ui/node/{id}/{chart}.svg
    if parts.len() == 5 {
        let chart_file = parts[4];

        // Parse date range from query parameters
        let range = parse_date_range_from_query(query);

        let (status, body) = match chart_file {
            "cpu.svg" => {
                charts::handle_cpu_chart(node_id, &range, &state.date_range_reader).await
            }
            "memory.svg" => {
                charts::handle_memory_chart(node_id, &range, &state.date_range_reader).await
            }
            "temperature.svg" => {
                charts::handle_temperature_chart(node_id, &range, &state.date_range_reader).await
            }
            "network.svg" => {
                charts::handle_network_chart(node_id, &range, &state.date_range_reader).await
            }
            "disk.svg" => {
                charts::handle_disk_chart(node_id, &range, &state.date_range_reader).await
            }
            "forks.svg" => {
                charts::handle_forks_chart(node_id, &range, &state.date_range_reader).await
            }
            _ => api::handle_not_found().await,
        };
        let content_type = if chart_file.ends_with(".svg") {
            "image/svg+xml"
        } else {
            "application/json"
        };
        return (status, body, content_type);
    }

    // Dashboard route: /ui/node/{id}
    let (s, b) = web::handle_node_dashboard(node_id, &state.data_dir).await;
    (s, b, "text/html; charset=utf-8")
}

/// Parse query string into a HashMap
fn parse_query_string(query: Option<&str>) -> HashMap<String, String> {
    let mut params = HashMap::new();

    if let Some(q) = query {
        for pair in q.split('&') {
            if let Some((key, value)) = pair.split_once('=') {
                params.insert(key.to_string(), value.to_string());
            }
        }
    }

    params
}

/// Parse date range from query parameters
fn parse_date_range_from_query(query: Option<&str>) -> DateRange {
    let params = parse_query_string(query);

    match params.get("range").map(|s| s.as_str()) {
        Some("today") => DateRange::today(),
        Some("7d") => DateRange::last_n_days(7),
        Some("30d") => DateRange::last_n_days(30),
        Some("custom") => {
            let start = params.get("start").map(|s| s.as_str()).unwrap_or("");
            let end = params.get("end").map(|s| s.as_str()).unwrap_or("");
            DateRange::custom(start, end).unwrap_or_else(|_| DateRange::today())
        }
        _ => DateRange::today(), // Default to today
    }
}
