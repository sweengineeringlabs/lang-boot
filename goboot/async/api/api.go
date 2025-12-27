// Package api defines the async module's public interfaces.
package api

import (
	"context"
	"time"
)

// Result represents an async operation result.
type Result[T any] struct {
	Value T
	Err   error
}

// Task represents an async task.
type Task[T any] interface {
	// Execute runs the task.
	Execute() (T, error)
	// ExecuteContext runs the task with context.
	ExecuteContext(ctx context.Context) (T, error)
}

// Executor executes tasks concurrently.
type Executor interface {
	// Submit adds a task to be executed.
	Submit(task func())
	// Wait waits for all tasks to complete.
	Wait()
	// Close shuts down the executor.
	Close()
}

// RateLimitedExecutor limits concurrent executions.
type RateLimitedExecutor interface {
	Executor
	// TryExecute attempts to execute without blocking.
	TryExecute(task func()) bool
}

// Pool represents a worker pool.
type Pool interface {
	Executor
	// Size returns the number of workers.
	Size() int
	// Active returns the number of active workers.
	Active() int
	// Pending returns the number of pending tasks.
	Pending() int
}

// Future represents a value that will be available in the future.
type Future[T any] interface {
	// Get blocks until the value is available.
	Get() (T, error)
	// GetTimeout blocks with a timeout.
	GetTimeout(timeout time.Duration) (T, error)
	// Ready returns true if the value is available.
	Ready() bool
	// Cancel cancels the computation.
	Cancel()
}

// Promise represents a writable future.
type Promise[T any] interface {
	Future[T]
	// Complete sets the result.
	Complete(value T)
	// Fail sets an error.
	Fail(err error)
}
