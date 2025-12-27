# Compression Overview

## WHAT: Data Compression

**Gzip**: Universal, slower | **Zstd**: Modern, faster, better compression

## WHY: Reduce Size & Bandwidth

**Use Cases**: File storage, API responses, backups | **Benefits**: 50-80% size reduction

## HOW: Choose Algorithm

```rust
// Gzip (compatibility)
gzip_compress(data)?;

// Zstd (performance, level 1-22)
zstd_compress(data, 3)?;
```

**Tip**: Use gzip for HTTP (universal), zstd for storage/backups.


## Examples and Tests

> **⚠️ Required**: Every module must have working examples and tests to guide users.

### Examples

**Location**: [xamples/](../examples/) directory

**Current examples**:
- [compress_basic.rs](../examples/compress_basic.rs) - Basic usage demonstration
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
cargo test -p dev-engineeringlabs-rustboot-compress
cargo run --example compress_basic
```

---

**Status**: Stable  
**Roadmap**: See [backlog.md](../backlog.md)