# [Module/Component Name] Overview

> **📝 Important**: This overview must link to:
> - Working code examples in `examples/` directory
> - Integration tests in `tests/` directory  
> - Testing guides for developers
> 
> See "Examples and Tests" section below.

## WHAT: [Brief Description]

[1-2 sentence description of what this module/component provides]

Key capabilities:
- **Capability 1** - Brief description
- **Capability 2** - Brief description
- **Capability 3** - Brief description

## WHY: [Problem Statement]

**Problems Solved**:
1. [Problem 1] - [Impact]
2. [Problem 2] - [Impact]
3. [Problem 3] - [Impact]

**When to Use**: [Describe scenarios where this module/component should be used]

**When NOT to Use**: [Edge cases or alternative solutions]

## HOW: [Usage Guide]

### Basic Example

```
// Pseudocode or language-agnostic example
module.initialize(configuration)
result = module.performAction(input)
```

### Feature/Capability 1

[Explanation of the feature]

```
// Example usage
feature1.execute(parameters)
```

**Available**:
- [Implemented functionality]
- [Implemented functionality]

**Planned**:
- [Future functionality]
- [Future functionality]

### Feature/Capability 2

[Explanation of the feature]

```
// Example usage
feature2.configure(options)
```

**Available**:
- [Implemented functionality]

**Planned**:
- [Future functionality]

## Relationship to Other Modules

| Module/Component | Purpose | Relationship |
|------------------|---------|--------------|
| [Module A] | [Purpose] | [Dependency/Integration] |
| [Module B] | [Purpose] | [Dependency/Integration] |

**Integration Points**:
- [Where this module interfaces with others]
- [Data flows or dependencies]

## Examples and Tests

> **⚠️ Required**: Every module must have working examples and tests to guide users.

### Examples

**Location**: [`examples/`](../examples/) directory

**Required files**:
- `basic.rs` - Minimal working example (always create this)
- `[feature].rs` - One example per major feature
- `advanced.rs` - Complex use cases (if applicable)

**Purpose**: Show users HOW to use your module in real applications.

**Current examples**:
- [`basic.rs`](../examples/basic.rs) - [Describe what it demonstrates]
- [List all other examples with descriptions]

### Tests

**Location**: [`tests/`](../tests/) directory

**Required files**:
- `integration.rs` - Integration tests using public API

**Purpose**: Show users HOW to test code that uses your module.

**Current tests**:
- [`integration.rs`](../tests/integration.rs) - [Describe test scenarios]

### Testing Guidance

**For developers using this module**: See [Rust Test Organization](../../docs/4-development/guide/rust-test-organization.md)

**For contributors**: Run tests with:
```bash
cargo test --manifest-path [path-to-crate]/Cargo.toml
cargo run --example basic
```

---

**Status**: [Implementation status - e.g., Stable, Beta, Planned]  
**Roadmap**: See [backlog.md](../backlog.md) or [ROADMAP.md](../ROADMAP.md)
