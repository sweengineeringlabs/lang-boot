# Resilience Patterns Overview

> **📝 Important**: This overview links to:
> - Working code examples in `examples/` directory
> - Integration tests in `tests/` directory  
> - Testing guides for developers

## WHAT: Fault Tolerance and Resilience Patterns

The `rustboot-resilience` crate provides battle-tested patterns for building fault-tolerant systems:

Key capabilities:
- **Retry policies** - Automatic retry with exponential backoff
- **Circuit breakers** - Prevent cascading failures
- **Timeouts** - Prevent indefinite waits
- **Bulkheads** - Isolate failures
- **Pattern composition** - Layer patterns for robustness

## WHY: Prevent Cascading Failures

**Problems Solved**:
1. **Transient failures** - Temporary network glitches causing permanent failures
2. **Cascading failures** - One service failure bringing down the entire system
3. **Resource exhaustion** - Indefinite waits consuming all resources
4. **Poor user experience** - Failures without graceful degradation

**Impact if not addressed**:
- System-wide outages from single component failures
- Resource exhaustion from hanging operations
- Poor reliability and availability
- Difficult troubleshooting and recovery

**When to Use**: Distributed systems, microservices, external API calls, database operations - anywhere failures are expected.

**When NOT to Use**: Simple, synchronous,local operations where failures are fatal anyway.

## HOW: Resilience Pattern Guide

### Basic Example

```rust
use dev_engineeringlabs_rustboot_resilience::*;
use std::time::Duration;

let policy = RetryPolicy::new(3); // max 3 attempts

let result = policy.execute(|| async {
    // Your operation here
    api_call().await
}).await?;
```

### Retry Pattern

Automatically retry failed operations with configurable backoff strategies.

```rust
// Simple retry
let policy = RetryPolicy::new(3);

// With exponential backoff
let backoff = ExponentialBackoff::new(
    Duration::from_millis(100),  // initial delay
    Duration::from_secs(10),     // max delay
    2.0,                         // multiplier
);

let policy = RetryPolicy::new(5)
    .with_backoff(backoff);

let result = policy.execute(|| async {
    flaky_operation().await
}).await?;
```

**Available**:
- Configurable max retries
- Exponential backoff
- Jitter for thundering herd prevention
- Custom retry conditions

**Planned**:
- Fibonacci backoff
- Decorrelated jitter
- Retry budgets
- Per-operation retry policies

### Circuit Breaker Pattern

Prevent repeated calls to failing services to allow recovery time.

#### States

- **Closed**: Normal operation, requests flow through
- **Open**: Too many failures, fail fast
- **Half-Open**: Testing if service recovered

#### Configuration

```rust
let config = CircuitBreakerConfig {
    failure_threshold: 5,          // failures before opening
    timeout: Duration::from_secs(60), // wait before retry
    success_threshold: 2,          // successes to close
};

let breaker = CircuitBreaker::new(config);
```

#### Usage

```rust
// Check state
match breaker.state().await {
    CircuitState::Closed => println!("Normal operation"),
    CircuitState::Open => println!("Circuit is open!"),
    CircuitState::HalfOpen => println!("Testing recovery"),
}

// Execute with protection
let result = breaker.execute(|| async {
    external_api_call().await
}).await;

match result {
    Ok(data) => process(data),
    Err(ResilienceError::CircuitOpen) => {
        // Circuit is open, fail fast
        return_cached_data()
    }
    Err(e) => handle_error(e),
}
```

**Available**:
- Three-state circuit breaker
- Configurable thresholds and timeouts
- State inspection

**Planned**:
- Sliding window failure detection
- Percentage-based thresholds
- Circuit breaker events/metrics
- Adaptive timeout adjustment

### Timeout Pattern

Prevent indefinite waits and resource exhaustion.

