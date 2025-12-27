// Package core contains the implementation details for the streams module.
package core

import (
	"context"
	"sync"
)

// SliceStream is a Stream implementation backed by a slice.
type SliceStream[T any] struct {
	data []T
}

// Of creates a new SliceStream from values.
func Of[T any](values ...T) *SliceStream[T] {
	return &SliceStream[T]{data: values}
}

// FromSlice creates a new SliceStream from a slice.
func FromSlice[T any](slice []T) *SliceStream[T] {
	return &SliceStream[T]{data: slice}
}

// Map transforms elements.
func (s *SliceStream[T]) Map(fn func(T) T) *SliceStream[T] {
	result := make([]T, len(s.data))
	for i, v := range s.data {
		result[i] = fn(v)
	}
	return &SliceStream[T]{data: result}
}

// Filter filters elements.
func (s *SliceStream[T]) Filter(fn func(T) bool) *SliceStream[T] {
	result := make([]T, 0)
	for _, v := range s.data {
		if fn(v) {
			result = append(result, v)
		}
	}
	return &SliceStream[T]{data: result}
}

// ForEach executes a function for each element.
func (s *SliceStream[T]) ForEach(fn func(T)) {
	for _, v := range s.data {
		fn(v)
	}
}

// Collect collects all elements into a slice.
func (s *SliceStream[T]) Collect() []T {
	result := make([]T, len(s.data))
	copy(result, s.data)
	return result
}

// First returns the first element.
func (s *SliceStream[T]) First() (T, bool) {
	if len(s.data) == 0 {
		var zero T
		return zero, false
	}
	return s.data[0], true
}

// Count returns the number of elements.
func (s *SliceStream[T]) Count() int {
	return len(s.data)
}

// Reduce reduces elements to a single value.
func (s *SliceStream[T]) Reduce(initial T, fn func(T, T) T) T {
	result := initial
	for _, v := range s.data {
		result = fn(result, v)
	}
	return result
}

// Take limits to n elements.
func (s *SliceStream[T]) Take(n int) *SliceStream[T] {
	if n > len(s.data) {
		n = len(s.data)
	}
	return &SliceStream[T]{data: s.data[:n]}
}

// Skip skips n elements.
func (s *SliceStream[T]) Skip(n int) *SliceStream[T] {
	if n > len(s.data) {
		n = len(s.data)
	}
	return &SliceStream[T]{data: s.data[n:]}
}

// Any returns true if any element matches.
func (s *SliceStream[T]) Any(fn func(T) bool) bool {
	for _, v := range s.data {
		if fn(v) {
			return true
		}
	}
	return false
}

// All returns true if all elements match.
func (s *SliceStream[T]) All(fn func(T) bool) bool {
	for _, v := range s.data {
		if !fn(v) {
			return false
		}
	}
	return true
}

// ParallelMap maps in parallel.
func (s *SliceStream[T]) ParallelMap(fn func(T) T, workers int) *SliceStream[T] {
	if workers <= 0 {
		workers = 4
	}

	result := make([]T, len(s.data))
	var wg sync.WaitGroup

	chunkSize := (len(s.data) + workers - 1) / workers

	for i := 0; i < workers; i++ {
		start := i * chunkSize
		end := start + chunkSize
		if end > len(s.data) {
			end = len(s.data)
		}
		if start >= len(s.data) {
			break
		}

		wg.Add(1)
		go func(startIdx, endIdx int, data []T) {
			defer wg.Done()
			for j := startIdx; j < endIdx; j++ {
				result[j] = fn(data[j])
			}
		}(start, end, s.data)
	}

	wg.Wait()
	return &SliceStream[T]{data: result}
}

// SimplePipeline is a simple pipeline implementation.
type SimplePipeline[T, R any] struct {
	processors []func(context.Context, any) (any, error)
}

// NewPipeline creates a new SimplePipeline.
func NewPipeline[T, R any]() *SimplePipeline[T, R] {
	return &SimplePipeline[T, R]{
		processors: make([]func(context.Context, any) (any, error), 0),
	}
}

// Add adds a processor.
func (p *SimplePipeline[T, R]) Add(processor func(context.Context, any) (any, error)) *SimplePipeline[T, R] {
	p.processors = append(p.processors, processor)
	return p
}

// Execute executes the pipeline.
func (p *SimplePipeline[T, R]) Execute(ctx context.Context, input T) (R, error) {
	var current any = input
	var err error

	for _, processor := range p.processors {
		current, err = processor(ctx, current)
		if err != nil {
			var zero R
			return zero, err
		}
	}

	return current.(R), nil
}

// ExecuteBatch executes the pipeline for multiple inputs.
func (p *SimplePipeline[T, R]) ExecuteBatch(ctx context.Context, inputs []T) ([]R, error) {
	results := make([]R, len(inputs))
	for i, input := range inputs {
		result, err := p.Execute(ctx, input)
		if err != nil {
			return nil, err
		}
		results[i] = result
	}
	return results, nil
}

// Channel creates a stream from a channel.
func Channel[T any](ch <-chan T) *SliceStream[T] {
	var data []T
	for v := range ch {
		data = append(data, v)
	}
	return &SliceStream[T]{data: data}
}

// Generate generates a stream using a generator function.
func Generate[T any](count int, generator func(int) T) *SliceStream[T] {
	data := make([]T, count)
	for i := 0; i < count; i++ {
		data[i] = generator(i)
	}
	return &SliceStream[T]{data: data}
}

// Range generates a stream of integers.
func Range(start, end int) *SliceStream[int] {
	data := make([]int, end-start)
	for i := start; i < end; i++ {
		data[i-start] = i
	}
	return &SliceStream[int]{data: data}
}
