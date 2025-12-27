# Database Migrations

The `rustboot-database` migration framework provides a comprehensive system for managing database schema changes over time. This guide covers all aspects of using migrations in your Rustboot applications.

## Table of Contents

1. [Overview](#overview)
2. [Core Concepts](#core-concepts)
3. [Getting Started](#getting-started)
4. [Migration Types](#migration-types)
5. [Migration Runner](#migration-runner)
6. [File-Based Migrations](#file-based-migrations)
7. [Programmatic Migrations](#programmatic-migrations)
8. [Best Practices](#best-practices)
9. [API Reference](#api-reference)

## Overview

The migration framework provides:

- **Version Tracking**: Automatic tracking of applied migrations
- **Bidirectional Migrations**: Both up (apply) and down (rollback) support
- **SQL File Support**: Load migrations from `.sql` files
- **Programmatic Migrations**: Implement custom migration logic in Rust
- **Status Checking**: Query the state of migrations
- **Checksum Validation**: Detect changes to applied migrations
- **Rollback Support**: Rollback individual or multiple migrations

## Core Concepts

### Version

Migrations are identified by unique versions. Versions are typically timestamp-based strings:

```rust
use dev_engineeringlabs_rustboot_database::Version;

// Manual version
let version = Version::new("20231201120000");

// Auto-generated timestamp version
let version = Version::timestamp();

// Parse and validate
let version = Version::parse("v1_initial")?;
```

### Migration Trait

All migrations implement the `Migration` trait:

```rust
#[async_trait]
pub trait Migration: Send + Sync {
    fn version(&self) -> &Version;
    fn description(&self) -> &str;
    async fn up(&self, db: &dyn Database) -> MigrationResult<()>;
    async fn down(&self, db: &dyn Database) -> MigrationResult<()>;
    fn checksum(&self) -> String;
}
```

### Migration Status

Migrations can be in one of two states:

- `Pending`: Not yet applied
- `Applied`: Successfully applied to the database

## Getting Started

### 1. Initialize the Migration System

```rust
use dev_engineeringlabs_rustboot_database::MigrationRunner;

let db = /* your database implementation */;
let runner = MigrationRunner::new(Box::new(db));

// Initialize migrations table
runner.init().await?;
```

### 2. Create Migrations

#### SQL-based Migration

```rust
use dev_engineeringlabs_rustboot_database::{SqlMigration, Version};

let migration = SqlMigration::new(
    Version::new("20231201120000"),
    "create users table",
    // Up SQL
    "CREATE TABLE users (
        id INTEGER PRIMARY KEY,
        username TEXT NOT NULL,
        email TEXT UNIQUE
    );",
    // Down SQL
    "DROP TABLE users;"
);
```

### 3. Run Migrations

```rust
let migrations = vec![migration];

// Apply all pending migrations
let executed = runner.migrate(&migrations).await?;
println!("Applied {} migrations", executed.len());
```

## Migration Types

### SQL Migrations

SQL migrations are the most common type:

```rust
let migration = SqlMigration::new(
    Version::new("001"),
    "create products table",
    r#"
    CREATE TABLE products (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        price DECIMAL(10,2)
    );
    CREATE INDEX idx_products_name ON products(name);
    "#,
    r#"
    DROP INDEX idx_products_name;
    DROP TABLE products;
    "#
);
```

**Features:**
- Multiple SQL statements (separated by `;`)
- Automatic statement execution
- Checksum generation

### Custom Migrations

Implement the `Migration` trait for complex logic:

```rust
use async_trait::async_trait;
use dev_engineeringlabs_rustboot_database::{Migration, MigrationError, Version};

struct CustomMigration {
    version: Version,
}

#[async_trait]
impl Migration for CustomMigration {
    fn version(&self) -> &Version {
        &self.version
    }

    fn description(&self) -> &str {
        "Custom data transformation"
    }

    async fn up(&self, db: &dyn Database) -> Result<(), MigrationError> {
        // Create table
        db.execute("CREATE TABLE users (id INTEGER, data TEXT)").await
            .map_err(|e| MigrationError::Database(e.to_string()))?;

        // Load and transform data
        let old_data = db.query("SELECT * FROM legacy_users").await
            .map_err(|e| MigrationError::Database(e.to_string()))?;

        for row in old_data {
            // Transform and insert
            // ... custom logic ...
        }

        Ok(())
    }

    async fn down(&self, db: &dyn Database) -> Result<(), MigrationError> {
        db.execute("DROP TABLE users").await
            .map_err(|e| MigrationError::Database(e.to_string()))?;
        Ok(())
    }

    fn checksum(&self) -> String {
        "custom_migration_v1".to_string()
    }
}
```

## Migration Runner

The `MigrationRunner` manages migration execution:

### Basic Operations

```rust
use dev_engineeringlabs_rustboot_database::MigrationRunner;

let runner = MigrationRunner::new(Box::new(db));

// Initialize migrations table
runner.init().await?;

// Run all pending migrations
let executed = runner.migrate(&migrations).await?;

// Check status of all migrations
let status = runner.get_status(&migrations).await?;
for record in status {
    println!("{}: {} - {:?}",
        record.version,
        record.description,
        record.status
    );
}
```

### Targeted Migrations

```rust
// Migrate up to a specific version
runner.migrate_to(&migrations, &Version::new("20231201130000")).await?;
```

### Rollbacks

```rust
// Rollback the last migration
runner.rollback(&migrations, 1).await?;

// Rollback last N migrations
runner.rollback(&migrations, 3).await?;

// Rollback to a specific version
runner.rollback_to(&migrations, &Version::new("20231201120000")).await?;
```

### Custom Table Name

```rust
let runner = MigrationRunner::with_table_name(
    Box::new(db),
    "my_custom_migrations_table"
);
```

## File-Based Migrations

### Directory Structure

```
migrations/
├── 20231201120000__create_users_table.sql
├── 20231201130000__create_posts_table.sql
└── 20231201140000__add_indexes.sql
```

### File Format

Each `.sql` file should have the format: `{version}__{description}.sql`

```sql
-- Migration: create users table
-- Created: 2023-12-01

-- migrate:up
CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_email ON users(email);

-- migrate:down
DROP INDEX idx_users_email;
DROP INDEX idx_users_username;
DROP TABLE users;
```

### Loading Migrations

```rust
use dev_engineeringlabs_rustboot_database::MigrationLoader;

let loader = MigrationLoader::new("./migrations");

// Load all migrations from directory
let migrations = loader.load_migrations().await?;

// Run loaded migrations
let runner = MigrationRunner::new(Box::new(db));
runner.init().await?;
runner.migrate(&migrations).await?;
```

### Creating New Migration Files

```rust
let loader = MigrationLoader::new("./migrations");

// Create a new migration file with template
let (version, path) = loader.create_migration("add user roles").await?;

println!("Created migration {} at {:?}", version, path);
// Outputs: Created migration 20231201153045 at "migrations/20231201153045__add_user_roles.sql"
```

This generates a template file you can edit:

```sql
-- Migration: add user roles
-- Created: 2023-12-01 15:30:45

-- migrate:up
-- Add your up migration SQL here


-- migrate:down
-- Add your down migration SQL here

```

## Programmatic Migrations

### Building a Migration System

```rust
use dev_engineeringlabs_rustboot_database::{
    MigrationRunner, SqlMigration, Version
};

pub struct AppMigrations;

impl AppMigrations {
    pub fn all() -> Vec<SqlMigration> {
        vec![
            Self::create_users_table(),
            Self::create_posts_table(),
            Self::add_user_roles(),
        ]
    }

    fn create_users_table() -> SqlMigration {
        SqlMigration::new(
            Version::new("20231201120000"),
            "create users table",
            include_str!("../sql/001_users_up.sql"),
            include_str!("../sql/001_users_down.sql"),
        )
    }

    fn create_posts_table() -> SqlMigration {
        SqlMigration::new(
            Version::new("20231201130000"),
            "create posts table",
            include_str!("../sql/002_posts_up.sql"),
            include_str!("../sql/002_posts_down.sql"),
        )
    }

    fn add_user_roles() -> SqlMigration {
        SqlMigration::new(
            Version::new("20231201140000"),
            "add user roles",
            r#"
            CREATE TABLE roles (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE
            );

            ALTER TABLE users ADD COLUMN role_id INTEGER REFERENCES roles(id);
            "#,
            r#"
            ALTER TABLE users DROP COLUMN role_id;
            DROP TABLE roles;
            "#,
        )
    }
}

// Usage
pub async fn run_migrations(db: Box<dyn Database>) -> Result<(), MigrationError> {
    let runner = MigrationRunner::new(db);
    runner.init().await?;

    let migrations = AppMigrations::all();
    runner.migrate(&migrations).await?;

    Ok(())
}
```

## Best Practices

### 1. Version Naming

Use timestamp-based versions for chronological ordering:

```rust
// Good: Timestamp-based (YYYYMMDDHHmmss)
Version::new("20231201120000")
Version::timestamp()

// Acceptable: Sequential with zero-padding
Version::new("001")
Version::new("002")

// Avoid: Non-sortable versions
Version::new("v1")
Version::new("initial")
```

### 2. Migration Atomicity

Keep migrations focused on a single logical change:

```rust
// Good: Single concern
"create users table"
"add email index to users"
"add posts table"

// Avoid: Multiple concerns
"create users and posts tables and add all indexes"
```

### 3. Reversibility

Always provide down migrations:

```sql
-- migrate:up
CREATE TABLE users (id INTEGER PRIMARY KEY);
ALTER TABLE users ADD COLUMN email TEXT;

-- migrate:down
ALTER TABLE users DROP COLUMN email;
DROP TABLE users;
```

### 4. Testing Migrations

Test both up and down migrations:

```rust
#[tokio::test]
async fn test_migration_up_and_down() {
    let db = TestDatabase::new();
    let migration = SqlMigration::new(/* ... */);

    // Test up
    migration.up(&db).await.unwrap();
    assert!(db.table_exists("users"));

    // Test down
    migration.down(&db).await.unwrap();
    assert!(!db.table_exists("users"));
}
```

### 5. Data Migrations

For data migrations, handle existing data carefully:

```rust
async fn up(&self, db: &dyn Database) -> MigrationResult<()> {
    // 1. Create new column
    db.execute("ALTER TABLE users ADD COLUMN status TEXT").await?;

    // 2. Set default value for existing rows
    db.execute("UPDATE users SET status = 'active' WHERE status IS NULL").await?;

    // 3. Make column non-nullable (if needed)
    db.execute("ALTER TABLE users ALTER COLUMN status SET NOT NULL").await?;

    Ok(())
}
```

### 6. Checksum Validation

The framework automatically validates checksums:

```rust
// If you modify an applied migration, you'll get an error:
Error: ChecksumMismatch(
    "20231201120000",
    "abc123",  // Original checksum
    "def456"   // New checksum
)
```

**Solution**: Create a new migration instead of modifying existing ones.

### 7. Production Rollbacks

Be cautious with rollbacks in production:

```rust
// Safe: Rollback just-applied migration
runner.rollback(&migrations, 1).await?;

// Risky: Rolling back old migrations may lose data
runner.rollback(&migrations, 10).await?;
```

## API Reference

### Version

```rust
impl Version {
    pub fn new(version: impl Into<String>) -> Self
    pub fn as_str(&self) -> &str
    pub fn parse(s: &str) -> MigrationResult<Self>
    pub fn timestamp() -> Self
}
```

### SqlMigration

```rust
impl SqlMigration {
    pub fn new(
        version: Version,
        description: impl Into<String>,
        up_sql: impl Into<String>,
        down_sql: impl Into<String>,
    ) -> Self

    pub fn up_sql(&self) -> &str
    pub fn down_sql(&self) -> &str
}
```

### MigrationRunner

```rust
impl MigrationRunner {
    pub fn new(db: Box<dyn Database>) -> Self
    pub fn with_table_name(db: Box<dyn Database>, table_name: impl Into<String>) -> Self

    pub async fn init(&self) -> MigrationResult<()>
    pub async fn migrate(&self, migrations: &[SqlMigration]) -> MigrationResult<Vec<Version>>
    pub async fn migrate_to(&self, migrations: &[SqlMigration], target: &Version) -> MigrationResult<Vec<Version>>
    pub async fn rollback(&self, migrations: &[SqlMigration], count: usize) -> MigrationResult<Vec<Version>>
    pub async fn rollback_to(&self, migrations: &[SqlMigration], target: &Version) -> MigrationResult<Vec<Version>>
    pub async fn get_status(&self, migrations: &[SqlMigration]) -> MigrationResult<Vec<MigrationRecord>>
    pub async fn get_applied_migrations(&self) -> MigrationResult<HashMap<Version, MigrationRecord>>
}
```

### MigrationLoader

```rust
impl MigrationLoader {
    pub fn new(migrations_dir: impl Into<PathBuf>) -> Self

    pub async fn load_migrations(&self) -> MigrationResult<Vec<SqlMigration>>
    pub async fn create_migration(&self, description: &str) -> MigrationResult<(Version, PathBuf)>
}
```

### MigrationRecord

```rust
pub struct MigrationRecord {
    pub version: Version,
    pub description: String,
    pub status: MigrationStatus,
    pub checksum: String,
    pub applied_at: Option<String>,
}
```

### MigrationError

```rust
pub enum MigrationError {
    Database(String),
    NotFound(String),
    AlreadyApplied(String),
    NotApplied(String),
    InvalidVersion(String),
    Io(std::io::Error),
    Parse(String),
    Validation(String),
    ChecksumMismatch(String, String, String),
}
```

## Examples

See the `examples/` directory for complete working examples:

- `migration_basic.rs` - Basic programmatic migrations
- `migration_file_based.rs` - File-based migrations
- `migration_custom.rs` - Custom migration implementations

Run examples with:

```bash
cargo run --example migration_basic
cargo run --example migration_file_based
cargo run --example migration_custom
```
