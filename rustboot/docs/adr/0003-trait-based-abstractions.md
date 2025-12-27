# 3. Trait-Based Abstractions

**Status**: Accepted

**Date**: 2025-12-24

**Decision Makers**: Framework Architecture Team

## Context

A fundamental question when designing rustboot was: how should we provide functionality? Should we offer concrete implementations that users must use directly, or should we define abstract interfaces (traits) with pluggable implementations?

This decision affects:
1. **Testability**: Can users mock dependencies in tests?
2. **Flexibility**: Can users swap implementations (e.g., different databases, HTTP clients)?
3. **Extensibility**: Can users provide their own implementations?
4. **Simplicity**: How complex is the API for basic use cases?
5. **Type Safety**: How much compile-time checking do we get?
6. **Performance**: Does abstraction introduce runtime overhead?

Key framework components that need this decision:
- Database access (`Database`, `Transaction`, `Repository`)
- HTTP client (`HttpClient`)
- Message brokers (`MessageBroker`)
- Caching (`Cache`)
- State machines (`StateMachine`)
- Middleware (`Middleware`)
- Connection pooling (`ConnectionPool`)

## Decision

Rustboot will use **trait-based abstractions as the primary API pattern** for all major components.

Core principles:

1. **Traits Define Interfaces**: Define behavior through traits, not concrete types
2. **Multiple Implementations**: Provide at least one concrete implementation, allow others
3. **Generic Over Traits**: Functions and structs accept `impl Trait` or `&dyn Trait`
4. **Concrete Convenience**: Provide concrete types for common cases, but still implement traits
5. **Marker Bounds**: Use `Send + Sync` bounds for multi-threaded safety
6. **Sealed Traits**: Use sealed traits for internal implementation details only

Example pattern:
```rust
// 1. Define the trait
#[async_trait]
pub trait Database: Send + Sync {
    async fn query(&self, sql: &str) -> DatabaseResult<Vec<Row>>;
    async fn execute(&self, sql: &str) -> DatabaseResult<u64>;
}

// 2. Provide concrete implementation(s)
pub struct SqlxDatabase { /* ... */ }

#[async_trait]
impl Database for SqlxDatabase {
    async fn query(&self, sql: &str) -> DatabaseResult<Vec<Row>> {
        // implementation
    }
}

// 3. Accept trait in APIs
pub async fn migrate_database(db: &impl Database) -> Result<(), MigrationError> {
    // works with any Database implementation
}
```

## Consequences

### Positive

- **Testability**: Easy to create mock implementations for testing
  ```rust
  struct MockDatabase { /* ... */ }

  #[async_trait]
  impl Database for MockDatabase {
      async fn query(&self, sql: &str) -> DatabaseResult<Vec<Row>> {
          // return test data
      }
  }
  ```

- **Flexibility**: Users can swap implementations without code changes
  ```rust
  // Development: use SQLite
  let db: Box<dyn Database> = Box::new(SqlxDatabase::connect_sqlite("...").await?);

  // Production: use PostgreSQL
  let db: Box<dyn Database> = Box::new(SqlxDatabase::connect_postgres("...").await?);
  ```

- **Extensibility**: Users can provide custom implementations
  ```rust
  struct MyCustomDatabase { /* ... */ }

  impl Database for MyCustomDatabase {
      // custom implementation
  }
  ```

- **Composition**: Easy to create decorators and adapters
  ```rust
  struct CachedDatabase<D: Database> {
      inner: D,
      cache: Cache,
  }

  impl<D: Database> Database for CachedDatabase<D> {
      // delegate with caching
  }
  ```

- **Type Safety**: Compiler enforces trait bounds and method signatures
- **Documentation**: Traits clearly document expected behavior and contracts
- **Framework Integration**: Other frameworks can implement our traits
- **Dependency Injection**: Works naturally with DI containers

### Negative

- **Complexity**: More abstract than concrete types, steeper learning curve
- **Dynamic Dispatch Overhead**: `&dyn Trait` has small runtime cost (vtable lookup)
- **Binary Size**: Can increase binary size with monomorphization (`impl Trait`)
- **Error Messages**: Trait bound errors can be verbose and confusing
- **Lifetime Complexity**: Traits with lifetimes are harder to work with
- **Object Safety**: Not all traits can be used as `dyn Trait` (object-safe constraints)
- **Documentation Verbosity**: Must document both traits and implementations

### Neutral

- **Learning Curve**: Requires understanding Rust's trait system
- **API Stability**: Trait changes are breaking changes, need careful evolution
- **Implementation Burden**: Each abstraction needs at least one quality implementation

## Alternatives Considered

### 1. Concrete Types Only

**Approach**: Provide only concrete implementations, no traits.

```rust
pub struct SqlxDatabase { /* ... */ }

impl SqlxDatabase {
    pub async fn query(&self, sql: &str) -> DatabaseResult<Vec<Row>> { /* ... */ }
}
```

**Rejected because**:
- **No Testing**: Can't mock for unit tests without runtime database
- **Vendor Lock-in**: Users are locked to our specific implementations
- **No Extensibility**: Users can't provide alternatives
- **Framework Rigidity**: Can't adapt to new backends without breaking changes
- **Poor Composition**: Hard to add cross-cutting concerns (caching, logging)

### 2. Function Pointers / Closures

**Approach**: Use function pointers or closures instead of traits.

```rust
pub struct Database {
    query: Box<dyn Fn(&str) -> Pin<Box<dyn Future<Output = DatabaseResult<Vec<Row>>>>>>,
}
```

