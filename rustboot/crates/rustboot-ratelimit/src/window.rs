//! Window-based rate limiters

use crate::error::{RateLimitError, RateLimitResult};
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

/// Fixed window counter
pub struct FixedWindow {
    state: Arc<Mutex<WindowState>>,
    max_requests: usize,
    window_size: Duration,
}

struct WindowState {
    count: usize,
    window_start: Instant,
}

impl FixedWindow {
    /// Create a new fixed window rate limiter
    pub fn new(max_requests: usize, window_size: Duration) -> Self {
        Self {
            state: Arc::new(Mutex::new(WindowState {
                count: 0,
                window_start: Instant::now(),
            })),
            max_requests,
            window_size,
        }
    }

    /// Try to acquire a slot in current window
    pub async fn try_acquire(&self) -> RateLimitResult<()> {
        let mut state = self.state.lock().await;
        
        let now = Instant::now();
        
        // Reset window if expired
        if now.duration_since(state.window_start) >= self.window_size {
            state.count = 0;
            state.window_start = now;
        }

        // Check if under limit
        if state.count < self.max_requests {
            state.count += 1;
            Ok(())
        } else {
            Err(RateLimitError::RateLimitExceeded)
        }
    }

    /// Get current window count
    pub async fn current_count(&self) -> usize {
        let state = self.state.lock().await;
        state.count
    }
}

/// Sliding window log
pub struct SlidingWindow {
    state: Arc<Mutex<SlidingState>>,
    max_requests: usize,
    window_size: Duration,
}

struct SlidingState {
    requests: VecDeque<Instant>,
}

impl SlidingWindow {
    /// Create a new sliding window rate limiter
    pub fn new(max_requests: usize, window_size: Duration) -> Self {
        Self {
            state: Arc::new(Mutex::new(SlidingState {
                requests: VecDeque::new(),
            })),
            max_requests,
            window_size,
        }
    }

    /// Try to acquire a slot in sliding window
    pub async fn try_acquire(&self) -> RateLimitResult<()> {
        let mut state = self.state.lock().await;
        
        let now = Instant::now();
        
        // Remove expired requests
        while let Some(oldest) = state.requests.front() {
            if now.duration_since(*oldest) >= self.window_size {
                state.requests.pop_front();
            } else {
                break;
            }
        }

        // Check if under limit
        if state.requests.len() < self.max_requests {
            state.requests.push_back(now);
            Ok(())
        } else {
            Err(RateLimitError::RateLimitExceeded)
        }
    }

    /// Get current request count in window
    pub async fn current_count(&self) -> usize {
        let state = self.state.lock().await;
        state.requests.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fixed_window_basic() {
        let limiter = FixedWindow::new(5, Duration::from_secs(1));

        // Should accept 5 requests
        for _ in 0..5 {
            assert!(limiter.try_acquire().await.is_ok());
        }

        // Should reject 6th
        assert!(limiter.try_acquire().await.is_err());
    }

    #[tokio::test]
    async fn test_fixed_window_reset() {
        let limiter = FixedWindow::new(3, Duration::from_millis(100));

        // Fill window
        for _ in 0..3 {
            limiter.try_acquire().await.unwrap();
        }

        // Wait for window reset
        tokio::time::sleep(Duration::from_millis(110)).await;

        // Should accept new requests
        assert!(limiter.try_acquire().await.is_ok());
    }

    #[tokio::test]
    async fn test_sliding_window_basic() {
        let limiter = SlidingWindow::new(5, Duration::from_secs(1));

        // Should accept 5 requests
        for _ in 0..5 {
            assert!(limiter.try_acquire().await.is_ok());
        }

        // Should reject 6th
        assert!(limiter.try_acquire().await.is_err());
    }

    #[tokio::test]
    async fn test_sliding_window_expiry() {
        let limiter = SlidingWindow::new(3, Duration::from_millis(100));

        // Add requests
        limiter.try_acquire().await.unwrap();
        tokio::time::sleep(Duration::from_millis(50)).await;
        limiter.try_acquire().await.unwrap();
        limiter.try_acquire().await.unwrap();

        // Window full
        assert!(limiter.try_acquire().await.is_err());

        // Wait for first request to expire
        tokio::time::sleep(Duration::from_millis(60)).await;

        // Should have room now
        assert!(limiter.try_acquire().await.is_ok());
    }
}
