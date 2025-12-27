// Test file for the #[retry] macro with RetryableError support

#![allow(dead_code)]

use rustboot_error::RetryableError;
use rustboot_macros::retry;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

// Test error type that implements RetryableError
#[derive(Debug, Clone)]
enum TestError {
    Retryable { retry_after_ms: Option<u64> },
    NotRetryable(String),
}

impl std::fmt::Display for TestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestError::Retryable { .. } => write!(f, "retryable error"),
            TestError::NotRetryable(msg) => write!(f, "not retryable: {}", msg),
        }
    }
}

impl RetryableError for TestError {
    fn is_retryable(&self) -> bool {
        matches!(self, TestError::Retryable { .. })
    }

    fn retry_after_ms(&self) -> Option<u64> {
        match self {
            TestError::Retryable { retry_after_ms } => *retry_after_ms,
            TestError::NotRetryable(_) => None,
        }
    }
}

// ============================================================================
// Basic retry tests (without retryable flag - retries on any error)
// ============================================================================

#[test]
fn test_retry_sync_success_first_attempt() {
    #[retry(max_attempts = 3)]
    fn always_succeeds() -> Result<i32, String> {
        Ok(42)
    }

    assert_eq!(always_succeeds().unwrap(), 42);
}

#[test]
fn test_retry_sync_success_after_failures() {
    static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

    #[retry(max_attempts = 3, delay = 1)]
    fn succeeds_on_third() -> Result<i32, String> {
        let count = CALL_COUNT.fetch_add(1, Ordering::SeqCst) + 1;
        if count < 3 {
            Err("temporary failure".to_string())
        } else {
            Ok(42)
        }
    }

    CALL_COUNT.store(0, Ordering::SeqCst);
    assert_eq!(succeeds_on_third().unwrap(), 42);
    assert_eq!(CALL_COUNT.load(Ordering::SeqCst), 3);
}

#[tokio::test]
async fn test_retry_async_success_first_attempt() {
    #[retry(max_attempts = 3)]
    async fn async_succeeds() -> Result<String, String> {
        Ok("success".to_string())
    }

    assert_eq!(async_succeeds().await.unwrap(), "success");
}

#[tokio::test]
async fn test_retry_async_success_after_failures() {
    static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

    #[retry(max_attempts = 3, delay = 1)]
    async fn async_succeeds_on_second() -> Result<i32, String> {
        let count = CALL_COUNT.fetch_add(1, Ordering::SeqCst) + 1;
        if count < 2 {
            Err("temporary".to_string())
        } else {
            Ok(100)
        }
    }

    CALL_COUNT.store(0, Ordering::SeqCst);
    assert_eq!(async_succeeds_on_second().await.unwrap(), 100);
    assert_eq!(CALL_COUNT.load(Ordering::SeqCst), 2);
}

// ============================================================================
// RetryableError tests (with retryable = true flag)
// ============================================================================

#[tokio::test]
async fn test_retry_retryable_success_after_retryable_errors() {
    static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

    #[retry(max_attempts = 5, delay = 1, retryable = true)]
    async fn retryable_succeeds() -> Result<i32, TestError> {
        let count = CALL_COUNT.fetch_add(1, Ordering::SeqCst) + 1;
        if count < 3 {
            Err(TestError::Retryable { retry_after_ms: None })
        } else {
            Ok(42)
        }
    }

    CALL_COUNT.store(0, Ordering::SeqCst);
    assert_eq!(retryable_succeeds().await.unwrap(), 42);
    assert_eq!(CALL_COUNT.load(Ordering::SeqCst), 3);
}

#[tokio::test]
async fn test_retry_retryable_stops_on_non_retryable() {
    static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

    #[retry(max_attempts = 5, delay = 1, retryable = true)]
    async fn fails_non_retryable() -> Result<i32, TestError> {
        let count = CALL_COUNT.fetch_add(1, Ordering::SeqCst) + 1;
        if count == 1 {
            // First call: retryable error
            Err(TestError::Retryable { retry_after_ms: None })
        } else {
            // Second call: non-retryable error - should stop immediately
            Err(TestError::NotRetryable("permanent failure".to_string()))
        }
    }

    CALL_COUNT.store(0, Ordering::SeqCst);
    let result = fails_non_retryable().await;
    assert!(result.is_err());
    // Should have stopped after 2 calls (not tried all 5)
    assert_eq!(CALL_COUNT.load(Ordering::SeqCst), 2);
}

