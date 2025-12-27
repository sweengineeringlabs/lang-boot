# 5. Axum as Web Framework Integration

**Status**: Accepted

**Date**: 2025-12-24

**Decision Makers**: Framework Architecture Team

## Context

Rustboot provides web server abstractions through the `rustboot-web` crate. A key decision was which web framework to integrate with for the concrete implementation. The web framework choice affects:

1. **Performance**: Request throughput, latency, resource usage
2. **Developer Experience**: API ergonomics, learning curve, documentation
3. **Ecosystem**: Middleware availability, community support
4. **Type Safety**: Compile-time guarantees, error handling
5. **Async Runtime**: Compatibility with tokio and async ecosystem
6. **Flexibility**: Extensibility, customization options
7. **Stability**: Maturity, maintenance, breaking changes

Requirements:
- Built on tokio (rustboot's async runtime)
- Strong type safety and compile-time guarantees
- Good performance characteristics
- Active maintenance and community
- Middleware ecosystem
- Compatible with tower ecosystem
- Support for HTTP/1.1 and HTTP/2

Major Rust web framework options:
- **Axum** - Ergonomic, type-safe, built on tower/hyper
- **Actix-Web** - High performance, actor-based
- **Rocket** - Batteries-included, macro-heavy
- **Warp** - Filter-based, functional style
- **Tide** - Async-std based (different runtime)

## Decision

We will use **Axum** as the web framework integration for the `rustboot-web` crate.

Implementation details:

1. **Feature Flag**: Axum integration is behind the `axum` feature (enabled by default)
2. **Bridge Layer**: Create adapter layer between rustboot traits and axum types
3. **Middleware Integration**: Bridge rustboot middleware to tower middleware
4. **Extractors**: Provide rustboot-style extractors that wrap axum extractors
5. **Router Builder**: Ergonomic builder API that generates axum routers
6. **Future Compatibility**: Design allows for additional framework integrations

Architecture:
```rust
// rustboot-web defines abstractions
pub trait Router { /* ... */ }

// Axum integration (optional feature)
#[cfg(feature = "axum")]
pub struct AxumRouterBuilder { /* ... */ }

#[cfg(feature = "axum")]
impl Router for AxumRouterBuilder { /* ... */ }
```

Dependencies:
```toml
[dependencies]
axum = { version = "0.7", optional = true }
tower = { version = "0.5", optional = true }
tower-http = { version = "0.5", features = ["trace", "cors"], optional = true }
hyper = { version = "1.0", optional = true }
tokio = { version = "1.0", features = ["net", "rt"], optional = true }
```

## Consequences

### Positive

- **Type Safety**: Axum's type-safe extractors catch errors at compile time
  ```rust
  async fn handler(
      Path(id): Path<String>,
      Json(payload): Json<CreateUser>,
  ) -> Result<Json<User>, StatusCode> {
      // Types are checked at compile time
  }
  ```

- **Performance**: Excellent performance benchmarks, competitive with actix-web
  - Built on hyper (one of the fastest HTTP implementations)
  - Zero-cost abstractions through tower middleware
  - Efficient routing with matchit-based router

- **Tokio Integration**: First-class tokio support (same runtime as rustboot)
- **Tower Ecosystem**: Access to extensive tower middleware
  - Timeout, rate limiting, retry, load balancing
  - Compression, tracing, CORS
  - Easy to compose middleware

- **Ergonomic API**: Clean, intuitive API with minimal boilerplate
  ```rust
  let app = Router::new()
      .route("/users/:id", get(get_user))
      .route("/users", post(create_user))
      .layer(TraceLayer::new_for_http());
  ```

- **Active Development**: Well-maintained by the tokio team
- **Minimal Macros**: Relies on Rust's type system, not proc macros
- **WebSocket Support**: Built-in WebSocket support
- **HTTP/2**: Full HTTP/2 support through hyper
- **Community**: Growing ecosystem with good documentation
- **Error Handling**: Type-safe error handling with `IntoResponse`

### Negative

- **Younger Framework**: Less mature than actix-web (but rapidly evolving)
- **Learning Curve**: Tower middleware concepts can be complex
- **API Churn**: API has had breaking changes (though stabilizing)
- **Documentation Gaps**: Some advanced patterns are under-documented
- **Ecosystem Size**: Smaller ecosystem compared to actix-web
- **Version 0.x**: Still pre-1.0, API stability not guaranteed
- **Complexity**: Type-heavy API can lead to complex error messages
- **Lock-in**: Tight coupling to tower/hyper ecosystem

### Neutral

- **Macro Light**: Minimal macros is good for transparency, but more verbose than Rocket
- **Performance Trade-offs**: Slightly slower than actix-web in some benchmarks, but more ergonomic
- **Framework Size**: Medium-sized dependency tree

## Alternatives Considered

### 1. Actix-Web

**Approach**: Use actix-web as the web framework.

**Pros**:
- Proven performance leader in benchmarks
- Mature ecosystem with many plugins
- Comprehensive documentation
- Feature-rich out of the box

**Cons**:
- Actor-based model adds complexity
- Different async runtime (actix-rt, though compatible with tokio)
- More macro-heavy approach
- Past maintainer issues (though resolved)
- Different mental model than tower ecosystem

**Rejected because**: Axum's type safety and tower integration align better with rustboot's design philosophy. The performance difference is negligible for most use cases.

### 2. Rocket

**Approach**: Use Rocket as the web framework.

**Pros**:
- Batteries-included approach
- Excellent documentation
- Mature and stable
- Great developer experience

**Cons**:
- Heavy use of proc macros (compile times, transparency)
- Moved to async relatively recently
- Custom codegen can conflict with rustboot macros
- Less flexible than axum for custom integrations

**Rejected because**: Heavy macro usage conflicts with rustboot's trait-based approach. Less composable with tower ecosystem.

### 3. Warp

**Approach**: Use warp as the web framework.

**Pros**:
- Built on hyper and tokio
- Filter-based composition
- Type-safe
- Good performance

**Cons**:
- Functional filter style is less intuitive for many developers
- Steep learning curve
- Error messages can be cryptic
- Less active development recently
- Smaller community

**Rejected because**: Filter-based API is less ergonomic than axum's routing. Axum evolved from warp's lessons learned.

### 4. Tide

**Approach**: Use tide as the web framework.

**Pros**:
- Simple, ergonomic API
- Middleware-centric design

**Cons**:
- Built on async-std, not tokio (incompatible with rustboot's runtime choice)
- Development has slowed significantly
- Smaller ecosystem
- Would require dual runtime support

**Rejected because**: Incompatible async runtime. Development activity has decreased.

### 5. Custom Implementation

**Approach**: Build custom web framework on top of hyper/tower directly.

**Pros**:
- Full control over API design
- Optimized for rustboot use cases
- No external framework dependency

**Cons**:
- Massive development effort
- Maintenance burden
- Reinventing the wheel
- Missing ecosystem integrations
- Fewer battle-tested features

**Rejected because**: Not a good use of resources. Axum already provides what we need.

### 6. Multiple Framework Support

**Approach**: Support multiple frameworks behind feature flags.

```toml
[features]
axum = [...]
actix = [...]
rocket = [...]
```

**Rejected because**:
- Massive maintenance burden
- Testing complexity (must test all combinations)
- API fragmentation (different frameworks have different patterns)
- Dependency conflicts
- Not enough user demand to justify cost

## Integration Architecture

### Layer 1: Rustboot Abstractions

```rust
// rustboot-web core traits
pub trait Router {
    fn route(&mut self, path: &str, method: RouteMethod, handler: Box<dyn Handler>);
    fn serve(&self, addr: &str) -> impl Future<Output = Result<(), WebError>>;
}
```

### Layer 2: Axum Bridge

```rust
// Axum-specific implementation
#[cfg(feature = "axum")]
pub struct AxumRouterBuilder {
    router: axum::Router,
}

#[cfg(feature = "axum")]
impl AxumRouterBuilder {
    pub fn new() -> Self { /* ... */ }

    pub fn into_axum_router(self) -> axum::Router {
        self.router
    }
}
```

### Layer 3: Middleware Bridge

```rust
// Convert rustboot middleware to tower layers
pub fn rustboot_to_tower<M: WebMiddleware>(
    middleware: M
) -> tower::Layer { /* ... */ }
```

## Migration Strategy

If we need to support additional frameworks:

1. **Abstract Further**: Refine rustboot-web traits to be framework-agnostic
2. **Feature Flags**: Add new framework behind separate feature flag
3. **Compatibility Module**: Create bridge adapters for new framework
4. **Documentation**: Provide migration guide between frameworks
5. **Deprecation**: If replacing axum, provide long deprecation period

## Performance Benchmarks

From [TechEmpower Framework Benchmarks Round 22](https://www.techempower.com/benchmarks/):

| Framework | Requests/sec (plaintext) | Relative to Axum |
|-----------|--------------------------|------------------|
| Axum      | ~600,000                 | 1.0x             |
| Actix-Web | ~650,000                 | 1.08x            |
| Warp      | ~580,000                 | 0.97x            |
| Rocket    | ~500,000                 | 0.83x            |

**Conclusion**: Performance differences are minimal and unlikely to be bottleneck for most applications.

## Version Strategy

- **Axum 0.7.x**: Current version (as of Dec 2024)
- **Monitoring**: Watch for axum 1.0 release
- **Updates**: Stay on latest stable axum minor version
- **Breaking Changes**: Handle axum breaking changes in rustboot major versions

## Future Enhancements

- [ ] Custom error handling integration
- [ ] OpenAPI/Swagger generation hooks
- [ ] GraphQL support via async-graphql integration
- [ ] WebSocket abstractions
- [ ] Server-Sent Events (SSE) support
- [ ] gRPC integration via tonic

## References

- [Axum Documentation](https://docs.rs/axum/)
- [Axum GitHub](https://github.com/tokio-rs/axum)
- [Tower Documentation](https://docs.rs/tower/)
- [Hyper Documentation](https://docs.rs/hyper/)
- [TechEmpower Benchmarks](https://www.techempower.com/benchmarks/)
- [Rust Web Framework Comparison](https://www.arewewebyet.org/topics/frameworks/)

---

**Related ADRs**:
- [ADR-0001: Use async-trait](./0001-use-async-trait.md)
- [ADR-0002: Modular Crate Structure](./0002-modular-crate-structure.md)
- [ADR-0003: Trait-Based Abstractions](./0003-trait-based-abstractions.md)
