// Package api contains the public interfaces and types for the streams module.
package api

import (
	"context"
)

// Stream represents a data stream.
type Stream[T any] interface {
	// Map transforms elements.
	Map(fn func(T) T) Stream[T]

	// Filter filters elements.
	Filter(fn func(T) bool) Stream[T]

	// ForEach executes a function for each element.
	ForEach(fn func(T))

	// Collect collects all elements into a slice.
	Collect() []T

	// First returns the first element.
	First() (T, bool)

	// Count returns the number of elements.
	Count() int

	// Reduce reduces elements to a single value.
	Reduce(initial T, fn func(T, T) T) T

	// Take limits to n elements.
	Take(n int) Stream[T]

	// Skip skips n elements.
	Skip(n int) Stream[T]

	// Any returns true if any element matches.
	Any(fn func(T) bool) bool

	// All returns true if all elements match.
	All(fn func(T) bool) bool
}

// Publisher publishes items to subscribers.
type Publisher[T any] interface {
	// Subscribe subscribes to the publisher.
	Subscribe(subscriber Subscriber[T])

	// Unsubscribe unsubscribes.
	Unsubscribe(subscriber Subscriber[T])
}

// Subscriber receives items from a publisher.
type Subscriber[T any] interface {
	// OnNext is called for each item.
	OnNext(item T)

	// OnError is called on error.
	OnError(err error)

	// OnComplete is called when complete.
	OnComplete()
}

// Processor transforms items in a pipeline.
type Processor[T, R any] interface {
	// Process processes an item.
	Process(ctx context.Context, item T) (R, error)
}

// Pipeline represents a processing pipeline.
type Pipeline[T, R any] interface {
	// Add adds a processor to the pipeline.
	Add(processor Processor[any, any]) Pipeline[T, R]

	// Execute executes the pipeline.
	Execute(ctx context.Context, input T) (R, error)

	// ExecuteBatch executes the pipeline for multiple inputs.
	ExecuteBatch(ctx context.Context, inputs []T) ([]R, error)
}

// Observable represents an observable stream.
type Observable[T any] interface {
	// Subscribe subscribes to the observable.
	Subscribe(onNext func(T), onError func(error), onComplete func()) Subscription

	// Map transforms elements.
	Map(fn func(T) T) Observable[T]

	// Filter filters elements.
	Filter(fn func(T) bool) Observable[T]

	// Take takes n elements.
	Take(n int) Observable[T]

	// Merge merges with another observable.
	Merge(other Observable[T]) Observable[T]
}

// Subscription represents a subscription.
type Subscription interface {
	// Unsubscribe unsubscribes.
	Unsubscribe()

	// IsUnsubscribed returns true if unsubscribed.
	IsUnsubscribed() bool
}
