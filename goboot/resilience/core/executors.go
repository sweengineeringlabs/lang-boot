// Package core contains the implementation details for the resilience module.
package core

import (
	"context"
	"fmt"
	"math/rand"
	"sync"
	"time"

	"dev.engineeringlabs/goboot/resilience/api"
)

// RetryExecutor executes operations with retry logic.
type RetryExecutor struct {
	config api.RetryConfig
}

// NewRetryExecutor creates a new RetryExecutor.
func NewRetryExecutor(config api.RetryConfig) *RetryExecutor {
	return &RetryExecutor{config: config}
}

// Execute runs the operation with retries.
func (r *RetryExecutor) Execute(ctx context.Context, operation func() error) error {
	var lastErr error
	delay := r.config.Delay

	for attempt := 1; attempt <= r.config.MaxAttempts; attempt++ {
		if err := ctx.Err(); err != nil {
			return fmt.Errorf("context cancelled: %w", err)
		}

		lastErr = operation()
		if lastErr == nil {
			return nil
		}

		if attempt < r.config.MaxAttempts {
			// Calculate delay with jitter
			jitter := time.Duration(float64(delay) * r.config.Jitter * (rand.Float64()*2 - 1))
			sleepDuration := delay + jitter

			select {
			case <-time.After(sleepDuration):
			case <-ctx.Done():
				return fmt.Errorf("context cancelled during retry: %w", ctx.Err())
			}

			// Apply backoff
			delay = time.Duration(float64(delay) * r.config.Backoff)
			if delay > r.config.MaxDelay {
				delay = r.config.MaxDelay
			}
		}
	}

	return &RetryExhaustedError{
		Attempts: r.config.MaxAttempts,
		LastErr:  lastErr,
	}
}

// ExecuteWithResult runs the operation with retries and returns a result.
func (r *RetryExecutor) ExecuteWithResult(ctx context.Context, operation func() (any, error)) (any, error) {
	var result any
	var lastErr error
	delay := r.config.Delay

	for attempt := 1; attempt <= r.config.MaxAttempts; attempt++ {
		if err := ctx.Err(); err != nil {
			return nil, fmt.Errorf("context cancelled: %w", err)
		}

		result, lastErr = operation()
		if lastErr == nil {
			return result, nil
		}

		if attempt < r.config.MaxAttempts {
			jitter := time.Duration(float64(delay) * r.config.Jitter * (rand.Float64()*2 - 1))
			sleepDuration := delay + jitter

			select {
			case <-time.After(sleepDuration):
			case <-ctx.Done():
				return nil, fmt.Errorf("context cancelled during retry: %w", ctx.Err())
			}

			delay = time.Duration(float64(delay) * r.config.Backoff)
			if delay > r.config.MaxDelay {
				delay = r.config.MaxDelay
			}
		}
	}

	return nil, &RetryExhaustedError{
		Attempts: r.config.MaxAttempts,
		LastErr:  lastErr,
	}
}

// RetryExhaustedError is returned when all retry attempts are exhausted.
type RetryExhaustedError struct {
	Attempts int
	LastErr  error
}

func (e *RetryExhaustedError) Error() string {
	return fmt.Sprintf("retry exhausted after %d attempts: %v", e.Attempts, e.LastErr)
}

func (e *RetryExhaustedError) Unwrap() error {
	return e.LastErr
}

// CircuitBreaker implements the circuit breaker pattern.
type CircuitBreaker struct {
	name        string
	config      api.CircuitBreakerConfig
	state       api.CircuitState
	failures    int
	successes   int
	lastFailure time.Time
	mu          sync.RWMutex
}

// NewCircuitBreaker creates a new CircuitBreaker.
func NewCircuitBreaker(name string, config api.CircuitBreakerConfig) *CircuitBreaker {
	return &CircuitBreaker{
		name:   name,
		config: config,
		state:  api.CircuitClosed,
	}
}

// Name returns the circuit breaker name.
func (cb *CircuitBreaker) Name() string {
	return cb.name
}

// State returns the current circuit state.
func (cb *CircuitBreaker) State() api.CircuitState {
	cb.mu.RLock()
	defer cb.mu.RUnlock()
	return cb.state
}

// Execute runs the operation through the circuit breaker.
func (cb *CircuitBreaker) Execute(ctx context.Context, operation func() error) error {
	if !cb.allowRequest() {
		return &CircuitOpenError{Name: cb.name}
	}

	err := operation()
	cb.recordResult(err)
	return err
}

// ExecuteWithResult runs the operation and returns a result.
func (cb *CircuitBreaker) ExecuteWithResult(ctx context.Context, operation func() (any, error)) (any, error) {
	if !cb.allowRequest() {
		return nil, &CircuitOpenError{Name: cb.name}
	}

	result, err := operation()
	cb.recordResult(err)
	return result, err
}

