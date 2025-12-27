# Async Runtime Overview

## WHAT is This Crate?

Utilities for managing async tasks and operations on tokio runtime:

- **spawn_task**: Launch async operations
- **spawn_blocking_task**: Run CPU-intensive work  
- **TimeoutPool**: Execute multiple operations with timeout

## WHY Use These Utilities?

### Problems They Solve

1. **Simplified Task Management**: Cleaner spawn API
2. **Blocking Work Isolation**: Prevent blocking async runtime
3. **Timeout Management**: Built-in timeout pools
4. **Error Handling**: Unified async error types

### When to Use

- **spawn_task**: I/O-bound async work (HTTP, DB queries)
- **spawn_blocking_task**: CPU-bound work (encryption, parsing large files)
- **TimeoutPool**: Multiple operations with same timeout

## HOW to Use

### Spawning Async Tasks

```rust
use dev_engineeringlabs_rustboot_async::*;

// Simple spawn
let handle = spawn_task(async {
    fetch_api_data().await
});

// Wait for result
let data = handle.await.unwrap();
```

**Why use this instead of tokio::spawn?**
- Cleaner imports
- Consistent with other rustboot APIs

### Spawning Blocking Tasks

```rust
// CPU-intensive work
let handle = spawn_blocking_task(|| {
    // Heavy computation that would block async runtime
    parse_large_json_file()
});

let result = handle.await.unwrap();
```

**IMPORTANT**: Use for:
- ✅ File parsing
- ✅ Encryption/hashing  
- ✅ Complex calculations
- ❌ I/O operations (use async instead)

### Timeout Pool

```rust
use std::time::Duration;

let pool = TimeoutPool::new(Duration::from_secs(10));

// Single operation
let result = pool.execute(async {
    slow_database_query().await
}).await;

match result {
    Ok(data) => process(data),
    Err(_timeout) => handle_timeout(),
}
```

## Common Patterns

### Concurrent API Calls

```rust
let handles: Vec<_> = urls.into_iter()
    .map(|url| spawn_task(fetch(url)))
    .collect();

for handle in handles {
    let result = handle.await?;
    process(result);
}
```

### Background Jobs

```rust
// Fire and forget
spawn_task(async {
    send_email(user).await;
});

// Or track completion
let handle = spawn_task(async {
    process_batch().await
});

// Later...
if handle.is_finished() {
    println!("Done!");
}
```

### CPU + I/O Mix

```rust
// Fetch data (async)
let data = fetch_raw_data().await?;

// Parse (blocking) 
let parsed = spawn_blocking_task(move || {
    parse_complex_format(&data)
}).await?;

// Save (async)
save_to_database(parsed).await?;
```

## Best Practices

1. **Use spawn_blocking** for CPU work (>10ms)
2. **Use spawn_task** for I/O-bound operations
3. **Set appropriate timeouts** in TimeoutPool
4. **Handle JoinErrors** from spawned tasks
5. **Avoid spawning too many tasks** (consider semaphores)

## Performance Tips

- spawn_blocking uses dedicated thread pool
- Don't block async runtime (use spawn_blocking)
- TimeoutPool reuses timeout logic efficiently


## Examples and Tests

> **⚠️ Required**: Every module must have working examples and tests to guide users.

### Examples

**Location**: [xamples/](../examples/) directory

**Current examples**:
- [async_basic.rs](../examples/async_basic.rs) - Basic usage demonstration
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
cargo test -p dev-engineeringlabs-rustboot-async
cargo run --example async_basic
```

---

**Status**: Stable  
**Roadmap**: See [backlog.md](../backlog.md)