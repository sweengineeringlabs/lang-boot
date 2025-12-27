# rustboot-async

Async runtime utilities and task management.

## Features

- Task spawning (async and blocking)
- Timeout pools for concurrent operations
- Built on tokio runtime

## Quick Start

```toml
[dependencies]
dev-engineeringlabs-rustboot-async = "0.1"
tokio = { version = "1.0", features = ["full"] }
```

```rust
use dev_engineeringlabs_rustboot_async::*;

// Spawn async task
let handle = spawn_task(async {
    fetch_data().await
});
let result = handle.await?;

// Spawn blocking task (CPU-bound)
let handle = spawn_blocking_task(|| {
    expensive_computation()
});

// Timeout pool
let pool = TimeoutPool::new(Duration::from_secs(5));
pool.execute(slow_operation()).await?;
```

## Documentation

- [Overview](docs/overview.md) - Detailed guide
- [Examples](../../examples/) - Usage examples

## License

MIT
