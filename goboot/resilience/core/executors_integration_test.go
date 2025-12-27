package core

import (
	"context"
	"errors"
	"sync/atomic"
	"testing"
	"time"

	"dev.engineeringlabs/goboot/resilience/api"
)

// Integration tests combining multiple resilience patterns

func TestRetryWithCircuitBreaker(t *testing.T) {
	// Circuit breaker wrapping operations that fail
	cb := NewCircuitBreaker("test", api.CircuitBreakerConfig{
		FailureThreshold: 3,
		SuccessThreshold: 2,
		Timeout:          100 * time.Millisecond,
	})

	// Simulate failing service - always fails
	operation := func() error {
		return errors.New("service unavailable")
	}

	// Each call should fail and count as a circuit breaker failure
	for i := 0; i < 3; i++ {
		cb.Execute(context.Background(), operation)
	}

	// Circuit should be open now after 3 failures
	if cb.State() != api.CircuitOpen {
		t.Errorf("Circuit should be open, got %s", cb.State())
	}

	// Calls should be rejected immediately when open
	errOpen := cb.Execute(context.Background(), func() error { return nil })
	if errOpen == nil {
		t.Error("Expected error when circuit is open")
	}

	// Wait for circuit to transition to half-open
	time.Sleep(150 * time.Millisecond)

	// Should be in half-open, next success should help close it
	err := cb.Execute(context.Background(), func() error {
		return nil // Now succeeds
	})
	if err != nil {
		t.Errorf("Unexpected error after circuit half-open: %v", err)
	}
}

func TestBulkheadWithTimeout(t *testing.T) {
	bulkhead := NewBulkheadExecutor(api.BulkheadConfig{
		MaxConcurrent: 2,
		MaxWait:       50 * time.Millisecond,
	})

	timeout := NewTimeoutExecutor(api.TimeoutConfig{
		Duration: 100 * time.Millisecond,
	})

	// Execute with both patterns
	err := bulkhead.Execute(context.Background(), func() error {
		return timeout.Execute(context.Background(), func() error {
			time.Sleep(50 * time.Millisecond)
			return nil
		})
	})

	if err != nil {
		t.Errorf("Should succeed: %v", err)
	}

	// Test timeout
	err = bulkhead.Execute(context.Background(), func() error {
		return timeout.Execute(context.Background(), func() error {
			time.Sleep(200 * time.Millisecond)
			return nil
		})
	})

	var timeoutErr *TimeoutError
	if !errors.As(err, &timeoutErr) {
		t.Error("Expected TimeoutError")
	}
}

func TestRateLimiterUnderLoad(t *testing.T) {
	limiter := NewRateLimiter(api.RateLimitConfig{
		Calls:  10,
		Period: 100 * time.Millisecond,
	})

	allowed := 0
	blocked := 0

	// Make 20 requests rapidly
	for i := 0; i < 20; i++ {
		if limiter.Allow() {
			allowed++
		} else {
			blocked++
		}
	}

	if allowed != 10 {
		t.Errorf("Expected 10 allowed, got %d", allowed)
	}
	if blocked != 10 {
		t.Errorf("Expected 10 blocked, got %d", blocked)
	}

	// Wait for refill
	time.Sleep(150 * time.Millisecond)

	// Should allow more
	if !limiter.Allow() {
		t.Error("Should allow after refill")
	}
}

func TestRetryExecutor_WithResult(t *testing.T) {
	config := api.RetryConfig{
		MaxAttempts: 3,
		Delay:       10 * time.Millisecond,
		Backoff:     1.0,
		MaxDelay:    time.Second,
		Jitter:      0,
	}

	executor := NewRetryExecutor(config)
	attempts := 0

	result, err := executor.ExecuteWithResult(context.Background(), func() (any, error) {
		attempts++
		if attempts < 2 {
			return nil, errors.New("temporary failure")
		}
		return "success", nil
	})

	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
	if result != "success" {
		t.Errorf("Expected 'success', got %v", result)
	}
	if attempts != 2 {
		t.Errorf("Expected 2 attempts, got %d", attempts)
	}
}

func TestCircuitBreaker_ConcurrentAccess(t *testing.T) {
	cb := NewCircuitBreaker("concurrent", api.CircuitBreakerConfig{
		FailureThreshold: 5,
		SuccessThreshold: 2,
		Timeout:          time.Second,
	})

	var successCount int32
	var errorCount int32

	// Concurrent successful requests should not trip the circuit
	done := make(chan bool)
	for i := 0; i < 100; i++ {
		go func() {
			err := cb.Execute(context.Background(), func() error {
				return nil
			})
			if err != nil {
				atomic.AddInt32(&errorCount, 1)
			} else {
				atomic.AddInt32(&successCount, 1)
			}
			done <- true
		}()
	}

	// Wait for all goroutines
	for i := 0; i < 100; i++ {
		<-done
	}

	if successCount != 100 {
		t.Errorf("All requests should succeed, got %d successes, %d errors",
			successCount, errorCount)
	}

	// Circuit should remain closed
	if cb.State() != api.CircuitClosed {
		t.Errorf("Circuit should be closed, got %s", cb.State())
	}
}

