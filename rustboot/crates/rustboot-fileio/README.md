# rustboot-fileio

Safe file operations with atomic writes.

## Quick Start

```toml
[dependencies]
dev-engineeringlabs-rustboot-fileio = "0.1"
```

```rust
use dev_engineeringlabs_rustboot_fileio::*;

// Atomic write (no partial writes)
write_atomic("config.json", data)?;

// Safe path joining (prevent traversal)
let path = safe_join("/base", "file.txt")?;

// Ensure directory
ensure_dir("/path/to/dir")?;
```

See [overview](docs/overview.md) for details.
