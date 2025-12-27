# HTTP Rate Limiting Middleware

This document describes the HTTP rate limiting middleware implementation for the Rustboot framework.

## Overview

The rate limiting middleware provides flexible, algorithm-based request throttling for HTTP applications. It integrates seamlessly with the `rustboot-ratelimit` crate's algorithms and the middleware pipeline pattern.

## Features

- **Multiple Rate Limiting Algorithms**:
  - Fixed Window Counter
  - Sliding Window Log
  - Token Bucket
  - Leaky Bucket

- **Flexible Key Extraction**:
  - IP-based rate limiting (default)
  - API key/header-based rate limiting
  - Custom key extraction logic

- **Standard Rate Limit Headers**:
  - `X-RateLimit-Limit`: Maximum requests allowed
  - `X-RateLimit-Remaining`: Requests remaining in current window
  - `Retry-After`: Seconds to wait before retrying (when rate limited)

- **429 Too Many Requests**: Automatic rejection with appropriate HTTP status code

## Usage

### Basic IP-Based Rate Limiting

```rust
use rustboot_middleware::{RateLimitConfig, RateLimitMiddleware, Pipeline};
use std::time::Duration;

// Create a Fixed Window rate limiter: 100 requests per minute
let config = RateLimitConfig::FixedWindow {
    max_requests: 100,
    window_size: Duration::from_secs(60),
};

let middleware = RateLimitMiddleware::new(config);
let pipeline = Pipeline::new().with_middleware(middleware);

// Process requests
let ctx = HttpContext::new("GET".to_string(), "/api/data".to_string())
    .with_client_ip("192.168.1.100".to_string());

let result = pipeline.execute(ctx).await;
```

### API Key-Based Rate Limiting

```rust
use rustboot_middleware::{
    RateLimitConfig, RateLimitMiddleware, HeaderKeyExtractor, Pipeline
};
use std::sync::Arc;
use std::time::Duration;

// Create a Token Bucket: 1000 tokens, refill 10 per second
let config = RateLimitConfig::TokenBucket {
    capacity: 1000,
    refill_rate: 10,
    refill_interval: Duration::from_secs(1),
};

let extractor = Arc::new(HeaderKeyExtractor::new("X-API-Key"));
let middleware = RateLimitMiddleware::with_key_extractor(config, extractor);

// Share middleware across multiple requests using Arc
let middleware = Arc::new(middleware);

// Process request with API key
let ctx = HttpContext::new("POST".to_string(), "/api/create".to_string())
    .with_header("X-API-Key".to_string(), "user-api-key-123".to_string());

let pipeline = Pipeline::new().with_middleware(Arc::clone(&middleware));
let result = pipeline.execute(ctx).await;
```

### Custom Key Extraction

```rust
use rustboot_middleware::{
    RateLimitConfig, RateLimitMiddleware, CustomKeyExtractor, HttpContext
};
use std::sync::Arc;

// Extract user ID from URL path for per-user rate limiting
let extractor = Arc::new(CustomKeyExtractor::new(|ctx: &HttpContext| {
    // Extract from path like /users/123/profile
    ctx.url
        .split('/')
        .nth(2)
        .map(|id| format!("user:{}", id))
}));

let config = RateLimitConfig::SlidingWindow {
    max_requests: 50,
    window_size: Duration::from_secs(60),
};

let middleware = RateLimitMiddleware::with_key_extractor(config, extractor);
```

### Disabling Rate Limit Headers

```rust
let middleware = RateLimitMiddleware::new(config)
    .with_headers(false); // Disable X-RateLimit-* headers
```

## Rate Limiting Algorithms

### Fixed Window Counter

Limits requests to a fixed number within a time window. Simple and efficient, but can allow bursts at window boundaries.

```rust
let config = RateLimitConfig::FixedWindow {
    max_requests: 100,
    window_size: Duration::from_secs(60),
};
```

**Use cases**: General API rate limiting, simple quota enforcement

### Sliding Window Log

Tracks individual request timestamps for precise rate limiting. More accurate than fixed window, but uses more memory.

