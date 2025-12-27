# 2. Modular Crate Structure

**Status**: Accepted

**Date**: 2025-12-24

**Decision Makers**: Framework Architecture Team

## Context

When designing the rustboot framework, we needed to decide on the fundamental organization of the codebase. The main question was: should we build a monolithic crate with all features, or a modular workspace with separate crates?

Key considerations:
1. **Framework Scope**: Rustboot aims to provide comprehensive building blocks for backend applications (HTTP, database, messaging, caching, validation, state machines, resilience, etc.)
2. **User Needs**: Different applications need different subsets of functionality
3. **Compilation Time**: Rust's compilation times can be significant for large codebases
4. **Dependency Management**: How to handle optional vs required dependencies
5. **Maintenance**: Team ability to work on different components independently
6. **Versioning**: How to handle breaking changes in different components

Current architecture spans 28 separate crates:
- Core infrastructure: `rustboot-di`, `rustboot-config`, `rustboot-async`
- Web/HTTP: `rustboot-http`, `rustboot-web`, `rustboot-middleware`
- Data: `rustboot-database`, `rustboot-cache`, `rustboot-serialization`
- Messaging: `rustboot-messaging`, `rustboot-streams`
- Resilience: `rustboot-resilience`, `rustboot-ratelimit`, `rustboot-state-machine`
- Utilities: `rustboot-validation`, `rustboot-crypto`, `rustboot-datetime`, etc.

## Decision

We will organize rustboot as a **Cargo workspace with multiple focused crates**, each providing a specific domain of functionality.

Architecture principles:
1. **One Concern Per Crate**: Each crate addresses a single domain (HTTP, database, messaging, etc.)
2. **Clear Dependencies**: Crates can depend on each other with explicit relationships
3. **Optional Integration**: Features enable integration between crates (e.g., database pooling, messaging backends)
4. **Workspace Management**: Shared version, edition, license, and common dependencies
5. **Facade Crate**: A top-level `rustboot` crate can re-export commonly used items

Directory structure:
```
rustboot/
├── Cargo.toml (workspace)
├── rustboot/ (facade crate - optional)
└── crates/
    ├── rustboot-http/
    ├── rustboot-database/
    ├── rustboot-messaging/
    ├── rustboot-middleware/
    └── ... (28 total crates)
```

## Consequences

### Positive

- **Compile Time Optimization**: Users only compile what they use, significantly reducing build times
- **Clear Boundaries**: Each crate has well-defined responsibilities and API surface
- **Independent Versioning**: Can version crates independently if needed (though we use workspace versioning)
- **Parallel Development**: Teams can work on different crates without conflicts
- **Dependency Hygiene**: Each crate's dependencies are explicit and minimal
- **Testing Isolation**: Can test crates independently, faster test iterations
- **Documentation Clarity**: Each crate has focused documentation on docs.rs
- **Feature Composition**: Users can mix and match only the features they need
- **Easier Maintenance**: Smaller codebases per crate are easier to understand
- **Security Updates**: Can patch individual crates without full framework release
- **Progressive Adoption**: Users can adopt rustboot gradually, crate by crate

### Negative

- **Dependency Management Complexity**: Managing inter-crate dependencies requires care
- **Version Synchronization**: Must coordinate versions across crates (mitigated by workspace)
- **Discovery Overhead**: Users must find and choose the right crates
- **Import Verbosity**: More `use` statements across multiple crates
- **Workspace Overhead**: Initial setup and CI/CD configuration more complex
- **Cross-Crate Refactoring**: Changing interfaces affects multiple crates
- **Documentation Fragmentation**: Documentation spread across multiple crates
- **Release Coordination**: Must decide whether to release all crates together

### Neutral

- **CI/CD Complexity**: Need to test, build, and publish multiple crates (but better caching)
- **Learning Curve**: Developers need to understand workspace organization
- **Monorepo vs Polyrepo**: Using workspace (monorepo) vs separate repos (polyrepo)

## Alternatives Considered

### 1. Single Monolithic Crate with Feature Flags

**Approach**: One `rustboot` crate with features like `http`, `database`, `messaging`, etc.

```toml
[features]
default = []
http = ["dep:reqwest"]
database = ["dep:sqlx"]
messaging = ["dep:kafka", "dep:rabbitmq"]
full = ["http", "database", "messaging", ...]
```

