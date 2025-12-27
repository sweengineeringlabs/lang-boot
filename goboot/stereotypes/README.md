# Stereotypes Module

**Go-idiomatic alternatives to annotations, decorators, and dependency injection markers.**

## TLDR

The `stereotypes` module provides a centralized set of utilities that work across ALL goboot modules:
- **Marker interfaces** for type classification (Service, Repository, Controller)
- **Decorator functions** for wrapping behavior (Retryable, Cached, Timed)
- **Annotation registry** for runtime metadata
- **Lifecycle hooks** for component initialization

---

## Table of Contents

1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Comparison with SEA Frameworks](#comparison-with-sea-frameworks)
4. [API Reference](#api-reference)
5. [Usage Examples](#usage-examples)
6. [Integration with Goboot Modules](#integration-with-goboot-modules)

---

## Overview

### The Problem

Go doesn't have:
- **Decorators** like Python (`@retryable`, `@cached`)
- **Derive macros** like Rust (`#[derive(Repository)]`)
- **Annotations** like Java (`@Service`, `@Transactional`)

### The Solution

The `stereotypes` module provides Go-idiomatic alternatives that align with the SEA (Stratified Encapsulation Architecture) pattern used across PyBoot, Rustboot, and Goboot:

| PyBoot (Python) | Rustboot (Rust) | **Goboot (Go)** |
|-----------------|-----------------|-----------------|
| `@service` decorator | `#[derive(Service)]` | `stereotypes.BaseService` |
| `@repository` decorator | `impl Repository for T` | `stereotypes.BaseRepository` |
| `@retryable(3, 1.0)` | `retry!(3, || {...})` | `stereotypes.Retryable[T](3, delay)` |
| `@cached` | `#[cached]` | `stereotypes.Cached()` |
| `@timed("name")` | `#[timed]` | `stereotypes.Timed[T]()` |

---

## Architecture

### Cross-Cutting Concern Pattern

The stereotypes module is a **cross-cutting concern** - a small module that provides utilities used by ALL other modules:

```
┌──────────────────────────────────────────────────────────────────┐
│                     stereotypes (4 files)                        │
│                                                                  │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐              │
│  │   Markers   │  │ Decorators  │  │ Annotations │              │
│  │  Service    │  │ Retryable   │  │  Registry   │              │
│  │  Repository │  │ Timed       │  │  Runtime    │              │
│  │  Controller │  │ Cached      │  │  Metadata   │              │
│  │  Handler    │  │ Validated   │  │             │              │
│  └─────────────┘  └─────────────┘  └─────────────┘              │
│                                                                  │
│  ┌─────────────────────────────────────────────┐                │
│  │            Lifecycle Hooks                   │                │
│  │  PostConstruct  │  PreDestroy  │  OnInit    │                │
│  └─────────────────────────────────────────────┘                │
└────────────────────────────┬─────────────────────────────────────┘
                             │
                             │ Used by ALL modules
                             │
         ┌───────────────────┼───────────────────┐
         ▼                   ▼                   ▼
┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐
│    database     │  │    security     │  │      web        │
│  UserRepository │  │  AuthService    │  │  UserController │
│                 │  │                 │  │                 │
│  BaseRepository │  │  BaseService    │  │ BaseController  │
└─────────────────┘  └─────────────────┘  └─────────────────┘
         │                   │                   │
         ▼                   ▼                   ▼
┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐
│     cache       │  │      http       │  │   messaging     │
│                 │  │                 │  │                 │
│  @Cached        │  │  @Retryable     │  │  @Handler       │
│  decorator      │  │  @Timed         │  │  annotation     │
└─────────────────┘  └─────────────────┘  └─────────────────┘
```

### File Structure

```
stereotypes/
├── api/
│   └── types.go              # Interfaces & types (120 lines)
├── core/
│   ├── stereotypes.go        # Implementations (250 lines)
│   └── stereotypes_test.go   # Tests (220 lines)
└── stereotypes.go            # Facade (130 lines)
```

| File | Purpose | Exports |
|------|---------|---------|
| `api/types.go` | Contracts | `Marker`, `Stereotype`, `Annotation`, `Lifecycle` |
| `core/stereotypes.go` | Implementations | Decorators, Base types, Registry |
| `stereotypes.go` | Public API | Re-exports everything |

---

## Comparison with SEA Frameworks

### Java / Spring Boot

```java
// Spring Boot - Java annotations
import org.springframework.stereotype.Service;
import org.springframework.retry.annotation.Retryable;
import javax.annotation.PostConstruct;
import javax.annotation.PreDestroy;

@Service
@Transactional
public class UserService {
    
    private final UserRepository repo;
    
    public UserService(UserRepository repo) {
        this.repo = repo;
    }
    
    @PostConstruct
    public void init() {
        System.out.println("UserService initialized");
    }
    
    @PreDestroy
    public void cleanup() {
        System.out.println("UserService shutting down");
    }
    
    @Retryable(maxAttempts = 3, backoff = @Backoff(delay = 1000))
    @Timed("find_user")
    public User findById(Long id) {
        return repo.findById(id).orElseThrow();
    }
}

@Repository
public class JpaUserRepository implements UserRepository {
    // Spring Data JPA implementation
}
```

```go
// Goboot - Go equivalent
type UserService struct {
    stereotypes.BaseService  // @Service equivalent
    repo UserRepository
}

func NewUserService(repo UserRepository) *UserService {
    return &UserService{repo: repo}
}

func (s *UserService) PostConstruct() error {  // @PostConstruct
    log.Println("UserService initialized")
    return nil
}

func (s *UserService) PreDestroy() error {     // @PreDestroy
    log.Println("UserService shutting down")
    return nil
}

// @Retryable + @Timed equivalent
var FindById = stereotypes.Wrap(
    findByIdImpl,
    stereotypes.Retryable[*User](3, time.Second),
    stereotypes.Timed[*User]("find_user", log.Printf),
)

// @Transactional equivalent (via annotation registry)
func init() {
    registry.Register("UserService", 
        stereotypes.NewAnnotation("Transactional"))
}
```

### PyBoot (Python)

```python
# PyBoot - Python decorators
from pyboot.resilience import retryable, timed
from pyboot.di import injectable

@injectable
class UserService:
    def __init__(self, repo: UserRepository):
        self.repo = repo
    
    @retryable(max_attempts=3, delay=1.0)
    @timed("find_user")
    async def find_by_id(self, id: str) -> User:
        return await self.repo.get(id)
```

```go
// Goboot - Go equivalent
type UserService struct {
    stereotypes.BaseService  // @injectable equivalent
    repo UserRepository
}

func NewUserService(repo UserRepository) *UserService {
    return &UserService{repo: repo}
}

// @retryable + @timed equivalent
var FindById = stereotypes.Wrap(
    findByIdImpl,
    stereotypes.Retryable[*User](3, time.Second),
    stereotypes.Timed[*User]("find_user", log.Printf),
)
```

### Rustboot (Rust)

```rust
// Rustboot - Rust traits and macros
use rustboot::stereotypes::{Service, Repository};
use rustboot::resilience::retry;

#[derive(Service)]
pub struct UserService {
    repo: Arc<dyn UserRepository>,
}

impl UserService {
    pub async fn find_by_id(&self, id: &str) -> Result<User> {
        retry!(3, || self.repo.get(id).await)
    }
}

#[derive(Repository)]
pub struct PostgresUserRepository {
    pool: PgPool,
}
```

```go
// Goboot - Go equivalent
type UserService struct {
    stereotypes.BaseService     // #[derive(Service)] equivalent
    repo UserRepository
}

type PostgresUserRepository struct {
    stereotypes.BaseRepository  // #[derive(Repository)] equivalent
    pool *sql.DB
}

// retry! macro equivalent
var FindById = stereotypes.Wrap(
    findByIdImpl,
    stereotypes.Retryable[*User](3, time.Second),
)
```

### Side-by-Side Comparison

#### Syntax Comparison Table

| Feature | Java/Spring | PyBoot | Rustboot | **Goboot** |
|---------|-------------|--------|----------|------------|
| Service marker | `@Service` | `@injectable` | `#[derive(Service)]` | `BaseService` |
| Repository marker | `@Repository` | `@repository` | `#[derive(Repository)]` | `BaseRepository` |
| Controller marker | `@Controller` | `@controller` | `#[derive(Controller)]` | `BaseController` |
| Retry | `@Retryable` | `@retryable` | `retry!` macro | `Retryable[T]()` |
| Timed/Metrics | `@Timed` | `@timed` | `#[timed]` | `Timed[T]()` |
| Caching | `@Cacheable` | `@cached` | `#[cached]` | `Cached()` |
| Init hook | `@PostConstruct` | `on_init()` | `Lifecycle` trait | `PostConstruct()` |
| Destroy hook | `@PreDestroy` | `on_destroy()` | `Drop` trait | `PreDestroy()` |
| Transaction | `@Transactional` | `@transactional` | `#[transactional]` | `NewAnnotation()` |

#### Type Markers (Service, Repository, Controller)

| Framework | Syntax | Runtime Discoverable |
|-----------|--------|---------------------|
| **Java/Spring** | `@Service` annotation | ✅ Yes (Reflection) |
| **PyBoot** | `@injectable` decorator | ✅ Yes |
| **Rustboot** | `#[derive(Service)]` macro | ✅ Yes |
| **Goboot** | `stereotypes.BaseService` embed | ✅ Yes |

```java
// Java/Spring
@Service
public class UserService { }
```

```python
# PyBoot
@injectable
class UserService:
    pass
```

```rust
// Rustboot
#[derive(Service)]
pub struct UserService;
```

```go
// Goboot
type UserService struct {
    stereotypes.BaseService
}
```

#### Decorator/Wrapper Functions

| Framework | Syntax | Composable |
|-----------|--------|------------|
| **Java/Spring** | `@Annotation` stacking | ✅ Yes |
| **PyBoot** | `@decorator` stacking | ✅ Yes |
| **Rustboot** | `attribute_macro` | ⚠️ Limited |
| **Goboot** | `Wrap(fn, d1, d2)` | ✅ Yes |

```java
// Java/Spring
@Retryable(maxAttempts = 3)
@Timed("fetch")
@Cacheable("products")
public Data fetchData() { ... }
```

```python
# PyBoot
@retryable(3, 1.0)
@timed("fetch")
@cached(ttl=60)
async def fetch_data():
    ...
```

```rust
// Rustboot
#[timed]
#[cached(ttl = 60)]
async fn fetch_data() -> Result<Data> {
    retry!(3, || do_fetch().await)
}
```

```go
// Goboot
fetchData := stereotypes.Wrap(
    fetchDataImpl,
    stereotypes.Retryable[*Data](3, time.Second),
    stereotypes.Timed[*Data]("fetch", logFn),
)
// + use cache module for caching
```

#### Lifecycle Hooks

| Framework | Init Hook | Destroy Hook |
|-----------|-----------|--------------|
| **Java/Spring** | `@PostConstruct` | `@PreDestroy` |
| **PyBoot** | `async def on_init(self)` | `async def on_destroy(self)` |
| **Rustboot** | `impl Lifecycle for T` | `impl Drop for T` |
| **Goboot** | `PostConstruct() error` | `PreDestroy() error` |

```java
// Java/Spring
@Service
public class DatabasePool {
    @PostConstruct
    public void init() {
        connect();
    }
    
    @PreDestroy
    public void cleanup() {
        disconnect();
    }
}
```

```python
# PyBoot
class DatabasePool:
    async def on_init(self):
        await self.connect()
    
    async def on_destroy(self):
        await self.disconnect()
```

```rust
// Rustboot
impl Lifecycle for DatabasePool {
    async fn on_init(&mut self) -> Result<()> {
        self.connect().await
    }
}

impl Drop for DatabasePool {
    fn drop(&mut self) {
        self.disconnect();
    }
}
```

```go
// Goboot
type DatabasePool struct {
    stereotypes.BaseComponent
}

func (p *DatabasePool) PostConstruct() error {
    return p.connect()
}

func (p *DatabasePool) PreDestroy() error {
    return p.disconnect()
}
```

### Feature Comparison Matrix

| Feature | Java/Spring | PyBoot | Rustboot | **Goboot** |
|---------|-------------|--------|----------|------------|
| **Type Markers** | ✅ `@Service` | ✅ `@injectable` | ✅ `#[derive]` | ✅ `BaseService` |
| **Function Decorators** | ✅ `@Annotation` | ✅ `@decorator` | ⚠️ `macro!` | ✅ `Wrap()` |
| **Lifecycle Hooks** | ✅ `@PostConstruct` | ✅ `on_init` | ✅ `Lifecycle` | ✅ `PostConstruct` |
| **Runtime Metadata** | ✅ Reflection | ✅ `__annotations__` | ⚠️ Limited | ✅ `AnnotationRegistry` |
| **Compile-time Safe** | ⚠️ Partial | ❌ Dynamic | ✅ Static | ✅ Generics |
| **Zero-cost Abstractions** | ❌ Reflection | ❌ Runtime | ✅ Monomorphization | ⚠️ Interface calls |
| **SEA Architecture** | ❌ Layered | ✅ API/Core/SPI | ✅ api/core/spi | ✅ api/core/spi |
| **DI Container** | ✅ Spring IoC | ✅ Built-in | ⚠️ Manual | ✅ `di` module |

### Language-Specific Idioms

| Concept | Java/Spring | PyBoot | Rustboot | Goboot |
|---------|-------------|--------|----------|--------|
| DI Marker | Annotation | Decorator | Derive macro | Embedded struct |
| Retry | Annotation | Decorator | Macro | Generic wrapper |
| Caching | Annotation | Decorator | Attribute macro | Higher-order function |
| Validation | Bean Validation | Pydantic | Derive macro | Struct tags |
| Error handling | Exceptions | Result monad | `Result<T, E>` | `Result[T]` generic |
| Transactions | `@Transactional` | Context manager | RAII | Annotation registry |

---

## API Reference

### Stereotype Constants

```go
const (
    StereotypeComponent  = "component"   // General component
    StereotypeService    = "service"     // Business logic
    StereotypeRepository = "repository"  // Data access
    StereotypeController = "controller"  // Web endpoints
    StereotypeGateway    = "gateway"     // External services
    StereotypeHandler    = "handler"     // Event handlers
    StereotypeMiddleware = "middleware"  // Request middleware
    StereotypeFactory    = "factory"     // Object factories
    StereotypeSingleton  = "singleton"   // Singletons
)
```

### Marker Interfaces

```go
// Marker is the base interface
type Marker interface {
    Stereotype() Stereotype
}

// Specialized markers
type Service interface { Marker }
type Repository interface { Marker }
type Controller interface { Marker }
```

### Base Types

```go
// Embed these in your types
type BaseComponent struct{}    // func Stereotype() -> "component"
type BaseService struct{}      // func Stereotype() -> "service"
type BaseRepository struct{}   // func Stereotype() -> "repository"
type BaseController struct{}   // func Stereotype() -> "controller"
```

### Decorator Functions

```go
// Retry failed operations
Retryable[T](maxAttempts int, delay time.Duration) Decorator[func(context.Context) (T, error)]

// Log execution time
Timed[T](name string, logFn func(string, time.Duration)) Decorator[func(context.Context) (T, error)]

// Cache results
Cached[K, V](cache map[K]V, mu *sync.RWMutex) func(func(K) V) func(K) V

// Validate inputs
Validated(validator func(any) error) Decorator[func(any) error]

// Mutex protection
Synchronized[T](mu *sync.Mutex) Decorator[func() T]

// Combine decorators
Wrap[T](fn T, decorators ...Decorator[T]) T
```

### Lifecycle Interfaces

```go
type Lifecycle interface {
    OnInit() error
    OnDestroy() error
}

type PostConstruct interface {
    PostConstruct() error
}

type PreDestroy interface {
    PreDestroy() error
}
```

### Annotation Registry

```go
registry := NewAnnotationRegistry()

// Register annotation
registry.Register("UserService", 
    NewAnnotation("Transactional").
        With("isolation", "SERIALIZABLE").
        With("readOnly", false))

// Check annotation
if registry.Has("UserService", "Transactional") {
    annotations := registry.Get("UserService")
}
```

---

## Usage Examples

### 1. Define a Service

```go
package services

import "dev.engineeringlabs/goboot/stereotypes"

type UserService struct {
    stereotypes.BaseService  // Marks as service
    repo UserRepository
}

func NewUserService(repo UserRepository) *UserService {
    return &UserService{repo: repo}
}

// Implements PostConstruct
func (s *UserService) PostConstruct() error {
    log.Println("UserService initialized")
    return nil
}

// Implements PreDestroy
func (s *UserService) PreDestroy() error {
    log.Println("UserService shutting down")
    return nil
}
```

### 2. Define a Repository

```go
package repositories

import "dev.engineeringlabs/goboot/stereotypes"

type UserRepository struct {
    stereotypes.BaseRepository  // Marks as repository
    db *sql.DB
}

func (r *UserRepository) FindById(id int64) (*User, error) {
    // Database query
}
```

### 3. Wrap Functions with Decorators

```go
package main

import (
    "dev.engineeringlabs/goboot/stereotypes"
    "time"
)

func fetchUserImpl(ctx context.Context) (*User, error) {
    // Fetch from API
}

// Wrap with retry and timing
var FetchUser = stereotypes.Wrap(
    fetchUserImpl,
    stereotypes.Retryable[*User](3, time.Second),
    stereotypes.Timed[*User]("FetchUser", func(name string, d time.Duration) {
        log.Printf("%s took %v", name, d)
    }),
)

func main() {
    user, err := FetchUser(ctx)  // Automatically retries and logs timing
}
```

### 4. Cached Function

```go
var productCache = make(map[string]*Product)
var cacheMu sync.RWMutex

func fetchProduct(id string) *Product {
    // Expensive database/API call
}

var GetProduct = stereotypes.Cached(productCache, &cacheMu)(fetchProduct)

func main() {
    p1 := GetProduct("123")  // Fetches from source
    p2 := GetProduct("123")  // Returns cached
}
```

### 5. Component Registration with DI

```go
package main

import (
    "dev.engineeringlabs/goboot/di"
    "dev.engineeringlabs/goboot/stereotypes"
)

func main() {
    container := di.NewContainer()
    
    // Register service
    service := services.NewUserService(repo)
    container.RegisterInstance("userService", service)
    
    // Initialize lifecycle
    stereotypes.InitializeLifecycle(service)  // Calls PostConstruct
    
    // Later...
    stereotypes.DestroyLifecycle(service)     // Calls PreDestroy
}
```

---

## Integration with Goboot Modules

### With `di` (Dependency Injection)

```go
container.Register(di.Registration{
    Type: "userService",
    Factory: func() (any, error) {
        service := NewUserService(repo)
        stereotypes.InitializeLifecycle(service)  // PostConstruct
        return service, nil
    },
    Scope: di.Singleton,
})
```

### With `http` (HTTP Client)

```go
// Retryable HTTP calls
fetchData := stereotypes.Wrap(
    func(ctx context.Context) (*Response, error) {
        return httpClient.Get(ctx, url)
    },
    stereotypes.Retryable[*Response](3, time.Second),
)
```

### With `cache` (Caching)

```go
// Cached data access
getData := stereotypes.Cached(dataCache, &mu)(
    func(key string) *Data {
        return repository.FindByKey(key)
    },
)
```

### With `database` (Data Access)

```go
type OrderRepository struct {
    stereotypes.BaseRepository
    db database.DB
}

// Repository is automatically recognized as data access layer
stereotype, _ := stereotypes.GetStereotype(repo)
// stereotype == "repository"
```

### With `web` (Web Controllers)

```go
type ProductController struct {
    stereotypes.BaseController
    service ProductService
}

// Controller is automatically recognized as web layer
stereotype, _ := stereotypes.GetStereotype(controller)
// stereotype == "controller"
```

---

## Best Practices

1. **Embed Base Types** - Always embed `BaseService`, `BaseRepository`, etc. for proper type classification

2. **Use Decorators for Cross-Cutting** - Retry, caching, logging, timing should use decorators, not inline code

3. **Implement Lifecycle Hooks** - Use `PostConstruct`/`PreDestroy` for resource initialization/cleanup

4. **Register Annotations Early** - Set up annotation registry at application startup

5. **Check Stereotypes at Runtime** - Use `GetStereotype()` for dynamic behavior based on type

---

## See Also

- [PyBoot Documentation](https://github.com/engineeringlabs/pyboot) - Python SEA framework
- [Rustboot Documentation](https://github.com/engineeringlabs/rustboot) - Rust SEA framework
- [DI Module](../di/README.md) - Dependency injection integration
- [Validation Module](../validation/README.md) - Input validation decorators
- [Resilience Module](../resilience/README.md) - Circuit breaker, retry patterns
