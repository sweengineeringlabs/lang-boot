//! Timeout utilities

use crate::error::{ResilienceError, ResilienceResult};
use std::future::Future;
use std::time::Duration;

/// Execute an operation with a timeout
pub async fn with_timeout<F, T>(duration: Duration, operation: F) -> ResilienceResult<T>
where
    F: Future<Output = T>,
{
    tokio::time::timeout(duration, operation)
        .await
        .map_err(|_| ResilienceError::Timeout(duration))
}

/// Execute an operation with a timeout
pub async fn execute_with_timeout<F, T>(duration: Duration, operation: F) -> ResilienceResult<T>
where
    F: Future<Output = T>,
{
    match tokio::time::timeout(duration, operation).await {
        Ok(result) => Ok(result),
        Err(_) => Err(ResilienceError::Timeout(duration)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_timeout_success() {
        let result = with_timeout(Duration::from_millis(100), async {
            tokio::time::sleep(Duration::from_millis(10)).await;
            42
        })
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[tokio::test]
    async fn test_timeout_exceeded() {
        let result = with_timeout(Duration::from_millis(10), async {
            tokio::time::sleep(Duration::from_millis(100)).await;
            42
        })
        .await;

        assert!(result.is_err());
    }
}
