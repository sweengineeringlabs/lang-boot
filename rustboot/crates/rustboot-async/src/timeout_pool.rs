//! Timeout pool for managing concurrent operations with timeouts

use std::future::Future;
use std::time::Duration;
use tokio::time::timeout;

/// Pool for executing multiple futures with a timeout
pub struct TimeoutPool {
    timeout_duration: Duration,
}

impl TimeoutPool {
    /// Create a new timeout pool
    pub fn new(timeout_duration: Duration) -> Self {
        Self { timeout_duration }
    }

    /// Execute a future with timeout
    pub async fn execute<F, T>(&self, future: F) -> Result<T, tokio::time::error::Elapsed>
    where
        F: Future<Output = T>,
    {
        timeout(self.timeout_duration, future).await
    }

    /// Execute multiple futures concurrently with timeout
    pub async fn execute_all<F, T>(&self, futures: Vec<F>) -> Vec<Result<T, tokio::time::error::Elapsed>>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        let timeout_duration = self.timeout_duration;
        let handles: Vec<_> = futures
            .into_iter()
            .map(|f| {
                tokio::spawn(async move {
                    timeout(timeout_duration, f).await
                })
            })
            .collect();

        let mut results = Vec::new();
        for handle in handles {
            match handle.await {
                Ok(result) => results.push(result),
                Err(_) => {
                    // Task panicked - treat as timeout by using a dummy timeout
                    let timeout_result: Result<T, _> = timeout(Duration::from_nanos(0), async {
                        // This will never complete
                        std::future::pending::<T>().await
                    }).await;
                    results.push(timeout_result);
                }
            }
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_timeout_pool_success() {
        let pool = TimeoutPool::new(Duration::from_secs(1));
        
        let result = pool.execute(async {
            tokio::time::sleep(Duration::from_millis(10)).await;
            42
        }).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[tokio::test]
    async fn test_timeout_pool_timeout() {
        let pool = TimeoutPool::new(Duration::from_millis(10));
        
        let result = pool.execute(async {
            tokio::time::sleep(Duration::from_secs(1)).await;
            42
        }).await;

        assert!(result.is_err());
    }
}
