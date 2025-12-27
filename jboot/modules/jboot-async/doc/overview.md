# Async Module Overview

> **📝 Important**: This overview links to working examples and tests.

## WHAT: Async Utilities for Virtual Threads

The `jboot-async` module provides utilities for concurrent programming using Java 21+ virtual threads, enabling high-performance async operations without callback complexity.

Key capabilities:
- **Parallel Execution** - Execute tasks concurrently and collect results
- **Structured Concurrency** - Scoped task execution with automatic cleanup
- **Timeout Handling** - Execute with timeouts and proper cancellation
- **Rate Limiting** - Control concurrent execution with semaphores

## WHY: Simplified Concurrency

**Problems Solved**:
1. **Callback Hell** - Virtual threads eliminate need for reactive chains
2. **Resource Management** - Structured concurrency ensures cleanup
3. **Concurrency Limits** - Rate limiting prevents resource exhaustion

**When to Use**: 
- Parallel API calls
- Batch processing
- IO-bound operations requiring concurrency

**When NOT to Use**: 
- CPU-bound computation (use parallel streams)
- Simple sequential operations

## HOW: Usage Guide

### Basic Parallel Execution

```java
import com.jboot.async.Async;

// Execute multiple tasks in parallel
List<User> users = Async.parallel(
    () -> userService.findById(1),
    () -> userService.findById(2),
    () -> userService.findById(3)
);
```

### With Timeout

```java
var result = Async.withTimeout(Duration.ofSeconds(5), () -> {
    return slowService.fetchData();
});
```

**Available**:
- `parallel()` - Execute varargs suppliers concurrently
- `withTimeout()` - Execute with timeout
- `async()` - Return CompletableFuture from virtual thread
- `awaitAll()` - Wait for all futures
- `awaitAny()` - Wait for first completed
- `mapParallel()` - Transform list in parallel
- `delay()` - Pause execution

**Planned**:
- Retry with backoff
- Circuit breaker integration
- Batch processing utilities

### Structured Concurrency

```java
try (var scope = Async.scope()) {
    var future1 = scope.fork(() -> fetchUser(1));
    var future2 = scope.fork(() -> fetchOrders(1));
    
    scope.join();
    
    var user = future1.get();
    var orders = future2.get();
}
```

### Rate Limited Execution

```java
var executor = Async.rateLimited(10); // Max 10 concurrent

for (var item : items) {
    executor.submit(() -> process(item));
}
```

## Relationship to Other Modules

| Module | Purpose | Relationship |
|--------|---------|--------------|
| jboot-resilience | Circuit breakers | Can wrap async calls |
| jboot-http | HTTP client | Async requests |
| jboot-observability | Tracing | Trace async operations |

## Examples and Tests

### Examples

**Location**: [`examples/`](../../examples/) directory

**Current examples**:
- See `AsyncExample.java` in examples README

### Tests

**Location**: [`src/test/java/com/jboot/async/`](../src/test/java/com/jboot/async/)

**Current tests**:
- `AsyncTest.java` - Parallel, timeout, rate limiting tests

### Testing Guidance

```bash
mvn test -pl modules/jboot-async
```

---

**Status**: Beta  
**Java Version**: 21+ (requires virtual threads)
