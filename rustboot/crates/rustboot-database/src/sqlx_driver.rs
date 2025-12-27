//! SQLx database driver implementation.
//!
//! This module provides concrete implementations of the Database and Transaction
//! traits using SQLx. It supports PostgreSQL, MySQL, and SQLite backends through
//! feature flags.
//!
//! # Features
//!
//! - `sqlx-postgres`: Enable PostgreSQL support
//! - `sqlx-mysql`: Enable MySQL support
//! - `sqlx-sqlite`: Enable SQLite support
//!
//! # Example
//!
//! ```ignore
//! use rustboot_database::{Database, sqlx_driver::SqlxDatabase};
//!
//! let db = SqlxDatabase::connect_sqlite("sqlite::memory:").await?;
//! let rows = db.query("SELECT * FROM users").await?;
//! ```

use crate::traits::{Database, DatabaseError, DatabaseResult, Row, Transaction, Value};
use async_trait::async_trait;
use sqlx::Column;
use std::sync::Arc;
use tokio::sync::Mutex;

// Conditional type aliases based on feature flags
#[cfg(feature = "sqlx-sqlite")]
pub type DbPool = sqlx::SqlitePool;
#[cfg(feature = "sqlx-sqlite")]
pub type DbRow = sqlx::sqlite::SqliteRow;
#[cfg(feature = "sqlx-sqlite")]
pub type DbTransaction = sqlx::Transaction<'static, sqlx::Sqlite>;

#[cfg(all(feature = "sqlx-postgres", not(feature = "sqlx-sqlite")))]
pub type DbPool = sqlx::PgPool;
#[cfg(all(feature = "sqlx-postgres", not(feature = "sqlx-sqlite")))]
pub type DbRow = sqlx::postgres::PgRow;
#[cfg(all(feature = "sqlx-postgres", not(feature = "sqlx-sqlite")))]
pub type DbTransaction = sqlx::Transaction<'static, sqlx::Postgres>;

#[cfg(all(
    feature = "sqlx-mysql",
    not(feature = "sqlx-sqlite"),
    not(feature = "sqlx-postgres")
))]
pub type DbPool = sqlx::MySqlPool;
#[cfg(all(
    feature = "sqlx-mysql",
    not(feature = "sqlx-sqlite"),
    not(feature = "sqlx-postgres")
))]
pub type DbRow = sqlx::mysql::MySqlRow;
#[cfg(all(
    feature = "sqlx-mysql",
    not(feature = "sqlx-sqlite"),
    not(feature = "sqlx-postgres")
))]
pub type DbTransaction = sqlx::Transaction<'static, sqlx::MySql>;

/// SQLx database connection wrapper.
///
/// This struct wraps an SQLx connection pool and implements the Database trait.
/// It provides a unified interface across different database backends (PostgreSQL,
/// MySQL, SQLite).
#[derive(Clone)]
pub struct SqlxDatabase {
    pool: Arc<DbPool>,
}

impl SqlxDatabase {
    /// Create a new SqlxDatabase from a connection pool.
    ///
    /// # Arguments
    ///
    /// * `pool` - An SQLx connection pool
    ///
    /// # Example
    ///
    /// ```ignore
    /// use sqlx::SqlitePool;
    /// use rustboot_database::sqlx_driver::SqlxDatabase;
    ///
    /// let pool = SqlitePool::connect("sqlite::memory:").await?;
    /// let db = SqlxDatabase::new(pool);
    /// ```
    pub fn new(pool: DbPool) -> Self {
        Self {
            pool: Arc::new(pool),
        }
    }

    /// Connect to a database using a connection string.
    ///
    /// The connection string format depends on the database backend:
    /// - PostgreSQL: `postgres://user:pass@localhost/dbname`
    /// - MySQL: `mysql://user:pass@localhost/dbname`
    /// - SQLite: `sqlite:file.db` or `sqlite::memory:`
    ///
    /// # Arguments
    ///
    /// * `url` - Database connection URL
    ///
    /// # Errors
    ///
    /// Returns a `DatabaseError::Connection` if the connection fails.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use rustboot_database::sqlx_driver::SqlxDatabase;
    ///
    /// let db = SqlxDatabase::connect("sqlite::memory:").await?;
    /// ```
    pub async fn connect(url: &str) -> DatabaseResult<Self> {
        let pool = DbPool::connect(url)
            .await
            .map_err(|e| DatabaseError::Connection(e.to_string()))?;
        Ok(Self::new(pool))
    }

