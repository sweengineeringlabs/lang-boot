# Web Module Overview

## WHAT: Web Framework Utilities

Router, middleware, request/response handling for web applications.

Key capabilities:
- **Router** - Path-based routing
- **Middleware** - Request/response pipeline
- **Request** - Typed request handling
- **Response** - Fluent response building

## WHY: Clean Web Layer

**Problems Solved**:
1. **Routing Complexity** - Declarative routes
2. **Cross-cutting** - Middleware pipeline
3. **Type Safety** - Typed handlers

**When to Use**: REST APIs, web applications

## HOW: Usage Guide

```java
var router = Router.create()
    .get("/users", this::listUsers)
    .get("/users/{id}", this::getUser)
    .post("/users", this::createUser)
    .middleware(loggingMiddleware)
    .middleware(authMiddleware);

Response getUser(Request request) {
    Long id = request.pathParam("id", Long.class);
    return userService.findById(id)
        .map(Response::json)
        .orElse(Response.notFound());
}
```

## Relationship to Other Modules

| Module | Relationship |
|--------|--------------|
| jboot-security | Auth middleware |
| jboot-validation | Request validation |
| jboot-observability | Request tracing |

---

**Status**: Stable