#[tokio::test]
async fn test_retry_retryable_honors_retry_after_hint() {
    static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

    #[retry(max_attempts = 3, delay = 1000, retryable = true)]
    async fn with_retry_hint() -> Result<i32, TestError> {
        let count = CALL_COUNT.fetch_add(1, Ordering::SeqCst) + 1;
        if count < 2 {
            // Error with short retry hint (should use this instead of 1000ms delay)
            Err(TestError::Retryable { retry_after_ms: Some(1) })
        } else {
            Ok(42)
        }
    }

    CALL_COUNT.store(0, Ordering::SeqCst);
    let start = std::time::Instant::now();
    let result = with_retry_hint().await;
    let elapsed = start.elapsed();

    assert!(result.is_ok());
    // Should have used 1ms hint, not 1000ms default
    assert!(elapsed.as_millis() < 100, "Should have used retry_after_ms hint");
}

#[tokio::test]
async fn test_retry_retryable_max_attempts_exceeded() {
    static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

    #[retry(max_attempts = 3, delay = 1, retryable = true)]
    async fn always_fails_retryable() -> Result<i32, TestError> {
        CALL_COUNT.fetch_add(1, Ordering::SeqCst);
        Err(TestError::Retryable { retry_after_ms: None })
    }

    CALL_COUNT.store(0, Ordering::SeqCst);
    let result = always_fails_retryable().await;
    assert!(result.is_err());
    assert_eq!(CALL_COUNT.load(Ordering::SeqCst), 3);
}

// ============================================================================
// Custom name parameter tests
// ============================================================================

#[tokio::test]
async fn test_retry_with_custom_name() {
    static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

    // Custom name for logging - e.g., "embedding.generate" instead of function name
    #[retry(max_attempts = 3, delay = 1, retryable = true, name = "embedding.generate")]
    async fn internal_fn_name() -> Result<i32, TestError> {
        let count = CALL_COUNT.fetch_add(1, Ordering::SeqCst) + 1;
        if count < 2 {
            Err(TestError::Retryable { retry_after_ms: None })
        } else {
            Ok(42)
        }
    }

    CALL_COUNT.store(0, Ordering::SeqCst);
    assert_eq!(internal_fn_name().await.unwrap(), 42);
}

// ============================================================================
// Backoff strategy tests
// ============================================================================

#[test]
fn test_retry_fixed_backoff() {
    #[retry(max_attempts = 2, backoff = "fixed", delay = 1)]
    fn fixed_backoff_fn() -> Result<(), String> {
        Err("fail".to_string())
    }

    assert!(fixed_backoff_fn().is_err());
}

#[test]
fn test_retry_exponential_backoff() {
    #[retry(max_attempts = 2, backoff = "exponential", delay = 1)]
    fn exp_backoff_fn() -> Result<(), String> {
        Err("fail".to_string())
    }

    assert!(exp_backoff_fn().is_err());
}

#[test]
fn test_retry_with_max_delay() {
    #[retry(max_attempts = 2, backoff = "exponential", delay = 1, max_delay = 10)]
    fn capped_backoff_fn() -> Result<(), String> {
        Err("fail".to_string())
    }

    assert!(capped_backoff_fn().is_err());
}

#[test]
fn test_retry_with_jitter() {
    #[retry(max_attempts = 2, delay = 1, jitter = true)]
    fn jitter_fn() -> Result<(), String> {
        Err("fail".to_string())
    }

    assert!(jitter_fn().is_err());
}

// ============================================================================
// Runtime max_attempts_param tests
// ============================================================================

