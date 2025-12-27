# Rustboot Macros Backlog

Future enhancements and improvements for the rustboot-macros crate.

## P0: Critical (Blocking Release)

- [x] Fix Syn 2.0 compatibility
- [x] Add basic tests
- [x] Create docs/overview.md
- [ ] Test with actual rustboot crates integration
- [ ] Verify all macros compile

## P1: High Priority (Should Have)

### Complete Stub Implementations

- [ ] **rate_limit.rs**
  - Implement token bucket rate limiting
  - Support custom key extraction
  - Integration with rustboot-ratelimit

- [ ] **validate_params.rs**
  - Parse parameter attributes
  - Generate validation code per-parameter
  - Support nested validation

- [ ] **cached.rs**
  - Complete runtime cache integration
  - Support async caching
  - Custom cache key generation from parameters

### Testing

- [ ] **Compile-fail tests**
  - Invalid attribute syntax
  - Incompatible macro compositions
  - Missing required attributes

- [ ] **Integration tests**
  - Test with rustboot-di container
  - Test with rustboot-validation framework
  - Test with rustboot-observability

- [ ] **Expansion tests** 
  - Use `cargo expand` in tests
  - Verify generated code matches expectations

### Error Messages

- [ ] **Better diagnostics**
  - Point to exact attribute causing error
  - Suggest correct syntax
  - Show examples of valid usage

- [ ] **Validation errors**
  - Clear messages for invalid combinations
  - Helpful suggestions for fixes

## P2: Medium Priority (Nice to Have)

### Additional Validators

- [ ] **regex validator**
  - `#[validate(regex = "pattern")]`
  - Integration with regex crate

- [ ] **custom validator**
  - `#[validate(custom = "function_name")]`
  - Allow user-defined validation functions

- [ ] **nested validation**
  - `#[validate(nested)]`  
  - Validate nested structs

- [ ] **collection validation**
  - `#[validate(each(length(min = 1)))]`
  - Validate Vec/HashMap elements

### Macro Enhancements

- [ ] **Conditional compilation**
  - `#[cached(cfg(feature = "caching"))]`
  - Feature-gated macro application

- [ ] **Async trait support**
  - Work with #[async_trait]
  - Handle trait methods

- [ ] **Macro composition validation**
  - Warn about invalid orderings
  - Suggest optimal macro order

### Examples

- [ ] **di_example.rs**
  - Real DI container usage
  - Multiple services with dependencies

- [ ] **validation_example.rs**
  - Complex validation scenarios
  - Custom validators

- [ ] **observability_example.rs**
  - traced + timed + audit together
  - Full observability stack

- [ ] **resilience_example.rs**
  - retry + circuit_breaker + rate_limit
  - Complete resilience patterns

### Performance

- [ ] **Benchmarks**
  - Measure compile-time overhead
  - Compare to hand-written code
  - Identify optimization opportunities

- [ ] **Optimization**
  - Reduce generated code size
  - Minimize TokenStream manipulations
  - Cache parsed results where possible

## P3: Low Priority (Future)

### Advanced Features

- [ ] **Declarative macros**
  - Add `macro_rules!` for common patterns
  - Simpler alternatives for simple cases

- [ ] **Code generation hooks**
  - Allow customization of generated code
  - Plugin system for custom generators

- [ ] **Macro debugging**
  - Better error traces
  - Step-through macro expansion

### Documentation

- [ ] **Macro expansion guide**
  - Document what each macro generates
  - Show before/after code

- [ ] **Best practices guide**
  - When to use which macro
  - Composition patterns
  - Performance considerations

- [ ] **Migration guide**
  - How to adopt macros in existing code
  - Step-by-step refactoring

### Tooling

- [ ] **cargo-expand integration**
  - Better formatting of expanded code
  - Diff against expected output

- [ ] **IDE support**
  - rust-analyzer hints
  - Inline macro expansion
  - go-to-definition for generated code

- [ ] **Linting**
  - Custom clippy lints for macro usage
  - Warn about anti-patterns

## Completed

- [x] Create rustboot-macros crate
- [x] Implement Injectable derive macro
- [x] Implement Validate derive macro
- [x] Implement traced attribute macro
- [x] Implement retry attribute macro
- [x] Implement timed attribute macro
- [x] Implement circuit_breaker attribute macro
- [x] Implement audit attribute macro
- [x] Create comprehensive README
- [x] Create usage examples
- [x] Fix Syn 2.0 compatibility (utils.rs)
- [x] Create docs/overview.md

## Ideas / Brainstorm

### Potential New Macros

- [ ] `#[memoize]` - Function result memoization
- [ ] `#[timeout(duration)]` - Automatic timeout wrapper
- [ ] `#[metrics]` - Comprehensive metrics collection
- [ ] `#[authorize(role = "admin")]` - Role-based authorization
- [ ] `#[rate_limit_per_user]` - User-specific rate limiting
- [ ] `#[deprecation(since = "1.2", note = "...")]` - Enhanced deprecation
- [ ] `#[feature_flag("feature_name")]` - Feature flag gating

### Integration Ideas

- [ ] OpenTelemetry integration for traced
- [ ] Prometheus metrics for timed
- [ ] Custom audit backends
- [ ] Distributed tracing support

---

**Last Updated**: 2025-12-22
**Maintained By**: Rustboot Team
