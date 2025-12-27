# Cache Overview

## WHAT: TTL-Based Caching Abstraction

The `rustboot-cache` crate provides a simple, flexible caching layer with TTL (time-to-live) support.

Key features:
- **In-memory caching** - Fast local cache
- **TTL support** - Automatic expiration
- **Generic interface** - Cache any serializable type
- **Backend-agnostic** - Swap implementations easily

## WHY: Reduce Database Load and Improve Performance

**Problems Solved**:
1. Repeated database queries for same data
2. Slow external API calls
3. Performance bottlenecks
4. Manual cache management complexity

**When to Use**: Applications with frequently accessed data, expensive computations, or external API calls.

## HOW: Simple Caching API

### Basic Example

```rust
use dev_engineeringlabs_rustboot_cache::*;

let cache = InMemoryCache::new();

// Set with TTL
cache.set("user:123", user_data, Duration::from_secs(300))?;

// Get
if let Some(data) = cache.get("user:123")? {
    // Use cached data
}
```

**Available**:
- `set()` - Cache value with TTL
- `get()` - Retrieve cached value
- `remove()` - Invalidate entry
- In-memory backend

**Planned**: See [backlog.md](../backlog.md)

---

**Status**: Stable  
**Backlog**: See [backlog.md](../backlog.md)


## Examples and Tests

> **⚠️ Required**: Every module must have working examples and tests to guide users.

### Examples

**Location**: [xamples/](../examples/) directory

**Current examples**:
- [cache_basic.rs](../examples/cache_basic.rs) - Basic usage demonstration
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
cargo test -p dev-engineeringlabs-rustboot-cache
cargo run --example cache_basic
```

---

**Status**: Stable  
**Roadmap**: See [backlog.md](../backlog.md)