//! Rustboot Resilience - Fault tolerance patterns
//!
//! Retry, circuit breaker, timeout, and bulkhead patterns for building
//! resilient applications.

pub mod circuit_breaker;
pub mod error;
pub mod retry;
pub mod timeout;

pub use circuit_breaker::{CircuitBreaker, CircuitBreakerConfig, CircuitState};
pub use error::{ResilienceError, ResilienceResult};
pub use retry::{ExponentialBackoff, RetryConfig, RetryPolicy};
pub use timeout::with_timeout;