#[tokio::test]
async fn test_retry_max_attempts_param_runtime_value() {
    static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

    // max_attempts comes from `max_retries` parameter at runtime
    #[retry(max_attempts_param = "max_retries", delay = 1, retryable = true)]
    async fn retry_with_param(max_retries: usize) -> Result<i32, TestError> {
        CALL_COUNT.fetch_add(1, Ordering::SeqCst);
        Err(TestError::Retryable { retry_after_ms: None })
    }

    // Test with 2 retries
    CALL_COUNT.store(0, Ordering::SeqCst);
    let result = retry_with_param(2).await;
    assert!(result.is_err());
    assert_eq!(CALL_COUNT.load(Ordering::SeqCst), 2);

    // Test with 5 retries
    CALL_COUNT.store(0, Ordering::SeqCst);
    let result = retry_with_param(5).await;
    assert!(result.is_err());
    assert_eq!(CALL_COUNT.load(Ordering::SeqCst), 5);
}

#[tokio::test]
async fn test_retry_max_attempts_param_success_before_limit() {
    static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

    #[retry(max_attempts_param = "max_retries", delay = 1, retryable = true)]
    async fn retry_succeeds_on_third(max_retries: usize) -> Result<i32, TestError> {
        let count = CALL_COUNT.fetch_add(1, Ordering::SeqCst) + 1;
        if count < 3 {
            Err(TestError::Retryable { retry_after_ms: None })
        } else {
            Ok(42)
        }
    }

    // With 5 retries, should succeed on 3rd attempt
    CALL_COUNT.store(0, Ordering::SeqCst);
    let result = retry_succeeds_on_third(5).await;
    assert_eq!(result.unwrap(), 42);
    assert_eq!(CALL_COUNT.load(Ordering::SeqCst), 3);
}

#[test]
fn test_retry_max_attempts_param_sync() {
    static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

    #[retry(max_attempts_param = "max_retries", delay = 1)]
    fn sync_retry_with_param(max_retries: usize) -> Result<i32, String> {
        CALL_COUNT.fetch_add(1, Ordering::SeqCst);
        Err("fail".to_string())
    }

    CALL_COUNT.store(0, Ordering::SeqCst);
    let result = sync_retry_with_param(4);
    assert!(result.is_err());
    assert_eq!(CALL_COUNT.load(Ordering::SeqCst), 4);
}

// ============================================================================
// RetryConfig parameter tests
// ============================================================================

/// Mock RetryConfig for testing (matches rustboot_resilience::RetryConfig)
#[derive(Debug, Clone)]
struct RetryConfig {
    pub max_attempts: usize,
    pub initial_delay_ms: u64,
    pub max_delay_ms: u64,
}

impl RetryConfig {
    fn new(max_attempts: usize, initial_delay_ms: u64, max_delay_ms: u64) -> Self {
        Self { max_attempts, initial_delay_ms, max_delay_ms }
    }
}

#[tokio::test]
async fn test_retry_config_param_basic() {
    static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

    #[retry(config_param = "config", retryable = true)]
    async fn retry_with_config(config: &RetryConfig) -> Result<i32, TestError> {
        CALL_COUNT.fetch_add(1, Ordering::SeqCst);
        Err(TestError::Retryable { retry_after_ms: None })
    }

    // Test with 2 attempts
    let config = RetryConfig::new(2, 1, 100);
    CALL_COUNT.store(0, Ordering::SeqCst);
    let result = retry_with_config(&config).await;
    assert!(result.is_err());
    assert_eq!(CALL_COUNT.load(Ordering::SeqCst), 2);

    // Test with 5 attempts
    let config = RetryConfig::new(5, 1, 100);
    CALL_COUNT.store(0, Ordering::SeqCst);
    let result = retry_with_config(&config).await;
    assert!(result.is_err());
    assert_eq!(CALL_COUNT.load(Ordering::SeqCst), 5);
}

#[tokio::test]
async fn test_retry_config_param_success() {
    static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

    #[retry(config_param = "config", retryable = true)]
    async fn retry_succeeds(config: &RetryConfig) -> Result<i32, TestError> {
        let count = CALL_COUNT.fetch_add(1, Ordering::SeqCst) + 1;
        if count < 3 {
            Err(TestError::Retryable { retry_after_ms: None })
        } else {
            Ok(42)
        }
    }

    let config = RetryConfig::new(5, 1, 100);
    CALL_COUNT.store(0, Ordering::SeqCst);
    let result = retry_succeeds(&config).await;
    assert_eq!(result.unwrap(), 42);
    assert_eq!(CALL_COUNT.load(Ordering::SeqCst), 3);
}
