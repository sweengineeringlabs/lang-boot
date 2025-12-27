# SQLx Database Driver

This document describes the SQLx database driver implementation for the `rustboot-database` crate.

## Overview

The SQLx driver provides concrete implementations of the `Database` and `Transaction` traits using the popular [SQLx](https://github.com/launchbadge/sqlx) library. It supports multiple database backends through compile-time feature flags.

## Features

Enable the SQLx driver by adding one or more of these features to your `Cargo.toml`:

- `sqlx-sqlite`: SQLite database support
- `sqlx-postgres`: PostgreSQL database support
- `sqlx-mysql`: MySQL database support
- `sqlx-all`: All database backends

```toml
[dependencies]
dev-engineeringlabs-rustboot-database = { version = "0.1.0", features = ["sqlx-sqlite"] }
```

## Architecture

### Type System

The driver uses conditional compilation to provide a unified API across different database backends:

```rust
// Conditional type aliases based on feature flags
#[cfg(feature = "sqlx-sqlite")]
pub type DbPool = sqlx::SqlitePool;
pub type DbRow = sqlx::sqlite::SqliteRow;
pub type DbTransaction = sqlx::Transaction<'static, sqlx::Sqlite>;

#[cfg(feature = "sqlx-postgres")]
pub type DbPool = sqlx::PgPool;
// ... etc
```

This approach provides:
- **Type safety**: Compile-time verification of database operations
- **Zero overhead**: No runtime polymorphism or virtual dispatch
- **Feature parity**: Consistent API regardless of backend

### Core Components

1. **SqlxDatabase**: Main database connection wrapper
   - Wraps an SQLx connection pool (`Arc<DbPool>`)
   - Implements the `Database` trait
   - Provides connection management and query execution

2. **SqlxTransaction**: Thread-safe transaction wrapper
   - Uses `Arc<Mutex<Option<DbTransaction>>>` for interior mutability
   - Implements the `Transaction` trait
   - Supports commit and rollback operations

3. **SqlxMutTransaction**: Ergonomic transaction wrapper
   - Mutable transaction type for more ergonomic usage
   - Doesn't implement `Transaction` trait but provides similar API
   - Recommended for most use cases

## Usage Examples

### Basic Connection and Queries

```rust
use dev_engineeringlabs_rustboot_database::{Database, SqlxDatabase, Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to SQLite
    let db = SqlxDatabase::connect_sqlite("sqlite::memory:").await?;

    // Create a table
    db.execute("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, age INTEGER)")
        .await?;

    // Insert data
    db.execute("INSERT INTO users (name, age) VALUES ('Alice', 30)")
        .await?;

    // Query data
    let rows = db.query("SELECT * FROM users").await?;

    for row in rows {
        let name = match row.get("name") {
            Some(Value::String(v)) => v,
            _ => "unknown",
        };
        println!("User: {}", name);
    }

    Ok(())
}
```

### Transactions with Commit

```rust
use dev_engineeringlabs_rustboot_database::{SqlxDatabase, SqlxMutTransaction};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = SqlxDatabase::connect_sqlite("sqlite::memory:").await?;

    // Create table
    db.execute("CREATE TABLE accounts (id INTEGER PRIMARY KEY, balance REAL)")
        .await?;

    // Start a transaction
    let mut tx = SqlxMutTransaction::begin(db.pool()).await?;

    // Execute multiple operations
    tx.execute("INSERT INTO accounts (id, balance) VALUES (1, 100.0)").await?;
    tx.execute("INSERT INTO accounts (id, balance) VALUES (2, 200.0)").await?;
    tx.execute("UPDATE accounts SET balance = balance - 50.0 WHERE id = 1").await?;
    tx.execute("UPDATE accounts SET balance = balance + 50.0 WHERE id = 2").await?;

    // Commit the transaction
    tx.commit().await?;

    Ok(())
}
```

### Transactions with Rollback

```rust
use dev_engineeringlabs_rustboot_database::{SqlxDatabase, SqlxMutTransaction};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = SqlxDatabase::connect_sqlite("sqlite::memory:").await?;

    db.execute("CREATE TABLE logs (id INTEGER PRIMARY KEY, message TEXT)")
        .await?;

    let mut tx = SqlxMutTransaction::begin(db.pool()).await?;

    tx.execute("INSERT INTO logs (message) VALUES ('Operation started')")
        .await?;

    // Something went wrong, rollback
    if some_error_condition {
        tx.rollback().await?;
        println!("Transaction rolled back");
    } else {
        tx.commit().await?;
    }

    Ok(())
}
```

### Using the Transaction Trait

```rust
use dev_engineeringlabs_rustboot_database::{Database, SqlxDatabase};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = SqlxDatabase::connect_sqlite("sqlite::memory:").await?;

    db.execute("CREATE TABLE items (id INTEGER PRIMARY KEY, name TEXT)")
        .await?;

    // Begin transaction via Database trait
    let tx = db.begin_transaction().await?;

    // Execute operations
    tx.execute("INSERT INTO items (name) VALUES ('Item 1')").await?;
    tx.execute("INSERT INTO items (name) VALUES ('Item 2')").await?;

    // Commit (consumes the transaction)
    tx.commit().await?;

    Ok(())
}
```

### PostgreSQL Example

```rust
use dev_engineeringlabs_rustboot_database::{Database, SqlxDatabase};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = SqlxDatabase::connect_postgres(
        "postgres://user:password@localhost/mydb"
    ).await?;

    db.execute(
        "CREATE TABLE IF NOT EXISTS products (
            id SERIAL PRIMARY KEY,
            name VARCHAR(255),
            price DECIMAL(10, 2)
        )"
    ).await?;

    db.execute(
        "INSERT INTO products (name, price) VALUES ('Laptop', 999.99)"
    ).await?;

    let rows = db.query("SELECT * FROM products").await?;
    println!("Found {} products", rows.len());

    Ok(())
}
```

### MySQL Example

```rust
use dev_engineeringlabs_rustboot_database::{Database, SqlxDatabase};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = SqlxDatabase::connect_mysql(
        "mysql://user:password@localhost/mydb"
    ).await?;

    db.execute(
        "CREATE TABLE IF NOT EXISTS orders (
            id INT AUTO_INCREMENT PRIMARY KEY,
            customer VARCHAR(255),
            total DECIMAL(10, 2)
        )"
    ).await?;

    db.execute(
        "INSERT INTO orders (customer, total) VALUES ('John Doe', 149.99)"
    ).await?;

    Ok(())
}
```

## Type Mapping

The driver automatically converts between SQL types and the `Value` enum:

| SQL Type | Value Variant |
|----------|---------------|
| BOOLEAN, BOOL | `Value::Bool(bool)` |
| INTEGER, INT, BIGINT, SMALLINT, TINYINT | `Value::Int(i64)` |
| REAL, DOUBLE, FLOAT, NUMERIC, DECIMAL | `Value::Float(f64)` |
| TEXT, VARCHAR, CHAR, STRING | `Value::String(String)` |
| BLOB, BYTEA, BINARY, VARBINARY | `Value::Bytes(Vec<u8>)` |
| NULL | `Value::Null` |

For unknown types, the driver attempts to decode in the following order:
1. i64 (integer)
2. f64 (float)
3. bool
4. String
5. Vec<u8> (bytes)

## Error Handling

All operations return `DatabaseResult<T>`, which is an alias for `Result<T, DatabaseError>`.

Error types:
- `DatabaseError::Connection`: Connection-related errors
- `DatabaseError::Query`: Query execution errors
- `DatabaseError::NotFound`: Record not found
- `DatabaseError::Serialization`: Serialization errors

Example error handling:

```rust
match db.query("SELECT * FROM users").await {
    Ok(rows) => {
        println!("Found {} rows", rows.len());
    }
    Err(DatabaseError::Query(msg)) => {
        eprintln!("Query failed: {}", msg);
    }
    Err(DatabaseError::Connection(msg)) => {
        eprintln!("Connection failed: {}", msg);
    }
    Err(e) => {
        eprintln!("Other error: {}", e);
    }
}
```

## Connection Pool Management

The driver uses SQLx's built-in connection pooling:

```rust
let db = SqlxDatabase::connect("sqlite::memory:").await?;

// Get access to the underlying pool for advanced configuration
let pool = db.pool();

// Close the connection pool when done
db.close().await;
```

## Thread Safety

All components are thread-safe and can be safely shared across threads:

- `SqlxDatabase` is `Clone + Send + Sync`
- `SqlxTransaction` uses `Arc<Mutex<>>` for interior mutability
- `SqlxMutTransaction` is `Send` (not `Sync` due to mutable operations)

Example multi-threaded usage:

```rust
use std::sync::Arc;
use tokio::task;

let db = Arc::new(SqlxDatabase::connect_sqlite("sqlite::memory:").await?);

let handles: Vec<_> = (0..10).map(|i| {
    let db = Arc::clone(&db);
    task::spawn(async move {
        db.execute(&format!("INSERT INTO data (value) VALUES ({})", i))
            .await
    })
}).collect();

for handle in handles {
    handle.await??;
}
```

## Testing

The driver includes comprehensive unit tests:

```bash
# Run tests for SQLite
cargo test --features sqlx-sqlite

# Run tests for PostgreSQL (requires running PostgreSQL)
cargo test --features sqlx-postgres

# Run tests for MySQL (requires running MySQL)
cargo test --features sqlx-mysql
```

## Examples

See the `examples/` directory for complete working examples:

- `sqlx_sqlite.rs`: Comprehensive SQLite example
- `sqlx_postgres.rs`: PostgreSQL example with advanced features
- `sqlx_mysql.rs`: MySQL example with transactions

Run examples:

```bash
# SQLite
cargo run --features sqlx-sqlite --example sqlx_sqlite

# PostgreSQL (set DATABASE_URL)
export DATABASE_URL="postgres://user:pass@localhost/db"
cargo run --features sqlx-postgres --example sqlx_postgres

# MySQL (set DATABASE_URL)
export DATABASE_URL="mysql://user:pass@localhost/db"
cargo run --features sqlx-mysql --example sqlx_mysql
```

## Performance Considerations

1. **Connection Pooling**: SQLx manages connections efficiently. Reuse the same `SqlxDatabase` instance.

2. **Prepared Statements**: For repeated queries, consider using SQLx's prepared statements directly through `db.pool()`.

3. **Batch Operations**: Use transactions for multiple related operations to reduce round trips.

4. **Type Conversions**: The generic `Value` enum adds overhead. For performance-critical code, use SQLx directly.

## Limitations

1. **Single Backend**: Only one database backend can be active at compile time per feature flag.

2. **Type Mapping**: Complex types (JSON, arrays, custom types) may require manual handling.

3. **Migration Support**: Database migrations are not included. Use SQLx's migration tools or a dedicated migration library.

## Future Enhancements

- [ ] Support for prepared statements through the Database trait
- [ ] Batch insert/update operations
- [ ] Query builder integration
- [ ] Custom type mapping configuration
- [ ] Connection pool configuration API
- [ ] Streaming query results for large datasets

## Contributing

Contributions are welcome! Please ensure:

1. All tests pass
2. New features include tests
3. Documentation is updated
4. Code follows existing patterns

## License

MIT
