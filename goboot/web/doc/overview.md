# Web Module Overview

## WHAT: Web Framework Utilities

Router, middleware, and request handling.

Key capabilities:
- **Router** - Path-based routing
- **Middleware** - Request pipeline
- **Request** - Typed handling
- **Response** - Response builders

## WHY: Clean Web Layer

**Problems Solved**: Routing, cross-cutting

**When to Use**: REST APIs, web apps

## HOW: Usage Guide

```go
router := web.NewRouter()

router.Get("/users", listUsers)
router.Get("/users/{id}", getUser)
router.Post("/users", createUser)

router.Use(web.Logger())
router.Use(web.Recovery())
router.Use(web.CORS())

http.ListenAndServe(":8080", router)
```

---

**Status**: Stable
