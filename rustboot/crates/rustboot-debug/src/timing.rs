//! Timing utilities for profiling and performance debugging.

use std::time::{Duration, Instant};
use tracing::{debug, info, warn};

/// Timing threshold configuration.
#[derive(Debug, Clone)]
pub struct TimingThresholds {
    /// Warn if operation takes longer than this (default: 1s).
    pub warn_threshold: Duration,
    /// Log as info if operation takes longer than this (default: 100ms).
    pub info_threshold: Duration,
    /// Always log debug (default: true).
    pub always_debug: bool,
}

impl Default for TimingThresholds {
    fn default() -> Self {
        Self {
            warn_threshold: Duration::from_secs(1),
            info_threshold: Duration::from_millis(100),
            always_debug: true,
        }
    }
}

impl TimingThresholds {
    /// Create new thresholds with custom values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set warn threshold.
    pub fn with_warn_threshold(mut self, duration: Duration) -> Self {
        self.warn_threshold = duration;
        self
    }

    /// Set info threshold.
    pub fn with_info_threshold(mut self, duration: Duration) -> Self {
        self.info_threshold = duration;
        self
    }

    /// Set whether to always log debug.
    pub fn with_always_debug(mut self, enabled: bool) -> Self {
        self.always_debug = enabled;
        self
    }
}

/// A timing guard that logs elapsed time on drop.
pub struct TimingGuard {
    name: String,
    start: Instant,
    thresholds: TimingThresholds,
}

impl TimingGuard {
    /// Create a new timing guard with default thresholds.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            start: Instant::now(),
            thresholds: TimingThresholds::default(),
        }
    }

    /// Create a new timing guard with custom thresholds.
    pub fn with_thresholds(name: impl Into<String>, thresholds: TimingThresholds) -> Self {
        Self {
            name: name.into(),
            start: Instant::now(),
            thresholds,
        }
    }

    /// Get elapsed time so far (without dropping the guard).
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    /// Log a checkpoint without ending the timing.
    pub fn checkpoint(&self, checkpoint_name: &str) {
        let elapsed = self.elapsed();
        debug!(
            target: "rustboot::debug::timing",
            operation = %self.name,
            checkpoint = %checkpoint_name,
            elapsed_ms = elapsed.as_millis(),
            "Timing checkpoint"
        );
    }
}

impl Drop for TimingGuard {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed();
        let elapsed_ms = elapsed.as_millis();

        if elapsed >= self.thresholds.warn_threshold {
            warn!(
                target: "rustboot::debug::timing",
                operation = %self.name,
                elapsed_ms = elapsed_ms,
                "Slow operation detected (>{:?})",
                self.thresholds.warn_threshold
            );
        } else if elapsed >= self.thresholds.info_threshold {
            info!(
                target: "rustboot::debug::timing",
                operation = %self.name,
                elapsed_ms = elapsed_ms,
                "Operation completed (>{:?})",
                self.thresholds.info_threshold
            );
        } else if self.thresholds.always_debug {
            debug!(
                target: "rustboot::debug::timing",
                operation = %self.name,
                elapsed_ms = elapsed_ms,
                "Operation completed"
            );
        }
    }
}

/// Async-friendly timing scope using RAII.
pub struct TimingScope {
    guard: TimingGuard,
}

impl TimingScope {
    /// Create a new timing scope with default thresholds.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            guard: TimingGuard::new(name),
        }
    }

    /// Create a new timing scope with custom thresholds.
    pub fn with_thresholds(name: impl Into<String>, thresholds: TimingThresholds) -> Self {
        Self {
            guard: TimingGuard::with_thresholds(name, thresholds),
        }
    }

    /// Get elapsed time.
    pub fn elapsed(&self) -> Duration {
        self.guard.elapsed()
    }

    /// Log a checkpoint.
    pub fn checkpoint(&self, checkpoint_name: &str) {
        self.guard.checkpoint(checkpoint_name);
    }
}

/// Time a synchronous operation.
pub fn time_sync<F, R>(name: impl Into<String>, f: F) -> R
where
    F: FnOnce() -> R,
{
    let _guard = TimingGuard::new(name);
    f()
}

/// Time an async operation.
pub async fn time_async<F, R>(name: impl Into<String>, f: F) -> R
where
    F: std::future::Future<Output = R>,
{
    let _guard = TimingGuard::new(name);
    f.await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_timing_guard() {
        let guard = TimingGuard::new("test_operation");
        std::thread::sleep(Duration::from_millis(10));
        assert!(guard.elapsed() >= Duration::from_millis(10));
    }

    #[test]
    fn test_timing_scope() {
        let scope = TimingScope::new("test_scope");
        std::thread::sleep(Duration::from_millis(10));
        assert!(scope.elapsed() >= Duration::from_millis(10));
    }

    #[test]
    fn test_time_sync() {
        let result = time_sync("test_sync", || {
            std::thread::sleep(Duration::from_millis(10));
            42
        });
        assert_eq!(result, 42);
    }

    #[tokio::test]
    async fn test_time_async() {
        let result = time_async("test_async", async {
            tokio::time::sleep(Duration::from_millis(10)).await;
            42
        })
        .await;
        assert_eq!(result, 42);
    }

    #[test]
    fn test_checkpoint() {
        let guard = TimingGuard::new("checkpointed_operation");
        std::thread::sleep(Duration::from_millis(5));
        guard.checkpoint("midpoint");
        std::thread::sleep(Duration::from_millis(5));
        assert!(guard.elapsed() >= Duration::from_millis(10));
    }

    #[test]
    fn test_custom_thresholds() {
        let thresholds = TimingThresholds::new()
            .with_warn_threshold(Duration::from_secs(2))
            .with_info_threshold(Duration::from_millis(50))
            .with_always_debug(false);

        let guard = TimingGuard::with_thresholds("custom_threshold", thresholds);
        std::thread::sleep(Duration::from_millis(10));
        drop(guard);
    }
}