```rust
use dev_engineeringlabs_rustboot_resilience::with_timeout;

// Timeout after 5 seconds
let result = with_timeout(Duration::from_secs(5), async {
    slow_database_query().await
}).await;

match result {
    Ok(data) => println!("Got data: {:?}", data),
    Err(ResilienceError::Timeout(d)) => {
        eprintln!("Operation timed out after {:?}", d);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

### Combining Patterns

Layer patterns for maximum resilience:

```rust
// Retry with timeout and circuit breaker
let breaker = CircuitBreaker::new(config);
let retry = RetryPolicy::new(3);

let result = retry.execute(|| async {    
    breaker.execute(|| async {
        with_timeout(Duration::from_secs(5), async {
            risky_operation().await
        }).await
    }).await
}).await?;
```

**Pattern Order**:
1. **Timeout** (innermost) - Prevent individual calls from hanging
2. **Circuit Breaker** (middle) - Fail fast when service is down
3. **Retry** (outermost) - Retry transient failures

## Common Patterns

### Resilient HTTP Client

```rust
let breaker = CircuitBreaker::new(CircuitBreakerConfig::default());
let retry = RetryPolicy::new(3);

async fn fetch(url: &str) -> Result<String, Error> {
    retry.execute(|| async {
        breaker.execute(|| async {
            with_timeout(Duration::from_secs(10), async {
                reqwest::get(url).await?.text().await
            }).await
        }).await
    }).await
}
```

### Database with Fallback

```rust
async fn get_user(id: UserId) -> Result<User, Error> {
    let policy = RetryPolicy::new(2);
    
    policy.execute(|| async {
        match database.query(id).await {
            Ok(user) => Ok(user),
            Err(e) if is_transient(&e) => {
                // Retry transient errors
                Err(e)
            }
            Err(e) => {
                // Non-transient error, fail immediately
                return Err(e);
            }
        }
    }).await
    .or_else(|_| get_user_from_cache(id))
}
```

## Error Types

```rust
pub enum ResilienceError {
    Timeout(Duration),
    MaxRetriesExceeded(usize),
    CircuitOpen,
    OperationFailed(String),
}
```

## Best Practices

1. **Retry**: Use for transient failures (network glitches, temporary unavailability)
2. **Circuit Breaker**: Prevent cascading failures in distributed systems
3. **Timeout**: Prevent indefinite waits (use conservative timeouts)
4. **Combine**: Layer patterns for robust systems
5. **Fail Fast**: Don't retry non-transient errors
6. **Logging**: Log resilience events for debugging

## Relationship to Other Modules

| Module/Component | Purpose | Relationship |
|------------------|---------|--------------|
| `rustboot-http` | HTTP client | Protected with retry and circuit breakers |
| `rustboot-database` | Database access | Uses retry for transient failures |
| `rustboot-observability` | Logging/metrics | Logs resilience events |
| `rustboot-ratelimit` | Rate limiting | Complements resilience patterns |

**Integration Points**:
- HTTP clients wrapped with resilience patterns
- Database operations protected by retry policies
- Metrics tracking circuit breaker state changes

## Examples and Tests

> **⚠️ Required**: Every module must have working examples and tests to guide users.

### Examples

**Location**: [`examples/`](../examples/) directory

**Current examples**:
- [`resilience_basic.rs`](../examples/resilience_basic.rs) - Basic retry and circuit breaker usage
- [`resilient_http.rs`](../examples/resilient_http.rs) - HTTP client with full resilience patterns

**Purpose**: Demonstrate retry policies, circuit breakers, timeouts, and pattern composition.

### Tests

**Location**: [`tests/`](../tests/) directory

**Current tests**:
- [`integration.rs`](../tests/integration.rs) - Integration tests for all resilience patterns

### Testing Guidance

**For developers using this module**: See [Rust Test Organization](../../docs/4-development/guide/rust-test-organization.md)

**For contributors**: Run tests with:
```bash
cargo test -p dev-engineeringlabs-rustboot-resilience
cargo run --example resilience_basic
```

---

**Status**: Stable  
**Roadmap**: See [backlog.md](../backlog.md)
