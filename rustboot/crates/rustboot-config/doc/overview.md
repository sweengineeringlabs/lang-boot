# Configuration Management Overview

> **📝 Important**: This overview links to:
> - Working code examples in `examples/` directory
> - Integration tests in `tests/` directory  
> - Testing guides for developers

## WHAT: Multi-Format Configuration Loading

The `rustboot-config` crate provides flexible configuration management with support for multiple formats:

Key capabilities:
- **Multi-format support** - YAML, TOML, JSON, environment variables
- **Hierarchical merging** - Layer configurations (base → environment → secrets)
- **Type-safe** - Deserialize to Rust structs with serde
- **Flexible sources** - Files, environment variables, custom sources

## WHY: Centralized Configuration Management

**Problems Solved**:
1. **Scattered configuration** - Config spread across code, files, and environment
2. **Environment-specific duplication** - Repeated config for dev/staging/prod
3. **Unsafe secrets** - Hardcoded secrets in configuration files
4. **Type errors** - Runtime failures from invalid configuration values

**Impact if not addressed**:
- Configuration drift between environments
- Security vulnerabilities from exposed secrets
- Runtime crashes from type mismatches
- Difficult configuration management

**When to Use**: Any application needing configuration from files or environment variables, especially with multiple environments (dev/staging/prod).

**When NOT to Use**: For simple single-file configs with no environment variation (use direct file reading instead).

## HOW: Configuration Loading Guide

### Basic Example

```rust
use dev_engineeringlabs_rustboot_config::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct AppConfig {
    database_url: String,
    server_port: u16,
}

// Load from YAML
let config: AppConfig = ConfigLoader::new()
    .load(FileSource::auto("config.yaml")?)
    .unwrap()
    .build();
```

### Supported Formats

#### YAML
```yaml
# config.yaml
database:
  host: localhost
  port: 5432
server:
  port: 8080
```

#### TOML
```toml
# config.toml
[database]
host = "localhost"
port = 5432

[server]
port = 8080
```

#### JSON
```json
{
  "database": {
    "host": "localhost",
    "port": 5432
  },
  "server": {
    "port": 8080
  }
}
```

#### Environment Variables
```bash
APP_DATABASE_HOST=localhost
APP_DATABASE_PORT=5432
APP_SERVER_PORT=8080
```

### Hierarchical Merging

Load base config, then override with environment-specific values:

```rust
let env = std::env::var("APP_ENV").unwrap_or("dev".to_string());

let config: AppConfig = ConfigLoader::new()
    .load(FileSource::auto("config/base.yaml")?)
    .unwrap()
    .load(FileSource::auto(&format!("config/{}.yaml", env))?)
    .unwrap()
    .load(EnvSource::new(Some("APP".to_string())))
    .unwrap()
    .build();
```

**Available**:
- File sources (YAML, TOML, JSON)
- Environment variable source
- Automatic format detection
- Hierarchical merging

**Planned**:
- Remote configuration (etcd, Consul)
- Configuration validation
- Hot reloading
- Encrypted configuration values

### Custom Merging

Implement custom merge logic for your config types:

```rust
use dev_engineeringlabs_rustboot_config::Mergeable;

#[derive(Deserialize, Default)]
struct Config {
    name: String,
    value: i32,
}

impl Mergeable for Config {
    fn merge(&mut self, other: Self) {
        if !other.name.is_empty() {
            self.name = other.name;
        }
        if other.value != 0 {
            self.value = other.value;
        }
    }
}
```

### API Reference

#### `FileSource`
Load configuration from files.

**Methods**:
- `new(path, format)` - Create with explicit format
- `auto(path)` - Auto-detect from extension (.yaml, .toml, .json)

#### `EnvSource`
Load from environment variables.

**Methods**:
- `new(prefix)` - Create with optional prefix (e.g., "APP_")
- `with_separator(sep)` - Set separator (default: "_")

#### `ConfigLoader`
Hierarchical config loader.

**Methods**:
- `new()` - Create loader
- `load(source)` - Load from source
- `build()` - Build final merged config

### Error Handling

```rust
match ConfigLoader::new()
    .load(FileSource::auto("config.yaml")?)
    .unwrap()
    .try_build()
{
    Ok(config) => println!("Config loaded"),
    Err(ConfigError::FileLoad(e)) => eprintln!("File error: {}", e),
    Err(ConfigError::YamlParse(e)) => eprintln!("Parse error: {}", e),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Relationship to Other Modules

| Module/Component | Purpose | Relationship |
|------------------|---------|--------------|
| `rustboot-validation` | Input validation | Validates loaded configuration values |
| `rustboot-secrets` | Secret management | Loads secrets from environment |
| `rustboot-database` | Database access | Uses config for connection strings |
| `rustboot-http` | HTTP client | Uses config for API endpoints |

**Integration Points**:
- Configuration loaded at application startup
- Secrets masked in logs
- Validation applied to config values

## Examples and Tests

> **⚠️ Required**: Every module must have working examples and tests to guide users.

### Examples

**Location**: [`examples/`](../examples/) directory

**Current examples**:
- [`config_basic.rs`](../examples/config_basic.rs) - Basic configuration loading
- [`hierarchical_config.rs`](../examples/hierarchical_config.rs) - Multi-layer configuration

**Purpose**: Show users HOW to load and merge configurations from multiple sources.

### Tests

**Location**: [`tests/`](../tests/) directory

**Current tests**:
- [`integration.rs`](../tests/integration.rs) - Configuration loading and merging tests

### Testing Guidance

**For developers using this module**: See [Rust Test Organization](../../docs/4-development/guide/rust-test-organization.md)

**For contributors**: Run tests with:
```bash
cargo test -p dev-engineeringlabs-rustboot-config
cargo run --example config_basic
```

---

**Status**: Stable  
**Roadmap**: See [backlog.md](../backlog.md)
