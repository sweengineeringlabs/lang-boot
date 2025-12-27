# rustboot-serialization

JSON and MessagePack serialization helpers.

## Features

- Simple JSON encoding/decoding
- MessagePack for efficient binary format
- Pretty printing support
- Unified error handling

## Quick Start

```toml
[dependencies]
dev-engineeringlabs-rustboot-serialization = "0.1"
```

```rust
use dev_engineeringlabs_rustboot_serialization::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    age: u32,
}

// JSON
let user = User { name: "Alice".into(), age: 30 };
let json = to_json(&user)?;
let decoded: User = from_json(&json)?;

// MessagePack (smaller, faster)
let bytes = to_msgpack(&user)?;
let decoded: User = from_msgpack(&bytes)?;
```

## Documentation

- [Overview](docs/overview.md) - Detailed guide
- [Examples](../../examples/) - Usage examples

## License

MIT