    /// Connect to a PostgreSQL database.
    ///
    /// This is a convenience method that requires the `sqlx-postgres` feature.
    ///
    /// # Arguments
    ///
    /// * `url` - PostgreSQL connection URL
    ///
    /// # Errors
    ///
    /// Returns a `DatabaseError::Connection` if the connection fails.
    #[cfg(feature = "sqlx-postgres")]
    pub async fn connect_postgres(url: &str) -> DatabaseResult<Self> {
        Self::connect(url).await
    }

    /// Connect to a MySQL database.
    ///
    /// This is a convenience method that requires the `sqlx-mysql` feature.
    ///
    /// # Arguments
    ///
    /// * `url` - MySQL connection URL
    ///
    /// # Errors
    ///
    /// Returns a `DatabaseError::Connection` if the connection fails.
    #[cfg(feature = "sqlx-mysql")]
    pub async fn connect_mysql(url: &str) -> DatabaseResult<Self> {
        Self::connect(url).await
    }

    /// Connect to a SQLite database.
    ///
    /// This is a convenience method that requires the `sqlx-sqlite` feature.
    ///
    /// # Arguments
    ///
    /// * `url` - SQLite connection URL (e.g., "sqlite::memory:" or "sqlite:file.db")
    ///
    /// # Errors
    ///
    /// Returns a `DatabaseError::Connection` if the connection fails.
    #[cfg(feature = "sqlx-sqlite")]
    pub async fn connect_sqlite(url: &str) -> DatabaseResult<Self> {
        Self::connect(url).await
    }

    /// Get the underlying SQLx pool.
    ///
    /// This method provides access to the raw SQLx pool for advanced usage.
    pub fn pool(&self) -> &DbPool {
        &self.pool
    }

    /// Close the database connection pool.
    ///
    /// This will close all connections in the pool and prevent new connections
    /// from being acquired.
    pub async fn close(&self) {
        self.pool.close().await;
    }

    /// Convert SQLx row to our Row type.
    fn convert_row(sqlx_row: DbRow) -> DatabaseResult<Row> {
        use sqlx::Row as SqlxRow;

        let mut row = Row::new();
        let columns = sqlx_row.columns();

        for column in columns {
            let name = column.name();
            let value = Self::extract_value(&sqlx_row, name)?;
            row.set(name, value);
        }

        Ok(row)
    }

