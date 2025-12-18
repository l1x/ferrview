// ferrview-collector/src/store/date_range_reader.rs

use sqlx::sqlite::{SqliteConnectOptions, SqliteConnection};
use sqlx::{Connection, Row};
use std::path::Path;
use std::str::FromStr;
use tracing::{debug, info, warn};

use crate::store::date_range::DateRange;
use crate::store::errors::StoreError;
use crate::store::queries::MetricDataPoint;

/// Reader for querying metrics across multiple date-based database files
pub struct DateRangeReader {
    data_dir: String,
}

impl DateRangeReader {
    /// Create a new DateRangeReader for the given data directory
    pub fn new(data_dir: &str) -> Self {
        info!("Initializing DateRangeReader for: {}", data_dir);
        Self {
            data_dir: data_dir.to_string(),
        }
    }

    /// List all available dates that have database files
    #[allow(dead_code)]
    pub fn list_available_dates(&self) -> Vec<String> {
        let mut dates = Vec::new();

        let entries = match std::fs::read_dir(&self.data_dir) {
            Ok(entries) => entries,
            Err(e) => {
                warn!("Failed to read data directory: {}", e);
                return dates;
            }
        };

        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                // Match ferrview_YYYY-MM-DD.db
                if name.starts_with("ferrview_") && name.ends_with(".db") && name.len() == 23 {
                    let date = &name[9..19]; // Extract YYYY-MM-DD
                    dates.push(date.to_string());
                }
            }
        }

        dates.sort();
        debug!("Found {} database files", dates.len());
        dates
    }

    /// Query metrics for a specific node across the date range
    pub async fn query_node_metrics(
        &self,
        node_id: &str,
        metric_pattern: &str,
        range: &DateRange,
    ) -> Result<Vec<MetricDataPoint>, StoreError> {
        let dates = range.dates();
        let start_str = range.start_time_str();
        let end_str = range.end_time_str();

        debug!(
            "Querying metrics for node {} with pattern '{}' across {} dates ({} to {})",
            node_id,
            metric_pattern,
            dates.len(),
            start_str,
            end_str
        );

        let mut all_results = Vec::new();

        for date in &dates {
            let db_path = format!("{}/ferrview_{}.db", self.data_dir, date);

            if !Path::new(&db_path).exists() {
                debug!("Database file not found, skipping: {}", db_path);
                continue;
            }

            match self
                .query_single_db(&db_path, node_id, metric_pattern, &start_str, &end_str)
                .await
            {
                Ok(mut results) => {
                    debug!("Found {} results in {}", results.len(), date);
                    all_results.append(&mut results);
                }
                Err(e) => {
                    warn!("Failed to query {}: {}", db_path, e);
                    // Continue with other databases
                }
            }
        }

        // Results should already be sorted by timestamp within each db,
        // but we need to merge-sort across databases
        all_results.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        debug!(
            "Total results across all databases: {}",
            all_results.len()
        );

        Ok(all_results)
    }

    /// Query a single database file
    async fn query_single_db(
        &self,
        db_path: &str,
        node_id: &str,
        metric_pattern: &str,
        start_str: &str,
        end_str: &str,
    ) -> Result<Vec<MetricDataPoint>, StoreError> {
        let db_url = format!("sqlite://{}", db_path);

        let options = SqliteConnectOptions::from_str(&db_url)?
            .read_only(true)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);

        let mut conn = SqliteConnection::connect_with(&options).await?;

        let rows = sqlx::query(
            r#"
            SELECT node_id, timestamp, probe_type, probe_name, probe_value
            FROM probe_data
            WHERE node_id = ?1
              AND probe_name LIKE ?2
              AND timestamp >= ?3
              AND timestamp <= ?4
            ORDER BY timestamp ASC
            "#,
        )
        .bind(node_id)
        .bind(metric_pattern)
        .bind(start_str)
        .bind(end_str)
        .fetch_all(&mut conn)
        .await?;

        let results: Vec<MetricDataPoint> = rows
            .into_iter()
            .map(|row| MetricDataPoint {
                node_id: row.get("node_id"),
                timestamp: row.get("timestamp"),
                probe_type: row.get("probe_type"),
                probe_name: row.get("probe_name"),
                probe_value: row.get("probe_value"),
            })
            .collect();

        conn.close().await?;

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_reader() {
        let reader = DateRangeReader::new("/tmp/test");
        assert_eq!(reader.data_dir, "/tmp/test");
    }
}
