use bytes::Bytes;
use ferrview_common::ProbeDataPoint;
use http_body_util::Full;
use hyper::{Request, StatusCode};
use hyper_util::{
    client::legacy::{Client, connect::HttpConnector},
    rt::TokioExecutor,
};
use serde::Serialize;
use std::error::Error;
use std::fmt;
use tracing::{debug, error};

#[derive(Debug, Serialize)]
struct ProbeDataBatch {
    data: Vec<ProbeDataPoint>,
}

#[derive(Debug)]
pub enum ClientError {
    Http(String),
    Serialization(String),
    InvalidResponse(StatusCode),
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientError::Http(msg) => write!(f, "HTTP error: {}", msg),
            ClientError::Serialization(msg) => write!(f, "Serialization error: {}", msg),
            ClientError::InvalidResponse(status) => {
                write!(f, "Invalid response status: {}", status)
            }
        }
    }
}

impl Error for ClientError {}

pub struct HttpClient {
    client: Client<HttpConnector, Full<Bytes>>,
    collector_url: String,
}

impl HttpClient {
    pub fn new(collector_addr: &str) -> Self {
        let connector = HttpConnector::new();

        let client = Client::builder(TokioExecutor::new())
            .http2_only(false) // Allow HTTP/1.1 fallback
            .build(connector);

        // Ensure URL has http:// prefix
        let collector_url =
            if collector_addr.starts_with("http://") || collector_addr.starts_with("https://") {
                format!("{}/api/v1/probe", collector_addr)
            } else {
                format!("http://{}/api/v1/probe", collector_addr)
            };

        debug!("HTTP client initialized for: {}", collector_url);

        Self {
            client,
            collector_url,
        }
    }

    pub async fn send_batch(&self, data: Vec<ProbeDataPoint>) -> Result<(), ClientError> {
        let batch = ProbeDataBatch { data };
        let count = batch.data.len();

        debug!("Sending batch of {} probe data points", count);

        let json =
            serde_json::to_string(&batch).map_err(|e| ClientError::Serialization(e.to_string()))?;

        let request = Request::builder()
            .method("POST")
            .uri(&self.collector_url)
            .header("Content-Type", "application/json")
            .body(Full::new(Bytes::from(json)))
            .map_err(|e| ClientError::Http(e.to_string()))?;

        let response = self
            .client
            .request(request)
            .await
            .map_err(|e| ClientError::Http(e.to_string()))?;

        let status = response.status();

        if status != StatusCode::ACCEPTED && status != StatusCode::OK {
            error!("Collector returned error status: {}", status);
            return Err(ClientError::InvalidResponse(status));
        }

        debug!("Batch sent successfully, status: {}", status);
        Ok(())
    }
}
