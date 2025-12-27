# Rustboot Streams - Overview

**Layer**: L4 Core (Utility)  
**Purpose**: Async stream utilities for event-driven architectures

---

## WHAT: Async Event Streaming Utilities

Rustboot Streams provides type-safe wrappers around tokio streams and channels for building event-driven systems.

### Key Capabilities

1. **EventStream<T>** - Type alias for boxed async streams
2. **EventSender<T>** - Ergonomic wrapper around tokio mpsc sender
3. **StreamBuilder<T>** - Builder pattern for stream creation
4. **EventStreamExt** - Extension trait for stream operations

---

## WHY: Event-Driven Architecture Support

### Problem Statement

Building event-driven systems in Rust requires:
- Working with low-level tokio primitives
- Managing lifetimes and pinning
- Handling backpressure
- Type erasure for stream composition

### Impact

Without stream utilities:
- ❌ Boilerplate for each stream
- ❌ Complex types in signatures
- ❌ Inconsistent patterns
- ❌ Hard to compose streams

With rustboot-streams:
- ✅ Consistent API across codebase
- ✅ Clean type signatures  
- ✅ Easy stream creation
- ✅ Composable patterns

### When to Use

**Use rustboot-streams when:**
- Building event-driven systems
- Implementing pub/sub patterns
- Creating async data pipelines
- Need backpressure management

**Don't use when:**
- Synchronous iteration is sufficient
- Simple one-time operations
- No concurrency needed

---

## HOW: Usage Guide

### Basic Usage

```rust
use dev_engineeringlabs_rustboot_streams::*;
use futures::StreamExt;

#[tokio::main]
async fn main() {
    // Create stream with default buffer (100)
    let (sender, mut stream) = create_stream::<String>();

    // Send events
    sender.send("Hello".to_string()).await.unwrap();
    sender.send("World".to_string()).await.unwrap();
    
    drop(sender); // Close stream

    // Consume events
    while let Some(event) = stream.next().await {
        println!("Event: {}", event);
    }
}
```

### Custom Buffer Size

```rust
// For high-throughput scenarios
let (sender, stream) = create_stream_with_buffer::<String>(10000);

// Or use builder
let (sender, stream) = StreamBuilder::<String>::new()
    .buffer_size(5000)
    .build();
```

### Multiple Senders

```rust
let (sender, mut stream) = create_stream::<u32>();

// Clone sender for multiple producers
let sender1 = sender.clone();
let sender2 = sender.clone();

tokio::spawn(async move {
    sender1.send(1).await.unwrap();
});

tokio::spawn(async move {
    sender2.send(2).await.unwrap();
});
```

---

## Examples and Tests

### Examples

```bash
# Basic stream usage
cargo run --example basic_stream
```

### Tests

```bash
# Run all tests
cargo test -p dev-engineeringlabs-rustboot-streams
```

**Test coverage:**
- ✅ 6 unit tests
- ✅ 9 doc tests
- ✅ 3 integration tests

---

## Relationship to Other Modules

### Used By

- `rustboot-messaging` - Pub/sub event streams
- `rustratify` - SEA event-driven APIs

### Depends On

- `tokio` - Async runtime
- `tokio-stream` - Stream adapters
- `futures-core` - Stream trait

---

## Status

- ✅ **Implementation**: Complete
- ✅ **Testing**: 18/18 tests passing
- ✅ **Documentation**: Complete
- ✅ **Examples**: Basic example

---

## Roadmap

See [Framework Backlog](../../../docs/framework-backlog.md) for planned features.
