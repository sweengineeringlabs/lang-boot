# Toolchain Overview

## WHAT: Development Tooling and Utilities

The `rustboot-toolchain` crate provides development tools and utilities for building Rustboot applications.

Key features:
- **Code generation** - Scaffolding and boilerplate generation
- **CLI tools** - Development commands
- **Build utilities** - Build-time helpers
- **Testing tools** - Test scaffolding and mocks

## WHY: Streamline Development Workflow

**Problems Solved**:
1. Repetitive boilerplate code
2. Inconsistent project structure
3. Manual scaffolding
4. Time-consuming setup tasks

**When to Use**: Development phase, scaffolding new features, generating code, and automating build tasks.

## HOW: CLI and Build Tools

### Code Generation (Planned)

```bash
# Generate new crate
rustboot-cli new-crate my-service

# Generate controller
rustboot-cli generate controller UserController
```

### Build Utilities (Planned)

```rust
use dev_engineeringlabs_rustboot_toolchain::*;

// Build-time code generation
```

**Planned Features**:
- Project scaffolding
- Code generators
- Migration tools
- Development CLI
- Build macros

---

**Status**: Planned  
**Backlog**: See [backlog.md](../backlog.md)


## Examples and Tests

> **⚠️ Required**: Every module must have working examples and tests to guide users.

### Examples

**Location**: [xamples/](../examples/) directory

**Current examples**:
- [toolchain_basic.rs](../examples/toolchain_basic.rs) - Basic usage demonstration
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
cargo test -p dev-engineeringlabs-rustboot-toolchain
cargo run --example toolchain_basic
```

---

**Status**: Stable  
**Roadmap**: See [backlog.md](../backlog.md)