    /// Extract a value from an SQLx row.
    fn extract_value(sqlx_row: &DbRow, column: &str) -> DatabaseResult<Value> {
        use sqlx::{Column, Row as SqlxRow, TypeInfo, ValueRef};

        let column_info = sqlx_row
            .try_column(column)
            .map_err(|e| DatabaseError::Query(format!("Column not found: {}", e)))?;

        let value_ref = sqlx_row
            .try_get_raw(column)
            .map_err(|e| DatabaseError::Query(format!("Failed to get raw value: {}", e)))?;

        // Check if value is NULL
        if value_ref.is_null() {
            return Ok(Value::Null);
        }

        let type_info = column_info.type_info();
        let type_name = type_info.name();

        // Try to decode based on type name
        match type_name {
            "BOOL" | "BOOLEAN" => {
                let val: bool = sqlx_row
                    .try_get(column)
                    .map_err(|e| DatabaseError::Query(format!("Failed to decode bool: {}", e)))?;
                Ok(Value::Bool(val))
            }
            "INT2" | "INT4" | "INT8" | "INTEGER" | "BIGINT" | "SMALLINT" | "TINYINT" | "INT"
            | "MEDIUMINT" => {
                let val: i64 = sqlx_row
                    .try_get(column)
                    .map_err(|e| DatabaseError::Query(format!("Failed to decode int: {}", e)))?;
                Ok(Value::Int(val))
            }
            "FLOAT4" | "FLOAT8" | "REAL" | "DOUBLE" | "NUMERIC" | "DECIMAL" | "FLOAT" => {
                let val: f64 = sqlx_row
                    .try_get(column)
                    .map_err(|e| DatabaseError::Query(format!("Failed to decode float: {}", e)))?;
                Ok(Value::Float(val))
            }
            "TEXT" | "VARCHAR" | "CHAR" | "STRING" | "TINYTEXT" | "MEDIUMTEXT" | "LONGTEXT" => {
                let val: String = sqlx_row
                    .try_get(column)
                    .map_err(|e| DatabaseError::Query(format!("Failed to decode string: {}", e)))?;
                Ok(Value::String(val))
            }
            "BLOB" | "BYTEA" | "BINARY" | "VARBINARY" | "TINYBLOB" | "MEDIUMBLOB"
            | "LONGBLOB" => {
                let val: Vec<u8> = sqlx_row
                    .try_get(column)
                    .map_err(|e| DatabaseError::Query(format!("Failed to decode bytes: {}", e)))?;
                Ok(Value::Bytes(val))
            }
            _ => {
                // For unknown types, try common types in order
                // Try integer first
                if let Ok(val) = sqlx_row.try_get::<i64, _>(column) {
                    return Ok(Value::Int(val));
                }
                // Try float
                if let Ok(val) = sqlx_row.try_get::<f64, _>(column) {
                    return Ok(Value::Float(val));
                }
                // Try bool
                if let Ok(val) = sqlx_row.try_get::<bool, _>(column) {
                    return Ok(Value::Bool(val));
                }
                // Try string
                if let Ok(val) = sqlx_row.try_get::<String, _>(column) {
                    return Ok(Value::String(val));
                }
                // Try bytes as last resort
                if let Ok(val) = sqlx_row.try_get::<Vec<u8>, _>(column) {
                    return Ok(Value::Bytes(val));
                }
                // If all attempts fail, return error
                Err(DatabaseError::Query(format!(
                    "Unable to decode column '{}' with type '{}'",
                    column, type_name
                )))
            }
        }
    }
}

#[async_trait]
impl Database for SqlxDatabase {
    /// Execute a query and return the results.
    ///
    /// # Arguments
    ///
    /// * `sql` - SQL query string
    ///
    /// # Errors
    ///
    /// Returns a `DatabaseError::Query` if the query fails.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let rows = db.query("SELECT id, name FROM users WHERE active = true").await?;
    /// for row in rows {
    ///     let id = row.get("id");
    ///     let name = row.get("name");
    /// }
    /// ```
    async fn query(&self, sql: &str) -> DatabaseResult<Vec<Row>> {
        let sqlx_rows = sqlx::query(sql)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| DatabaseError::Query(e.to_string()))?;

        sqlx_rows
            .into_iter()
            .map(Self::convert_row)
            .collect::<DatabaseResult<Vec<Row>>>()
    }

    /// Execute a command (INSERT, UPDATE, DELETE) and return the number of affected rows.
    ///
    /// # Arguments
    ///
    /// * `sql` - SQL command string
    ///
    /// # Errors
    ///
    /// Returns a `DatabaseError::Query` if the command fails.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let affected = db.execute("UPDATE users SET active = false WHERE last_login < '2020-01-01'").await?;
    /// println!("Updated {} users", affected);
    /// ```
    async fn execute(&self, sql: &str) -> DatabaseResult<u64> {
        let result = sqlx::query(sql)
            .execute(&*self.pool)
            .await
            .map_err(|e| DatabaseError::Query(e.to_string()))?;

        Ok(result.rows_affected())
    }

    /// Begin a new database transaction.
    ///
    /// The transaction must be explicitly committed or rolled back.
    ///
    /// # Errors
    ///
    /// Returns a `DatabaseError::Connection` if starting the transaction fails.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let tx = db.begin_transaction().await?;
    /// tx.execute("INSERT INTO users (name) VALUES ('Alice')").await?;
    /// tx.execute("INSERT INTO users (name) VALUES ('Bob')").await?;
    /// tx.commit().await?;
    /// ```
    async fn begin_transaction(&self) -> DatabaseResult<Box<dyn Transaction>> {
        let tx = self
            .pool
            .begin()
            .await
            .map_err(|e| DatabaseError::Connection(e.to_string()))?;

        Ok(Box::new(SqlxTransaction::new(tx)))
    }
}

