# Visibility & Access Conventions

Idiomatic visibility and access modifier patterns for each language.

---

## 🦀 Rust

### Keywords

| Keyword | Visibility |
|---------|------------|
| (none) | Private to module |
| `pub` | Public |
| `pub(crate)` | Public within crate |
| `pub(super)` | Public to parent module |
| `pub(in path)` | Public to specific module |

### Example

```rust
mod internal {
    pub struct Public;           // Public everywhere
    pub(crate) struct CrateOnly; // Public in this crate
    pub(super) struct ParentOnly;// Public to parent module
    struct Private;              // Private to this module
    
    pub fn public_fn() {}
    fn private_fn() {}           // Only this module
}
```

### Conventions

- Default to private; add `pub` only when needed
- Use `pub(crate)` for internal APIs
- Module boundary = visibility boundary
- Fields are private by default (even in `pub struct`)

---

## 🦫 Go

### Case-Based Visibility

| First Letter | Visibility |
|--------------|------------|
| Uppercase | Exported (public) |
| lowercase | Unexported (package-private) |

### Example

```go
package user

type User struct {              // Exported
    Name  string               // Exported field
    email string               // Unexported field
}

func NewUser(name string) *User {  // Exported
    return createUser(name)
}

func createUser(name string) *User {  // Unexported
    return &User{Name: name}
}
```

### Conventions

- Uppercase = public, lowercase = private
- No other access levels (no protected)
- `internal/` directory = not importable from outside
- Package boundary = visibility boundary

---

## ☕ Java

### Keywords

| Modifier | Class | Package | Subclass | World |
|----------|-------|---------|----------|-------|
| `public` | ✅ | ✅ | ✅ | ✅ |
| `protected` | ✅ | ✅ | ✅ | ❌ |
| (default) | ✅ | ✅ | ❌ | ❌ |
| `private` | ✅ | ❌ | ❌ | ❌ |

### Example

```java
public class UserService {
    private final UserRepository repo;      // Private
    protected Logger logger;                // Protected (subclasses)
    String internalState;                   // Package-private
    public UserService(UserRepository r) {  // Public
        this.repo = r;
    }
}
```

### Conventions

- Start with most restrictive (`private`)
- Use `private` for fields (encapsulation)
- Use package-private for internal classes
- Use `protected` sparingly (inheritance)
- Use `public` only for API

---

## 🐍 Python

### Conventions (Not Enforced)

| Pattern | Meaning |
|---------|---------|
| `name` | Public |
| `_name` | Private by convention |
| `__name` | Name mangling (class-private) |
| `__name__` | Magic/dunder methods |

### Example

```python
class UserService:
    def __init__(self, repo):
        self._repo = repo          # Private by convention
        self.__secret = "hidden"   # Name-mangled to _UserService__secret
    
    def get_user(self, id):        # Public
        return self._fetch(id)
    
    def _fetch(self, id):          # Private by convention
        return self._repo.find(id)
    
    def __str__(self):             # Magic method
        return f"UserService(...)"
```

### Conventions

- Single `_` prefix = private (convention only)
- Double `__` prefix = name mangling (discouraged)
- Python has no real access control
- "We're all consenting adults"

---

## Comparison

| Aspect | Rust | Go | Java | Python |
|--------|------|-----|------|--------|
| Private | (default) | lowercase | `private` | `_` prefix |
| Public | `pub` | Uppercase | `public` | (default) |
| Package | `pub(crate)` | (default) | (default) | N/A |
| Protected | N/A | N/A | `protected` | `_` prefix |
| Enforced | ✅ Compile-time | ✅ Compile-time | ✅ Compile-time | ❌ Convention |

---

## Best Practices

- **Rust**: Start private, add `pub` as needed
- **Go**: Uppercase only for exported API
- **Java**: Always `private` fields, expose via methods
- **Python**: Use `_prefix` for implementation details
