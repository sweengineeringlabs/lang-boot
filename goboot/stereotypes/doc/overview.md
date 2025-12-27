# Stereotypes Module

> **Go-idiomatic alternatives to Java/Python annotations for component classification and behavior decoration.**

---

## TLDR

The `stereotypes` module provides marker interfaces, decorator functions, and an annotation registry to enable annotation-like patterns in Go, comparable to Spring's `@Service`, `@Repository`, `@Controller` annotations.

---

## Table of Contents

- [What is Stereotypes?](#what-is-stereotypes)
- [Why Stereotypes?](#why-stereotypes)
- [How to Use](#how-to-use)
- [Comparison with Other Frameworks](#comparison-with-other-frameworks)
- [API Reference](#api-reference)

---

## What is Stereotypes?

The stereotypes module provides Go-idiomatic patterns for:

| Feature | Description |
|---------|-------------|
| **Marker Interfaces** | Type classification (`Service`, `Repository`, `Controller`) |
| **Decorator Functions** | Behavior wrapping (`Retryable`, `Timed`, `Cached`) |
| **Annotation Registry** | Runtime metadata storage |
| **Lifecycle Hooks** | `PostConstruct`, `PreDestroy` callbacks |

---

## Why Stereotypes?

Go lacks built-in annotations/decorators. This module bridges that gap:

| Problem | Solution |
|---------|----------|
| No `@Service` annotation | `BaseService` marker interface |
| No `@Transactional` | `Annotation` registry with metadata |
| No decorators | Generic `Wrap()` with decorator functions |
| No lifecycle callbacks | `PostConstruct`/`PreDestroy` interfaces |

---

## How to Use

### Marker Interfaces

```go
import "dev.engineeringlabs/goboot/stereotypes"

// Mark as a service
type UserService struct {
    stereotypes.BaseService
}

// Now UserService.Stereotype() returns "service"
func main() {
    svc := &UserService{}
    fmt.Println(svc.Stereotype()) // "service"
}
```

### Decorator Functions

```go
import (
    "dev.engineeringlabs/goboot/stereotypes"
    "dev.engineeringlabs/goboot/stereotypes/core"
)

// Original function
func fetchUser(id int) (User, error) {
    // ... database call
}

// Wrap with retry and timing
fetchUserWrapped := stereotypes.Wrap(
    fetchUser,
    core.Retryable[User](3, time.Second),
    core.Timed[User]("fetchUser", log.Printf),
)

// Use wrapped function
user, err := fetchUserWrapped(123)
```

### Annotation Registry

```go
registry := stereotypes.NewAnnotationRegistry()

// Register annotations
registry.Register("UserService", 
    stereotypes.NewAnnotation("Transactional").
        With("isolation", "READ_COMMITTED").
        With("timeout", "30s"))

// Retrieve annotations
ann := registry.Get("UserService", "Transactional")
isolation := ann.Get("isolation") // "READ_COMMITTED"
```

### Lifecycle Hooks

```go
type DatabasePool struct {
    stereotypes.BaseComponent
    conn *sql.DB
}

func (p *DatabasePool) PostConstruct() error {
    var err error
    p.conn, err = sql.Open("postgres", connStr)
    return err
}

func (p *DatabasePool) PreDestroy() error {
    return p.conn.Close()
}

// Initialize all lifecycle hooks
func main() {
    pool := &DatabasePool{}
    stereotypes.InitializeLifecycle(pool) // calls PostConstruct
    defer stereotypes.DestroyLifecycle(pool) // calls PreDestroy
}
```

---

## Comparison with Other Frameworks

### vs Spring Framework (Java)

| Spring | Goboot Stereotypes | Notes |
|--------|-------------------|-------|
| `@Component` | `BaseComponent` | Embed struct |
| `@Service` | `BaseService` | Embed struct |
| `@Repository` | `BaseRepository` | Embed struct |
| `@Controller` | `BaseController` | Embed struct |
| `@Transactional` | `NewAnnotation("Transactional")` | Runtime registry |
| `@PostConstruct` | `PostConstruct` interface | Implement method |
| `@PreDestroy` | `PreDestroy` interface | Implement method |
| `@Autowired` | DI Container | Use `goboot/di` module |
| `@Cacheable` | `core.Cached[T]()` | Decorator function |
| `@Retryable` | `core.Retryable[T]()` | Decorator function |
| `@Timed` | `core.Timed[T]()` | Decorator function |

### vs NestJS (TypeScript)

| NestJS | Goboot Stereotypes | Notes |
|--------|-------------------|-------|
| `@Injectable()` | `BaseComponent` | Embed struct |
| `@Controller()` | `BaseController` | Embed struct |
| `@Module()` | Package organization | Go packages |
| `@Get()/@Post()` | Route handlers | Use `goboot/web` |
| `@UseGuards()` | Middleware | Use `goboot/web` |
| `@UsePipes()` | Validation | Use `goboot/validation` |

### vs Rustboot Macros

| Rustboot | Goboot Stereotypes | Notes |
|----------|-------------------|-------|
| `#[derive(Injectable)]` | `BaseComponent` | No code generation |
| `#[cached(ttl = 600)]` | `core.Cached[T]()` | Runtime decorator |
| `#[traced]` | `core.Timed[T]()` | Runtime decorator |
| `#[retry(max = 3)]` | `core.Retryable[T]()` | Runtime decorator |

### vs Pyboot Decorators

| Pyboot | Goboot Stereotypes | Notes |
|--------|-------------------|-------|
| `@service` | `BaseService` | Embed struct |
| `@memoize` | `core.Cached[T]()` | Decorator function |
| `@retry(max_attempts=3)` | `core.Retryable[T]()` | Decorator function |
| `@logged` | `core.Logged[T]()` | Decorator function |

---

## API Reference

### Stereotype Constants

```go
const (
    StereotypeComponent  = "component"
    StereotypeService    = "service"
    StereotypeRepository = "repository"
    StereotypeController = "controller"
    StereotypeGateway    = "gateway"
    StereotypeHandler    = "handler"
    StereotypeMiddleware = "middleware"
    StereotypeFactory    = "factory"
    StereotypeSingleton  = "singleton"
)
```

### Base Types

| Type | Stereotype | Use Case |
|------|-----------|----------|
| `BaseComponent` | `component` | Generic component |
| `BaseService` | `service` | Business logic |
| `BaseRepository` | `repository` | Data access |
| `BaseController` | `controller` | HTTP handlers |

### Decorator Functions

| Decorator | Parameters | Description |
|-----------|------------|-------------|
| `Retryable[T]` | `maxAttempts, delay` | Retry on failure |
| `Timed[T]` | `name, logFunc` | Log execution time |
| `Cached[T]` | `cache, keyFunc` | Cache results |
| `Logged[T]` | `name, logFunc` | Log calls |
| `Synchronized[T]` | `mutex` | Thread-safe execution |
| `Validated[T]` | `validator` | Validate inputs |

### Lifecycle Functions

| Function | Description |
|----------|-------------|
| `InitializeLifecycle(c)` | Calls `PostConstruct` if implemented |
| `DestroyLifecycle(c)` | Calls `PreDestroy` if implemented |

---

**Module**: `dev.engineeringlabs/goboot/stereotypes`  
**Architecture**: SEA (API/Core/SPI layers)  
**Go Version**: 1.21+
