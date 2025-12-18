// ferrview-collector/src/store/date_range.rs

use time::{Date, Duration, Month, OffsetDateTime};

use crate::store::errors::StoreError;

/// Represents a date range for querying metrics across multiple database files
#[derive(Debug, Clone)]
pub struct DateRange {
    /// Start date in YYYY-MM-DD format
    pub start_date: String,
    /// End date in YYYY-MM-DD format
    pub end_date: String,
    /// Start timestamp for filtering within the range
    pub start_time: OffsetDateTime,
    /// End timestamp for filtering within the range
    pub end_time: OffsetDateTime,
}

impl DateRange {
    /// Create a date range for today only
    pub fn today() -> Self {
        let now = OffsetDateTime::now_utc();
        let today = format_date(now.date());

        // Start at midnight UTC today, end at current time
        let start_of_day = now.replace_time(time::Time::MIDNIGHT);

        Self {
            start_date: today.clone(),
            end_date: today,
            start_time: start_of_day,
            end_time: now,
        }
    }

    /// Create a date range for the last N days (including today)
    pub fn last_n_days(n: u32) -> Self {
        let now = OffsetDateTime::now_utc();
        let today = now.date();
        let start = today - Duration::days(i64::from(n) - 1);

        let start_time = OffsetDateTime::new_utc(start, time::Time::MIDNIGHT);

        Self {
            start_date: format_date(start),
            end_date: format_date(today),
            start_time,
            end_time: now,
        }
    }

    /// Create a custom date range
    pub fn custom(start: &str, end: &str) -> Result<Self, StoreError> {
        let start_date = parse_date(start)?;
        let end_date = parse_date(end)?;

        if start_date > end_date {
            return Err(StoreError::InvalidQuery(
                "Start date must be before or equal to end date".to_string(),
            ));
        }

        let now = OffsetDateTime::now_utc();
        let start_time = OffsetDateTime::new_utc(start_date, time::Time::MIDNIGHT);

        // End time is either end of the end_date or now, whichever is earlier
        let end_of_end_date =
            OffsetDateTime::new_utc(end_date, time::Time::MIDNIGHT) + Duration::days(1);
        let end_time = if end_of_end_date > now {
            now
        } else {
            end_of_end_date
        };

        Ok(Self {
            start_date: start.to_string(),
            end_date: end.to_string(),
            start_time,
            end_time,
        })
    }

    /// Get all dates in this range as YYYY-MM-DD strings
    pub fn dates(&self) -> Vec<String> {
        let mut dates = Vec::new();
        let start = parse_date(&self.start_date).expect("start_date should be valid");
        let end = parse_date(&self.end_date).expect("end_date should be valid");

        let mut current = start;
        while current <= end {
            dates.push(format_date(current));
            current = current.next_day().unwrap_or(current);
            if current == start {
                // Overflow protection
                break;
            }
        }

        dates
    }

    /// Format timestamps for SQL queries (RFC3339)
    pub fn start_time_str(&self) -> String {
        format_timestamp(self.start_time)
    }

    pub fn end_time_str(&self) -> String {
        format_timestamp(self.end_time)
    }
}

/// Parse a YYYY-MM-DD string into a Date
fn parse_date(s: &str) -> Result<Date, StoreError> {
    if s.len() != 10 {
        return Err(StoreError::InvalidQuery(format!(
            "Invalid date format: {}",
            s
        )));
    }

    let year: i32 = s[0..4]
        .parse()
        .map_err(|_| StoreError::InvalidQuery(format!("Invalid year in date: {}", s)))?;
    let month: u8 = s[5..7]
        .parse()
        .map_err(|_| StoreError::InvalidQuery(format!("Invalid month in date: {}", s)))?;
    let day: u8 = s[8..10]
        .parse()
        .map_err(|_| StoreError::InvalidQuery(format!("Invalid day in date: {}", s)))?;

    let month = Month::try_from(month)
        .map_err(|_| StoreError::InvalidQuery(format!("Invalid month in date: {}", s)))?;

    Date::from_calendar_date(year, month, day)
        .map_err(|_| StoreError::InvalidQuery(format!("Invalid date: {}", s)))
}

/// Format a Date as YYYY-MM-DD
fn format_date(date: Date) -> String {
    format!(
        "{:04}-{:02}-{:02}",
        date.year(),
        date.month() as u8,
        date.day()
    )
}

/// Format timestamp in RFC3339 format for SQLite queries
fn format_timestamp(dt: OffsetDateTime) -> String {
    dt.format(&time::format_description::well_known::Rfc3339)
        .unwrap_or_else(|_| String::from("1970-01-01T00:00:00Z"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_today() {
        let range = DateRange::today();
        assert_eq!(range.start_date, range.end_date);
        assert!(range.start_time <= range.end_time);
    }

    #[test]
    fn test_last_7_days() {
        let range = DateRange::last_n_days(7);
        let dates = range.dates();
        assert_eq!(dates.len(), 7);
    }

    #[test]
    fn test_last_1_day() {
        let range = DateRange::last_n_days(1);
        let dates = range.dates();
        assert_eq!(dates.len(), 1);
    }

    #[test]
    fn test_custom_range() {
        let range = DateRange::custom("2024-12-01", "2024-12-03").unwrap();
        let dates = range.dates();
        assert_eq!(dates, vec!["2024-12-01", "2024-12-02", "2024-12-03"]);
    }

    #[test]
    fn test_custom_range_invalid_order() {
        let result = DateRange::custom("2024-12-10", "2024-12-01");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_date() {
        assert!(parse_date("2024-12-08").is_ok());
        assert!(parse_date("invalid").is_err());
        assert!(parse_date("2024-13-01").is_err()); // Invalid month
    }
}
