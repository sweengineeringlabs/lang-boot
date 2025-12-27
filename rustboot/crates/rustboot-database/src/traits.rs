//! Database abstraction (L4: Core - Database).
//!
//! Generic database trait for CRUD operations.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Database errors.
#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    /// Connection error.
    #[error("Connection error: {0}")]
    Connection(String),
    
    /// Query error.
    #[error("Query error: {0}")]
    Query(String),
    
    /// Not found error.
    #[error("Record not found")]
    NotFound,
    
    /// Serialization error.
    #[error("Serialization error: {0}")]
    Serialization(String),
}

/// Result type for database operations.
pub type DatabaseResult<T> = Result<T, DatabaseError>;

/// Database connection trait.
#[async_trait]
pub trait Database: Send + Sync {
    /// Execute a query.
    async fn query(&self, sql: &str) -> DatabaseResult<Vec<Row>>;
    
    /// Execute a command (INSERT, UPDATE, DELETE).
    async fn execute(&self, sql: &str) -> DatabaseResult<u64>;
    
    /// Begin a transaction.
    async fn begin_transaction(&self) -> DatabaseResult<Box<dyn Transaction>>;
}

/// Database transaction trait.
#[async_trait]
pub trait Transaction: Send + Sync {
    /// Execute a query within the transaction.
    async fn query(&self, sql: &str) -> DatabaseResult<Vec<Row>>;
    
    /// Execute a command within the transaction.
    async fn execute(&self, sql: &str) -> DatabaseResult<u64>;
    
    /// Commit the transaction.
    async fn commit(self: Box<Self>) -> DatabaseResult<()>;
    
    /// Rollback the transaction.
    async fn rollback(self: Box<Self>) -> DatabaseResult<()>;
}

/// Database row.
#[derive(Debug, Clone)]
pub struct Row {
    columns: std::collections::HashMap<String, Value>,
}

impl Row {
    /// Create a new row.
    pub fn new() -> Self {
        Self {
            columns: std::collections::HashMap::new(),
        }
    }
    
    /// Get a value by column name.
    pub fn get(&self, column: &str) -> Option<&Value> {
        self.columns.get(column)
    }
    
    /// Set a value.
    pub fn set(&mut self, column: impl Into<String>, value: Value) {
        self.columns.insert(column.into(), value);
    }
}

impl Default for Row {
    fn default() -> Self {
        Self::new()
    }
}

/// Database value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    /// Null value.
    Null,
    /// Boolean value.
    Bool(bool),
    /// Integer value.
    Int(i64),
    /// Float value.
    Float(f64),
    /// String value.
    String(String),
    /// Binary data.
    Bytes(Vec<u8>),
}

/// Repository trait for CRUD operations.
#[async_trait]
pub trait Repository<T>: Send + Sync
where
    T: Send + Sync,
{
    /// Find by ID.
    async fn find_by_id(&self, id: &str) -> DatabaseResult<Option<T>>;
    
    /// Find all records.
    async fn find_all(&self) -> DatabaseResult<Vec<T>>;
    
    /// Find records with pagination.
    async fn find_paginated(&self, page: usize, page_size: usize) -> DatabaseResult<Vec<T>>;
    
    /// Count total records.
    async fn count(&self) -> DatabaseResult<usize>;
    
    /// Create a new record.
    async fn create(&self, entity: &T) -> DatabaseResult<String>;
    
    /// Update an existing record.
    async fn update(&self, id: &str, entity: &T) -> DatabaseResult<()>;
    
    /// Save (insert or update) - convenience method.
    async fn save(&self, entity: &T) -> DatabaseResult<()>;
    
    /// Delete by ID.
    async fn delete(&self, id: &str) -> DatabaseResult<()>;
    
    /// Delete multiple records by IDs.
    async fn delete_many(&self, ids: &[&str]) -> DatabaseResult<usize>;
    
    /// Save multiple records.
    async fn save_many(&self, entities: &[T]) -> DatabaseResult<()>;
}
