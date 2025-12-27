//go:build ignore

// Package main demonstrates the resilience module usage.
package main

import (
	"context"
	"errors"
	"fmt"
	"math/rand"
	"time"

	"dev.engineeringlabs/goboot/resilience"
)

func main() {
	fmt.Println("=== Goboot Resilience Module Example ===\n")
	ctx := context.Background()

	// Example 1: Retry with exponential backoff
	fmt.Println("1. Retry Pattern:")
	retryConfig := resilience.DefaultRetryConfig()
	retryConfig.MaxAttempts = 3
	retryConfig.Delay = 100 * time.Millisecond

	retryExec := resilience.NewRetryExecutor(retryConfig)

	attempts := 0
	err := retryExec.Execute(ctx, func() error {
		attempts++
		fmt.Printf("   Attempt %d\n", attempts)
		if attempts < 3 {
			return errors.New("temporary failure")
		}
		return nil
	})
	if err != nil {
		fmt.Printf("   Failed: %v\n", err)
	} else {
		fmt.Println("   Success!")
	}

	// Example 2: Circuit Breaker
	fmt.Println("\n2. Circuit Breaker Pattern:")
	cbConfig := resilience.DefaultCircuitBreakerConfig()
	cbConfig.FailureThreshold = 3
	cbConfig.Timeout = 1 * time.Second

	cb := resilience.NewCircuitBreaker("example-api", cbConfig)

	// Simulate failures
	for i := 0; i < 5; i++ {
		err := cb.Execute(ctx, func() error {
			if rand.Float32() < 0.7 {
				return errors.New("api error")
			}
			return nil
		})
		fmt.Printf("   Call %d: state=%s, err=%v\n", i+1, cb.State().String(), err)
	}

	// Example 3: Timeout
	fmt.Println("\n3. Timeout Pattern:")
	timeoutConfig := resilience.TimeoutConfig{
		Duration: 500 * time.Millisecond,
	}
	timeoutExec := resilience.NewTimeoutExecutor(timeoutConfig)

	err = timeoutExec.Execute(ctx, func() error {
		time.Sleep(100 * time.Millisecond)
		return nil
	})
	fmt.Printf("   Fast operation: %v\n", err)

	err = timeoutExec.Execute(ctx, func() error {
		time.Sleep(1 * time.Second)
		return nil
	})
	fmt.Printf("   Slow operation: %v\n", err)

	// Example 4: Rate Limiter
	fmt.Println("\n4. Rate Limiter Pattern:")
	rlConfig := resilience.RateLimitConfig{
		Calls:  5,
		Period: time.Second,
	}
	rl := resilience.NewRateLimiter(rlConfig)

	for i := 0; i < 8; i++ {
		allowed := rl.Allow()
		fmt.Printf("   Request %d: allowed=%v\n", i+1, allowed)
	}

	// Example 5: Bulkhead
	fmt.Println("\n5. Bulkhead Pattern:")
	bhConfig := resilience.BulkheadConfig{
		MaxConcurrent: 2,
		MaxWait:       100 * time.Millisecond,
	}
	bh := resilience.NewBulkheadExecutor(bhConfig)

	for i := 0; i < 4; i++ {
		go func(id int) {
			err := bh.Execute(ctx, func() error {
				fmt.Printf("   Task %d executing\n", id)
				time.Sleep(200 * time.Millisecond)
				return nil
			})
			if err != nil {
				fmt.Printf("   Task %d rejected: %v\n", id, err)
			}
		}(i)
	}
	time.Sleep(500 * time.Millisecond)
}
