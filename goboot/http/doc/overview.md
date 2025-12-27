# HTTP Module Overview

## WHAT: HTTP Client

Type-safe HTTP client with middleware and retries.

Key capabilities:
- **Fluent API** - Readable requests
- **Middleware** - Request interceptors
- **Retry** - Configurable retries
- **Timeout** - Request timeouts

## WHY: Clean HTTP Calls

**Problems Solved**: Boilerplate, error handling

**When to Use**: Calling external APIs

## HOW: Usage Guide

```go
client := http.NewClient(http.Config{
    BaseURL: "https://api.example.com",
    Timeout: 30 * time.Second,
})

var user User
resp, err := client.Get("/users/123").
    Header("Authorization", "Bearer "+token).
    Do(&user)
```

---

**Status**: Stable
