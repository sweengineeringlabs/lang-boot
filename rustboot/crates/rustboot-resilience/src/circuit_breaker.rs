//! Circuit breaker pattern

use crate::error::{ResilienceError, ResilienceResult};
use std::future::Future;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Circuit breaker state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    /// Circuit is closed, requests flow through
    Closed,
    /// Circuit is open, requests fail fast
    Open,
    /// Circuit is half-open, testing if service recovered
    HalfOpen,
}

/// Circuit breaker configuration
pub struct CircuitBreakerConfig {
    /// Number of failures before opening circuit
    pub failure_threshold: usize,
    /// Duration to wait before trying again
    pub timeout: Duration,
    /// Number of successes needed to close circuit
    pub success_threshold: usize,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            timeout: Duration::from_secs(60),
            success_threshold: 2,
        }
    }
}

struct CircuitBreakerState {
    state: CircuitState,
    failure_count: usize,
    success_count: usize,
    last_failure_time: Option<Instant>,
}

/// Circuit breaker for fault tolerance
pub struct CircuitBreaker {
    config: CircuitBreakerConfig,
    state: Arc<RwLock<CircuitBreakerState>>,
}

impl CircuitBreaker {
    /// Create a new circuit breaker
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            config,
            state: Arc::new(RwLock::new(CircuitBreakerState {
                state: CircuitState::Closed,
                failure_count: 0,
                success_count: 0,
                last_failure_time: None,
            })),
        }
    }

    /// Get current circuit state
    pub async fn state(&self) -> CircuitState {
        self.state.read().await.state
    }

    /// Execute an operation through the circuit breaker
    pub async fn execute<F, Fut, T, E>(&self, operation: F) -> ResilienceResult<T>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<T, E>>,
        E: std::fmt::Display,
    {
        // Check if we should attempt the operation
        {
            let mut state = self.state.write().await;
            
            match state.state {
                CircuitState::Open => {
                    // Check if timeout has passed
                    if let Some(last_failure) = state.last_failure_time {
                        if last_failure.elapsed() >= self.config.timeout {
                            state.state = CircuitState::HalfOpen;
                            state.success_count = 0;
                        } else {
                            return Err(ResilienceError::CircuitOpen);
                        }
                    }
                }
                CircuitState::Closed | CircuitState::HalfOpen => {
                    // Proceed with operation
                }
            }
        }

        // Execute the operation
        match operation().await {
            Ok(result) => {
                self.on_success().await;
                Ok(result)
            }
            Err(e) => {
                self.on_failure().await;
                Err(ResilienceError::OperationFailed(e.to_string()))
            }
        }
    }

    async fn on_success(&self) {
        let mut state = self.state.write().await;
        
        match state.state {
            CircuitState::HalfOpen => {
                state.success_count += 1;
                if state.success_count >= self.config.success_threshold {
                    state.state = CircuitState::Closed;
                    state.failure_count = 0;
                    state.success_count = 0;
                }
            }
            CircuitState::Closed => {
                state.failure_count = 0;
            }
            CircuitState::Open => {}
        }
    }

    async fn on_failure(&self) {
        let mut state = self.state.write().await;
        
        state.failure_count += 1;
        state.last_failure_time = Some(Instant::now());

        match state.state {
            CircuitState::Closed => {
                if state.failure_count >= self.config.failure_threshold {
                    state.state = CircuitState::Open;
                }
            }
            CircuitState::HalfOpen => {
                state.state = CircuitState::Open;
                state.success_count = 0;
            }
            CircuitState::Open => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_circuit_breaker_closed_to_open() {
        let config = CircuitBreakerConfig {
            failure_threshold: 3,
            timeout: Duration::from_millis(100),
            success_threshold: 2,
        };
        let cb = CircuitBreaker::new(config);

        // Should be closed initially
        assert_eq!(cb.state().await, CircuitState::Closed);

        // Trigger failures
        for _ in 0..3 {
            let _ = cb.execute(|| async { Err::<(), _>("error") }).await;
        }

        // Should be open now
        assert_eq!(cb.state().await, CircuitState::Open);
    }

    #[tokio::test]
    async fn test_circuit_breaker_half_open_recovery() {
        let config = CircuitBreakerConfig {
            failure_threshold: 2,
            timeout: Duration::from_millis(50),
            success_threshold: 2,
        };
        let cb = CircuitBreaker::new(config);

        // Open the circuit
        for _ in 0..2 {
            let _ = cb.execute(|| async { Err::<(), _>("error") }).await;
        }
        assert_eq!(cb.state().await, CircuitState::Open);

        // Wait for timeout
        tokio::time::sleep(Duration::from_millis(60)).await;

        // Should transition to half-open and allow through
        let _ = cb.execute(|| async { Ok::<_, String>(()) }).await;
        assert_eq!(cb.state().await, CircuitState::HalfOpen);

        // Another success should close it
        let _ = cb.execute(|| async { Ok::<_, String>(()) }).await;
        assert_eq!(cb.state().await, CircuitState::Closed);
    }
}
