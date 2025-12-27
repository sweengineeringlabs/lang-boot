// Package streams provides reactive stream utilities for the goboot framework.
//
// This module provides:
//   - API layer: Stream, Publisher, Subscriber, Observable interfaces
//   - Core layer: SliceStream, SimplePipeline, parallel processing
//
// Example:
//
//	import "dev.engineeringlabs/goboot/streams"
//
//	// Create a stream
//	result := streams.Of(1, 2, 3, 4, 5).
//	    Filter(func(n int) bool { return n%2 == 0 }).
//	    Map(func(n int) int { return n * 2 }).
//	    Collect()
//	// result = [4, 8]
//
//	// Range stream
//	sum := streams.Range(1, 101).
//	    Reduce(0, func(a, b int) int { return a + b })
//	// sum = 5050
//
//	// Parallel processing
//	results := streams.Of(items...).
//	    ParallelMap(processItem, 8).
//	    Collect()
package streams

import (
	"dev.engineeringlabs/goboot/streams/api"
	"dev.engineeringlabs/goboot/streams/core"
)

// Re-export API types
type (
	// Stream represents a data stream.
	Stream[T any] interface {
		api.Stream[T]
	}
	// Publisher publishes items.
	Publisher[T any] interface {
		api.Publisher[T]
	}
	// Subscriber receives items.
	Subscriber[T any] interface {
		api.Subscriber[T]
	}
	// Processor transforms items.
	Processor[T, R any] interface {
		api.Processor[T, R]
	}
	// Pipeline represents a processing pipeline.
	Pipeline[T, R any] interface {
		api.Pipeline[T, R]
	}
	// Observable represents an observable stream.
	Observable[T any] interface {
		api.Observable[T]
	}
	// Subscription represents a subscription.
	Subscription = api.Subscription
)

// Re-export Core types
type (
	// SliceStream is a Stream backed by a slice.
	SliceStream[T any] struct {
		*core.SliceStream[T]
	}
	// SimplePipeline is a simple pipeline.
	SimplePipeline[T, R any] struct {
		*core.SimplePipeline[T, R]
	}
)

// Of creates a new stream from values.
func Of[T any](values ...T) *core.SliceStream[T] {
	return core.Of(values...)
}

// FromSlice creates a stream from a slice.
func FromSlice[T any](slice []T) *core.SliceStream[T] {
	return core.FromSlice(slice)
}

// Range generates a stream of integers.
func Range(start, end int) *core.SliceStream[int] {
	return core.Range(start, end)
}

// Generate generates a stream using a function.
func Generate[T any](count int, generator func(int) T) *core.SliceStream[T] {
	return core.Generate(count, generator)
}

// Channel creates a stream from a channel.
func Channel[T any](ch <-chan T) *core.SliceStream[T] {
	return core.Channel(ch)
}

// NewPipeline creates a new pipeline.
func NewPipeline[T, R any]() *core.SimplePipeline[T, R] {
	return core.NewPipeline[T, R]()
}
