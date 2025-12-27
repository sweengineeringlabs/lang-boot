# HTTP Module Overview

## WHAT: HTTP Client

Type-safe HTTP client with middleware, retries, and response handling.

Key capabilities:
- **Fluent API** - Readable request building
- **Middleware** - Request/response interceptors
- **Serialization** - Automatic JSON handling
- **Retry** - Configurable retry policies

## WHY: Clean HTTP Calls

**Problems Solved**:
1. **Boilerplate** - Fluent request building
2. **Error Handling** - Typed response handling
3. **Observability** - Request logging/tracing

**When to Use**: Calling external APIs

## HOW: Usage Guide

```java
var client = HttpClient.builder()
    .baseUrl("https://api.example.com")
    .timeout(Duration.ofSeconds(30))
    .header("Authorization", "Bearer " + token)
    .build();

HttpResponse<User> response = client.get("/users/123", User.class);

if (response.isSuccess()) {
    User user = response.getBody();
}
```

## Relationship to Other Modules

| Module | Relationship |
|--------|--------------|
| jboot-resilience | Circuit breakers |
| jboot-observability | Request tracing |

---

**Status**: Stable
