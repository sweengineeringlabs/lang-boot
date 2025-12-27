# Rustboot Macros Overview

**Audience**: Developers, Framework Users, Contributors

## WHAT: Procedural Macros for Rustboot Framework

Rustboot-macros provides compile-time code generation through procedural macros, enabling declarative programming patterns for cross-cutting concerns.

### Capabilities

**Derive Macros**:
- `#[derive(Injectable)]` - Automatic dependency injection registration
- `#[derive(Validate)]` - Field-level input validation

**Attribute Macros**:
- `#[cached]` - Result caching with TTL
- `#[traced]` - Automatic tracing spans
- `#[retry]` - Configurable retry logic
- `#[timed]` - Performance timing
- `#[circuit_breaker]` - Fault tolerance
- `#[rate_limit]` - API rate limiting
- `#[audit]` - Security audit logging
- `#[validate_params]` - Parameter validation

### Scope

**In Scope**:
- Compile-time code generation
- Integration with Rustboot crates (DI, validation, observability, resilience)
- Composable macros for cross-cutting concerns
- Zero runtime overhead for most macros

**Out of Scope**:
- Runtime behavior (delegated to core crates)
- Complex business logic generation
- Dynamic code modification

## WHY: Problems and Benefits

### Problems Addressed

**1. Boilerplate Code**
- **Current Impact**: Repetitive code for DI, validation, logging
- **Consequence**: Reduced productivity, inconsistent implementations

**2. Cross-Cutting Concerns**
- **Current Impact**: Manual instrumentation scattered across codebase
- **Consequence**: Hard to maintain, easy to forget, inconsistent

**3. Error-Prone Manual Implementation**
- **Current Impact**: Developers must remember to add logging, retries, etc.
- **Consequence**: Missing observability, inconsistent resilience

**4. Lack of Compile-Time Validation**
- **Current Impact**: Validation logic only runs at runtime
- **Consequence**: Later error detection, runtime failures

### Benefits

**For Developers**:
- ✅ **Reduced Boilerplate** - Write less code, express more intent
- ✅ **Consistency** - Same patterns across entire codebase
- ✅ **Compile-Time Safety** - Errors caught during compilation
- ✅ **Composition** - Combine multiple concerns declaratively

**For Codebase**:
- ✅ **Maintainability** - Centralized cross-cutting logic
- ✅ **Readability** - Clear intent through declarative syntax
- ✅ **Testability** - Generated code is predictable
- ✅ **Performance** - Zero runtime overhead for most macros

**For Framework**:
- ✅ **Integration** - Seamless with existing Rustboot crates
- ✅ **Extensibility** - Easy to add new macros
- ✅ **Standards** - Enforces framework conventions

## HOW: Implementation and Usage

### Architecture

```
rustboot-macros (proc-macro crate)
├── derive/
│   ├── injectable.rs    → Generates DI registration
│   └── validate.rs      → Generates validation logic
├── attrs/
│   ├── cached.rs        → Wraps with caching
│   ├── traced.rs        → Adds tracing spans
│   ├── retry.rs         → Adds retry logic
│   ├── timed.rs         → Adds timing
│   ├── circuit_breaker.rs → Adds fault tolerance
│   ├── audit.rs         → Adds audit logging
│   ├── rate_limit.rs    → Adds rate limiting
│   └── validate_params.rs → Validates parameters
└── utils.rs             → Shared parsing utilities
```

### Usage Examples

#### Dependency Injection

```rust
use rustboot_macros::Injectable;

#[derive(Injectable)]
struct UserService {
    repository: Arc<dyn UserRepository>,
    cache: Arc<Cache>,
}

// Generated:
impl UserService {
    pub fn from_container(container: &Arc<dyn Container>) 
        -> Result<Self, DiError> { ... }
    
    pub fn register(container: &mut dyn Container) 
        -> Result<(), DiError> { ... }
}
```

#### Input Validation

```rust
use rustboot_macros::Validate;

#[derive(Validate)]
struct CreateUser {
    #[validate(length(min = 3, max = 50))]
    username: String,
    
    #[validate(email)]
    email: String,
    
    #[validate(range(min = 18, max = 120))]
    age: u8,
}

// Usage:
let request = CreateUser { ... };
request.validate()?; // Returns ValidationResult
```

#### Observability

```rust
use rustboot_macros::{traced, timed};

#[traced(level = "info")]
#[timed(slow_threshold = 100)]
async fn process_order(order: Order) -> Result<()> {
    // Automatically:
    // - Creates tracing span with parameters
    // - Logs execution duration
    // - Warns if > 100ms
}
```

#### Resilience

