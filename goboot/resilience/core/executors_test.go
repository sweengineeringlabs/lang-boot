package core

import (
	"context"
	"errors"
	"sync/atomic"
	"testing"
	"time"

	"dev.engineeringlabs/goboot/resilience/api"
)

func TestRetryExecutor_Success(t *testing.T) {
	config := api.RetryConfig{
		MaxAttempts: 3,
		Delay:       10 * time.Millisecond,
		Backoff:     2.0,
		MaxDelay:    time.Second,
		Jitter:      0,
	}

	executor := NewRetryExecutor(config)
	attempts := 0

	err := executor.Execute(context.Background(), func() error {
		attempts++
		return nil
	})

	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
	if attempts != 1 {
		t.Errorf("Expected 1 attempt, got %d", attempts)
	}
}

func TestRetryExecutor_RetryOnFailure(t *testing.T) {
	config := api.RetryConfig{
		MaxAttempts: 3,
		Delay:       10 * time.Millisecond,
		Backoff:     1.0,
		MaxDelay:    time.Second,
		Jitter:      0,
	}

	executor := NewRetryExecutor(config)
	attempts := 0

	err := executor.Execute(context.Background(), func() error {
		attempts++
		if attempts < 3 {
			return errors.New("temporary failure")
		}
		return nil
	})

	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
	if attempts != 3 {
		t.Errorf("Expected 3 attempts, got %d", attempts)
	}
}

func TestRetryExecutor_Exhausted(t *testing.T) {
	config := api.RetryConfig{
		MaxAttempts: 3,
		Delay:       1 * time.Millisecond,
		Backoff:     1.0,
		MaxDelay:    time.Second,
		Jitter:      0,
	}

	executor := NewRetryExecutor(config)

	err := executor.Execute(context.Background(), func() error {
		return errors.New("always fails")
	})

	if err == nil {
		t.Error("Expected error")
	}

	var retryErr *RetryExhaustedError
	if !errors.As(err, &retryErr) {
		t.Error("Expected RetryExhaustedError")
	}
	if retryErr.Attempts != 3 {
		t.Errorf("Expected 3 attempts, got %d", retryErr.Attempts)
	}
}

func TestRetryExecutor_ContextCancelled(t *testing.T) {
	config := api.RetryConfig{
		MaxAttempts: 10,
		Delay:       100 * time.Millisecond,
		Backoff:     1.0,
		MaxDelay:    time.Second,
		Jitter:      0,
	}

	executor := NewRetryExecutor(config)
	ctx, cancel := context.WithTimeout(context.Background(), 50*time.Millisecond)
	defer cancel()

	err := executor.Execute(ctx, func() error {
		return errors.New("always fails")
	})

	if err == nil {
		t.Error("Expected error")
	}
}

func TestCircuitBreaker_ClosedState(t *testing.T) {
	cb := NewCircuitBreaker("test", api.CircuitBreakerConfig{
		FailureThreshold: 5,
		SuccessThreshold: 3,
		Timeout:          time.Second,
	})

	if cb.State() != api.CircuitClosed {
		t.Error("Initial state should be CLOSED")
	}

	err := cb.Execute(context.Background(), func() error {
		return nil
	})

	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
	if cb.State() != api.CircuitClosed {
		t.Error("State should remain CLOSED after success")
	}
}

func TestCircuitBreaker_OpensAfterFailures(t *testing.T) {
	cb := NewCircuitBreaker("test", api.CircuitBreakerConfig{
		FailureThreshold: 3,
		SuccessThreshold: 2,
		Timeout:          time.Second,
	})

	// Cause failures
	for i := 0; i < 3; i++ {
		cb.Execute(context.Background(), func() error {
			return errors.New("failure")
		})
	}

	if cb.State() != api.CircuitOpen {
		t.Errorf("State should be OPEN after %d failures, got %s", 3, cb.State())
	}

	// Verify requests are blocked
	err := cb.Execute(context.Background(), func() error {
		return nil
	})

	var openErr *CircuitOpenError
	if !errors.As(err, &openErr) {
		t.Error("Expected CircuitOpenError when circuit is open")
	}
}

