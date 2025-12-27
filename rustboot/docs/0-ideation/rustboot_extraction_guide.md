# Rustboot Extraction - Implementation Guide

## Current State

**Rustratify** (`c:\phd-systems\rustratify`):
- ✅ Stable v2.0.0 build
- ✅ All framework code in `src/core/`
- ✅ 34 files, 4000+ lines

**Rustboot** (`c:\phd-systems\rustboot`):
- ⚠️ Created but outside workspace access
- Needs manual setup

---

## Step 1: Manual Rustboot Setup

Since I can't access `c:\phd-systems\rustboot`, you need to:

```powershell
cd c:\phd-systems\rustboot
git init
```

Create `Cargo.toml`:
```toml
[workspace]
resolver = "2"
members = [
    "rustboot",
    "crates/rustboot-validation",
    "crates/rustboot-cache",
    "crates/rustboot-di",
    "crates/rustboot-state-machine",
    "crates/rustboot-http",
    "crates/rustboot-messaging",
    "crates/rustboot-database",
    "crates/rustboot-middleware",
    "crates/rustboot-observability",
    "crates/rustboot-testing",
]

[workspace.package]
version = "0.1.0"
edition = "2021"
license = "MIT"
authors = ["Elvis Chidera <elvischidera@gmail.com>"]
repository = "https://github.com/phdsystems/rustboot"
```

---

## Step 2: Copy Framework Code

From `c:\phd-systems\rustratify`, copy these directories to Rustboot:

```powershell
# From rustratify root
xcopy /E /I src\core\validation ..\rustboot\crates\rustboot-validation\src
xcopy /E /I src\core\caching ..\rustboot\crates\rustboot-cache\src
xcopy /E /I src\core\di ..\rustboot\crates\rustboot-di\src
xcopy /E /I src\core\state_machine ..\rustboot\crates\rustboot-state-machine\src
xcopy /E /I src\core\http ..\rustboot\crates\rustboot-http\src
xcopy /E /I src\core\messaging ..\rustboot\crates\rustboot-messaging\src
xcopy /E /I src\core\database ..\rustboot\crates\rustboot-database\src
xcopy /E /I src\core\middleware ..\rustboot\crates\rustboot-middleware\src
xcopy /E /I src\core\observability ..\rustboot\crates\rustboot-observability\src
xcopy /E /I src\core\testing ..\rustboot\crates\rustboot-testing\src
```

---

## Step 3: Create Cargo.toml for Each Crate

### rustboot-validation/Cargo.toml
```toml
[package]
name = "rustboot-validation"
version.workspace = true
edition.workspace = true
license.workspace = true
authors.workspace = true
repository.workspace = true
description = "Type-safe validation framework"

[dependencies]
# Pure validation - no dependencies
```

### rustboot-cache/Cargo.toml
```toml
[package]
name = "rustboot-cache"
version.workspace = true
edition.workspace = true
license.workspace = true
authors.workspace = true
repository.workspace = true
description = "Caching with TTL support"

[dependencies]
thiserror = "1.0"
```

### rustboot-di/Cargo.toml
```toml
[package]
name = "rustboot-di"
version.workspace = true
edition.workspace = true
license.workspace = true
authors.workspace = true
repository.workspace = true
description = "Dependency injection container"

[dependencies]
# Pure DI - no dependencies
```

...(continue for all 10 crates)

---

## Step 4: Create lib.rs for Each Crate

Each crate needs a `src/lib.rs` that re-exports its modules. Example pattern:

```rust
// rustboot-validation/src/lib.rs
pub mod builder;
pub mod rules;
pub mod traits;

pub use builder::*;
pub use rules::*;
pub use traits::*;
```

---

## Step 5: Create Main Rustboot Crate

`rustboot/Cargo.toml`:
```toml
[package]
name = "rustboot"
version.workspace = true
edition.workspace = true
license.workspace = true
authors.workspace = true
repository.workspace = true
description = "Application framework with validation, caching, DI, and more"

[dependencies]
rustboot-validation = { version = "0.1", path = "../crates/rustboot-validation" }
rustboot-cache = { version = "0.1", path = "../crates/rustboot-cache" }
# ... all 10 crates
```

`rustboot/src/lib.rs`:
```rust
pub use rustboot_validation as validation;
pub use rustboot_cache as cache;
// ... re-export all

// Convenience flat re-exports
pub use rustboot_validation::*;
pub use rustboot_cache::*;
// ...
```

---

## Step 6: Clean Up Rustratify

Remove framework code from Rustratify:

```powershell
cd c:\phd-systems\rustratify

# Remove framework directories
Remove-Item -Recurse src\core\validation
Remove-Item -Recurse src\core\caching
Remove-Item -Recurse src\core\di
Remove-Item -Recurse src\core\state_machine
Remove-Item -Recurse src\core\http
Remove-Item -Recurse src\core\messaging
Remove-Item -Recurse src\core\database
Remove-Item -Recurse src\core\middleware
Remove-Item -Recurse src\core\observability
Remove-Item -Recurse src\core\testing
```

Update `src/core/mod.rs` to only keep:
```rust
pub mod config;
pub mod error;
pub mod registry;
pub mod stream;
```

Update `src/lib.rs` to remove framework re-exports.

---

## Step 7: Update Documentation

**Rustratify README**:
- Focus on SEA architecture
- Provider pattern example
- Link to Rustboot for framework features

**Rustboot README**:
- Application framework features
- Link to Rustratify for architecture pattern

---

## Summary

**What gets moved**: 10 framework modules (4000 lines)  
**What stays in Rustratify**: Provider, Registry, Config, Streams (~500 lines)

**Result**:
- Rustratify: Clean SEA reference
- Rustboot: Full application framework

---

## Next Actions

**Option A**: I help once Rustboot is in workspace  
**Option B**: You manually set up Rustboot following this guide  
**Option C**: Create Rustboot in rustratify workspace temporarily, then move

Which approach would you prefer?
