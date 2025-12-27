# DI Module Overview

## WHAT: Dependency Injection

Container-based DI with scopes and lifecycle management.

Key capabilities:
- **Container** - Service registration
- **Scopes** - Singleton, transient
- **Lifecycle** - Init/destroy hooks
- **Interfaces** - Interface-based wiring

## WHY: Loose Coupling

**Problems Solved**: Tight coupling, testability

**When to Use**: Medium to large applications

## HOW: Usage Guide

```go
container := di.NewContainer()

container.Register(func() UserRepository {
    return NewUserRepositoryImpl(db)
})

container.RegisterSingleton(func() *Config {
    return LoadConfig()
})

userRepo := di.Resolve[UserRepository](container)
```

---

**Status**: Stable
