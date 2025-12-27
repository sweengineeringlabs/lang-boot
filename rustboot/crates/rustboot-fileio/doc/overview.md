# File I/O Overview

## WHAT: Atomic File Operations

- **write_atomic**: Write-then-rename for crash safety
- **safe_join**: Prevent directory traversal attacks  
- **ensure_dir**: Create directories recursively

## WHY: Safety & Reliability

**Problems Solved**:
1. Partial writes on crash → Atomic writes
2. Directory traversal attacks → safe_join validation
3. Missing parent directories → ensure_dir

**Use Cases**: Config files, data persistence, user uploads

## HOW: Usage

```rust
// Atomic write (crash-safe)
write_atomic("data.json", bytes)?;

// Path safety
let safe_path = safe_join(base_dir, user_input)?;

// Directory creation
ensure_dir("logs/2024/01")?;
```

**Best Practices**: Always use atomic writes for important data, validate user-provided paths with safe_join.


## Examples and Tests

> **⚠️ Required**: Every module must have working examples and tests to guide users.

### Examples

**Location**: [xamples/](../examples/) directory

**Current examples**:
- [fileio_basic.rs](../examples/fileio_basic.rs) - Basic usage demonstration
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
cargo test -p dev-engineeringlabs-rustboot-fileio
cargo run --example fileio_basic
```

---

**Status**: Stable  
**Roadmap**: See [backlog.md](../backlog.md)