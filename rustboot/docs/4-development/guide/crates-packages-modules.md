# Crates, Packages, and Modules Guide

A comprehensive guide to understanding Rust's code organization in the Rustboot framework.

## Table of Contents
- [Terminology](#terminology)
- [Packages vs Crates vs Modules](#packages-vs-crates-vs-modules)
- [Visibility and Accessibility](#visibility-and-accessibility)
- [Transitive Dependencies](#transitive-dependencies)
- [Best Practices](#best-practices)

---

## Terminology

### Package
A **package** is a Cargo project containing one or more crates. Defined by `Cargo.toml`.

```
rustboot-cache/          ← Package/Crate
├── Cargo.toml          (package definition)
└── src/
    └── lib.rs          (crate root)
```

### Crate
A **crate** is a compilation unit - the smallest amount of code the Rust compiler considers at a time. Two types:
- **Library crate**: Has `src/lib.rs` (used by other code)
- **Binary crate**: Has `src/main.rs` (executable)

**Package name** (in Cargo.toml): Uses hyphens
```toml
name = "dev-engineeringlabs-rustboot-cache"
```

**Crate reference** (in code): Hyphens become underscores
```rust
use dev_engineeringlabs_rustboot_cache::*;
```

### Module
A **module** is a namespace for organizing code within a crate.

```rust
pub mod cache {              // Module
    pub mod backend {        // Sub-module
        pub fn store() {}    // Function in module
    }
}
```

---

## Packages vs Crates vs Modules

### Hierarchy

```
📦 Package (Cargo project)
  └─ 📚 Crate (compilation unit)
      └─ 📁 Module (namespace)
          └─ 📁 Sub-module
              └─ 🔧 Functions, Types, etc.
```

### Example: Rustboot Cache

```
rustboot-cache/                                    ← Package
├── Cargo.toml
│   [package]
│   name = "dev-engineeringlabs-rustboot-cache"   ← Crate name
└── src/
    └── lib.rs                                     ← Crate root
        pub mod cache {                            ← Module
            pub mod backend {                      ← Sub-module
                pub fn get() {}
            }
        }
```

**Usage**:
```rust
use dev_engineeringlabs_rustboot_cache::cache::backend::get;
//  └─────────── crate ────────────┘ └─ mod ─┘ └─ mod ─┘
```

---

## Visibility and Accessibility

### Crate-Level Accessibility

**1. Publishing Control**
```toml
[package]
name = "my-crate"
publish = false              # Private, not on crates.io
# OR
publish = ["my-registry"]    # Only specific registry
```

**2. Workspace-Only Crates**
```toml
# Workspace Cargo.toml
[workspace]
members = ["internal-crate"]  # Not published, internal use
```

### Code-Level Visibility

| Modifier | Visibility Scope | Example |
|----------|------------------|---------|
| `pub` | Public API (everyone) | `pub fn api()` |
| `pub(crate)` | Current crate only | `pub(crate) fn internal()` |
| `pub(super)` | Parent module only | `pub(super) fn helper()` |
| `pub(in path)` | Specific module path | `pub(in crate::cache) fn util()` |
| (none) | Private to current module | `fn private()` |

### Example: Scoped Visibility

```rust
// src/lib.rs
pub mod cache {
    pub mod backend {
        pub fn public_api() {}           // ✅ Anyone
        
        pub(crate) fn crate_only() {}    // 🔒 This crate only
        
        pub(super) fn parent_only() {}   // 🔒 `cache` module only
        
        fn private() {}                  // 🔒 `backend` module only
    }
    
    pub mod frontend {
        use super::backend;
        
        pub fn use_backend() {
            backend::public_api();     // ✅ Works
            backend::crate_only();     // ✅ Works (same crate)
            backend::parent_only();    // ❌ Error! (wrong parent)
            backend::private();        // ❌ Error! (private)
        }
    }
}
```

---

## Transitive Dependencies

### How Cargo Handles Dependencies

**Your app**:
```toml
[dependencies]
dev-engineeringlabs-rustboot = "0.1"
```

**Dependency chain**:
```
Your App
  └─> dev-engineeringlabs-rustboot (facade)
        ├─> dev-engineeringlabs-rustboot-cache
        │     └─> thiserror (transitive!)
        └─> dev-engineeringlabs-rustboot-http
              ├─> async-trait (transitive!)
              ├─> serde (transitive!)
              └─> serde_json (transitive!)
```

**Cargo automatically**:
- ✅ Fetches all transitive dependencies
- ✅ Resolves version conflicts
- ✅ Deduplicates same packages
- ✅ Tracks exact versions in `Cargo.lock`

### Version Resolution

If multiple crates need the same dependency:
```
cache needs serde 1.0.200
http needs serde 1.0.228
```

**Cargo picks**: Compatible version satisfying both (usually latest: `1.0.228`)

### Visibility of Transitive Dependencies

**By default, transitive deps are PRIVATE**:
```rust
// Your app
use dev_engineeringlabs_rustboot::cache::*;  // ✅ Works

use thiserror::Error;  // ❌ Error! thiserror is private
```

**To expose a transitive dependency**:
```rust
// In rustboot-cache/src/lib.rs
pub use thiserror;  // Now users can access it
```

---

## Best Practices

### 1. Choosing Dependencies

**Option A: Individual Crates** (Minimal binary size)
```toml
[dependencies]
dev-engineeringlabs-rustboot-cache = "0.1"
dev-engineeringlabs-rustboot-validation = "0.1"
```

**Option B: Main Facade** (Convenience)
```toml
[dependencies]
dev-engineeringlabs-rustboot = "0.1"
```

⚠️ **Don't mix!** Avoid duplicates:
```toml
# ❌ BAD - cache included twice!
dev-engineeringlabs-rustboot = "0.1"
dev-engineeringlabs-rustboot-cache = "0.1"
```

### 2. Module Organization

**Keep modules focused**:
```rust
// ✅ Good
pub mod cache {
    pub mod backend;
    pub mod frontend;
}

// ❌ Too deep
pub mod cache {
    pub mod storage {
        pub mod backend {
            pub mod impl_details {
                // Too nested!
            }
        }
    }
}
```

### 3. Visibility Guidelines

```rust
// Start private, make public as needed
fn helper() {}                    // Default: private

pub(crate) fn internal_api() {}   // Crate-wide utility

pub fn public_api() {}            // External API
```

### 4. Re-exports

**Flatten complex hierarchies**:
```rust
// Internal structure
mod internal {
    pub mod deep {
        pub mod nested {
            pub struct ImportantType;
        }
    }
}

// Public API (flattened)
pub use internal::deep::nested::ImportantType;

// Users see:
use my_crate::ImportantType;  // Not my_crate::internal::deep::nested::!
```

---

## Rustboot Architecture

### Workspace Structure

```
rustboot/
├── Cargo.toml                    # Workspace definition
├── rustboot/
│   ├── Cargo.toml               # Main facade crate
│   └── src/lib.rs               # Re-exports all sub-crates
└── crates/
    ├── rustboot-validation/
    ├── rustboot-cache/
    ├── rustboot-di/
    └── ...                       # 9 independent crates
```

### Dependency Graph

```
Individual     ┌─────────────────────────────────┐
Crates ───────►│  All 9 crates are independent   │
(standalone)   │  No inter-crate dependencies    │
               └─────────────────────────────────┘
                          ▲
                          │
                          │ Re-exports
                          │
                  ┌───────┴────────┐
Main Facade ─────┤  rustboot      │ Convenience wrapper
(all features)   │  (facade)      │ Users import from here
                  └────────────────┘
```

### Usage Patterns

**Direct crate reference**:
```rust
use dev_engineeringlabs_rustboot_cache::InMemoryCache;
```

**Via facade (recommended)**:
```rust
use dev_engineeringlabs_rustboot::cache::InMemoryCache;
// OR
use dev_engineeringlabs_rustboot::prelude::*;
```

---

## Common Questions

### Q: Why hyphens in Cargo.toml but underscores in code?

**A**: Rust convention. Package names use hyphens (kebab-case), but Rust identifiers can't contain hyphens, so they become underscores.

```toml
name = "my-awesome-crate"  # Cargo.toml
```
```rust
use my_awesome_crate;       // Code
```

### Q: Can I build crates independently?

**A**: Yes! Each Rustboot crate is standalone:
```bash
cd crates/rustboot-cache
cargo build  # Works independently
```

### Q: What's the difference between workspace and package?

**A**:
- **Workspace**: Container for multiple packages (one `Cargo.toml` at root)
- **Package**: Single crate with its own `Cargo.toml`

### Q: How do I make a crate private?

**A**: Add to Cargo.toml:
```toml
[package]
publish = false
```

---

## Further Reading

- [The Rust Book - Packages and Crates](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html)
- [The Rust Book - Modules](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html)
- [Cargo Book - Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)
- [Cargo Book - Dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)