```rust
use rustboot_macros::{retry, circuit_breaker};

#[retry(max_attempts = 3, backoff = "exponential")]
#[circuit_breaker(failure_threshold = 5)]
async fn call_external_api() -> Result<Response> {
    // Automatically:
    // - Retries with exponential backoff
    // - Protected by circuit breaker
}
```

#### Macro Composition

```rust
#[traced(level = "info")]
#[timed]
#[retry(max_attempts = 3)]
#[cached(ttl = 600)]
async fn expensive_operation(&self, id: u64) -> Result<Data> {
    // Combines:
    // - Tracing for debugging
    // - Timing for performance monitoring
    // - Retry for resilience
    // - Caching for efficiency
    self.repository.fetch(id).await
}
```

### Technical Implementation

**Derive Macros**:
1. Parse struct definition
2. Extract field types and attributes
3. Generate implementation code
4. Return as TokenStream

**Attribute Macros**:
1. Parse function signature
2. Extract and validate parameters
3. Wrap function body with additional logic
4. Return modified function

**Code Generation**:
- Uses `syn` for parsing Rust code
- Uses `quote` for generating code
- Uses `darling` for attribute parsing
- Leverages `proc-macro2` for token manipulation

### Integration with Rustboot

```rust
// DI Integration
rustboot-macros → rustboot-di
  Injectable generates code using Container trait

// Validation Integration
rustboot-macros → rustboot-validation
  Validate generates code using Validator traits

// Observability Integration
rustboot-macros → rustboot-observability
  traced/timed use tracing and metrics APIs

// Resilience Integration
rustboot-macros → rustboot-resilience
  retry/circuit_breaker use RetryPolicy, CircuitBreaker
```

### Current Status

| Component | Status | Notes |
|-----------|--------|-------|
| Injectable | ✅ Implemented | Full DI integration |
| Validate | ✅ Implemented | email, length, range validators |
| traced | ✅ Implemented | Full tracing support |
| retry | ✅ Implemented | Multiple backoff strategies |
| timed | ✅ Implemented | Metrics integration |
| circuit_breaker | ✅ Implemented | Fault tolerance |
| audit | ✅ Implemented | Security logging |
| cached | ⚠️ Partial | Needs runtime cache |
| rate_limit | ⚠️ Stub | Implementation needed |
| validate_params | ⚠️ Stub | Implementation needed |
| Tests | ✅ In Progress | Basic tests added |
| Syn 2.0 | ✅ Complete | API updated |

### Best Practices

#### DO ✅

- **Compose macros** for multiple concerns
- **Order matters** - outermost macro executes first
- **Use validation** for all user input
- **Add tracing** to important operations
- **Apply retry** to flaky external calls
- **Cache expensive** operations

#### DON'T ❌

- **Over-use macros** - Not everything needs to be macro-ized
- **Ignore errors** - Check macro-generated error messages
- **Skip testing** - Test functions with macros applied
- **Nest too deeply** - Keep composition reasonable
- **Use for business logic** - Macros are for cross-cutting only

### Troubleshooting

**Problem**: Macro doesn't compile
- **Solution**: Check attribute syntax, ensure required crates are available

**Problem**: Generated code doesn't match expectations
- **Solution**: Use `cargo expand` to see generated code

**Problem**: Macros conflict when composed
- **Solution**: Check macro ordering, some combinations invalid

**Problem**: IDE shows errors but code compiles
- **Solution**: Restart rust-analyzer, proc-macros are complex

## Summary

Rustboot-macros provides powerful compile-time code generation for common patterns:

**Key Features**:
- Derive macros for DI and validation
- Attribute macros for observability, resilience, caching
- Full integration with Rustboot framework
- Composable for complex scenarios

**When to Use**:
- Reducing boilerplate (DI, validation)
- Adding cross-cutting concerns (logging, retry)
- Enforcing patterns (audit, rate limiting)
- Compile-time validation

**When Not to Use**:
- Complex business logic
- Runtime-dynamic behavior
- Simple one-off cases

---

**Related Documentation**:
- [Rustboot Overview](../../../docs/overview.md) - Framework overview
- [Developer Guide](../../../docs/4-development/developer-guide.md) - Development patterns
- [Backlog](backlog.md) - Future enhancements
- [README](../README.md) - Quick start and examples

**External Resources**:
- [The Rust Reference: Procedural Macros](https://doc.rust-lang.org/reference/procedural-macros.html)
- [syn Documentation](https://docs.rs/syn/)
- [quote Documentation](https://docs.rs/quote/)
- [darling Documentation](https://docs.rs/darling/)

**Last Updated**: 2025-12-22  
**Version**: 1.0  
**Status**: Work in Progress
