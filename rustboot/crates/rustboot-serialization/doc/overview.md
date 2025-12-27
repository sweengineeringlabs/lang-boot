# Serialization Overview

## WHAT is Serialization?

Serialization converts data structures to/from bytes or strings for storage or transmission. This crate provides helpers for:

- **JSON**: Human-readable text format
- **MessagePack**: Efficient binary format

## WHY Use This Crate?

### Problems it Solves

1. **Boilerplate Reduction**: Simple functions vs verbose serde calls
2. **Consistent Error Handling**: Unified error types
3. **Best Practices**: Sensible defaults built-in
4. **Format Flexibility**: Easy to switch between JSON and MessagePack

### Use Cases

- **APIs**: JSON for HTTP responses
- **IPC**: MessagePack for inter-process communication
- **File Storage**: JSON for configs, MessagePack for data
- **Network Protocols**: MessagePack for efficiency

## HOW to Use

### JSON Serialization

```rust
use dev_engineeringlabs_rustboot_serialization::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Config {
    database_url: String,
    port: u16,
}

let config = Config {
    database_url: "localhost".into(),
    port: 5432,
};

// To JSON string
let json = to_json(&config)?;
// {"database_url":"localhost","port":5432}

// Pretty printed
let json = to_json_pretty(&config)?;
/* 
{
  "database_url": "localhost",
  "port": 5432
}
*/

// From JSON
let decoded: Config = from_json(&json)?;
```

### JSON Bytes (for I/O)

```rust
// To bytes (UTF-8)
let bytes = to_json_bytes(&config)?;
write_file("config.json", &bytes)?;

// From bytes
let bytes = read_file("config.json")?;
let config: Config = from_json_bytes(&bytes)?;
```

### MessagePack (Binary Format)

```rust
// Smaller and faster than JSON
let bytes = to_msgpack(&config)?;
write_file("config.msgpack", &bytes)?;

let config: Config = from_msgpack(&bytes)?;
```

**When to use MessagePack**:
- ✅ Performance critical
- ✅ Binary protocols
- ✅ Large data structures
- ❌ Need human readability

## Format Comparison

| Feature | JSON | MessagePack |
|---------|------|-------------|
| Size | Larger | **Smaller (30-40% reduction)** |
| Speed | Slower | **Faster** |
| Human-readable | ✅ Yes | ❌ No |
| Browser support | ✅ Native | ⚠️ Requires library |
| Best for | APIs, configs | IPC, binary protocols |

## Common Patterns

### Config Files

```rust
// Development: JSON (readable)
#[cfg(debug_assertions)]
fn save_config(config: &Config) -> Result<()> {
    let json = to_json_pretty(config)?;
    write_file("config.json", json.as_bytes())
}

// Production: MessagePack (efficient)
#[cfg(not(debug_assertions))]
fn save_config(config: &Config) -> Result<()> {
    let bytes = to_msgpack(config)?;
    write_file("config.msgpack", &bytes)
}
```

### API Responses

```rust
async fn handle_request() -> HttpResponse {
    let data = get_data().await?;
    let json = to_json(&data)?;
    HttpResponse::ok()
        .content_type("application/json")
        .body(json)
}
```

### Binary Protocol

```rust
// Send over network
async fn send_message(msg: &Message) -> Result<()> {
    let bytes = to_msgpack(msg)?;
    socket.write_all(&bytes).await?;
    Ok(())
}

async fn recv_message() -> Result<Message> {
    let bytes = read_bytes().await?;
    from_msgpack(&bytes)
}
```

## Error Handling

```rust
match to_json(&data) {
    Ok(json) => send_response(json),
    Err(SerializationError::Json(e)) => {
        log::error!("JSON error: {}", e);
        error_response()
    }
    Err(e) => panic!("Unexpected error: {}", e),
}
```

## Best Practices

1. **Use JSON for configs**: Human-readable, easy to edit
2. **Use MessagePack for perf**: IPC, caching, binary protocols
3. **Handle errors gracefully**: Don't panic on deserialization
4. **Version your data**: Add version fields for compatibility
5. **Validate after deserialize**: Use validation crate

## Performance Tips

- MessagePack is ~2x faster than JSON
- MessagePack reduces size by 30-40%
- Use `to_json_bytes()` for I/O (avoids string allocation)
- Pre-allocate buffers for repeated serialization

## Integration Examples

### With Compression

```rust
use dev_engineeringlabs_rustboot_compress::*;

// Serialize → Compress
let json = to_json(&data)?;
let compressed = gzip_compress(json.as_bytes())?;

// Decompress → Deserialize
let decompressed = gzip_decompress(&compressed)?;
let data = from_json_bytes(&decompressed)?;
```

### With File I/O

```rust
use dev_engineeringlabs_rustboot_fileio::*;

// Atomic write
let json = to_json_pretty(&config)?;
write_atomic("config.json", json.as_bytes())?;
```


## Examples and Tests

> **⚠️ Required**: Every module must have working examples and tests to guide users.

### Examples

**Location**: [xamples/](../examples/) directory

**Current examples**:
- [serialization_basic.rs](../examples/serialization_basic.rs) - Basic usage demonstration
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
cargo test -p dev-engineeringlabs-rustboot-serialization
cargo run --example serialization_basic
```

---

**Status**: Stable  
**Roadmap**: See [backlog.md](../backlog.md)