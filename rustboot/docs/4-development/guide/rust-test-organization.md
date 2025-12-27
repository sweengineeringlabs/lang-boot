# Idiomatic Rust Test Organization

**Audience**: Developers coming from Java, C#, or other languages

## WHAT: Inline Tests with Conditional Compilation

Rust organizes tests differently from Java:

- **Unit tests**: Inline with code using `#[cfg(test)]` modules
- **Integration tests**: Separate `tests/` directory for public API verification
- **Documentation tests**: Code examples in doc comments

```
crate/
├── src/
│   ├── lib.rs              # Code + inline unit tests
│   ├── auth.rs             # Code + inline unit tests
│   └── authz.rs            # Code + inline unit tests
└── tests/
    └── integration.rs      # Integration tests only
```

**Not like Java**:
```
❌ src/main/    (production code)
❌ src/test/    (tests separated)
```

**Rust way**:
```
✅ src/         (code + inline tests)
✅ tests/       (integration only)
```

## WHY: Three Core Benefits

### 1. Zero Runtime Cost

**Problem in Java**: Tests always compiled, filtered at runtime
```java
// Always in classpath, even in production
public class UserServiceTest { }
```

**Solution in Rust**: Tests compiled only when testing
```rust
#[cfg(test)]  // Only compiled with `cargo test`
mod tests {
    #[test]
    fn test_user_service() { }
}
```

**Result**: Production binaries have **zero test overhead**.

### 2. Test Private Functions

**Problem in Java**: Can't test package-private/private methods
```java
class UserService {
    private void validateUser() { }  // ❌ Can't test without reflection
}
```

**Solution in Rust**: Tests in same file have full access
```rust
fn validate_user() { }  // Private function

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_user() {
        validate_user();  // ✅ Can test private!
    }
}
```

**Result**: No reflection hacks needed.

### 3. Co-Located Context

**Problem**: Context switching between files, hard to keep in sync

**Solution**: Tests next to code they verify
- Tests updated when code changes
- Tests deleted when code deleted
- No orphaned test files

## HOW: Organizing Tests

### Unit Tests (Inline)

```rust
// src/auth.rs
pub fn generate_jwt(user_id: &str, duration: Duration) -> Result<String> {
    validate_user_id(user_id)?;
    create_token(user_id, duration)
}

// Private helper
fn validate_user_id(user_id: &str) -> Result<()> {
    if user_id.is_empty() {
        return Err("Invalid user ID");
    }
    Ok(())
}

// ============================================================================
// TESTS - Only compiled during `cargo test`
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;  // Import parent module
    
    #[test]
    fn test_generate_jwt_success() {
        let result = generate_jwt("user123", Duration::from_secs(3600));
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_validate_user_id_rejects_empty() {
        // Can test private function!
        assert!(validate_user_id("").is_err());
    }
}
```

### Integration Tests (Separate)

```rust
// tests/integration.rs - Tests public API only
use my_crate::*;

#[test]
fn full_authentication_flow() {
    let token = generate_jwt("user", Duration::from_secs(3600)).unwrap();
    let claims = validate_jwt(&token).unwrap();
    assert_eq!(claims.sub, "user");
}
```

### Organizing Large Test Suites

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // Shared test fixtures
    fn create_test_user() -> User {
        User { id: "test".to_string() }
    }
    
    // Group by function being tested
    mod generate_jwt_tests {
        use super::*;
        
        #[test]
        fn valid_input() { /* ... */ }
        
        #[test]
        #[should_panic(expected = "Invalid")]
        fn invalid_duration() { /* ... */ }
    }
    
    mod validate_jwt_tests {
        use super::*;
        
        #[test]
        fn valid_token() { /* ... */ }
        
        #[test]
        fn expired_token() { /* ... */ }
    }
}
```

### Test Attributes

```rust
#[test]
fn basic_test() { }

#[test]
#[should_panic]  // Expect panic
fn test_that_panics() { }

#[test]
#[ignore]  // Skip unless --include-ignored
fn expensive_test() { }

#[test]
#[cfg(target_os = "linux")]  // Platform-specific
fn linux_only_test() { }
```

### Async Tests

```rust
// Cargo.toml
[dev-dependencies]
tokio = { version = "1", features = ["test-util"] }

// Test
#[tokio::test]
async fn test_async_function() {
    let result = async_operation().await;
    assert!(result.is_ok());
}
```

### Parameterized Tests

```rust
// Cargo.toml
[dev-dependencies]
rstest = "0.18"

// Test
use rstest::rstest;

#[rstest]
#[case("valid@email.com", true)]
#[case("invalid", false)]
#[case("@example.com", false)]
fn test_email_validation(#[case] input: &str, #[case] expected: bool) {
    assert_eq!(is_valid_email(input), expected);
}
```

### Running Tests

```bash
# All tests (unit + integration)
cargo test

# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test integration

# Specific test
cargo test test_generate_jwt

# With output (show println!)
cargo test -- --nocapture

# Include ignored tests
cargo test -- --include-ignored
```

## Comparison: Java vs Rust

| Aspect | Java | Rust |
|--------|------|------|
| **Unit test location** | `src/test/java/` | Inline `#[cfg(test)]` |
| **Integration tests** | `src/test/java/` | `tests/` directory |
| **Private testing** | Reflection hacks | Direct access |
| **Test compilation** | Always | Only with `cargo test` |
| **Test framework** | JUnit (external) | Built-in |
| **Assertions** | `assertEquals(expected, actual)` | `assert_eq!(actual, expected)` |

## Best Practices

### ✅ DO

- Keep unit tests inline with `#[cfg(test)]`
- Test private functions in unit tests
- Use integration tests for public API
- Write descriptive test names
- Use `assert_eq!` for better error messages

### ❌ DON'T

- Don't create `src/test/` directory
- Don't duplicate tests
- Don't test private details in integration tests
- Don't ignore test failures

## Real Example

See `crates/rustboot-security/src/auth.rs`:

```bash
cargo test --manifest-path crates/rustboot-security/Cargo.toml
```

Output:
```
running 7 tests
test auth::tests::claims_tests::can_create_claims ... ok
test auth::tests::generate_jwt_tests::generates_valid_token ... ignored
test auth::tests::validate_jwt_tests::validates_correct_token ... ignored

test result: ok. 1 passed; 0 failed; 6 ignored
```

---

**Related**: [rust-packaging-vs-java.md](rust-packaging-vs-java.md) - Why Rust's structure differs from Java
