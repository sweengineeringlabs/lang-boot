# Async Module Overview

> **📝 Important**: This overview links to working examples and tests.

## WHAT: Async Utilities for Concurrent Programming

The `async` package provides Go-idiomatic utilities for concurrent programming with goroutines, worker pools, and parallel execution patterns.

Key capabilities:
- **Parallel Execution** - Execute functions concurrently and collect results
- **Worker Pools** - Manage pools of goroutines for task processing
- **Timeout Handling** - Execute with context timeouts
- **Rate Limiting** - Control concurrent execution with semaphores

## WHY: Simplified Concurrency Patterns

**Problems Solved**:
1. **Boilerplate Reduction** - Common patterns pre-implemented
2. **Error Aggregation** - Collect errors from parallel operations
3. **Resource Control** - Limit concurrent goroutines

**When to Use**: 
- Parallel API calls
- Batch processing
- Fan-out/fan-in patterns

**When NOT to Use**: 
- Simple sequential operations
- Single goroutine is sufficient

## HOW: Usage Guide

### Parallel Execution

```go
import "dev.engineeringlabs/goboot/async"

results := async.Parallel(
    func() (int, error) { return fetchUser(1) },
    func() (int, error) { return fetchUser(2) },
    func() (int, error) { return fetchUser(3) },
)

for _, r := range results {
    if r.Err != nil {
        log.Printf("Error: %v", r.Err)
    } else {
        log.Printf("Result: %v", r.Value)
    }
}
```

**Available**:
- `Parallel()` - Execute functions concurrently
- `WithTimeout()` - Execute with context timeout
- `Map()` - Transform items in parallel
- `ForEach()` - Apply function to each item
- `First()` - Return first successful result

**Planned**:
- Retry with backoff
- Circuit breaker integration
- Batch processing

### Worker Pool

```go
pool := async.NewWorkerPool(10) // 10 workers

for _, item := range items {
    pool.Submit(func() {
        process(item)
    })
}

pool.Wait()
pool.Close()
```

### Timeout Handling

```go
result, err := async.WithTimeout(5*time.Second, func(ctx context.Context) (string, error) {
    select {
    case <-time.After(2 * time.Second):
        return "done", nil
    case <-ctx.Done():
        return "", ctx.Err()
    }
})
```

### Rate Limiting

```go
limiter := async.NewRateLimiter(5) // Max 5 concurrent

for _, item := range items {
    go func(it Item) {
        limiter.Execute(func() {
            process(it)
        })
    }(item)
}
```

## Relationship to Other Modules

| Module | Purpose | Relationship |
|--------|---------|--------------|
| resilience | Circuit breakers | Wrap async calls |
| http | HTTP client | Async requests |
| observability | Tracing | Trace operations |

## Examples and Tests

### Examples

**Location**: [`examples/`](../examples/)

**Current examples**:
- [`async_example.go`](../examples/async_example.go) - Parallel, worker pool, rate limiting

### Tests

**Location**: [`async_test.go`](async_test.go)

**Current tests**:
- `TestParallel` - Concurrent execution
- `TestWithTimeout` - Timeout handling
- `TestWorkerPool` - Pool management
- `TestRateLimiter` - Concurrency limiting

### Testing Guidance

```bash
go test ./async/...
```

---

**Status**: Beta  
**Go Version**: 1.21+
