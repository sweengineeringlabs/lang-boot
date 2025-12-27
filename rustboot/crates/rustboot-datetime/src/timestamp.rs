//! Timestamp utilities

use chrono::{DateTime, Utc};

/// Get current UTC timestamp
pub fn now() -> DateTime<Utc> {
    Utc::now()
}

/// Get current timestamp as milliseconds since epoch
pub fn now_millis() -> i64 {
    Utc::now().timestamp_millis()
}

/// Get current timestamp as seconds since epoch
pub fn now_secs() -> i64 {
    Utc::now().timestamp()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_now() {
        let ts = now();
        assert!(ts.timestamp() > 0);
    }

    #[test]
    fn test_now_millis() {
        let millis = now_millis();
        assert!(millis > 0);
    }

    #[test]
    fn test_now_secs() {
        let secs = now_secs();
        assert!(secs > 0);
    }
}
