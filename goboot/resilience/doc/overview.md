# Resilience Module Overview

## WHAT: Resilience Patterns

Circuit breakers, retries, timeouts.

Key capabilities:
- **Circuit Breaker** - Fail fast
- **Retry** - Automatic retries
- **Timeout** - Time-bound ops
- **Bulkhead** - Isolation

## WHY: Fault Tolerance

**Problems Solved**: Cascading failures, transient errors

**When to Use**: External service calls

## HOW: Usage Guide

```go
cb := resilience.NewCircuitBreaker(resilience.CBConfig{
    FailureThreshold: 5,
    Timeout:          30 * time.Second,
})

result, err := cb.Execute(func() (interface{}, error) {
    return api.Call()
})

// Retry
retry := resilience.NewRetry(resilience.RetryConfig{
    MaxAttempts: 3,
    Backoff:     resilience.Exponential,
})
```

---

**Status**: Stable