func (cb *CircuitBreaker) allowRequest() bool {
	cb.mu.Lock()
	defer cb.mu.Unlock()

	switch cb.state {
	case api.CircuitClosed:
		return true
	case api.CircuitOpen:
		if time.Since(cb.lastFailure) >= cb.config.Timeout {
			cb.state = api.CircuitHalfOpen
			cb.successes = 0
			return true
		}
		return false
	case api.CircuitHalfOpen:
		return true
	}
	return false
}

func (cb *CircuitBreaker) recordResult(err error) {
	cb.mu.Lock()
	defer cb.mu.Unlock()

	if err != nil {
		cb.failures++
		cb.lastFailure = time.Now()
		cb.successes = 0

		if cb.failures >= cb.config.FailureThreshold {
			cb.state = api.CircuitOpen
		}
	} else {
		if cb.state == api.CircuitHalfOpen {
			cb.successes++
			if cb.successes >= cb.config.SuccessThreshold {
				cb.state = api.CircuitClosed
				cb.failures = 0
			}
		} else {
			cb.failures = 0
		}
	}
}

// CircuitOpenError is returned when the circuit is open.
type CircuitOpenError struct {
	Name string
}

func (e *CircuitOpenError) Error() string {
	return fmt.Sprintf("circuit breaker '%s' is open", e.Name)
}

// TimeoutExecutor executes operations with a timeout.
type TimeoutExecutor struct {
	config api.TimeoutConfig
}

// NewTimeoutExecutor creates a new TimeoutExecutor.
func NewTimeoutExecutor(config api.TimeoutConfig) *TimeoutExecutor {
	return &TimeoutExecutor{config: config}
}

// Execute runs the operation with a timeout.
func (t *TimeoutExecutor) Execute(ctx context.Context, operation func() error) error {
	ctx, cancel := context.WithTimeout(ctx, t.config.Duration)
	defer cancel()

	done := make(chan error, 1)
	go func() {
		done <- operation()
	}()

	select {
	case err := <-done:
		return err
	case <-ctx.Done():
		return &TimeoutError{Duration: t.config.Duration}
	}
}

// TimeoutError is returned when an operation times out.
type TimeoutError struct {
	Duration time.Duration
}

func (e *TimeoutError) Error() string {
	return fmt.Sprintf("operation timed out after %v", e.Duration)
}

// BulkheadExecutor limits concurrent executions.
type BulkheadExecutor struct {
	config api.BulkheadConfig
	sem    chan struct{}
}

// NewBulkheadExecutor creates a new BulkheadExecutor.
func NewBulkheadExecutor(config api.BulkheadConfig) *BulkheadExecutor {
	return &BulkheadExecutor{
		config: config,
		sem:    make(chan struct{}, config.MaxConcurrent),
	}
}

// Execute runs the operation with concurrency limiting.
func (b *BulkheadExecutor) Execute(ctx context.Context, operation func() error) error {
	select {
	case b.sem <- struct{}{}:
		defer func() { <-b.sem }()
		return operation()
	case <-time.After(b.config.MaxWait):
		return &BulkheadFullError{MaxConcurrent: b.config.MaxConcurrent}
	case <-ctx.Done():
		return ctx.Err()
	}
}

// BulkheadFullError is returned when the bulkhead is at capacity.
type BulkheadFullError struct {
	MaxConcurrent int
}

func (e *BulkheadFullError) Error() string {
	return fmt.Sprintf("bulkhead at capacity (max %d concurrent)", e.MaxConcurrent)
}

// RateLimiter implements token bucket rate limiting.
type RateLimiter struct {
	config      api.RateLimitConfig
	tokens      float64
	lastRefresh time.Time
	mu          sync.Mutex
}

// NewRateLimiter creates a new RateLimiter.
func NewRateLimiter(config api.RateLimitConfig) *RateLimiter {
	return &RateLimiter{
		config:      config,
		tokens:      float64(config.Calls),
		lastRefresh: time.Now(),
	}
}

// Allow checks if a request is allowed and consumes a token.
func (r *RateLimiter) Allow() bool {
	r.mu.Lock()
	defer r.mu.Unlock()

	now := time.Now()
	elapsed := now.Sub(r.lastRefresh)
	r.lastRefresh = now

	// Refill tokens
	refillRate := float64(r.config.Calls) / r.config.Period.Seconds()
	r.tokens += elapsed.Seconds() * refillRate
	if r.tokens > float64(r.config.Calls) {
		r.tokens = float64(r.config.Calls)
	}

	if r.tokens >= 1 {
		r.tokens--
		return true
	}
	return false
}

// Execute runs the operation if rate limit allows.
func (r *RateLimiter) Execute(ctx context.Context, operation func() error) error {
	if !r.Allow() {
		return &RateLimitExceededError{Limit: r.config.Calls, Period: r.config.Period}
	}
	return operation()
}

// RateLimitExceededError is returned when the rate limit is exceeded.
type RateLimitExceededError struct {
	Limit  int
	Period time.Duration
}

func (e *RateLimitExceededError) Error() string {
	return fmt.Sprintf("rate limit exceeded (%d calls per %v)", e.Limit, e.Period)
}
