//! Retry policies with exponential backoff

use crate::error::{ResilienceError, ResilienceResult};
use std::future::Future;
use std::time::Duration;

/// Configuration for retry behavior.
///
/// This struct is designed to be serializable and loadable from external
/// configuration (e.g., YAML, TOML, environment variables).
///
/// # Example
///
/// ```rust
/// use rustboot_resilience::RetryConfig;
///
/// // Default configuration
/// let config = RetryConfig::default();
///
/// // Custom configuration
/// let config = RetryConfig::new(5, 200, 30000);
///
/// // Builder pattern
/// let config = RetryConfig::default()
///     .with_max_attempts(10)
///     .with_initial_delay_ms(50);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_attempts: usize,
    /// Initial delay between retries in milliseconds
    pub initial_delay_ms: u64,
    /// Maximum delay between retries in milliseconds
    pub max_delay_ms: u64,
}

impl RetryConfig {
    /// Create a new retry configuration
    pub fn new(max_attempts: usize, initial_delay_ms: u64, max_delay_ms: u64) -> Self {
        Self {
            max_attempts,
            initial_delay_ms,
            max_delay_ms,
        }
    }

    /// Set the maximum number of attempts
    pub fn with_max_attempts(mut self, max_attempts: usize) -> Self {
        self.max_attempts = max_attempts;
        self
    }

    /// Set the initial delay in milliseconds
    pub fn with_initial_delay_ms(mut self, delay: u64) -> Self {
        self.initial_delay_ms = delay;
        self
    }

    /// Set the maximum delay in milliseconds
    pub fn with_max_delay_ms(mut self, delay: u64) -> Self {
        self.max_delay_ms = delay;
        self
    }

    /// Convert to ExponentialBackoff
    pub fn to_backoff(&self) -> ExponentialBackoff {
        ExponentialBackoff::new(
            Duration::from_millis(self.initial_delay_ms),
            Duration::from_millis(self.max_delay_ms),
            2.0,
        )
    }

    /// Convert to RetryPolicy
    pub fn to_policy(&self) -> RetryPolicy {
        RetryPolicy::new(self.max_attempts).with_backoff(self.to_backoff())
    }
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay_ms: 100,
            max_delay_ms: 10000,
        }
    }
}

/// Retry policy for failed operations
pub struct RetryPolicy {
    max_retries: usize,
    backoff: ExponentialBackoff,
}

impl RetryPolicy {
    /// Create a new retry policy
    pub fn new(max_retries: usize) -> Self {
        Self {
            max_retries,
            backoff: ExponentialBackoff::default(),
        }
    }

    /// Set the backoff strategy
    pub fn with_backoff(mut self, backoff: ExponentialBackoff) -> Self {
        self.backoff = backoff;
        self
    }

    /// Execute an operation with retry
    pub async fn execute<F, Fut, T, E>(&self, mut operation: F) -> ResilienceResult<T>
    where
        F: FnMut() -> Fut,
        Fut: Future<Output = Result<T, E>>,
        E: std::fmt::Display,
    {
        let mut attempts = 0;

        loop {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(_) => {
                    attempts += 1;
                    if attempts >= self.max_retries {
                        return Err(ResilienceError::MaxRetriesExceeded(self.max_retries));
                    }

                    let delay = self.backoff.next_delay(attempts);
                    tokio::time::sleep(delay).await;
                }
            }
        }
    }
}

/// Exponential backoff strategy
#[derive(Debug, Clone)]
pub struct ExponentialBackoff {
    initial_delay: Duration,
    max_delay: Duration,
    multiplier: f64,
}

impl ExponentialBackoff {
    /// Create a new exponential backoff
    pub fn new(initial_delay: Duration, max_delay: Duration, multiplier: f64) -> Self {
        Self {
            initial_delay,
            max_delay,
            multiplier,
        }
    }

    /// Calculate the next delay based on attempt number
    pub fn next_delay(&self, attempt: usize) -> Duration {
        let delay_ms = self.initial_delay.as_millis() as f64
            * self.multiplier.powi(attempt as i32 - 1);
        
        let delay = Duration::from_millis(delay_ms as u64);
        delay.min(self.max_delay)
    }
}

impl Default for ExponentialBackoff {
    fn default() -> Self {
        Self {
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            multiplier: 2.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_retry_success_first_attempt() {
        let policy = RetryPolicy::new(3);
        let calls = Arc::new(AtomicUsize::new(0));
        let calls_clone = Arc::clone(&calls);

        let result = policy
            .execute(|| {
                let calls = Arc::clone(&calls_clone);
                async move {
                    calls.fetch_add(1, Ordering::SeqCst);
                    Ok::<_, String>(42)
                }
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_retry_success_after_failures() {
        let policy = RetryPolicy::new(3);
        let calls = Arc::new(AtomicUsize::new(0));
        let calls_clone = Arc::clone(&calls);

        let result = policy
            .execute(|| {
                let calls = Arc::clone(&calls_clone);
                async move {
                    let call_count = calls.fetch_add(1, Ordering::SeqCst) + 1;
                    if call_count < 3 {
                        Err("temporary failure")
                    } else {
                        Ok(42)
                    }
                }
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        assert_eq!(calls.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_retry_max_retries_exceeded() {
        let policy = RetryPolicy::new(3);
        let calls = Arc::new(AtomicUsize::new(0));
        let calls_clone = Arc::clone(&calls);

        let result = policy
            .execute(|| {
                let calls = Arc::clone(&calls_clone);
                async move {
                    calls.fetch_add(1, Ordering::SeqCst);
                    Err::<i32, _>("persistent failure")
                }
            })
            .await;

        assert!(result.is_err());
        assert_eq!(calls.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn test_exponential_backoff() {
        let backoff = ExponentialBackoff::new(
            Duration::from_millis(100),
            Duration::from_secs(5),
            2.0,
        );

        assert_eq!(backoff.next_delay(1), Duration::from_millis(100));
        assert_eq!(backoff.next_delay(2), Duration::from_millis(200));
        assert_eq!(backoff.next_delay(3), Duration::from_millis(400));
        assert_eq!(backoff.next_delay(4), Duration::from_millis(800));
    }
}