func TestCircuitBreaker_HalfOpenAfterTimeout(t *testing.T) {
	cb := NewCircuitBreaker("test", api.CircuitBreakerConfig{
		FailureThreshold: 2,
		SuccessThreshold: 2,
		Timeout:          50 * time.Millisecond,
	})

	// Open the circuit
	for i := 0; i < 2; i++ {
		cb.Execute(context.Background(), func() error {
			return errors.New("failure")
		})
	}

	if cb.State() != api.CircuitOpen {
		t.Error("Should be OPEN")
	}

	// Wait for timeout
	time.Sleep(100 * time.Millisecond)

	// Next request should transition to half-open
	cb.Execute(context.Background(), func() error {
		return nil
	})

	// After success in half-open, should close eventually
	cb.Execute(context.Background(), func() error {
		return nil
	})

	if cb.State() != api.CircuitClosed {
		t.Errorf("Should be CLOSED after successes, got %s", cb.State())
	}
}

func TestTimeoutExecutor_Success(t *testing.T) {
	executor := NewTimeoutExecutor(api.TimeoutConfig{
		Duration: time.Second,
	})

	err := executor.Execute(context.Background(), func() error {
		time.Sleep(10 * time.Millisecond)
		return nil
	})

	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
}

func TestTimeoutExecutor_Timeout(t *testing.T) {
	executor := NewTimeoutExecutor(api.TimeoutConfig{
		Duration: 50 * time.Millisecond,
	})

	err := executor.Execute(context.Background(), func() error {
		time.Sleep(200 * time.Millisecond)
		return nil
	})

	var timeoutErr *TimeoutError
	if !errors.As(err, &timeoutErr) {
		t.Error("Expected TimeoutError")
	}
}

func TestBulkheadExecutor_AllowsConcurrent(t *testing.T) {
	executor := NewBulkheadExecutor(api.BulkheadConfig{
		MaxConcurrent: 2,
		MaxWait:       100 * time.Millisecond,
	})

	var concurrent int32
	done := make(chan bool, 2)

	for i := 0; i < 2; i++ {
		go func() {
			executor.Execute(context.Background(), func() error {
				atomic.AddInt32(&concurrent, 1)
				time.Sleep(50 * time.Millisecond)
				atomic.AddInt32(&concurrent, -1)
				return nil
			})
			done <- true
		}()
	}

	<-done
	<-done
}

func TestBulkheadExecutor_RejectsOverflow(t *testing.T) {
	executor := NewBulkheadExecutor(api.BulkheadConfig{
		MaxConcurrent: 1,
		MaxWait:       10 * time.Millisecond,
	})

	started := make(chan bool)
	done := make(chan bool)

	// Start first task
	go func() {
		executor.Execute(context.Background(), func() error {
			started <- true
			time.Sleep(500 * time.Millisecond)
			return nil
		})
		done <- true
	}()

	<-started // Wait for first task to start

	// Try second task
	err := executor.Execute(context.Background(), func() error {
		return nil
	})

	var bulkheadErr *BulkheadFullError
	if !errors.As(err, &bulkheadErr) {
		t.Error("Expected BulkheadFullError")
	}
}

func TestRateLimiter_AllowsWithinLimit(t *testing.T) {
	limiter := NewRateLimiter(api.RateLimitConfig{
		Calls:  10,
		Period: time.Second,
	})

	for i := 0; i < 10; i++ {
		if !limiter.Allow() {
			t.Errorf("Request %d should be allowed", i+1)
		}
	}
}

func TestRateLimiter_BlocksOverLimit(t *testing.T) {
	limiter := NewRateLimiter(api.RateLimitConfig{
		Calls:  5,
		Period: time.Second,
	})

	// Use all tokens
	for i := 0; i < 5; i++ {
		limiter.Allow()
	}

	// Next should be blocked
	if limiter.Allow() {
		t.Error("Should be rate limited")
	}
}

func TestRateLimiter_RefillsOverTime(t *testing.T) {
	limiter := NewRateLimiter(api.RateLimitConfig{
		Calls:  10,
		Period: 100 * time.Millisecond,
	})

	// Use all tokens
	for i := 0; i < 10; i++ {
		limiter.Allow()
	}

	// Wait for refill
	time.Sleep(150 * time.Millisecond)

	// Should have tokens again
	if !limiter.Allow() {
		t.Error("Should have refilled tokens")
	}
}
