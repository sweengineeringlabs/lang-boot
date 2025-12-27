# rustboot-config

Configuration management for Rust applications.

## Features

- ✅ Load from YAML, TOML, JSON, environment variables
- ✅ Hierarchical config merging
- ✅ Type-safe deserialization
- ✅ Auto-format detection
- ✅ Prefix-based environment variables

## Quick Start

```toml
[dependencies]
dev-engineeringlabs-rustboot-config = "0.1"
```

```rust
use dev_engineeringlabs_rustboot_config::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Default)]
struct AppConfig {
    database_url: String,
    port: u16,
}

// Load from file
let config: AppConfig = ConfigLoader::new()
    .load(FileSource::auto("config.yaml")?)
    .unwrap()
    .build();

// Or merge multiple sources
let config: AppConfig = ConfigLoader::new()
    .load(FileSource::auto("base.yaml")?)
    .unwrap()
    .load(FileSource::auto("prod.yaml")?)
    .unwrap()
    .load(EnvSource::new(Some("APP".to_string())))
    .unwrap()
    .build();
```

## Documentation

- [Overview](docs/overview.md) - Detailed documentation
- [Examples](../../examples/) - Usage examples

## License

MIT
