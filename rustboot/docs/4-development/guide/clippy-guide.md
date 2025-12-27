# Clippy Guide for Rustboot

**Audience:** Developers contributing to Rustboot  
**Purpose:** Understand and use Clippy effectively for code quality and maintainability

---

## Table of Contents

- [What is Clippy?](#what-is-clippy)
- [Quick Start](#quick-start)
- [Lint Categories](#lint-categories)
- [Common Lints in Rustboot](#common-lints-in-rustboot)
- [Configuration](#configuration)
- [Best Practices](#best-practices)
- [Troubleshooting](#troubleshooting)

---

## What is Clippy?

Clippy is Rust's official linter that catches common mistakes and suggests idiomatic improvements. It provides over 600 lints organized into categories based on severity and purpose.

**Why we use it:**
- Catches bugs before they reach production
- Enforces consistent code style
- Suggests performance improvements
- Teaches Rust best practices

---

## Quick Start

### Running Clippy

```bash
# Basic clippy check
cargo clippy

# Check with all features enabled
cargo clippy --all-features

# Check only library code
cargo clippy --lib

# Treat warnings as errors (CI/CD mode)
cargo clippy -- -D warnings

# Fix auto-fixable issues
cargo clippy --fix
```

### Our Standard Command

For Rustboot development, always run:

```bash
cargo clippy --lib --all-features -- -D warnings
```

This ensures:
- ✅ All features are checked
- ✅ Warnings are treated as errors (strict mode)
- ✅ Only library code is checked (excludes flaky tests/examples)

---

## Lint Categories

Clippy organizes lints into categories. Understanding these helps you know what runs automatically and what you need to enable.

### Default Categories (Always Run)

#### 1. **clippy::all** ✅
Most common lints that catch typical mistakes.

**Examples:**
- `type_complexity` - Warns about overly complex types
- `needless_return` - Suggests removing unnecessary `return` statements
- `redundant_closure` - Simplifies closure syntax
- `unused_unit` - Removes unnecessary `()`

**Why it matters:** These are the most frequently encountered issues.

#### 2. **clippy::correctness** ✅
Critical lints that catch bugs or undefined behavior.

**Examples:**
- `out_of_bounds_indexing` - Array access that will panic
- `infinite_iter` - Iterators that never terminate
- `clone_double_ref` - Cloning references instead of values

**Why it matters:** These prevent runtime crashes and bugs.

#### 3. **clippy::style** ✅
Code style improvements that don't affect correctness.

**Examples:**
- `redundant_pattern_matching` - Use `if let` instead of `match`
- `collapsible_if` - Combine nested `if` statements
- `should_implement_trait` - Suggests implementing standard traits

**Why it matters:** Consistent, idiomatic Rust code.

#### 4. **clippy::complexity** ✅
Suggests simplifications to make code easier to understand.

**Examples:**
- `unnecessary_unwrap` - Remove redundant unwrap calls
- `needless_collect` - Skip intermediate collections
- `manual_filter_map` - Use `filter_map` instead of `filter().map()`

**Why it matters:** Simpler code is easier to maintain.

### Optional Categories (Must Enable)

#### 5. **clippy::pedantic** ❌
Very strict style enforcement. Not enabled by default.

**Enable with:**
```bash
cargo clippy -- -W clippy::pedantic
```

**Examples:**
- `must_use_candidate` - Suggests `#[must_use]` attribute
- `similar_names` - Warns about confusingly similar variable names
- `wildcard_imports` - Discourages `use foo::*`

**When to use:** For very clean, production-critical code.

#### 6. **clippy::restriction** ❌
Restricts certain patterns. Use selectively.

**Enable specific lints:**
```bash
cargo clippy -- -W clippy::indexing_slicing
```

**Examples:**
- `indexing_slicing` - Forbids `array[i]` (use `.get(i)`)
- `unwrap_used` - Forbids `.unwrap()` calls
- `panic` - Forbids `panic!()` macro

**When to use:** Safety-critical code or library APIs.

#### 7. **clippy::nursery** ❌
Experimental lints being tested. May have false positives.

**Enable with:**
```bash
cargo clippy -- -W clippy::nursery
```

**When to use:** To help test new lints (report bugs!).

#### 8. **clippy::cargo** ❌
Checks Cargo.toml and dependencies.

**Enable with:**
```bash
cargo clippy -- -W clippy::cargo
```

**Examples:**
- `multiple_crate_versions` - Detects duplicate dependencies
- `wildcard_dependencies` - Warns about `version = "*"`

**When to use:** CI/CD dependency audits.

### Summary Table

| Lint Group | Default? | Example Lints | When to Enable |
|------------|----------|---------------|----------------|
| `clippy::all` | ✅ YES | `type_complexity`, `needless_return` | Always (automatic) |
| `clippy::correctness` | ✅ YES | `out_of_bounds_indexing` | Always (automatic) |
| `clippy::style` | ✅ YES | `redundant_pattern` | Always (automatic) |
| `clippy::complexity` | ✅ YES | `needless_collect` | Always (automatic) |
| `clippy::pedantic` | ❌ NO | `must_use_candidate` | For very clean code |
| `clippy::restriction` | ❌ NO | `indexing_slicing` | For strict projects |
| `clippy::nursery` | ❌ NO | Experimental lints | For testing |
| `clippy::cargo` | ❌ NO | `multiple_crate_versions` | CI/CD checks |

**Key Takeaway:** The first 4 categories run automatically when you type `cargo clippy`. The last 4 require explicit enabling with `-W` flags.

---

## Common Lints in Rustboot

### Type Complexity

**Lint:** `clippy::type_complexity`  
**Category:** `clippy::all` (always runs)

**What it catches:**
```rust
// ❌ Too complex - hard to read
HashMap<(S, E), Box<dyn Fn(&S, &S) -> bool + Send + Sync>>
```

**How we fixed it:**
```rust
// ✅ Better - use type alias
type GuardFn<S> = Box<dyn Fn(&S, &S) -> bool + Send + Sync>;
HashMap<(S, E), GuardFn<S>>
```

**Location in codebase:** `rustboot-state-machine/src/machine.rs:24`

**Threshold:** Configurable in `.clippy.toml` (default: 250)

---

### Should Implement Trait

**Lint:** `clippy::should_implement_trait`  
**Category:** `clippy::style` (always runs)

**What it catches:**
```rust
// ❌ Method named after std trait
pub fn add(self, item: T) -> Self {
    // ... but doesn't implement std::ops::Add
}
```

**How we fixed it:**
```rust
// ✅ Renamed to avoid confusion
pub fn with_validator(self, validator: impl Validator<T>) -> Self {
    self.validators.push(Box::new(validator));
    self
}
```

**Location in codebase:** 
- `rustboot-validation/src/traits.rs:176`
- `rustboot-middleware/src/chain.rs:28`

**Why:** Follows Rust builder pattern conventions (`with_*` methods).

---

### Unused Imports

**Lint:** `unused_imports`  
**Category:** Rust compiler warning (promoted to error with `-D warnings`)

**What it catches:**
```rust
// ❌ Import not used
use std::collections::HashMap;

fn foo() {
    // No HashMap usage
}
```

**How we fixed it:**
```rust
// ✅ Removed completely
fn foo() {
    // Clean!
}
```

**Why it matters:** Clean imports = clear dependencies.

---

### Redundant Closure

**Lint:** `clippy::redundant_closure`  
**Category:** `clippy::complexity` (always runs)

**What it catches:**
```rust
// ❌ Unnecessary wrapper
let chars: Vec<u8> = input.chars().map(|c| from_base64_char(c)).collect();
```

**How we fixed it:**
```rust
// ✅ Direct function reference
let chars: Vec<u8> = input.chars().map(from_base64_char).collect();
```

**Location in codebase:** `rustboot-security/src/auth.rs:138`

---

### Unwrap or Default

**Lint:** `clippy::unwrap_or_default`  
**Category:** `clippy::complexity` (always runs)

**What it catches:**
```rust
// ❌ Verbose
map.entry(key).or_insert_with(Vec::new)
```

**How we fixed it:**
```rust
// ✅ Simpler
map.entry(key).or_default()
```

**Location in codebase:** `rustboot-messaging/src/bus.rs:132`

---

## Configuration

### Project Configuration File

Create `.clippy.toml` in the project root:

```toml
# Type complexity threshold (default: 250)
# Lower = stricter, catches simpler complex types
type-complexity-threshold = 200

# Cognitive complexity threshold (default: 25)
# Lower = catches simpler complex functions
cognitive-complexity-threshold = 20

# Disallow certain methods (security-critical projects)
disallowed-methods = [
    # Example: forbid direct .unwrap() in production code
    # "core::option::Option::unwrap",
]

# Allow certain noisy lints
# Use sparingly - prefer fixing the code
too-many-arguments-threshold = 7
```

### Per-File Configuration

```rust
// Disable specific lints for entire file
#![allow(clippy::type_complexity)]

// Enable stricter lints
#![warn(clippy::pedantic)]
```

### Per-Item Configuration

```rust
// Disable for specific function/struct
#[allow(clippy::type_complexity)]
pub struct ComplexType {
    // ...
}

// Multiple lints
#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn complex_function(...) { }
```

---

## Best Practices

### 1. **Always Fix, Don't Suppress**

```rust
// ❌ BAD - Hiding the problem
#[allow(clippy::type_complexity)]
type Bad = HashMap<String, Vec<Box<dyn Trait>>>;

// ✅ GOOD - Solving the problem
type Handler = Box<dyn Trait>;
type HandlerMap = HashMap<String, Vec<Handler>>;
```

### 2. **Use Descriptive Names When Fixing**

```rust
// ❌ Lazy naming
type T1 = Box<dyn Fn(&str) -> bool>;
type T2 = HashMap<String, T1>;

// ✅ Clear intent
type ValidationFn = Box<dyn Fn(&str) -> bool>;
type ValidatorMap = HashMap<String, ValidationFn>;
```

### 3. **Understand Before Allowing**

```rust
// ❌ Suppressing without understanding
#[allow(clippy::must_use_candidate)]  // Why? Should this return Result?
pub fn important_operation() -> bool { }

// ✅ Intentional decision with comment
// We don't use #[must_use] because this is a pure check
// that doesn't mutate state or have side effects
#[allow(clippy::must_use_candidate)]
pub fn is_valid(&self) -> bool { }
```

### 4. **Keep Test Code Clean Too**

```rust
// ❌ Sloppy test code
#[test]
fn test_something() {
    assert!(true);  // Useless assertion
}

// ✅ Meaningful tests
#[test]
fn test_validation_rejects_empty_string() {
    let validator = NotEmptyValidator::new("field");
    assert!(validator.validate(&String::new()).is_err());
}
```

### 5. **Run Clippy in CI/CD**

```yaml
# .github/workflows/ci.yml
- name: Run Clippy
  run: cargo clippy --lib --all-features -- -D warnings
```

This ensures no warnings slip through.

---

## Troubleshooting

### "Clippy complains but I disagree"

**Steps:**
1. **Read the explanation:** `rustc --explain E0308` or visit [Clippy Lints](https://rust-lang.github.io/rust-clippy/)
2. **Check if there's a better way** (usually there is!)
3. **If you're certain**, document why and suppress:
   ```rust
   // SAFETY: This pattern is required for FFI compatibility
   #[allow(clippy::not_unsafe_ptr_arg_deref)]
   ```

### "Too many warnings to fix at once"

**Approach:**
1. Fix category by category:
   ```bash
   # Start with correctness (bugs)
   cargo clippy -- -W clippy::correctness
   
   # Then complexity
   cargo clippy -- -W clippy::complexity
   
   # Finally style
   cargo clippy -- -W clippy::style
   ```

2. Or allow temporarily and create issues:
   ```rust
   #![allow(clippy::type_complexity)]  // TODO: Issue #123
   ```

### "Clippy is slow"

**Solutions:**
1. Check only changed files:
   ```bash
   cargo clippy --no-deps
   ```

2. Use incremental compilation (default).

3. Run on specific packages:
   ```bash
   cargo clippy -p rustboot-validation
   ```

### "False positives"

**Actions:**
1. Update Clippy: `rustup update`
2. Check if it's a known issue: [Clippy Issues](https://github.com/rust-lang/rust-clippy/issues)
3. Report it if new
4. Suppress with explanation:
   ```rust
   // False positive: clippy#12345
   #[allow(clippy::suspicious_else_formatting)]
   ```

---

## Quick Reference

### Common Commands

```bash
# Standard check
cargo clippy

# Strict mode (our standard)
cargo clippy --lib --all-features -- -D warnings

# Auto-fix
cargo clippy --fix

# Check specific lint
cargo clippy -- -W clippy::type_complexity

# Explain a lint
cargo clippy -- --explain type_complexity

# List all lints
cargo clippy -- -W help
```

### Lint Levels

```rust
// More permissive ←→ More strict
#[allow(clippy::foo)]    // Suppress warning
#[warn(clippy::foo)]     // Show warning (default)
#[deny(clippy::foo)]     // Error on violation
#[forbid(clippy::foo)]   // Error, can't be overridden
```

### Where to Learn More

- **Clippy Lints List:** https://rust-lang.github.io/rust-clippy/
- **Clippy Book:** https://doc.rust-lang.org/clippy/
- **Rustboot Examples:** Search codebase for `#[allow(clippy::`

---

## Changelog

- **2025-12-23:** Initial version
- Future: Add project-specific lint rules as we discover patterns

---

**Questions?** Ask in team discussions or create an issue with the `documentation` label.
