// Package async provides async utilities for concurrent programming.
//
// This module provides utilities for:
//   - Goroutine management
//   - Worker pools
//   - Rate-limited execution
//   - Timeout handling
//   - Error aggregation
//
// Example:
//
//	// Parallel execution
//	results := async.Parallel(
//	    func() (int, error) { return fetchUser(1) },
//	    func() (int, error) { return fetchUser(2) },
//	    func() (int, error) { return fetchUser(3) },
//	)
//
//	// With worker pool
//	pool := async.NewWorkerPool(10)
//	for _, id := range userIds {
//	    pool.Submit(func() { processUser(id) })
//	}
//	pool.Wait()
package async

import (
	"context"
	"sync"
	"time"
)

// Result represents an async operation result.
type Result[T any] struct {
	Value T
	Err   error
}

// Parallel executes functions in parallel and returns all results.
func Parallel[T any](fns ...func() (T, error)) []Result[T] {
	results := make([]Result[T], len(fns))
	var wg sync.WaitGroup
	
	for i, fn := range fns {
		wg.Add(1)
		go func(idx int, f func() (T, error)) {
			defer wg.Done()
			val, err := f()
			results[idx] = Result[T]{Value: val, Err: err}
		}(i, fn)
	}
	
	wg.Wait()
	return results
}

// WithTimeout executes a function with a timeout.
func WithTimeout[T any](timeout time.Duration, fn func(ctx context.Context) (T, error)) (T, error) {
	ctx, cancel := context.WithTimeout(context.Background(), timeout)
	defer cancel()
	
	resultCh := make(chan Result[T], 1)
	go func() {
		val, err := fn(ctx)
		resultCh <- Result[T]{Value: val, Err: err}
	}()
	
	select {
	case result := <-resultCh:
		return result.Value, result.Err
	case <-ctx.Done():
		var zero T
		return zero, ctx.Err()
	}
}

// WorkerPool manages a pool of workers.
type WorkerPool struct {
	tasks   chan func()
	wg      sync.WaitGroup
	workers int
}

// NewWorkerPool creates a new worker pool.
func NewWorkerPool(workers int) *WorkerPool {
	pool := &WorkerPool{
		tasks:   make(chan func(), workers*10),
		workers: workers,
	}
	
	for i := 0; i < workers; i++ {
		go pool.worker()
	}
	
	return pool
}

func (p *WorkerPool) worker() {
	for task := range p.tasks {
		task()
		p.wg.Done()
	}
}

// Submit adds a task to the pool.
func (p *WorkerPool) Submit(task func()) {
	p.wg.Add(1)
	p.tasks <- task
}

// Wait waits for all tasks to complete.
func (p *WorkerPool) Wait() {
	p.wg.Wait()
}

// Close shuts down the worker pool.
func (p *WorkerPool) Close() {
	close(p.tasks)
}

// RateLimiter limits concurrent executions.
type RateLimiter struct {
	sem chan struct{}
}

// NewRateLimiter creates a new rate limiter.
func NewRateLimiter(maxConcurrent int) *RateLimiter {
	return &RateLimiter{
		sem: make(chan struct{}, maxConcurrent),
	}
}

// Execute runs a function with rate limiting.
func (r *RateLimiter) Execute(fn func()) {
	r.sem <- struct{}{}
	defer func() { <-r.sem }()
	fn()
}

// Map applies a function to each item in parallel.
func Map[T, R any](items []T, fn func(T) R) []R {
	results := make([]R, len(items))
	var wg sync.WaitGroup
	
	for i, item := range items {
		wg.Add(1)
		go func(idx int, it T) {
			defer wg.Done()
			results[idx] = fn(it)
		}(i, item)
	}
	
	wg.Wait()
	return results
}

// ForEach applies a function to each item in parallel.
func ForEach[T any](items []T, fn func(T)) {
	var wg sync.WaitGroup
	
	for _, item := range items {
		wg.Add(1)
		go func(it T) {
			defer wg.Done()
			fn(it)
		}(item)
	}
	
	wg.Wait()
}

// First returns the first result that completes successfully.
func First[T any](fns ...func() (T, error)) (T, error) {
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()
	
	resultCh := make(chan Result[T], len(fns))
	
	for _, fn := range fns {
		go func(f func() (T, error)) {
			select {
			case <-ctx.Done():
				return
			default:
				val, err := f()
				if err == nil {
					resultCh <- Result[T]{Value: val}
					cancel()
				}
			}
		}(fn)
	}
	
	result := <-resultCh
	return result.Value, result.Err
}
