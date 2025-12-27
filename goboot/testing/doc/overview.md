# Testing Module Overview

## WHAT: Testing Utilities

Mocks, assertions, and fixtures.

Key capabilities:
- **Mocks** - Interface mocks
- **Assertions** - Rich asserts
- **Fixtures** - Test data
- **Containers** - Testcontainers

## WHY: Better Tests

**Problems Solved**: Boilerplate, readability

**When to Use**: All test code

## HOW: Usage Guide

```go
// Assertions
testing.AssertEqual(t, expected, actual)
testing.AssertNil(t, err)
testing.AssertContains(t, slice, item)

// Fixtures
user := fixtures.NewUser().
    WithEmail("test@example.com").
    Build()
```

---

**Status**: Stable