**Rejected because**:
- **Complex Signatures**: Function types for async are extremely verbose
- **Poor Discoverability**: No documentation or autocomplete for methods
- **No Static Checking**: Easy to pass wrong function signature
- **Ergonomics**: Very awkward to construct and use
- **No Method Syntax**: Can't use `db.query(...)` syntax

### 3. Enum-Based Dispatch

**Approach**: Use enums to represent different implementations.

```rust
pub enum Database {
    Sqlite(SqliteDatabase),
    Postgres(PostgresDatabase),
    Mysql(MysqlDatabase),
}

impl Database {
    pub async fn query(&self, sql: &str) -> DatabaseResult<Vec<Row>> {
        match self {
            Database::Sqlite(db) => db.query(sql).await,
            Database::Postgres(db) => db.query(sql).await,
            Database::Mysql(db) => db.query(sql).await,
        }
    }
}
```

**Rejected because**:
- **Closed Set**: Can't add new implementations without modifying framework
- **User Extensibility**: Users can't provide custom implementations
- **Framework Coupling**: All implementations must be in the framework crate
- **Code Bloat**: Match arms grow with every implementation
- **Dependency Bloat**: Must include all implementations even if unused

### 4. Macro-Based Code Generation

**Approach**: Use macros to generate implementations.

```rust
generate_database_impl! {
    for SqlxDatabase {
        async fn query(&self, sql: &str) -> DatabaseResult<Vec<Row>> { /* ... */ }
    }
}
```

**Rejected because**:
- **Magic**: Hard to understand what code is generated
- **Debugging**: Difficult to debug macro-generated code
- **IDE Support**: Poor autocomplete and error messages
- **Learning Curve**: Macros add complexity
- **Not Needed**: Traits already provide this functionality

### 5. Type Classes (Haskell-style)

**Approach**: Use generic parameters with trait bounds everywhere.

```rust
fn process<D: Database>(db: D) { /* ... */ }
```

**Rejected because**:
- **Monomorphization**: Generates separate code for each type, increases binary size
- **Inflexible**: Hard to store in collections or return from functions
- **Lifetime Issues**: Generic parameters complicate lifetimes
- **Not Always Appropriate**: Dynamic dispatch is better for some use cases

**Note**: We DO use this pattern where appropriate (e.g., builder patterns), but not exclusively.

## Design Patterns

### Pattern 1: Core Trait + Reference Impl

```rust
// Core trait
pub trait Cache: Send + Sync {
    async fn get(&self, key: &str) -> Option<Vec<u8>>;
    async fn set(&self, key: &str, value: Vec<u8>);
}

// Reference implementation
pub struct MemoryCache { /* ... */ }
impl Cache for MemoryCache { /* ... */ }

// Optional implementations (behind features)
#[cfg(feature = "redis")]
pub struct RedisCache { /* ... */ }

#[cfg(feature = "redis")]
impl Cache for RedisCache { /* ... */ }
```

### Pattern 2: Trait + Trait Object

```rust
// Accept trait objects for runtime polymorphism
pub struct CacheManager {
    cache: Box<dyn Cache>,
}

impl CacheManager {
    pub fn new(cache: Box<dyn Cache>) -> Self {
        Self { cache }
    }
}
```

### Pattern 3: Generic + Trait Bound

```rust
// Use generics for zero-cost abstraction
pub struct Repository<D: Database> {
    db: D,
}

impl<D: Database> Repository<D> {
    pub async fn find(&self, id: &str) -> Result<User, Error> {
        self.db.query(&format!("SELECT * FROM users WHERE id = {}", id)).await
    }
}
```

### Pattern 4: Extension Traits

```rust
// Add functionality through extension traits
#[async_trait]
pub trait DatabaseExt: Database {
    async fn query_one(&self, sql: &str) -> DatabaseResult<Option<Row>> {
        let mut rows = self.query(sql).await?;
        Ok(rows.pop())
    }
}

// Auto-implement for all Database types
impl<D: Database> DatabaseExt for D {}
```

## Implementation Guidelines

### Trait Design Checklist

- [ ] Is the trait object-safe? (Can it be used as `dyn Trait`?)
- [ ] Are all methods truly necessary? (Avoid trait bloat)
- [ ] Are there sensible default implementations?
- [ ] Does it need `Send + Sync` bounds?
- [ ] Are lifetimes minimized?
- [ ] Is it well-documented with examples?
- [ ] Are error types clear and consistent?

### When to Use Traits

**DO use traits for**:
- Core abstractions (database, HTTP, messaging)
- Pluggable backends
- Testable components
- Cross-cutting concerns (middleware, interceptors)
- Extension points for users

**DON'T use traits for**:
- Simple data structures
- Internal implementation details
- One-off utilities
- Pure functions with no state

### Performance Considerations

**Static Dispatch (`impl Trait`, generics)**:
- Zero runtime cost
- Larger binary size (monomorphization)
- Use for hot paths and library boundaries

**Dynamic Dispatch (`dyn Trait`, trait objects)**:
- Small vtable overhead (~1 pointer indirection)
- Smaller binary size
- Use for runtime polymorphism and plugin systems

## References

- [Rust Book: Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [Rust Book: Trait Objects](https://doc.rust-lang.org/book/ch17-02-trait-objects.html)
- [API Guidelines: Trait Design](https://rust-lang.github.io/api-guidelines/interoperability.html)
- [Object Safety](https://doc.rust-lang.org/reference/items/traits.html#object-safety)
- [The Rust Performance Book](https://nnethercote.github.io/perf-book/)

---

**Related ADRs**:
- [ADR-0001: Use async-trait](./0001-use-async-trait.md)
- [ADR-0002: Modular Crate Structure](./0002-modular-crate-structure.md)
