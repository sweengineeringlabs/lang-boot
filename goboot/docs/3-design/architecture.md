# Goboot Architecture

**Audience**: Architects, Technical Leadership, Security Teams

## WHAT: Architecture Overview

Goboot is a Go infrastructure framework that provides reusable cross-cutting concerns for production applications. It follows the Stratified Encapsulation Architecture (SEA) pattern.

| | |
|------|------|
| **Architecture** | SEA (Stratified Encapsulation Architecture) |
| **Language** | Go 1.21+ |
| **Dependencies** | Minimal core, optional extras |
| **Package Name** | `dev.engineeringlabs/goboot` |

## WHY: Design Rationale

### Problems Solved

1. **Boilerplate Reduction** - Common patterns implemented once
2. **Consistency** - Unified approach across applications
3. **Testability** - Interface-based abstractions
4. **Independence** - Modules have no internal dependencies

### Design Principles

- **Independent Modules** - Each module is self-contained
- **Interface-Based** - Use Go interfaces for abstractions
- **Generics Support** - Leverage Go generics where appropriate
- **No Reflection** - Avoid runtime reflection for type safety
- **Context-First** - All async operations accept context

## HOW: Architecture Details

### Module Structure

Each module follows the SEA layered pattern:

```
module/
├── module.go         # Public exports (facade)
├── doc.go            # Package documentation
├── api/              # Public interfaces (contracts)
│   └── interfaces.go
├── core/             # Implementations
│   └── impl.go
└── spi/              # Extension points (optional)
    └── provider.go
```

### Layer Responsibilities

| Layer | Purpose | Example |
|-------|---------|---------|
| **API** | Public contracts, interfaces, types | `Cache` interface |
| **Core** | Implementations | `MemoryCache`, `LRUCache` |
| **SPI** | Extension points for providers | `CacheBackend` |

### Module Independence

```
┌────────────────────────────────────────────────────────┐
│                    User Application                     │
├────────┬────────┬────────┬────────┬────────┬──────────┤
│ cache  │ config │ resil  │ di     │ valid  │  ...     │
│        │        │ ience  │        │ ation  │          │
└────────┴────────┴────────┴────────┴────────┴──────────┘
     │        │        │        │        │
     └────────┴────────┴────────┴────────┘
              (No internal dependencies)
```

### Import Pattern

```go
// Each module is imported independently
import (
    "dev.engineeringlabs/goboot/resilience"
    "dev.engineeringlabs/goboot/cache"
    "dev.engineeringlabs/goboot/observability"
)
```

## Related Documentation

- [Developer Guide](../4-development/developer-guide.md) - Development practices
- [Overview](../overview.md) - Module index
- [Backlog](../framework-backlog.md) - Planned features

---

**Status**: Active Development  
**Python Equivalent**: [pyboot](../../pyboot)
