//! Leaky bucket rate limiter

use crate::error::{RateLimitError, RateLimitResult};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

/// Leaky bucket rate limiter
pub struct LeakyBucket {
    state: Arc<Mutex<BucketState>>,
    capacity: usize,
    leak_rate: usize,
    leak_interval: Duration,
}

struct BucketState {
    level: usize,
    last_leak: Instant,
}

impl LeakyBucket {
    /// Create a new leaky bucket
    ///
    /// # Arguments
    /// * `capacity` - Maximum requests in bucket
    /// * `leak_rate` - Requests leaked per interval
    /// * `leak_interval` - Time between leaks
    pub fn new(capacity: usize, leak_rate: usize, leak_interval: Duration) -> Self {
        Self {
            state: Arc::new(Mutex::new(BucketState {
                level: 0,
                last_leak: Instant::now(),
            })),
            capacity,
            leak_rate,
            leak_interval,
        }
    }

    /// Try to add a request to the bucket
    pub async fn try_acquire(&self) -> RateLimitResult<()> {
        let mut state = self.state.lock().await;
        
        // Leak based on elapsed time
        let now = Instant::now();
        let elapsed = now.duration_since(state.last_leak);
        let leaks = (elapsed.as_millis() / self.leak_interval.as_millis()) as usize;
        
        if leaks > 0 {
            state.level = state.level.saturating_sub(leaks * self.leak_rate);
            state.last_leak = now;
        }

        // Try to add request
        if state.level < self.capacity {
            state.level += 1;
            Ok(())
        } else {
            Err(RateLimitError::RateLimitExceeded)
        }
    }

    /// Get current bucket level (after applying any pending leaks)
    pub async fn current_level(&self) -> usize {
        let mut state = self.state.lock().await;

        // Apply leak based on elapsed time
        let now = Instant::now();
        let elapsed = now.duration_since(state.last_leak);
        let leaks = (elapsed.as_millis() / self.leak_interval.as_millis()) as usize;

        if leaks > 0 {
            state.level = state.level.saturating_sub(leaks * self.leak_rate);
            state.last_leak = now;
        }

        state.level
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_leaky_bucket_basic() {
        let bucket = LeakyBucket::new(10, 2, Duration::from_millis(100));

        // Should accept initial requests
        for _ in 0..5 {
            assert!(bucket.try_acquire().await.is_ok());
        }

        assert_eq!(bucket.current_level().await, 5);
    }

    #[tokio::test]
    async fn test_leaky_bucket_overflow() {
        let bucket = LeakyBucket::new(5, 1, Duration::from_millis(100));

        // Fill bucket
        for _ in 0..5 {
            bucket.try_acquire().await.unwrap();
        }

        // Should reject when full
        assert!(bucket.try_acquire().await.is_err());
    }

    #[tokio::test]
    async fn test_leaky_bucket_leak() {
        let bucket = LeakyBucket::new(10, 3, Duration::from_millis(50));

        // Fill bucket
        for _ in 0..10 {
            bucket.try_acquire().await.unwrap();
        }

        // Wait for leak - give extra time for timing sensitivity
        tokio::time::sleep(Duration::from_millis(120)).await;

        // Should have leaked at least 3 requests (after ~2 intervals)
        let level = bucket.current_level().await;
        assert!(level <= 7, "Expected level <= 7, got {}", level);
        assert!(bucket.try_acquire().await.is_ok());
    }
}
