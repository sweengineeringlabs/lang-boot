//! Database query logging utilities for debugging.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{debug, info, warn};

/// Configuration for query logging.
#[derive(Debug, Clone)]
pub struct QueryLoggerConfig {
    /// Whether to log all queries (default: true).
    pub log_all_queries: bool,
    /// Whether to log query parameters (default: false, for security).
    pub log_parameters: bool,
    /// Warn threshold for slow queries (default: 100ms).
    pub slow_query_threshold: Duration,
    /// Whether to log query results count (default: true).
    pub log_result_count: bool,
    /// Whether to track query statistics (default: true).
    pub track_statistics: bool,
}

impl Default for QueryLoggerConfig {
    fn default() -> Self {
        Self {
            log_all_queries: true,
            log_parameters: false,
            slow_query_threshold: Duration::from_millis(100),
            log_result_count: true,
            track_statistics: true,
        }
    }
}

impl QueryLoggerConfig {
    /// Create a new config with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set whether to log all queries.
    pub fn with_log_all(mut self, enabled: bool) -> Self {
        self.log_all_queries = enabled;
        self
    }

    /// Set whether to log parameters (CAUTION: may expose sensitive data).
    pub fn with_log_parameters(mut self, enabled: bool) -> Self {
        self.log_parameters = enabled;
        self
    }

    /// Set slow query threshold.
    pub fn with_slow_threshold(mut self, threshold: Duration) -> Self {
        self.slow_query_threshold = threshold;
        self
    }

    /// Set whether to log result counts.
    pub fn with_log_result_count(mut self, enabled: bool) -> Self {
        self.log_result_count = enabled;
        self
    }

    /// Set whether to track statistics.
    pub fn with_track_statistics(mut self, enabled: bool) -> Self {
        self.track_statistics = enabled;
        self
    }
}

/// Query statistics tracker.
#[derive(Debug, Default)]
pub struct QueryStats {
    total_queries: AtomicU64,
    slow_queries: AtomicU64,
    failed_queries: AtomicU64,
    total_duration_ms: AtomicU64,
}

