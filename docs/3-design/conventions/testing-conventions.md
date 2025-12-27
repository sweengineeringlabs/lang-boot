# Testing Conventions by Language

Idiomatic test organization patterns for each language in the Lang-Boot ecosystem.

---

## 🦀 Rust

### Unit Tests (Co-located, Private Access)

```rust
// src/handler.rs
pub fn public_func() -> i32 { private_func() }
fn private_func() -> i32 { 42 }

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_private() {
        assert_eq!(private_func(), 42);  // ✅ Private access
    }
}
```

### Separate Test File (Still Co-located)

```
src/
├── handler.rs
└── handler_tests.rs   ← Separate but same directory
```

```rust
// handler.rs
#[cfg(test)]
#[path = "handler_tests.rs"]
mod tests;
```

### Integration Tests (Public API Only)

```
tests/
└── integration.rs     ← Separate crate, no private access
```

| Location | Type | Private Access | Convention |
|----------|------|----------------|------------|
| `#[cfg(test)]` in source | Unit | ✅ Yes | **Idiomatic** |
| `tests/*.rs` | Integration | ❌ No | External API testing |

---

## 🦫 Go

### Co-located Tests (Always)

```
mypackage/
├── handler.go         ← package mypackage
└── handler_test.go    ← Test file (same directory)
```

### White-box (Same Package)

```go
// handler_test.go
package mypackage  // Same package = private access

func TestPrivate(t *testing.T) {
    result := privateFunc()  // ✅ Can test private
}
```

### Black-box (Test Package)

```go
// handler_test.go
package mypackage_test  // _test suffix = public API only

import "mypackage"

func TestPublic(t *testing.T) {
    result := mypackage.PublicFunc()  // Public API only
}
```

| Package Declaration | Private Access | Convention |
|---------------------|----------------|------------|
| `package foo` | ✅ Yes | White-box unit tests |
| `package foo_test` | ❌ No | Black-box API tests |

**Go Convention**: Tests are ALWAYS co-located with `_test.go` suffix.

---

## ☕ Java

### Traditional (Separate Tree)

```
src/
├── main/java/com/example/Handler.java
└── test/java/com/example/HandlerTest.java
```

### Co-located Alternative

```
src/main/java/com/example/
├── Handler.java
└── HandlerTest.java   ← Same package, needs build config
```

Requires Maven/Gradle configuration to exclude `*Test.java` from production JAR.

| Location | Private Access | Convention |
|----------|----------------|------------|
| `src/test/java/` | Package-private only | **Traditional (Maven/Gradle)** |
| Same directory | Package-private only | Possible with config |

**Java Convention**: Separate `src/test/java/` tree is standard.

---

## 🐍 Python

### Separate tests/ Directory

```
mypackage/
├── __init__.py
├── handler.py
└── tests/
    ├── __init__.py
    └── test_handler.py
```

### Co-located Alternative

```
mypackage/
├── __init__.py
├── handler.py
└── test_handler.py    ← Same directory
```

Both work with pytest. Co-located requires:
```bash
pytest mypackage/
```

| Location | Convention |
|----------|------------|
| `tests/` directory | **Traditional** |
| Co-located `test_*.py` | Supported, less common |

---

## Summary

| Language | Unit Tests Location | Integration Tests |
|----------|---------------------|-------------------|
| **Rust** | `#[cfg(test)]` in same file | `tests/` directory |
| **Go** | `*_test.go` same directory | Same (use `_test` package) |
| **Java** | `src/test/java/` | `src/test/java/` |
| **Python** | `tests/` or co-located | `tests/` |

### Idiomatic Recommendations

- **Rust**: Co-located `#[cfg(test)]` for unit, `tests/` for integration
- **Go**: Always co-located `*_test.go`, use `_test` package suffix for black-box
- **Java**: Separate `src/test/java/` tree (standard)
- **Python**: `tests/` directory (pytest standard)
