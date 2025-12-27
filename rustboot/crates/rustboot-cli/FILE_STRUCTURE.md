# Rustboot CLI File Structure

This document provides a complete overview of the rustboot-cli crate structure.

## Complete Directory Tree

```
rustboot-cli/
│
├── Cargo.toml                      # Package manifest with dependencies
│
├── README.md                       # User-facing documentation
├── USAGE_EXAMPLES.md              # Detailed usage examples
├── DEVELOPMENT.md                 # Developer guide
├── FILE_STRUCTURE.md              # This file
│
├── src/
│   ├── main.rs                    # CLI entry point
│   │                              # - Defines CLI structure with Clap
│   │                              # - Routes commands to handlers
│   │                              # - Main function and error handling
│   │
│   └── commands/
│       ├── mod.rs                 # Module declarations
│       │
│       ├── new.rs                 # 'rustboot new' command
│       │                          # - Creates new Rustboot projects
│       │                          # - Validates project names
│       │                          # - Generates project structure
│       │                          # - Substitutes template placeholders
│       │
│       └── add.rs                 # 'rustboot add' command
│           │                      # - Adds features to existing projects
│           │                      # - Updates Cargo.toml dependencies
│           │                      # - Creates feature-specific files
│           │                      # - Provides integration instructions
│
├── templates/
│   │
│   ├── cargo.toml.template        # Project Cargo.toml template
│   │                              # - Rustboot dependencies
│   │                              # - Standard Rust project metadata
│   │
│   ├── main.rs.template           # Application entry point template
│   │                              # - Basic HTTP client example
│   │                              # - Logging setup with tracing
│   │                              # - Async runtime with Tokio
│   │
│   ├── config.toml.template       # Application configuration template
│   │                              # - Server settings
│   │                              # - Logging configuration
│   │                              # - HTTP client settings
│   │
│   ├── dockerfile.template        # Docker build configuration
│   │                              # - Multi-stage build
│   │                              # - Optimized for Rust projects
│   │                              # - Minimal runtime image
│   │
│   ├── deployment.yaml.template   # Kubernetes manifests
│   │                              # - Deployment configuration
│   │                              # - Service definition
│   │                              # - Health probes
│   │                              # - Resource limits
│   │
│   ├── gitignore.template         # Git ignore patterns
│   │                              # - Rust build artifacts
│   │                              # - IDE files
│   │                              # - Environment files
│   │
│   ├── readme.md.template         # Project README template
│   │                              # - Getting started guide
│   │                              # - Build instructions
│   │                              # - Deployment instructions
│   │
│   ├── database_setup.rs.template # Database module template
│   │                              # - Connection pool setup
│   │                              # - SQLx configuration
│   │                              # - Database initialization
│   │
│   ├── auth_middleware.rs.template # Authentication middleware template
│   │                               # - JWT verification
│   │                               # - Middleware implementation
│   │                               # - Authorization logic
│   │
│   └── api_models.rs.template     # API models template
│       │                          # - Request/response types
│       │                          # - Serde serialization
│       │                          # - OpenAPI-ready structures
│
└── tests/
    └── integration_test.rs         # Integration tests
        │                           # - Test project creation
        │                           # - Test feature addition
        │                           # - Verify generated files
```

## File Sizes (Approximate)

```
Templates:
  cargo.toml.template         ~540 bytes
  main.rs.template           ~800 bytes
  config.toml.template       ~150 bytes
  dockerfile.template        ~535 bytes
  deployment.yaml.template   ~1.1 KB
  gitignore.template         ~150 bytes
  readme.md.template         ~975 bytes
  database_setup.rs.template ~913 bytes
  auth_middleware.rs.template ~1.3 KB
  api_models.rs.template     ~824 bytes

Source Code:
  main.rs                    ~1.2 KB
  commands/new.rs            ~4.1 KB
  commands/add.rs            ~5.6 KB

Tests:
  integration_test.rs        ~3.4 KB

Binary:
  rustboot (debug)           ~15 MB
  rustboot (release)         ~1.2 MB
```

## Key Files Description

### Source Code

#### `src/main.rs`
- CLI entry point using Clap
- Defines command structure
- Routes to command handlers
- Simple and focused (~40 lines)

#### `src/commands/new.rs`
- Project scaffolding logic
- Template processing
- File system operations
- User feedback

