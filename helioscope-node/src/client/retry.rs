use std::future::Future;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, warn};

use super::http::ClientError;

pub async fn send_with_retry<F, Fut>(f: F, max_retries: u32) -> Result<(), ClientError>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<(), ClientError>>,
{
    let mut attempt = 0;

    loop {
        attempt += 1;

        match f().await {
            Ok(_) => {
                if attempt > 1 {
                    debug!("Successfully sent after {} attempts", attempt);
                }
                return Ok(());
            }
            Err(e) => {
                if attempt >= max_retries {
                    warn!("Failed to send after {} attempts: {}", max_retries, e);
                    return Err(e);
                }

                // Exponential backoff: 1s, 2s, 4s, 8s...
                let backoff_secs = 2u64.pow(attempt - 1);
                let backoff = Duration::from_secs(backoff_secs);

                warn!(
                    "Attempt {}/{} failed: {}. Retrying in {:?}",
                    attempt, max_retries, e, backoff
                );

                sleep(backoff).await;
            }
        }
    }
}