**Rejected because**:
- **Compilation Time**: Users would compile entire codebase even with selective features
- **Feature Explosion**: Combinatorial explosion of feature flags (2^n combinations to test)
- **Tight Coupling**: All code in one crate encourages internal dependencies
- **Documentation Overload**: Single docs page with all features is overwhelming
- **Dependency Conflicts**: All dependencies in one Cargo.toml, higher chance of conflicts
- **Testing Burden**: Must test all feature combinations, exponentially complex

### 2. Completely Separate Repositories

**Approach**: Each component in its own Git repository with independent releases.

**Rejected because**:
- **Coordination Overhead**: Cross-repo changes require multiple PRs
- **Version Hell**: Users must manually coordinate compatible versions
- **Code Sharing**: Difficult to share common code/utilities
- **Atomic Changes**: Can't make atomic changes across components
- **CI Complexity**: Need separate CI for each repo
- **Contributor Friction**: Higher barrier for contributors to work across components

### 3. Hybrid: Core + Plugins

**Approach**: One core crate with essential features, separate crates for optional plugins.

```
rustboot-core/  (includes HTTP, database, config)
rustboot-kafka/
rustboot-redis/
rustboot-postgres/
```

**Rejected because**:
- **Core Bloat**: Core would grow over time, defeating the purpose
- **Arbitrary Boundaries**: Difficult to decide what's "core" vs "plugin"
- **Inconsistent Experience**: Different patterns for core vs plugins
- **Still Couples Core**: Changes to core affect all users

### 4. Layer-Based Organization

**Approach**: Organize by architectural layers (e.g., `rustboot-l1-foundation`, `rustboot-l2-core`).

**Rejected because**:
- **Less Intuitive**: Developers think in domains (HTTP, database) not layers
- **Unclear Boundaries**: What layer does validation belong to?
- **Documentation Challenge**: Harder to explain and discover
- **Domain Knowledge**: Layers don't match how developers search for functionality

## Implementation Guidelines

### Workspace Configuration

All crates share common workspace metadata:

```toml
[workspace.package]
version = "0.1.0"
edition = "2021"
license = "MIT"
authors = ["Elvis Chidera <elvischidera@gmail.com>"]
repository = "https://github.com/phdsystems/rustboot"

[workspace.dependencies]
async-trait = "0.1"
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["sync"] }
thiserror = "1.0"
```

### Crate Naming Convention

- **Package Name**: `dev-engineeringlabs-rustboot-{component}` (for crates.io uniqueness)
- **Directory Name**: `rustboot-{component}` (for local development clarity)
- **Module Name**: Users import as `rustboot_{component}`

### Dependency Strategy

1. **Minimal Dependencies**: Each crate includes only required dependencies
2. **Workspace Dependencies**: Use workspace for common deps to ensure version consistency
3. **Optional Features**: Use features for optional integrations (e.g., `sqlx-postgres`, `pool-bb8`)
4. **Peer Dependencies**: Document when crates work together but don't force dependencies

### Versioning Strategy

Currently using **synchronized versioning**:
- All crates share the same version number (0.1.0)
- Releases are coordinated across the workspace
- Breaking changes in one crate trigger a workspace-wide major version bump

Future consideration: **Independent versioning** if crates mature at different rates.

## Real-World Examples

Similar approaches in the Rust ecosystem:

- **Tokio**: Multiple crates (`tokio`, `tokio-util`, `tokio-stream`, `tokio-macros`)
- **Tower**: Ecosystem of crates (`tower`, `tower-http`, `tower-service`)
- **Actix**: `actix-web`, `actix-rt`, `actix-cors`, etc.
- **SQLx**: Core + driver crates pattern
- **Serde**: `serde`, `serde_json`, `serde_derive`

## Migration Path

If we later need to consolidate:

1. **Create facade crate**: Re-export from specialized crates
2. **Feature mapping**: Map monolithic features to crate dependencies
3. **Deprecation period**: Maintain both paths during transition
4. **Documentation update**: Guide users through migration

## References

- [Cargo Workspaces Documentation](https://doc.rust-lang.org/cargo/reference/workspaces.html)
- [API Guidelines: Crate Organization](https://rust-lang.github.io/api-guidelines/)
- [Tokio's Architecture](https://github.com/tokio-rs/tokio)
- [The Rust Performance Book: Compilation Time](https://nnethercote.github.io/perf-book/compile-times.html)

---

**Related ADRs**:
- [ADR-0003: Trait-Based Abstractions](./0003-trait-based-abstractions.md)
