# Rustboot Extraction Plan

## Vision

**Rustratify → SEA Reference Implementation**
- Provider trait
- Registry pattern
- Config system  
- Event streams
- Pure architectural example

**Rustboot → Application Framework**
- 10 standalone crates
- Production-ready features
- Workspace organization

---

## Migration Steps

### Phase 1: Rustratify Cleanup ✅
- [x] Revert workspace changes
- [x] Remove `crates/` directory
- [x] Restore v2.0.0 stable state
- [ ] Remove framework features from `src/core/`
- [ ] Keep only: config, error, registry, stream
- [ ] Update documentation

### Phase 2: Create Rustboot Repository
- [ ] Create new repo: `rustboot`
- [ ] Initialize workspace structure
- [ ] Copy 10 framework modules from rustratify

### Phase 3: Rustboot Crates
Extract these from rustratify `src/core/`:

1. **rustboot-validation** ← `src/core/validation/`
2. **rustboot-cache** ← `src/core/caching/`
3. **rustboot-di** ← `src/core/di/`
4. **rustboot-state-machine** ← `src/core/state_machine/`
5. **rustboot-http** ← `src/core/http/`
6. **rustboot-messaging** ← `src/core/messaging/`
7. **rustboot-database** ← `src/core/database/`
8. **rustboot-middleware** ← `src/core/middleware/`
9. **rustboot-observability** ← `src/core/observability/`
10. **rustboot-testing** ← `src/core/testing/`

### Phase 4: Rustboot Main Crate
- Facade crate that re-exports all
- Clean documentation
- Examples

---

## File Removals from Rustratify

Remove these directories:
```
src/core/validation/
src/core/caching/
src/core/di/
src/core/state_machine/
src/core/http/
src/core/messaging/
src/core/database/
src/core/middleware/
src/core/observability/
src/core/testing/
```

Keep only:
```
src/core/config/
src/core/error/
src/core/registry/
src/core/stream/
src/spi/
```

---

## Rustboot Structure

```
rustboot/
├── Cargo.toml (workspace root)
├── rustboot/ (main facade crate)
│   ├── Cargo.toml
│   └── src/lib.rs
└── crates/
    ├── rustboot-validation/
    ├── rustboot-cache/
    ├── rustboot-di/
    ├── rustboot-state-machine/
    ├── rustboot-http/
    ├── rustboot-messaging/
    ├── rustboot-database/
    ├── rustboot-middleware/
    ├── rustboot-observability/
    └── rustboot-testing/
```

---

## Benefits

**Rustratify**:
- ✅ Stays focused on SEA architecture
- ✅ Clean reference implementation
- ✅ Easy to understand
- ✅ Minimal dependencies

**Rustboot**:
- ✅ Full-featured framework
- ✅ Independent evolution
- ✅ Workspace benefits
- ✅ Can reference Rustratify's SEA pattern

---

## Next Steps

1. Clean up Rustratify (remove framework code)
2. Create Rustboot repo
3. Move code to Rustboot
4. Update both READMEs
5. Cross-reference in docs