```rust
let config = RateLimitConfig::SlidingWindow {
    max_requests: 100,
    window_size: Duration::from_secs(60),
};
```

**Use cases**: Strict rate limiting, premium API tiers

### Token Bucket

Allows bursts while maintaining average rate. Tokens accumulate over time up to capacity.

```rust
let config = RateLimitConfig::TokenBucket {
    capacity: 100,
    refill_rate: 10,
    refill_interval: Duration::from_secs(1),
};
```

**Use cases**: Bursty traffic patterns, API with varying request costs

### Leaky Bucket

Smooths out traffic by processing requests at a constant rate. Overflow requests are rejected.

```rust
let config = RateLimitConfig::LeakyBucket {
    capacity: 50,
    leak_rate: 5,
    leak_interval: Duration::from_secs(1),
};
```

**Use cases**: Traffic shaping, protecting downstream services

## Key Extraction Strategies

### IP-Based (Default)

Rate limits based on client IP address.

```rust
let middleware = RateLimitMiddleware::new(config);
// Requires ctx.client_ip to be set
```

### Header-Based

Rate limits based on a specific HTTP header (e.g., API key, User ID).

```rust
let extractor = Arc::new(HeaderKeyExtractor::new("X-API-Key"));
let middleware = RateLimitMiddleware::with_key_extractor(config, extractor);
```

### Custom

Implement your own key extraction logic.

```rust
let extractor = Arc::new(CustomKeyExtractor::new(|ctx: &HttpContext| {
    // Custom logic to extract rate limit key
    Some(format!("{}:{}", ctx.method, ctx.url))
}));
```

## Response Behavior

### Allowed Requests

When a request is allowed:
- Request proceeds through the pipeline
- Response headers include rate limit information (if enabled)
- Status: Original response status

### Rate Limited Requests

When a request exceeds the rate limit:
- Request is rejected immediately
- Response headers include rate limit information
- Status: `429 Too Many Requests`
- Body: `"Too Many Requests"`

## Integration Testing

Run tests with the ratelimit feature:

```bash
cargo test -p dev-engineeringlabs-rustboot-middleware --features ratelimit
```

## Example

See `examples/http_ratelimit.rs` for a complete demonstration:

```bash
cargo run -p dev-engineeringlabs-rustboot-middleware --example http_ratelimit --features ratelimit
```

## Architecture

The rate limiting middleware:

1. **Extracts the rate limit key** from the HTTP context using the configured strategy
2. **Gets or creates a rate limiter** for that key (one limiter per unique key)
3. **Attempts to acquire** a slot/token from the limiter
4. **Allows or rejects** the request based on the limiter's decision
5. **Adds rate limit headers** to the response (if enabled)

Each middleware instance maintains its own `HashMap<String, RateLimiter>` to track limiters for different keys. Use `Arc<RateLimitMiddleware>` to share state across pipeline executions.

## Best Practices

1. **Choose the right algorithm** for your use case:
   - Fixed Window: Simple, low memory
   - Sliding Window: Accurate, higher memory
   - Token Bucket: Allows bursts
   - Leaky Bucket: Constant rate

2. **Share middleware instances** using `Arc` to maintain consistent state across requests

3. **Set appropriate limits** based on your application's capacity and requirements

4. **Monitor rate limit metrics** in production to tune limits

5. **Provide clear error messages** to clients when rate limited

6. **Consider different limits** for different endpoints or user tiers

## Thread Safety

The rate limiting middleware is fully thread-safe and can be shared across multiple async tasks using `Arc`. Internal state is protected by `RwLock`.

## Performance

- **Memory**: O(K) where K is the number of unique keys (IPs, API keys, etc.)
- **CPU**: O(1) for most operations
- **Lock contention**: Uses `RwLock` for efficient concurrent access

## Limitations

- Rate limit state is **in-memory only** (not distributed across multiple servers)
- No built-in persistence (state is lost on restart)
- No automatic cleanup of stale rate limiters (consider implementing TTL if needed)

For distributed rate limiting across multiple servers, consider using Redis or another distributed store with a custom implementation.
