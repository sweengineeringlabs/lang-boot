# rustboot-resilience

Resilience patterns for fault-tolerant Rust applications.

## Features

- ✅ Retry with exponential backoff
- ✅ Circuit breaker (closed/open/half-open states)
- ✅ Timeout wrapper
- ✅ Async/await support
- ✅ Configurable policies

## Quick Start

```toml
[dependencies]
dev-engineeringlabs-rustboot-resilience = "0.1"
```

### Retry

```rust
use dev_engineeringlabs_rustboot_resilience::*;

let policy = RetryPolicy::new(3);
let result = policy.execute(|| async {
    make_api_call().await
}).await?;
```

### Circuit Breaker

```rust
let config = CircuitBreakerConfig {
    failure_threshold: 5,
    timeout: Duration::from_secs(60),
    success_threshold: 2,
};
let breaker = CircuitBreaker::new(config);

let result = breaker.execute(|| async {
    call_external_service().await
}).await?;
```

### Timeout

```rust
let result = with_timeout(Duration::from_secs(5), async {
    slow_operation().await
}).await?;
```

## Documentation

- [Overview](docs/overview.md) - Detailed patterns
- [Examples](../../examples/) - Usage examples

## License

MIT
