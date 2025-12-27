# Database Module Overview

## WHAT: Database Abstractions

Repository patterns, transactions, and query builders.

Key capabilities:
- **Repository** - CRUD interfaces
- **Transactions** - Context-based transactions
- **Query Builder** - Type-safe queries
- **Connection Pool** - Efficient pooling

## WHY: Clean Data Access

**Problems Solved**: SQL injection, boilerplate, transaction leaks

**When to Use**: Database-backed applications

## HOW: Usage Guide

```go
type UserRepository interface {
    database.Repository[User, int64]
    FindByEmail(ctx context.Context, email string) (*User, error)
}

// Transaction
err := database.WithTransaction(ctx, db, func(tx *sql.Tx) error {
    // operations in transaction
    return nil
})
```

---

**Status**: Stable
