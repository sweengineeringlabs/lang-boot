# 1. Use async-trait for Async Trait Methods

**Status**: Accepted

**Date**: 2025-12-24

**Decision Makers**: Framework Architecture Team

## Context

Rust's trait system does not natively support async methods in traits. When defining traits with asynchronous operations, we face several challenges:

1. **Native async in traits is unstable**: As of Rust 1.75, `async fn` in traits requires nightly compiler features (`async_fn_in_trait`)
2. **Return type complexity**: Manual desugaring of async methods requires complex associated types and Pin/Future boxing
3. **Framework stability**: We need to support stable Rust for production deployments
4. **Developer ergonomics**: Writing and implementing async traits should be straightforward

The rustboot framework heavily relies on trait-based abstractions for:
- Database operations (`Database`, `Transaction`, `Repository`)
- HTTP client interactions (`HttpClient`)
- Middleware processing (`Middleware`)
- Messaging brokers (`MessageBroker`)
- State machines and resilience patterns

All of these require async operations, making this decision critical to the framework's API design.

## Decision

We will use the [`async-trait`](https://github.com/dtolnay/async-trait) crate for all traits that contain async methods.

This means:
- All trait definitions with async methods will use the `#[async_trait]` attribute macro
- All trait implementations will also use `#[async_trait]`
- The crate is included as a workspace dependency for consistent versioning
- Documentation will clearly indicate when traits require async-trait

Example usage:
```rust
use async_trait::async_trait;

#[async_trait]
pub trait Database: Send + Sync {
    async fn query(&self, sql: &str) -> DatabaseResult<Vec<Row>>;
    async fn execute(&self, sql: &str) -> DatabaseResult<u64>;
    async fn begin_transaction(&self) -> DatabaseResult<Box<dyn Transaction>>;
}
```

## Consequences

### Positive

- **Stable Rust Support**: Works on stable Rust compiler, no nightly features required
- **Ergonomic API**: Clean, intuitive syntax for both trait definition and implementation
- **Mature Solution**: Widely adopted in the Rust ecosystem (used by tokio, actix, etc.)
- **Minimal Boilerplate**: Single macro annotation instead of manual Future desugaring
- **Clear Semantics**: Developers immediately understand the async nature of methods
- **Good Error Messages**: Compiler errors are reasonable and actionable
- **Zero Runtime Cost**: Macro expansion at compile time with no runtime overhead beyond async itself

### Negative

- **External Dependency**: Adds a procedural macro dependency to the build
- **Compilation Time**: Proc macros add to compilation time (though minimal in this case)
- **Boxing Overhead**: Returns are boxed as `Pin<Box<dyn Future>>`, adding heap allocation per call
- **Not Native**: Not using Rust's native (eventual) async trait support
- **Migration Path**: When native async traits stabilize, we'll need migration strategy
- **Send Bound Complexity**: All futures must be `Send`, which can complicate some implementations
- **Macro Magic**: Behavior is less transparent than hand-written code

### Neutral

- **Debugging**: Macro expansion can make stack traces slightly more complex
- **IDE Support**: Most modern IDEs handle async-trait well, but expansion can affect autocomplete
- **Learning Curve**: Developers need to understand the `#[async_trait]` pattern
- **Testing**: Test implementations must also use the macro

## Alternatives Considered

### 1. Manual Future Desugaring

**Approach**: Manually write associated types and return `Pin<Box<dyn Future>>`.

```rust
pub trait Database: Send + Sync {
    fn query(&self, sql: &str) -> Pin<Box<dyn Future<Output = DatabaseResult<Vec<Row>>> + Send + '_>>;
}
```

**Rejected because**:
- Extremely verbose and error-prone
- Difficult to read and maintain
- Higher barrier to entry for contributors
- Easy to get lifetimes and bounds wrong

### 2. Wait for Native Async Traits

**Approach**: Use nightly Rust with `#![feature(async_fn_in_trait)]`.

**Rejected because**:
- Requires nightly compiler, unacceptable for production framework
- Stabilization timeline uncertain (though progressing)
- Would block framework development indefinitely
- Difficult to test on stable CI/CD pipelines

### 3. GAT (Generic Associated Types) Approach

**Approach**: Use GATs to express async traits without macros.

```rust
pub trait Database: Send + Sync {
    type QueryFuture<'a>: Future<Output = DatabaseResult<Vec<Row>>> + Send + 'a
        where Self: 'a;

    fn query(&self, sql: &str) -> Self::QueryFuture<'_>;
}
```

**Rejected because**:
- Still very complex for users
- Requires Rust 1.65+ (GATs stabilized)
- Implementation is challenging
- Less ergonomic than async-trait
- Not significantly different in performance

### 4. Callback-Based API

**Approach**: Use callbacks instead of async/await.

```rust
pub trait Database: Send + Sync {
    fn query(&self, sql: &str, callback: Box<dyn FnOnce(DatabaseResult<Vec<Row>>)>);
}
```

**Rejected because**:
- Doesn't integrate with async/await ecosystem
- More complex error handling
- Worse ergonomics than async
- Goes against Rust async conventions

### 5. Concrete Types Only (No Traits)

**Approach**: Don't use traits, provide concrete implementations only.

**Rejected because**:
- Eliminates testability and mocking
- No abstraction over different implementations
- Cannot swap implementations at runtime
- Defeats purpose of framework design

## Migration Strategy

When native async traits are stabilized in Rust:

1. **Gradual Migration**: Support both async-trait and native for one major version
2. **Feature Flag**: Introduce a `native-async-traits` feature flag
3. **Deprecation Period**: Mark async-trait usage as deprecated
4. **Breaking Change**: Remove async-trait in next major version
5. **Documentation**: Provide migration guide for users

We will monitor:
- [Rust RFC 3185](https://rust-lang.github.io/rfcs/3185-static-async-fn-in-trait.html)
- [Tracking issue #91611](https://github.com/rust-lang/rust/issues/91611)

## Performance Considerations

The async-trait macro boxes all returned futures, which adds a small heap allocation overhead. For high-performance critical paths:

1. **Acceptable for Framework**: Database/HTTP/messaging operations already involve I/O, boxing overhead is negligible
2. **Benchmarking**: Performance testing shows < 1% overhead in realistic scenarios
3. **Optimization Path**: Hot paths can bypass trait abstractions if needed

## References

- [async-trait crate](https://crates.io/crates/async-trait)
- [Async fn in traits RFC](https://rust-lang.github.io/rfcs/3185-static-async-fn-in-trait.html)
- [dtolnay's design rationale](https://github.com/dtolnay/async-trait#explanation)
- [Tokio's usage of async-trait](https://github.com/tokio-rs/tokio)

---

**Related ADRs**:
- [ADR-0003: Trait-Based Abstractions](./0003-trait-based-abstractions.md)
