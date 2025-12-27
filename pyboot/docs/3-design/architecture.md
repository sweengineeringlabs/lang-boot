# Pyboot Architecture

**Audience**: Architects, Technical Leadership, Security Teams

## WHAT: Architecture Overview

Pyboot is a Python infrastructure framework that provides reusable cross-cutting concerns for production applications. It follows the Stratified Encapsulation Architecture (SEA) pattern.

| | |
|------|------|
| **Architecture** | SEA (Stratified Encapsulation Architecture) |
| **Language** | Python 3.11+ |
| **Dependencies** | Minimal core, optional extras |
| **Package Name** | `dev.engineeringlabs.pyboot` |

## WHY: Design Rationale

### Problems Solved

1. **Boilerplate Reduction** - Common patterns implemented once
2. **Consistency** - Unified approach across applications
3. **Testability** - Protocol-based abstractions
4. **Independence** - Modules have no internal dependencies

### Design Principles

- **Independent Modules** - Each module is self-contained
- **Protocol-Based** - Use Python protocols for abstractions
- **Decorator-Driven** - Decorators for cross-cutting concerns
- **Zero Reflection** - No runtime introspection magic

## HOW: Architecture Details

### Module Structure

Each module follows the SEA layered pattern:

```
module/
├── __init__.py          # Public exports
├── api/                 # Public interfaces (protocols, types)
│   └── __init__.py
├── core/                # Implementations
│   └── __init__.py
└── spi/                 # Extension points (optional)
    └── __init__.py
```

### Layer Responsibilities

| Layer | Purpose | Example |
|-------|---------|---------|
| **API** | Public contracts, protocols, types | `RateLimiter` protocol |
| **Core** | Implementations | `TokenBucket`, `SlidingWindow` |
| **SPI** | Extension points for providers | `RateLimiterProvider` |

### Module Independence

```
┌────────────────────────────────────────────────────────┐
│                    User Application                     │
├────────┬────────┬────────┬────────┬────────┬──────────┤
│ cache  │ config │ resil  │ http   │ valid  │  ...     │
│        │        │ ience  │        │ ation  │          │
└────────┴────────┴────────┴────────┴────────┴──────────┘
     │        │        │        │        │
     └────────┴────────┴────────┴────────┘
              (No internal dependencies)
```

### Import Pattern

```python
# Each module is imported independently
from dev.engineeringlabs.pyboot.resilience import retryable, circuit_breaker
from dev.engineeringlabs.pyboot.cache import cached
from dev.engineeringlabs.pyboot.observability import traced
```

## Related Documentation

- [Developer Guide](../4-development/developer-guide.md) - Development practices
- [Overview](../overview.md) - Module index
- [Backlog](../backlog.md) - Planned features

---

**Status**: Active Development  
**Rust Equivalent**: [rustboot](../../rustboot)
