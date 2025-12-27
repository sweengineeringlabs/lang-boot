# rustboot-uuid

UUID generation and parsing.

```rust
use dev_engineeringlabs_rustboot_uuid::*;

// Random UUID
let id = new_v4();

// Time-based sortable UUID
let id = new_v7();

// Parse
let uuid = parse_uuid("550e8400-e29b-41d4-a716-446655440000")?;
```

See [overview](docs/overview.md).
