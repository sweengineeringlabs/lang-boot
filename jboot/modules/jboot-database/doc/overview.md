# Database Module Overview

## WHAT: Database Abstractions

Repository patterns, transaction management, and query builders for database operations.

Key capabilities:
- **Repository Pattern** - CRUD abstractions
- **Transactions** - Declarative transaction management
- **Query Builder** - Type-safe query construction
- **Connection Pooling** - Efficient connection management

## WHY: Clean Data Access

**Problems Solved**:
1. **SQL Injection** - Parameterized queries
2. **Boilerplate** - Generated CRUD operations
3. **Transaction Leaks** - Automatic cleanup

**When to Use**: Any database-backed application

## HOW: Usage Guide

```java
public interface UserRepository extends Repository<User, Long> {
    List<User> findByEmail(String email);
}

@Transactional
public void createUser(User user) {
    userRepository.save(user);
}
```

## Relationship to Other Modules

| Module | Relationship |
|--------|--------------|
| jboot-config | Database configuration |
| jboot-observability | Query metrics |

---

**Status**: Stable
