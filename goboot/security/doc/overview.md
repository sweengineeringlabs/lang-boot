# Security Module Overview

## WHAT: Authentication & Authorization

Complete security stack.

Key capabilities:
- **Auth** - Credential validation
- **AuthZ** - Role/permission-based
- **JWT** - Token handling
- **Secrets** - Secret management

## WHY: Secure Applications

**Problems Solved**: Auth complexity, token management

**When to Use**: Apps with authentication

## HOW: Usage Guide

```go
// JWT
jwt := security.NewJWT(secretKey)
token, _ := jwt.Generate(claims)
parsed, _ := jwt.Verify(token)

// Authorization
authz := security.NewAuthorizer()
if authz.HasPermission(user, "admin:write") {
    // allowed
}
```

---

**Status**: Stable