func TestBulkhead_MaxConcurrent(t *testing.T) {
	executor := NewBulkheadExecutor(api.BulkheadConfig{
		MaxConcurrent: 3,
		MaxWait:       10 * time.Millisecond,
	})

	var concurrent int32
	var maxConcurrent int32
	started := make(chan bool, 10)
	finish := make(chan bool)

	// Start 5 operations, only 3 should run concurrently
	for i := 0; i < 5; i++ {
		go func() {
			err := executor.Execute(context.Background(), func() error {
				current := atomic.AddInt32(&concurrent, 1)
				for {
					old := atomic.LoadInt32(&maxConcurrent)
					if current <= old {
						break
					}
					if atomic.CompareAndSwapInt32(&maxConcurrent, old, current) {
						break
					}
				}
				started <- true
				<-finish
				atomic.AddInt32(&concurrent, -1)
				return nil
			})
			if err != nil {
				started <- true // Still signal even on error
			}
		}()
	}

	// Wait for first 3 to start
	for i := 0; i < 3; i++ {
		<-started
	}
	time.Sleep(20 * time.Millisecond)

	// Check max concurrent never exceeded 3
	if atomic.LoadInt32(&maxConcurrent) > 3 {
		t.Errorf("Max concurrent exceeded limit: %d", maxConcurrent)
	}

	// Release all
	close(finish)

	// Drain remaining signals
	for i := 0; i < 2; i++ {
		select {
		case <-started:
		case <-time.After(100 * time.Millisecond):
			// Some may have been rejected
		}
	}
}

func TestCircuitBreaker_StateTransitions(t *testing.T) {
	cb := NewCircuitBreaker("state-test", api.CircuitBreakerConfig{
		FailureThreshold: 2,
		SuccessThreshold: 2,
		Timeout:          50 * time.Millisecond,
	})

	// Start closed
	if cb.State() != api.CircuitClosed {
		t.Error("Should start closed")
	}

	// 2 failures -> open
	cb.Execute(context.Background(), func() error { return errors.New("fail1") })
	cb.Execute(context.Background(), func() error { return errors.New("fail2") })

	if cb.State() != api.CircuitOpen {
		t.Errorf("Should be open after failures, got %s", cb.State())
	}

	// Wait for timeout -> half-open
	time.Sleep(60 * time.Millisecond)

	// First success in half-open
	cb.Execute(context.Background(), func() error { return nil })

	// Need 2 successes to close
	cb.Execute(context.Background(), func() error { return nil })

	if cb.State() != api.CircuitClosed {
		t.Errorf("Should be closed after recovery successes, got %s", cb.State())
	}
}

// Benchmark tests

func BenchmarkRetryExecutor_SingleAttempt(b *testing.B) {
	executor := NewRetryExecutor(api.RetryConfig{
		MaxAttempts: 3,
		Delay:       time.Millisecond,
		Backoff:     1.0,
		MaxDelay:    time.Second,
	})

	ctx := context.Background()
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		executor.Execute(ctx, func() error { return nil })
	}
}

func BenchmarkCircuitBreaker_Closed(b *testing.B) {
	cb := NewCircuitBreaker("bench", api.CircuitBreakerConfig{
		FailureThreshold: 5,
		SuccessThreshold: 2,
		Timeout:          time.Second,
	})

	ctx := context.Background()
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		cb.Execute(ctx, func() error { return nil })
	}
}

func BenchmarkBulkheadExecutor(b *testing.B) {
	executor := NewBulkheadExecutor(api.BulkheadConfig{
		MaxConcurrent: 10,
		MaxWait:       100 * time.Millisecond,
	})

	ctx := context.Background()
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		executor.Execute(ctx, func() error { return nil })
	}
}

func BenchmarkRateLimiter_Allow(b *testing.B) {
	limiter := NewRateLimiter(api.RateLimitConfig{
		Calls:  1000000,
		Period: time.Second,
	})

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		limiter.Allow()
	}
}

func BenchmarkTimeoutExecutor(b *testing.B) {
	executor := NewTimeoutExecutor(api.TimeoutConfig{
		Duration: time.Second,
	})

	ctx := context.Background()
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		executor.Execute(ctx, func() error { return nil })
	}
}
