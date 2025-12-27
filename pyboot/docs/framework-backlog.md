# Pyboot Framework-Wide Backlog

Framework-wide improvements and cross-cutting concerns for pyboot.

## Implementation Gaps

### Tests
- [ ] Add unit tests for all 37 modules (currently ~10% coverage)
- [ ] Add integration tests for cross-module functionality
- [ ] Add property-based testing for validation, serialization

### Examples
- [x] 12 example files created
- [ ] Add examples for remaining 25 modules
- [ ] Create end-to-end application example

### Documentation
- [x] Main docs/overview.md hub
- [x] Architecture documentation
- [x] Developer guide
- [ ] Per-module doc/overview.md files
- [ ] API documentation generation

## Module-Specific Gaps

### High Priority
- [ ] `config` - Hot-reload implementation
- [ ] `database` - Connection pooling
- [ ] `security` - JWT validation, OAuth2

### Medium Priority
- [ ] `cache` - Redis backend
- [ ] `http` - HTTP/2 support
- [ ] `observability` - OpenTelemetry integration

### Low Priority
- [ ] `compress` - Brotli, Zstd support
- [ ] `crypto` - Additional algorithms

## Developer Experience

- [ ] CLI tool for module scaffolding
- [ ] Type stubs for IDE support
- [ ] Debug utilities

## Infrastructure

- [ ] CI/CD pipeline
- [ ] Automated testing on PR
- [ ] Package publishing to PyPI
- [ ] Documentation generation

---

## Summary

| Category | Status | Notes |
|----------|--------|-------|
| Core Modules | 37/37 | All implemented |
| Tests | 196 tests | ~10% coverage |
| Examples | 12/37 | Need more examples |
| Documentation | 60% | Missing module docs |

**Completion Estimate**: ~40% complete  
**Last Updated**: 2025-12-26