#### `src/commands/add.rs`
- Feature addition logic
- Cargo.toml manipulation
- Dependency injection
- Feature-specific setup

### Templates

#### Project Templates
- `cargo.toml.template` - Rust package manifest
- `main.rs.template` - Application entry point
- `config.toml.template` - Runtime configuration
- `gitignore.template` - Git ignore rules
- `readme.md.template` - Project documentation

#### Deployment Templates
- `dockerfile.template` - Container build
- `deployment.yaml.template` - Kubernetes deployment

#### Feature Templates
- `database_setup.rs.template` - Database integration
- `auth_middleware.rs.template` - Authentication
- `api_models.rs.template` - API models

## Template Placeholders

All templates support the following placeholders:

- `{{project_name}}` - Replaced with the actual project name

Example:
```rust
// In template:
//! {{project_name}} - A Rustboot Application

// After substitution (project name: "my-api"):
//! my-api - A Rustboot Application
```

## Build Artifacts

When built, the CLI creates:

```
target/
├── debug/
│   └── rustboot              # Debug binary (~15 MB)
└── release/
    └── rustboot              # Release binary (~1.2 MB)
```

## Generated Project Structure

When you run `rustboot new myapp`, it creates:

```
myapp/
├── Cargo.toml                # Generated from cargo.toml.template
├── README.md                 # Generated from readme.md.template
├── config.toml               # Generated from config.toml.template
├── Dockerfile                # Generated from dockerfile.template
├── deployment.yaml           # Generated from deployment.yaml.template
├── .gitignore                # Generated from gitignore.template
└── src/
    └── main.rs               # Generated from main.rs.template
```

## Dependencies

```toml
[dependencies]
clap = { version = "4.5", features = ["derive"] }  # CLI framework
anyhow = "1.0"                                      # Error handling
thiserror = "1.0"                                   # Error types
serde = { version = "1.0", features = ["derive"] }  # Serialization
serde_json = "1.0"                                  # JSON support
toml = "0.8"                                        # TOML parsing
```

## Lines of Code

```
Source Code:
  src/main.rs:           ~45 lines
  src/commands/new.rs:   ~160 lines
  src/commands/add.rs:   ~195 lines
  Total:                 ~400 lines

Templates:
  Total:                 ~200 lines

Tests:
  integration_test.rs:   ~90 lines

Documentation:
  README.md:             ~180 lines
  USAGE_EXAMPLES.md:     ~380 lines
  DEVELOPMENT.md:        ~500 lines
  Total:                 ~1060 lines
```

## Template Details

### cargo.toml.template
Creates a Cargo.toml with:
- Project metadata (name, version, edition, license)
- Core rustboot dependencies (http, middleware, config, observability)
- Async runtime (tokio)
- Serialization (serde)
- Error handling (anyhow)
- Logging (tracing)

### main.rs.template
Creates a main.rs with:
- Tokio async runtime setup
- Tracing initialization
- HTTP client example
- Error handling
- Comments and documentation

### config.toml.template
Creates a config.toml with:
- Server configuration (host, port)
- Logging settings (level, format)
- HTTP settings (timeout, retries)

### dockerfile.template
Creates a Dockerfile with:
- Multi-stage build (builder + runtime)
- Rust 1.75 base image
- Debian runtime (bookworm-slim)
- Optimized for small image size
- Proper dependency caching

### deployment.yaml.template
Creates Kubernetes manifests with:
- Deployment with 3 replicas
- Service (LoadBalancer)
- Resource limits and requests
- Liveness and readiness probes
- Configurable environment variables

### database_setup.rs.template
Creates database module with:
- SQLx driver setup
- Connection pool configuration
- Environment variable support
- Error handling

### auth_middleware.rs.template
Creates auth module with:
- Middleware trait implementation
- JWT token verification (placeholder)
- Authorization header parsing
- Async middleware support

### api_models.rs.template
Creates models module with:
- Health check response
- Error response
- Example request/response types
- Serde serialization

## Testing Structure

### Integration Tests
- `test_new_project_creation()` - Verifies project scaffolding
- `test_add_database_feature()` - Tests database feature addition
- `test_invalid_project_name()` - Tests validation

Tests create temporary projects in `/tmp` and verify:
- Directory structure
- File existence
- File content
- Dependency injection
