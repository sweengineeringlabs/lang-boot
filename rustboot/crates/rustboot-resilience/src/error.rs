//! Resilience error types

use thiserror::Error;

pub type ResilienceResult<T> = Result<T, ResilienceError>;

#[derive(Debug, Error)]
pub enum ResilienceError {
    #[error("Operation timed out after {0:?}")]
    Timeout(std::time::Duration),

    #[error("Max retries ({0}) exceeded")]
    MaxRetriesExceeded(usize),

    #[error("Circuit breaker is open")]
    CircuitOpen,

    #[error("Operation failed: {0}")]
    OperationFailed(String),
}
