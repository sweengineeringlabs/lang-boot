//! Formatting utilities

use crate::error::{DateTimeError, DateTimeResult};
use chrono::{DateTime, Utc};
use std::time::Duration;

/// Format a timestamp in RFC3339 format
pub fn format_timestamp(timestamp: &DateTime<Utc>) -> String {
    timestamp.to_rfc3339()
}

/// Parse a timestamp from RFC3339 format
pub fn parse_timestamp(s: &str) -> DateTimeResult<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|e| DateTimeError::ParseError(e.to_string()))
}

/// Format a duration in human-readable form
pub fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    if secs < 60 {
        format!("{}s", secs)
    } else if secs < 3600 {
        format!("{}m {}s", secs / 60, secs % 60)
    } else {
        format!("{}h {}m", secs / 3600, (secs % 3600) / 60)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_and_parse_timestamp() {
        let now = Utc::now();
        let formatted = format_timestamp(&now);
        let parsed = parse_timestamp(&formatted).unwrap();
        
        // Compare timestamps (allowing small diff due to precision)
        assert!((now.timestamp() - parsed.timestamp()).abs() < 1);
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(Duration::from_secs(30)), "30s");
        assert_eq!(format_duration(Duration::from_secs(90)), "1m 30s");
        assert_eq!(format_duration(Duration::from_secs(3661)), "1h 1m");
    }
}
