# rustboot-ratelimit

Rate limiting for API throttling and request control.

## Features

- Token bucket algorithm
- Leaky bucket algorithm  
- Fixed window counter
- Sliding window log

## Quick Start

```toml
[dependencies]
dev-engineeringlabs-rustboot-ratelimit = "0.1"
```

```rust
use dev_engineeringlabs_rustboot_ratelimit::*;
use std::time::Duration;

// Token bucket: smooth rate limiting with burst capacity
let limiter = TokenBucket::new(100, 10, Duration::from_secs(1));
limiter.acquire().await?;

// Leaky bucket: constant rate processing
let limiter = LeakyBucket::new(50, 5, Duration::from_secs(1));
limiter.try_acquire().await?;

// Fixed window: simple time-based counting
let limiter = FixedWindow::new(1000, Duration::from_secs(60));
limiter.try_acquire().await?;

// Sliding window: precise distributed limiting
let limiter = SlidingWindow::new(100, Duration::from_secs(60));
limiter.try_acquire().await?;
```

## Documentation

- [Overview](docs/overview.md) - Detailed guide
- [Examples](../../examples/) - Usage examples

## License

MIT
