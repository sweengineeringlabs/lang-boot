# Resilience Module Overview

## WHAT: Resilience Patterns

Circuit breakers, retries, timeouts, and bulkheads for fault tolerance.

Key capabilities:
- **Circuit Breaker** - Fail fast on repeated failures
- **Retry** - Automatic retry with backoff
- **Timeout** - Time-bound operations
- **Bulkhead** - Isolation of failures

## WHY: Fault Tolerance

**Problems Solved**:
1. **Cascading Failures** - Circuit breaker isolation
2. **Transient Errors** - Automatic retry
3. **Slow Services** - Timeout protection

**When to Use**: External service calls, unreliable operations

## HOW: Usage Guide

```java
// Circuit breaker
var cb = CircuitBreaker.builder("api")
    .failureThreshold(5)
    .timeout(Duration.ofSeconds(30))
    .build();

var result = cb.execute(() -> api.call());

// Retry
var retry = RetryPolicy.builder()
    .maxAttempts(3)
    .backoff(BackoffStrategy.EXPONENTIAL)
    .build();

retry.execute(() -> unreliableOperation());
```

## Relationship to Other Modules

| Module | Relationship |
|--------|--------------|
| jboot-http | Wrap HTTP calls |
| jboot-observability | Resilience metrics |

---

**Status**: Stable
