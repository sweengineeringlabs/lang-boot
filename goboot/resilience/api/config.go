// Package api contains the public interfaces and types for the resilience module.
package api

import "time"

// RetryConfig configures retry behavior.
type RetryConfig struct {
	// MaxAttempts is the maximum number of attempts (including the first).
	MaxAttempts int
	// Delay is the initial delay between retries.
	Delay time.Duration
	// Backoff is the multiplier for exponential backoff.
	Backoff float64
	// MaxDelay is the maximum delay between retries.
	MaxDelay time.Duration
	// Jitter adds randomness to delays (0.0 to 1.0).
	Jitter float64
}

// DefaultRetryConfig returns a default retry configuration.
func DefaultRetryConfig() RetryConfig {
	return RetryConfig{
		MaxAttempts: 3,
		Delay:       time.Second,
		Backoff:     2.0,
		MaxDelay:    30 * time.Second,
		Jitter:      0.1,
	}
}

// CircuitState represents the state of a circuit breaker.
type CircuitState int

const (
	// CircuitClosed allows requests through.
	CircuitClosed CircuitState = iota
	// CircuitOpen blocks requests.
	CircuitOpen
	// CircuitHalfOpen allows limited requests to test recovery.
	CircuitHalfOpen
)

// String returns the string representation of the circuit state.
func (s CircuitState) String() string {
	switch s {
	case CircuitClosed:
		return "CLOSED"
	case CircuitOpen:
		return "OPEN"
	case CircuitHalfOpen:
		return "HALF_OPEN"
	default:
		return "UNKNOWN"
	}
}

// CircuitBreakerConfig configures circuit breaker behavior.
type CircuitBreakerConfig struct {
	// FailureThreshold is the number of failures before opening.
	FailureThreshold int
	// SuccessThreshold is the number of successes to close from half-open.
	SuccessThreshold int
	// Timeout is the duration the circuit stays open before half-open.
	Timeout time.Duration
	// HalfOpenMaxCalls is the max concurrent calls in half-open state.
	HalfOpenMaxCalls int
}

// DefaultCircuitBreakerConfig returns a default circuit breaker configuration.
func DefaultCircuitBreakerConfig() CircuitBreakerConfig {
	return CircuitBreakerConfig{
		FailureThreshold: 5,
		SuccessThreshold: 3,
		Timeout:          30 * time.Second,
		HalfOpenMaxCalls: 1,
	}
}

// TimeoutConfig configures timeout behavior.
type TimeoutConfig struct {
	// Duration is the maximum duration to wait.
	Duration time.Duration
}

// DefaultTimeoutConfig returns a default timeout configuration.
func DefaultTimeoutConfig() TimeoutConfig {
	return TimeoutConfig{
		Duration: 10 * time.Second,
	}
}

// BulkheadConfig configures bulkhead (concurrency limiting) behavior.
type BulkheadConfig struct {
	// MaxConcurrent is the maximum concurrent executions.
	MaxConcurrent int
	// MaxWait is the maximum duration to wait for a slot.
	MaxWait time.Duration
}

// DefaultBulkheadConfig returns a default bulkhead configuration.
func DefaultBulkheadConfig() BulkheadConfig {
	return BulkheadConfig{
		MaxConcurrent: 10,
		MaxWait:       time.Second,
	}
}

// RateLimitConfig configures rate limiting behavior.
type RateLimitConfig struct {
	// Calls is the maximum number of calls allowed.
	Calls int
	// Period is the time period for the rate limit.
	Period time.Duration
}

// DefaultRateLimitConfig returns a default rate limit configuration.
func DefaultRateLimitConfig() RateLimitConfig {
	return RateLimitConfig{
		Calls:  100,
		Period: time.Minute,
	}
}

// FallbackConfig configures fallback behavior.
type FallbackConfig struct {
	// Enabled indicates whether fallback is enabled.
	Enabled bool
}

// DefaultFallbackConfig returns a default fallback configuration.
func DefaultFallbackConfig() FallbackConfig {
	return FallbackConfig{
		Enabled: true,
	}
}
