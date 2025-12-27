// Package resilience provides fault tolerance patterns for the goboot framework.
//
// This module provides comprehensive resilience patterns for building
// robust, fault-tolerant applications:
//
// API Layer:
//   - Configuration models for all resilience patterns
//   - State models for runtime monitoring
//
// Core Layer:
//   - RetryExecutor: Exponential backoff with jitter
//   - CircuitBreaker: Three-state circuit breaker pattern
//   - TimeoutExecutor: Operation timeouts
//   - BulkheadExecutor: Concurrency limiting
//   - RateLimiter: Token bucket rate limiting
//
// Example:
//
//	import "dev.engineeringlabs/goboot/resilience"
//
//	// Create executors
//	retryExec := resilience.NewRetryExecutor(resilience.DefaultRetryConfig())
//	breaker := resilience.NewCircuitBreaker("api", resilience.DefaultCircuitBreakerConfig())
//
//	// Execute with retry
//	err := retryExec.Execute(ctx, func() error {
//	    return apiCall()
//	})
//
//	// Execute with circuit breaker
//	err = breaker.Execute(ctx, func() error {
//	    return apiCall()
//	})
package resilience

import (
	"dev.engineeringlabs/goboot/resilience/api"
	"dev.engineeringlabs/goboot/resilience/core"
)

// Re-export API types
type (
	// RetryConfig configures retry behavior.
	RetryConfig = api.RetryConfig
	// CircuitState represents the state of a circuit breaker.
	CircuitState = api.CircuitState
	// CircuitBreakerConfig configures circuit breaker behavior.
	CircuitBreakerConfig = api.CircuitBreakerConfig
	// TimeoutConfig configures timeout behavior.
	TimeoutConfig = api.TimeoutConfig
	// BulkheadConfig configures bulkhead behavior.
	BulkheadConfig = api.BulkheadConfig
	// RateLimitConfig configures rate limiting behavior.
	RateLimitConfig = api.RateLimitConfig
	// FallbackConfig configures fallback behavior.
	FallbackConfig = api.FallbackConfig
)

// Re-export API constants
const (
	CircuitClosed   = api.CircuitClosed
	CircuitOpen     = api.CircuitOpen
	CircuitHalfOpen = api.CircuitHalfOpen
)

// Re-export API functions
var (
	DefaultRetryConfig          = api.DefaultRetryConfig
	DefaultCircuitBreakerConfig = api.DefaultCircuitBreakerConfig
	DefaultTimeoutConfig        = api.DefaultTimeoutConfig
	DefaultBulkheadConfig       = api.DefaultBulkheadConfig
	DefaultRateLimitConfig      = api.DefaultRateLimitConfig
	DefaultFallbackConfig       = api.DefaultFallbackConfig
)

// Re-export Core types
type (
	// RetryExecutor executes operations with retry logic.
	RetryExecutor = core.RetryExecutor
	// CircuitBreaker implements the circuit breaker pattern.
	CircuitBreaker = core.CircuitBreaker
	// TimeoutExecutor executes operations with a timeout.
	TimeoutExecutor = core.TimeoutExecutor
	// BulkheadExecutor limits concurrent executions.
	BulkheadExecutor = core.BulkheadExecutor
	// RateLimiter implements token bucket rate limiting.
	RateLimiter = core.RateLimiter
)

// Re-export Core error types
type (
	// RetryExhaustedError is returned when all retry attempts are exhausted.
	RetryExhaustedError = core.RetryExhaustedError
	// CircuitOpenError is returned when the circuit is open.
	CircuitOpenError = core.CircuitOpenError
	// TimeoutError is returned when an operation times out.
	TimeoutError = core.TimeoutError
	// BulkheadFullError is returned when the bulkhead is at capacity.
	BulkheadFullError = core.BulkheadFullError
	// RateLimitExceededError is returned when the rate limit is exceeded.
	RateLimitExceededError = core.RateLimitExceededError
)

// Re-export Core constructors
var (
	NewRetryExecutor    = core.NewRetryExecutor
	NewCircuitBreaker   = core.NewCircuitBreaker
	NewTimeoutExecutor  = core.NewTimeoutExecutor
	NewBulkheadExecutor = core.NewBulkheadExecutor
	NewRateLimiter      = core.NewRateLimiter
)
