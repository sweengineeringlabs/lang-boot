# Error Handling Conventions

Idiomatic error handling patterns for each language.

---

## 🦀 Rust

### Result Type (No Exceptions)

```rust
fn parse_config(path: &str) -> Result<Config, ConfigError> {
    let content = std::fs::read_to_string(path)?;  // ? propagates errors
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

// Usage
match parse_config("config.toml") {
    Ok(config) => println!("Loaded: {:?}", config),
    Err(e) => eprintln!("Failed: {}", e),
}
```

### Conventions

- Use `Result<T, E>` for recoverable errors
- Use `Option<T>` for optional values (not errors)
- Use `?` operator for propagation
- Define custom error types with `thiserror`
- Never use `panic!` for expected errors

---

## 🦫 Go

### Multiple Return Values

```go
func parseConfig(path string) (*Config, error) {
    content, err := os.ReadFile(path)
    if err != nil {
        return nil, fmt.Errorf("read config: %w", err)  // Wrap errors
    }
    
    var config Config
    if err := json.Unmarshal(content, &config); err != nil {
        return nil, fmt.Errorf("parse config: %w", err)
    }
    return &config, nil
}

// Usage
config, err := parseConfig("config.json")
if err != nil {
    log.Fatalf("Failed: %v", err)
}
```

### Conventions

- Return `(value, error)` tuple
- Check errors immediately: `if err != nil`
- Wrap errors with context: `fmt.Errorf("context: %w", err)`
- Use `errors.Is()` and `errors.As()` for comparison
- Never ignore errors (use `_ = fn()` if intentional)

---

## ☕ Java

### Exceptions

```java
public Config parseConfig(String path) throws ConfigException {
    try {
        String content = Files.readString(Path.of(path));
        return objectMapper.readValue(content, Config.class);
    } catch (IOException e) {
        throw new ConfigException("Failed to read config", e);
    }
}

// Usage
try {
    Config config = parseConfig("config.json");
} catch (ConfigException e) {
    logger.error("Failed: {}", e.getMessage(), e);
}
```

### Conventions

- Use **checked exceptions** for recoverable errors
- Use **unchecked exceptions** (RuntimeException) for programming errors
- Always include cause: `new Exception("msg", cause)`
- Don't catch generic `Exception` (be specific)
- Use try-with-resources for cleanup

---

## 🐍 Python

### Exceptions

```python
def parse_config(path: str) -> Config:
    try:
        with open(path) as f:
            data = json.load(f)
        return Config(**data)
    except FileNotFoundError:
        raise ConfigError(f"Config not found: {path}") from None
    except json.JSONDecodeError as e:
        raise ConfigError(f"Invalid JSON: {path}") from e

# Usage
try:
    config = parse_config("config.json")
except ConfigError as e:
    logging.error(f"Failed: {e}")
```

### Conventions

- Use `raise ... from e` to chain exceptions
- Be specific with `except` clauses
- Use custom exception classes
- Don't use bare `except:`
- Use context managers (`with`) for cleanup

---

## Comparison

| Aspect | Rust | Go | Java | Python |
|--------|------|-----|------|--------|
| Mechanism | `Result<T,E>` | `(T, error)` | Exceptions | Exceptions |
| Propagation | `?` operator | Manual check | `throws` / throw | `raise` / reraise |
| Chaining | `.map_err()` | `fmt.Errorf("%w")` | Constructor cause | `from e` |
| Null Safety | `Option<T>` | nil checks | Optional | None checks |

---

## Idiomatic Summary

- **Rust**: `Result` everywhere, `?` for propagation
- **Go**: Return `error`, check immediately, wrap with context
- **Java**: Checked for recoverable, unchecked for bugs
- **Python**: Exceptions with `from` chaining