/// SQLx transaction wrapper.
///
/// This struct wraps an SQLx transaction and implements the Transaction trait.
/// It uses Arc<Mutex<>> to provide thread-safe interior mutability.
pub struct SqlxTransaction {
    tx: Arc<Mutex<Option<DbTransaction>>>,
}

impl SqlxTransaction {
    /// Create a new SqlxTransaction from an SQLx transaction.
    fn new(tx: DbTransaction) -> Self {
        Self {
            tx: Arc::new(Mutex::new(Some(tx))),
        }
    }
}

#[async_trait]
impl Transaction for SqlxTransaction {
    /// Execute a query within the transaction.
    ///
    /// # Arguments
    ///
    /// * `sql` - SQL query string
    ///
    /// # Errors
    ///
    /// Returns a `DatabaseError::Query` if the query fails.
    async fn query(&self, sql: &str) -> DatabaseResult<Vec<Row>> {
        let mut tx_guard = self.tx.lock().await;
        let tx = tx_guard
            .as_mut()
            .ok_or_else(|| DatabaseError::Query("Transaction already completed".to_string()))?;

        let sqlx_rows = sqlx::query(sql)
            .fetch_all(&mut **tx)
            .await
            .map_err(|e| DatabaseError::Query(e.to_string()))?;

        sqlx_rows
            .into_iter()
            .map(SqlxDatabase::convert_row)
            .collect::<DatabaseResult<Vec<Row>>>()
    }

    /// Execute a command within the transaction.
    ///
    /// # Arguments
    ///
    /// * `sql` - SQL command string
    ///
    /// # Errors
    ///
    /// Returns a `DatabaseError::Query` if the command fails.
    async fn execute(&self, sql: &str) -> DatabaseResult<u64> {
        let mut tx_guard = self.tx.lock().await;
        let tx = tx_guard
            .as_mut()
            .ok_or_else(|| DatabaseError::Query("Transaction already completed".to_string()))?;

        let result = sqlx::query(sql)
            .execute(&mut **tx)
            .await
            .map_err(|e| DatabaseError::Query(e.to_string()))?;

        Ok(result.rows_affected())
    }

    /// Commit the transaction.
    ///
    /// This consumes the transaction and persists all changes to the database.
    ///
    /// # Errors
    ///
    /// Returns a `DatabaseError::Query` if the commit fails.
    async fn commit(self: Box<Self>) -> DatabaseResult<()> {
        let mut tx_guard = self.tx.lock().await;
        let tx = tx_guard
            .take()
            .ok_or_else(|| DatabaseError::Query("Transaction already completed".to_string()))?;

        tx.commit()
            .await
            .map_err(|e| DatabaseError::Query(format!("Failed to commit transaction: {}", e)))
    }

    /// Rollback the transaction.
    ///
    /// This consumes the transaction and discards all changes.
    ///
    /// # Errors
    ///
    /// Returns a `DatabaseError::Query` if the rollback fails.
    async fn rollback(self: Box<Self>) -> DatabaseResult<()> {
        let mut tx_guard = self.tx.lock().await;
        let tx = tx_guard
            .take()
            .ok_or_else(|| DatabaseError::Query("Transaction already completed".to_string()))?;

        tx.rollback()
            .await
            .map_err(|e| DatabaseError::Query(format!("Failed to rollback transaction: {}", e)))
    }
}

/// Mutable transaction wrapper that allows query and execute operations.
///
/// This is a more ergonomic transaction type that allows mutable operations.
/// It doesn't implement the Transaction trait but provides similar functionality
/// with better ergonomics.
pub struct SqlxMutTransaction {
    tx: DbTransaction,
}

impl SqlxMutTransaction {
    /// Create a new mutable transaction.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use rustboot_database::sqlx_driver::{SqlxDatabase, SqlxMutTransaction};
    ///
    /// let db = SqlxDatabase::connect("sqlite::memory:").await?;
    /// let mut tx = SqlxMutTransaction::begin(db.pool()).await?;
    /// tx.execute("INSERT INTO users (name) VALUES ('Alice')").await?;
    /// tx.commit().await?;
    /// ```
    pub async fn begin(pool: &DbPool) -> DatabaseResult<Self> {
        let tx = pool
            .begin()
            .await
            .map_err(|e| DatabaseError::Connection(e.to_string()))?;
        Ok(Self { tx })
    }

