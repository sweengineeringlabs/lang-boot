//! Rustboot Database - Database abstraction with connection pooling
//!
//! This crate provides generic database abstractions including:
//! - Core database traits (`Database`, `Transaction`, `Repository`)
//! - Connection pooling with multiple implementations (deadpool, bb8)
//! - Migration support for database schema management
//! - Flexible configuration and lifecycle management
//!
//! # Features
//!
//! - `pool-deadpool` (default): Deadpool-based connection pooling
//! - `pool-bb8`: BB8-based connection pooling
//! - `sqlx`: SQLx database driver support
//!
//! # Examples
//!
//! ## Basic Database Operations
//!
//! ```ignore
//! use dev_engineeringlabs_rustboot_database::{Database, DatabaseResult};
//!
//! async fn query_users(db: &impl Database) -> DatabaseResult<()> {
//!     let rows = db.query("SELECT * FROM users").await?;
//!     Ok(())
//! }
//! ```
//!
//! ## Connection Pooling
//!
//! ```ignore
//! use dev_engineeringlabs_rustboot_database::pool::prelude::*;
//!
//! let config = PoolConfig::new("postgres://localhost/db")
//!     .with_max_size(20);
//!
//! let pool = DeadpoolConnectionPool::new(manager, config);
//! let conn = pool.get().await?;
//! ```

pub mod migration;
pub mod pool;
pub mod traits;

#[cfg(feature = "sqlx")]
pub mod sqlx_driver;

pub use migration::{
    Direction, Migration, MigrationError, MigrationLoader, MigrationRecord, MigrationResult,
    MigrationRunner, MigrationStatus, SqlMigration, Version,
};
pub use traits::{Database, DatabaseError, DatabaseResult, Repository, Row, Transaction, Value};

#[cfg(feature = "sqlx")]
pub use sqlx_driver::{SqlxDatabase, SqlxMutTransaction, SqlxTransaction};
