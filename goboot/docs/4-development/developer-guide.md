# Goboot Developer Guide

**Audience**: Developers, Contributors

## WHAT: Development Guide

This guide covers development practices for the Goboot framework.

## WHY: Consistent Development

Following these practices ensures:
- Consistent code style
- Maintainable modules
- Easy contribution

## HOW: Development Practices

### Module Structure

Every module MUST follow this structure:

```
module/
├── module.go         # Facade - re-exports API and Core
├── api/              # Public contracts
│   └── types.go      # Interfaces, types, constants
├── core/             # Implementations
│   └── impl.go
└── spi/              # Extension points (optional)
    └── provider.go
```

### Layer Rules

1. **API Layer**
   - Only interfaces, types, and constants
   - No implementation logic
   - No dependencies on Core or SPI

2. **Core Layer**
   - Implements API interfaces
   - Can depend on API layer
   - Contains all implementation logic

3. **SPI Layer**
   - Extension interfaces for external implementations
   - E.g., `CacheBackend` for Redis, Memcached

### Coding Standards

#### Naming Conventions

```go
// Interfaces - Descriptive names (not I-prefix)
type Cache interface {}
type Logger interface {}

// Implementations - Descriptive names
type MemoryCache struct {}
type JSONLogger struct {}

// Constructors - New prefix
func NewMemoryCache() *MemoryCache {}
func NewJSONLogger() *JSONLogger {}

// Config - Suffix with Config
type RetryConfig struct {}
type CircuitBreakerConfig struct {}
```

#### Error Handling

```go
// Return errors, don't panic
func Load() (*Settings, error) {
    if err != nil {
        return nil, fmt.Errorf("failed to load: %w", err)
    }
    return settings, nil
}

// Use Result monad for functional style
func Calculate() errors.Result[int] {
    if invalid {
        return errors.Err[int]("invalid input")
    }
    return errors.Ok(result)
}
```

#### Context Usage

```go
// Always accept context for cancellation
func Get(ctx context.Context, key string) (any, error) {
    select {
    case <-ctx.Done():
        return nil, ctx.Err()
    default:
        // proceed
    }
}
```

### Testing

```bash
# Run all tests
go test ./...

# Run with coverage
go test -coverprofile=coverage.out ./...

# View coverage
go tool cover -html=coverage.out
```

### Building

```bash
# Build
go build ./...

# Lint
golangci-lint run

# Format
go fmt ./...
```

---

**Next**: [Architecture Guide](../3-design/architecture.md)