    /// Execute a query within the transaction.
    pub async fn query(&mut self, sql: &str) -> DatabaseResult<Vec<Row>> {
        let sqlx_rows = sqlx::query(sql)
            .fetch_all(&mut *self.tx)
            .await
            .map_err(|e| DatabaseError::Query(e.to_string()))?;

        sqlx_rows
            .into_iter()
            .map(SqlxDatabase::convert_row)
            .collect::<DatabaseResult<Vec<Row>>>()
    }

    /// Execute a command within the transaction.
    pub async fn execute(&mut self, sql: &str) -> DatabaseResult<u64> {
        let result = sqlx::query(sql)
            .execute(&mut *self.tx)
            .await
            .map_err(|e| DatabaseError::Query(e.to_string()))?;

        Ok(result.rows_affected())
    }

    /// Commit the transaction.
    pub async fn commit(self) -> DatabaseResult<()> {
        self.tx
            .commit()
            .await
            .map_err(|e| DatabaseError::Query(format!("Failed to commit transaction: {}", e)))
    }

    /// Rollback the transaction.
    pub async fn rollback(self) -> DatabaseResult<()> {
        self.tx
            .rollback()
            .await
            .map_err(|e| DatabaseError::Query(format!("Failed to rollback transaction: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[cfg(feature = "sqlx-sqlite")]
    async fn test_sqlite_connection() {
        let db = SqlxDatabase::connect_sqlite("sqlite::memory:")
            .await
            .expect("Failed to connect to SQLite");

        // Create a test table
        db.execute("CREATE TABLE test (id INTEGER PRIMARY KEY, name TEXT)")
            .await
            .expect("Failed to create table");

        // Insert data
        db.execute("INSERT INTO test (id, name) VALUES (1, 'Alice')")
            .await
            .expect("Failed to insert data");

        // Query data
        let rows = db
            .query("SELECT id, name FROM test")
            .await
            .expect("Failed to query data");

        assert_eq!(rows.len(), 1);
        if let Some(Value::Int(id)) = rows[0].get("id") {
            assert_eq!(*id, 1);
        } else {
            panic!("Expected id to be an integer");
        }
    }

    #[tokio::test]
    #[cfg(feature = "sqlx-sqlite")]
    async fn test_mutable_transaction() {
        let db = SqlxDatabase::connect_sqlite("sqlite::memory:")
            .await
            .expect("Failed to connect to SQLite");

        // Create a test table
        db.execute("CREATE TABLE test (id INTEGER PRIMARY KEY, name TEXT)")
            .await
            .expect("Failed to create table");

        // Start transaction
        let mut tx = SqlxMutTransaction::begin(db.pool())
            .await
            .expect("Failed to begin transaction");

        // Insert data in transaction
        tx.execute("INSERT INTO test (id, name) VALUES (1, 'Alice')")
            .await
            .expect("Failed to insert in transaction");

        // Commit
        tx.commit().await.expect("Failed to commit transaction");

        // Verify data
        let rows = db
            .query("SELECT id, name FROM test")
            .await
            .expect("Failed to query data");

        assert_eq!(rows.len(), 1);
    }

    #[tokio::test]
    #[cfg(feature = "sqlx-sqlite")]
    async fn test_transaction_rollback() {
        let db = SqlxDatabase::connect_sqlite("sqlite::memory:")
            .await
            .expect("Failed to connect to SQLite");

        // Create a test table
        db.execute("CREATE TABLE test (id INTEGER PRIMARY KEY, name TEXT)")
            .await
            .expect("Failed to create table");

        // Start transaction
        let mut tx = SqlxMutTransaction::begin(db.pool())
            .await
            .expect("Failed to begin transaction");

        // Insert data in transaction
        tx.execute("INSERT INTO test (id, name) VALUES (1, 'Alice')")
            .await
            .expect("Failed to insert in transaction");

        // Rollback
        tx.rollback()
            .await
            .expect("Failed to rollback transaction");

        // Verify data was not inserted
        let rows = db
            .query("SELECT id, name FROM test")
            .await
            .expect("Failed to query data");

        assert_eq!(rows.len(), 0);
    }
}
