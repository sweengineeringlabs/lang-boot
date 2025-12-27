# Documentation Conventions

Idiomatic documentation patterns for each language.

---

## 🦀 Rust

### Doc Comments

```rust
/// A user in the system.
///
/// # Examples
///
/// ```
/// let user = User::new("alice");
/// assert_eq!(user.name(), "alice");
/// ```
pub struct User {
    name: String,
}

impl User {
    /// Creates a new user with the given name.
    ///
    /// # Arguments
    ///
    /// * `name` - The user's display name
    ///
    /// # Panics
    ///
    /// Panics if `name` is empty.
    pub fn new(name: &str) -> Self {
        assert!(!name.is_empty(), "name cannot be empty");
        Self { name: name.to_string() }
    }
}

//! Module-level documentation.
//! Use `//!` at the top of a file.
```

### Conventions

- Use `///` for items (functions, structs, etc.)
- Use `//!` for module/crate-level docs
- Include `# Examples` with runnable code
- Use `# Panics`, `# Errors`, `# Safety` sections
- Examples are **tested** by `cargo test`

---

## 🦫 Go

### Doc Comments

```go
// Package user provides user management functionality.
package user

// User represents a user in the system.
type User struct {
    Name string
}

// New creates a new user with the given name.
// It returns an error if the name is empty.
//
// Example:
//
//	user, err := New("alice")
//	if err != nil {
//	    log.Fatal(err)
//	}
func New(name string) (*User, error) {
    if name == "" {
        return nil, errors.New("name cannot be empty")
    }
    return &User{Name: name}, nil
}
```

### Conventions

- Comment starts with the item name
- Package comment at top of one file (usually `doc.go`)
- Use `Example` functions for runnable examples
- No special sections (just prose)
- Generated with `go doc`

### Example Functions

```go
func ExampleNew() {
    user, _ := New("alice")
    fmt.Println(user.Name)
    // Output: alice
}
```

---

## ☕ Java

### Javadoc

```java
/**
 * A user in the system.
 *
 * <p>Example usage:
 * <pre>{@code
 * User user = new User("alice");
 * System.out.println(user.getName());
 * }</pre>
 *
 * @author John Doe
 * @since 1.0
 */
public class User {
    private final String name;
    
    /**
     * Creates a new user with the given name.
     *
     * @param name the user's display name, must not be null
     * @throws IllegalArgumentException if name is empty
     * @return a new User instance
     */
    public User(String name) {
        if (name.isEmpty()) {
            throw new IllegalArgumentException("name cannot be empty");
        }
        this.name = name;
    }
}
```

### Conventions

- Use `/** ... */` for Javadoc
- Start with summary sentence (ends at first period)
- Use `@param`, `@return`, `@throws` tags
- Use `@see`, `@since`, `@deprecated`
- HTML allowed in documentation

---

## 🐍 Python

### Docstrings

```python
"""User management module.

This module provides user-related functionality.
"""

class User:
    """A user in the system.
    
    Example:
        >>> user = User("alice")
        >>> user.name
        'alice'
    
    Attributes:
        name: The user's display name.
    """
    
    def __init__(self, name: str) -> None:
        """Creates a new user with the given name.
        
        Args:
            name: The user's display name.
            
        Raises:
            ValueError: If name is empty.
        """
        if not name:
            raise ValueError("name cannot be empty")
        self.name = name
```

### Styles

| Style | Format |
|-------|--------|
| **Google** | `Args:`, `Returns:`, `Raises:` |
| **NumPy** | `Parameters`, `Returns`, `Raises` with dashes |
| **Sphinx** | `:param name:`, `:returns:`, `:raises:` |

### Conventions

- Triple quotes `"""`
- First line is summary
- Google style is common
- Doctests are tested with `pytest --doctest-modules`

---

## Comparison

| Feature | Rust | Go | Java | Python |
|---------|------|-----|------|--------|
| Syntax | `///` or `//!` | `// Comment` | `/** ... */` | `"""..."""` |
| Params | `# Arguments` | Prose | `@param` | `Args:` |
| Returns | `# Returns` | Prose | `@return` | `Returns:` |
| Errors | `# Errors` | Prose | `@throws` | `Raises:` |
| Examples | Tested code | Example funcs | `<pre>` block | Doctests |
| Generator | `cargo doc` | `go doc` | `javadoc` | Sphinx |
