# Errors Module Overview

## WHAT: Error Types and Result Monad

Go-idiomatic error handling with Result types and structured errors.

Key capabilities:
- **Result Type** - Generic Result[T] monad
- **Error Wrapping** - Context-rich errors
- **Error Codes** - Machine-readable codes
- **Stack Traces** - Debug information

## WHY: Better Error Handling

**Problems Solved**: Lost context, generic errors

**When to Use**: All Go code

## HOW: Usage Guide

```go
// Result type
func FindUser(id int64) errors.Result[User] {
    user, err := db.Find(id)
    if err != nil {
        return errors.Err[User](err)
    }
    return errors.Ok(user)
}

result := FindUser(123)
result.Map(func(u User) string { return u.Name })
      .OrElse("Unknown")
```

---

**Status**: Stable
