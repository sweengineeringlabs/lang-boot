# Rate Limiting Overview

## WHAT is Rate Limiting?

Rate limiting controls the rate of requests to prevent system overload. This crate provides 4 algorithms for different use cases:

- **Token Bucket**: Smooth rate limiting with burst capacity
- **Leaky Bucket**: Constant rate processing queue
- **Fixed Window**: Simple time-based counter
- **Sliding Window**: Precise distributed limits

## WHY Use Rate Limiting?

### Problems it Solves

1. **API Protection**: Prevent abuse and overload
2. **Fair Resource Allocation**: Ensure all clients get fair access
3. **Cost Control**: Limit expensive operations
4. **Quality of Service**: Prevent cascading failures

### Use Cases

- **Public APIs**: Throttle requests per client
- **Background Jobs**: Control processing rate
- **External Services**: Respect third-party limits
- **Database Queries**: Prevent connection exhaustion

## HOW to Use

### Token Bucket (Recommended for APIs)

**Best for**: Smooth traffic with occasional bursts

```rust
use dev_engineeringlabs_rustboot_ratelimit::*;
use std::time::Duration;

// 100 tokens capacity, refill 10/second
let limiter = TokenBucket::new(100, 10, Duration::from_secs(1));

// Non-blocking check
match limiter.try_acquire().await {
    Ok(()) => process_request().await,
    Err(_) => return_rate_limit_error(),
}

// Or wait for token
limiter.acquire().await?;
process_request().await;
```

**How it works**: Bucket holds tokens. Requests consume tokens. Tokens refill over time. Allows bursts up to capacity.

### Leaky Bucket

**Best for**: Constant rate processing

```rust
// 50 capacity, leak 5/second
let limiter = LeakyBucket::new(50, 5, Duration::from_secs(1));

limiter.try_acquire().await?;
```

**How it works**: Requests fill bucket. Bucket "leaks" at constant rate. Rejects when full.

### Fixed Window

**Best for**: Simple hourly/daily limits

```rust
// 1000 requests per hour
let limiter = FixedWindow::new(1000, Duration::from_secs(3600));

limiter.try_acquire().await?;
```

**How it works**: Counter resets at window boundary (e.g., every hour). Simple but can allow 2x traffic at boundaries.

### Sliding Window

**Best for**: Precise distributed limits

```rust
// 100 requests per minute, exactly
let limiter = SlidingWindow::new(100, Duration::from_secs(60));

limiter.try_acquire().await?;
```

**How it works**: Tracks exact request timestamps. Removes expired requests. Most accurate but higher memory usage.

## Algorithm Comparison

| Algorithm | Memory | Accuracy | Bursts | Best For |
|-----------|--------|----------|--------|----------|
| Token Bucket | Low | Medium | Yes | APIs with bursts |
| Leaky Bucket | Low | Medium | No | Constant rate |
| Fixed Window | Very Low | Low | Yes* | Simple limits |
| Sliding Window | High | High | No | Precise control |

*Can allow 2x at window boundaries

## Common Patterns

### Per-User Rate Limiting

```rust
use std::collections::HashMap;
use tokio::sync::Mutex;

struct UserLimiter {
    limiters: Mutex<HashMap<UserId, TokenBucket>>,
}

impl UserLimiter {
    async fn check(&self, user_id: UserId) -> Result<(), RateLimitError> {
        let mut limiters = self.limiters.lock().await;
        let limiter = limiters.entry(user_id)
            .or_insert_with(|| TokenBucket::new(100, 10, Duration::from_secs(1)));
        limiter.try_acquire().await
    }
}
```

### Global + Per-User Limits

```rust
// Check global limit first, then per-user
global_limiter.try_acquire().await?;
user_limiter.try_acquire().await?;
```

### Tiered Limits

```rust
match user.tier {
    Tier::Free => rate_limit_free.try_acquire().await?,
    Tier::Pro => rate_limit_pro.try_acquire().await?,
    Tier::Enterprise => Ok(()), // No limit
}
```

## Best Practices

1. **Choose the right algorithm**: Token bucket for most APIs
2. **Set appropriate limits**: Monitor and adjust based on capacity
3. **Return clear errors**: Include retry-after headers
4. **Clean up old limiters**: Prevent memory leaks for per-user limits
5. **Combine with circuit breakers**: Protect downstream services

## Error Handling

```rust
match limiter.try_acquire().await {
    Ok(()) => {
        // Process request
    }
    Err(RateLimitError::RateLimitExceeded) => {
        // Return 429 Too Many Requests
        // Add Retry-After header
    }
}
```

## Performance Tips

- **Token/Leaky Bucket**: O(1) time, minimal memory
- **Sliding Window**: O(n) where n = window size, higher memory
- **Use try_acquire()**: Non-blocking, fails fast
- **Use acquire()**: When you can wait for capacity


## Examples and Tests

> **⚠️ Required**: Every module must have working examples and tests to guide users.

### Examples

**Location**: [xamples/](../examples/) directory

**Current examples**:
- [ratelimit_basic.rs](../examples/ratelimit_basic.rs) - Basic usage demonstration
- See directory for additional examples

**Purpose**: Show users HOW to use this module in real applications.

### Tests

**Location**: [	ests/](../tests/) directory

**Current tests**:
- [integration.rs](../tests/integration.rs) - Integration tests using public API

**Purpose**: Show users HOW to test code that uses this module.

### Testing Guidance

**For developers using this module**: See [Rust Test Organization](../../docs/4-development/guide/rust-test-organization.md)

**For contributors**: Run tests with:
```bash
cargo test -p dev-engineeringlabs-rustboot-ratelimit
cargo run --example ratelimit_basic
```

---

**Status**: Stable  
**Roadmap**: See [backlog.md](../backlog.md)