# Rust Packaging vs Java

**Audience**: Java/JVM developers learning Rust

## WHAT: Different Packaging Philosophy

Rust's project structure is fundamentally different from Java:

### Java (Maven/Gradle)
```
project/
├── pom.xml
└── src/
    ├── main/java/com/company/product/
    │   ├── service/UserService.java
    │   └── model/User.java
    └── test/java/com/company/product/
        └── service/UserServiceTest.java
```

### Rust (Cargo)
```
project/
├── Cargo.toml
├── src/
│   ├── lib.rs              # Entry point
│   ├── service/
│   │   └── user.rs         # Code + inline tests
│   └── model/
│       └── user.rs         # Code + inline tests
└── tests/
    └── integration.rs      # Integration tests only
```

**Key Differences**:
- No `src/main/` and `src/test/` separation
- No reverse domain naming (`com.company.product`)
- Tests inline with code via `#[cfg(test)]`
- Flat crate-based namespace

## WHY: Four Fundamental Reasons

### 1. Conditional Compilation

**Java Problem**: Tests always compiled, runtime filtered
```java
// UserServiceTest.java - always in classpath
public class UserServiceTest {
    @Test
    public void testGetUser() { }
}
```

**Rust Solution**: Tests compiled only when needed
```rust
// user.rs
pub struct UserService { }

#[cfg(test)]  // Only with `cargo test`
mod tests {
    #[test]
    fn test_get_user() { }
}
```

**Benefit**: **Zero test overhead** in production binaries.

### 2. Private Member Access

**Java Problem**: Can't test private/package-private without reflection
```java
class UserService {
    private void validateUser() { }
}

// UserServiceTest.java
// ❌ Can't test validateUser() without reflection hacks
```

**Rust Solution**: Inline tests access private items
```rust
fn validate_user() { }  // Private

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_user() {
        validate_user();  // ✅ Works!
    }
}
```

**Benefit**: Test implementation details **without reflection**.

### 3. Zero Configuration Build

**Java**: Must configure source/test directories
```xml
<!-- pom.xml -->
<build>
    <sourceDirectory>src/main/java</sourceDirectory>
    <testSourceDirectory>src/test/java</testSourceDirectory>
</build>
```

**Rust**: Convention is the configuration
```toml
# Cargo.toml - no test config needed!
[package]
name = "my-crate"
```

Cargo automatically finds:
- `src/lib.rs` or `src/main.rs`
- `#[cfg(test)]` modules
- `tests/*.rs` files

**Benefit**: **Zero configuration** required.

### 4. Flexible Module System

**Java**: File path = package name (mandatory)
```java
// MUST be in src/main/java/com/company/User.java
package com.company;  // Must match directory
```

**Rust**: File path ≠ module path
```rust
// src/model/user.rs
pub struct User { }

// src/lib.rs
mod model;  // Declares module explicitly
```

**Benefit**: More **flexible organization**.

## HOW: Practical Migration

### Package Naming

**Java**: Reverse domain
```
com.google.guava.collections.ImmutableList
Import: import com.google.guava.collections.ImmutableList;
```

**Rust**: Crate-based
```
Crate: serde
Import: use serde::Serialize;
```

Why different? Cargo registry (`crates.io`) ensures global uniqueness.

### Dependency Management

**Java (Maven)**: Verbose XML
```xml
<dependency>
    <groupId>com.google.guava</groupId>
    <artifactId>guava</artifactId>
    <version>31.0.1-jre</version>
</dependency>
```

**Rust (Cargo)**: Minimal TOML
```toml
[dependencies]
serde = "1.0"  # Semantic versioning built-in
```

### Testing Setup

**From Java**:
```java
// src/main/java/com/company/UserService.java
public class UserService {
    public User getUser(int id) { }
}

// src/test/java/com/company/UserServiceTest.java
public class UserServiceTest {
    @Test
    public void testGetUser() { }
}
```

**To Rust**:
```rust
// src/service/user.rs

pub struct UserService { }

impl UserService {
    pub fn get_user(&self, id: i32) -> User {
        // Implementation
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_user() {
        let service = UserService {};
        let user = service.get_user(1);
        assert_eq!(user.id, 1);
    }
}
```

### Build Output

**Java**: Separate class files
```
target/
├── classes/              # Production .class
├── test-classes/         # Test .class
└── my-app-1.0.jar
```

**Rust**: Single artifacts
```
target/
├── debug/
│   └── libmy_crate.rlib  # Library + tests
└── release/
    └── libmy_crate.rlib  # Optimized (no tests)
```

### Real-World Example

**Java Spring Boot**:
```
spring-app/
├── pom.xml
└── src/
    ├── main/java/com/company/app/
    │   ├── controller/UserController.java
    │   ├── service/impl/UserServiceImpl.java
    │   └── repository/UserRepository.java
    └── test/java/com/company/app/
        ├── controller/UserControllerTest.java
        └── service/UserServiceTest.java
```

**Rust Web App (Axum)**:
```
rust-web/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── handlers/user.rs      # + inline tests
│   ├── services/user.rs      # + inline tests
│   └── models/user.rs        # + inline tests
└── tests/
    └── api_integration.rs
```

**Depth**: Java 8+ levels vs Rust 3-4 levels

## Comparison Table

| Aspect | Java | Rust |
|--------|------|------|
| **Package naming** | `com.company.product` | `my_crate` |
| **Test location** | `src/test/` | Inline + `tests/` |
| **Private testing** | Reflection | Direct access |
| **Build config** | Maven/Gradle XML | Cargo TOML |
| **Dependency format** | GroupId+ArtifactId | Crate name |
| **Module system** | Path = package | Path ≠ module |
| **Compilation** | Runtime filtered | Compile-time |

## When to Use Which

### Use Java Structure When:
- Large enterprise team (100+ devs)
- Strict conventions required
- Heavy IDE reliance (IntelliJ/Eclipse)
- Existing Java ecosystem

### Use Rust Structure When:
- Systems programming
- Performance critical
- Want minimal boilerplate
- Compile-time guarantees important

## Key Takeaways

1. **No src/main and src/test in Rust**: Tests inline with `#[cfg(test)]`
2. **No reverse domain naming**: Crates have unique names on crates.io
3. **Zero configuration**: Cargo conventions = configuration
4. **Compile-time optimization**: Tests removed in production builds

**Embrace Rust's idioms** - they're optimized for different goals than Java. 🦀

---

**Related**: [rust-test-organization.md](rust-test-organization.md) - Detailed test patterns
