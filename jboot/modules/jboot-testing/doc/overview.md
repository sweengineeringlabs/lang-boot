# Testing Module Overview

## WHAT: Testing Utilities

Mocks, assertions, fixtures, and test helpers.

Key capabilities:
- **Mocks** - Easy mock creation
- **Assertions** - Rich assertion library
- **Fixtures** - Test data builders
- **Containers** - Testcontainers integration

## WHY: Better Tests

**Problems Solved**:
1. **Boilerplate** - Pre-built test utilities
2. **Readability** - Fluent assertions
3. **Integration** - Container-based tests

**When to Use**: All test code

## HOW: Usage Guide

```java
// Assertions
assertThat(result)
    .isNotNull()
    .hasSize(3)
    .contains("expected");

// Mocks
var mockRepo = mock(UserRepository.class);
when(mockRepo.findById(1L)).thenReturn(user);

// Fixtures
var user = UserFixture.builder()
    .withEmail("test@example.com")
    .build();
```

## Relationship to Other Modules

| Module | Relationship |
|--------|--------------|
| All modules | Test support |

---

**Status**: Stable