impl QueryStats {
    /// Create new query statistics tracker.
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a query execution.
    pub fn record_query(&self, duration: Duration, is_slow: bool, is_error: bool) {
        self.total_queries.fetch_add(1, Ordering::Relaxed);
        self.total_duration_ms.fetch_add(duration.as_millis() as u64, Ordering::Relaxed);

        if is_slow {
            self.slow_queries.fetch_add(1, Ordering::Relaxed);
        }

        if is_error {
            self.failed_queries.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Get total query count.
    pub fn total_queries(&self) -> u64 {
        self.total_queries.load(Ordering::Relaxed)
    }

    /// Get slow query count.
    pub fn slow_queries(&self) -> u64 {
        self.slow_queries.load(Ordering::Relaxed)
    }

    /// Get failed query count.
    pub fn failed_queries(&self) -> u64 {
        self.failed_queries.load(Ordering::Relaxed)
    }

    /// Get average query duration.
    pub fn average_duration_ms(&self) -> u64 {
        let total = self.total_queries.load(Ordering::Relaxed);
        if total == 0 {
            0
        } else {
            self.total_duration_ms.load(Ordering::Relaxed) / total
        }
    }

    /// Reset all statistics.
    pub fn reset(&self) {
        self.total_queries.store(0, Ordering::Relaxed);
        self.slow_queries.store(0, Ordering::Relaxed);
        self.failed_queries.store(0, Ordering::Relaxed);
        self.total_duration_ms.store(0, Ordering::Relaxed);
    }

    /// Format statistics as a string.
    pub fn format_stats(&self) -> String {
        format!(
            "Total: {}, Slow: {}, Failed: {}, Avg: {}ms",
            self.total_queries(),
            self.slow_queries(),
            self.failed_queries(),
            self.average_duration_ms()
        )
    }
}

/// Query logger for debugging database operations.
pub struct QueryLogger {
    config: QueryLoggerConfig,
    stats: Arc<QueryStats>,
}

impl QueryLogger {
    /// Create a new query logger with default config.
    pub fn new() -> Self {
        Self {
            config: QueryLoggerConfig::default(),
            stats: Arc::new(QueryStats::new()),
        }
    }

    /// Create with custom config.
    pub fn with_config(config: QueryLoggerConfig) -> Self {
        Self {
            config,
            stats: Arc::new(QueryStats::new()),
        }
    }

    /// Get query statistics.
    pub fn stats(&self) -> Arc<QueryStats> {
        self.stats.clone()
    }

    /// Log a query execution.
    pub fn log_query(&self, sql: &str, params: &[&str], start: Instant) {
        let duration = start.elapsed();
        let is_slow = duration >= self.config.slow_query_threshold;

        if self.config.log_all_queries || is_slow {
            let duration_ms = duration.as_millis();

            if is_slow {
                warn!(
                    target: "rustboot::debug::database",
                    duration_ms = duration_ms,
                    sql = sql,
                    "Slow query detected"
                );
            } else {
                debug!(
                    target: "rustboot::debug::database",
                    duration_ms = duration_ms,
                    sql = sql,
                    "Query executed"
                );
            }

            if self.config.log_parameters && !params.is_empty() {
                debug!(
                    target: "rustboot::debug::database",
                    params = ?params,
                    "Query parameters"
                );
            }
        }

        if self.config.track_statistics {
            self.stats.record_query(duration, is_slow, false);
        }
    }

    /// Log a query error.
    pub fn log_error(&self, sql: &str, error: &str, start: Instant) {
        let duration = start.elapsed();

        warn!(
            target: "rustboot::debug::database",
            duration_ms = duration.as_millis(),
            sql = sql,
            error = error,
            "Query failed"
        );

        if self.config.track_statistics {
            self.stats.record_query(duration, false, true);
        }
    }

    /// Log query result count.
    pub fn log_result_count(&self, sql: &str, count: usize) {
        if self.config.log_result_count {
            debug!(
                target: "rustboot::debug::database",
                sql = sql,
                count = count,
                "Query returned {} rows", count
            );
        }
    }

    /// Print current statistics.
    pub fn print_stats(&self) {
        info!(
            target: "rustboot::debug::database",
            stats = %self.stats.format_stats(),
            "Database query statistics"
        );
    }
}

impl Default for QueryLogger {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper to time database queries.
pub struct QueryTimer {
    sql: String,
    start: Instant,
    logger: Arc<QueryLogger>,
}

impl QueryTimer {
    /// Create a new query timer.
    pub fn new(sql: impl Into<String>, logger: Arc<QueryLogger>) -> Self {
        Self {
            sql: sql.into(),
            start: Instant::now(),
            logger,
        }
    }

    /// Complete the query successfully.
    pub fn complete(self, params: &[&str]) {
        self.logger.log_query(&self.sql, params, self.start);
    }

    /// Complete the query with error.
    pub fn error(self, error: &str) {
        self.logger.log_error(&self.sql, error, self.start);
    }

    /// Record result count.
    pub fn with_result_count(self, count: usize) -> Self {
        self.logger.log_result_count(&self.sql, count);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_stats() {
        let stats = QueryStats::new();

        stats.record_query(Duration::from_millis(50), false, false);
        stats.record_query(Duration::from_millis(150), true, false);
        stats.record_query(Duration::from_millis(100), false, true);

        assert_eq!(stats.total_queries(), 3);
        assert_eq!(stats.slow_queries(), 1);
        assert_eq!(stats.failed_queries(), 1);
        assert_eq!(stats.average_duration_ms(), 100);
    }

    #[test]
    fn test_query_stats_reset() {
        let stats = QueryStats::new();

        stats.record_query(Duration::from_millis(50), false, false);
        assert_eq!(stats.total_queries(), 1);

        stats.reset();
        assert_eq!(stats.total_queries(), 0);
        assert_eq!(stats.slow_queries(), 0);
        assert_eq!(stats.failed_queries(), 0);
    }

    #[test]
    fn test_query_logger() {
        let logger = QueryLogger::new();
        let start = Instant::now();

        logger.log_query("SELECT * FROM users", &[], start);
        assert_eq!(logger.stats().total_queries(), 1);
    }

    #[test]
    fn test_query_logger_with_config() {
        let config = QueryLoggerConfig::new()
            .with_log_parameters(true)
            .with_slow_threshold(Duration::from_millis(50));

        let logger = QueryLogger::with_config(config);
        assert!(logger.config.log_parameters);
    }

    #[test]
    fn test_query_timer() {
        let logger = Arc::new(QueryLogger::new());
        let timer = QueryTimer::new("SELECT * FROM products", logger.clone());

        std::thread::sleep(Duration::from_millis(10));
        timer.complete(&["param1"]);

        assert_eq!(logger.stats().total_queries(), 1);
    }

    #[test]
    fn test_query_timer_error() {
        let logger = Arc::new(QueryLogger::new());
        let timer = QueryTimer::new("INVALID SQL", logger.clone());

        timer.error("Syntax error");

        assert_eq!(logger.stats().failed_queries(), 1);
    }
}
