# DI Module Overview

## WHAT: Dependency Injection

Container-based dependency injection with scopes, lifecycle management, and auto-wiring.

Key capabilities:
- **Container** - Service registration and resolution
- **Scopes** - Singleton, request, transient
- **Lifecycle** - PostConstruct, PreDestroy hooks
- **Auto-wiring** - Constructor injection

## WHY: Loose Coupling

**Problems Solved**:
1. **Tight Coupling** - Interface-based dependencies
2. **Testability** - Easy mocking
3. **Configuration** - Centralized wiring

**When to Use**: Medium to large applications

## HOW: Usage Guide

```java
var container = Container.builder()
    .register(UserRepository.class, UserRepositoryImpl.class)
    .register(UserService.class, Scope.SINGLETON)
    .build();

var service = container.resolve(UserService.class);

// Scoped
try (var scope = container.createScope()) {
    var handler = scope.resolve(RequestHandler.class);
}
```

## Relationship to Other Modules

| Module | Relationship |
|--------|--------------|
| All modules | Wire dependencies |

---

**Status**: Stable
