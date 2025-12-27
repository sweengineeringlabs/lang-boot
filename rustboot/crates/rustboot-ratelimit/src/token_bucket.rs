//! Token bucket rate limiter

use crate::error::{RateLimitError, RateLimitResult};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

/// Token bucket rate limiter
pub struct TokenBucket {
    state: Arc<Mutex<BucketState>>,
    capacity: usize,
    refill_rate: usize,
    refill_interval: Duration,
}

struct BucketState {
    tokens: usize,
    last_refill: Instant,
}

impl TokenBucket {
    /// Create a new token bucket
    ///
    /// # Arguments
    /// * `capacity` - Maximum tokens in bucket
    /// * `refill_rate` - Tokens added per interval
    /// * `refill_interval` - Time between refills
    pub fn new(capacity: usize, refill_rate: usize, refill_interval: Duration) -> Self {
        Self {
            state: Arc::new(Mutex::new(BucketState {
                tokens: capacity,
                last_refill: Instant::now(),
            })),
            capacity,
            refill_rate,
            refill_interval,
        }
    }

    /// Try to acquire a token
    pub async fn try_acquire(&self) -> RateLimitResult<()> {
        let mut state = self.state.lock().await;
        
        // Refill tokens based on elapsed time
        let now = Instant::now();
        let elapsed = now.duration_since(state.last_refill);
        let refills = (elapsed.as_millis() / self.refill_interval.as_millis()) as usize;
        
        if refills > 0 {
            state.tokens = (state.tokens + (refills * self.refill_rate)).min(self.capacity);
            state.last_refill = now;
        }

        // Try to consume a token
        if state.tokens > 0 {
            state.tokens -= 1;
            Ok(())
        } else {
            Err(RateLimitError::RateLimitExceeded)
        }
    }

    /// Acquire a token, waiting if necessary
    pub async fn acquire(&self) -> RateLimitResult<()> {
        loop {
            match self.try_acquire().await {
                Ok(()) => return Ok(()),
                Err(RateLimitError::RateLimitExceeded) => {
                    // Wait for next refill
                    tokio::time::sleep(self.refill_interval).await;
                }
                Err(e) => return Err(e),
            }
        }
    }

    /// Get current token count
    pub async fn available_tokens(&self) -> usize {
        let state = self.state.lock().await;
        state.tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_token_bucket_basic() {
        let bucket = TokenBucket::new(10, 5, Duration::from_millis(100));

        // Should have 10 tokens initially
        assert_eq!(bucket.available_tokens().await, 10);

        // Consume 5 tokens
        for _ in 0..5 {
            assert!(bucket.try_acquire().await.is_ok());
        }

        assert_eq!(bucket.available_tokens().await, 5);
    }

    #[tokio::test]
    async fn test_token_bucket_refill() {
        let bucket = TokenBucket::new(10, 5, Duration::from_millis(50));

        // Drain all tokens
        for _ in 0..10 {
            bucket.try_acquire().await.unwrap();
        }

        // No tokens left
        assert!(bucket.try_acquire().await.is_err());

        // Wait for refill
        tokio::time::sleep(Duration::from_millis(60)).await;

        // Should have 5 new tokens
        assert!(bucket.try_acquire().await.is_ok());
    }

    #[tokio::test]
    async fn test_token_bucket_max_capacity() {
        let bucket = TokenBucket::new(10, 20, Duration::from_millis(50));

        // Wait for multiple refills
        tokio::time::sleep(Duration::from_millis(200)).await;

        // Should not exceed capacity
        assert!(bucket.available_tokens().await <= 10);
    }
}
