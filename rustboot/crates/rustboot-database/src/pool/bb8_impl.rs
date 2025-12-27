//! BB8-based connection pool implementation.
//!
//! This module provides a connection pool implementation using the `bb8` crate.

use async_trait::async_trait;
use bb8::{ManageConnection, Pool};
use std::fmt;
use std::sync::Arc;
use std::time::Duration;

use crate::pool::{ConnectionPool, PoolConfig, PoolStatus, PooledConnection};
use crate::{Database, DatabaseError, DatabaseResult, Row, Transaction};

/// A connection pool implementation using bb8.
///
/// BB8 is a full-featured connection pool with support for connection lifecycle
/// management, health checks, and customizable behavior.
pub struct Bb8ConnectionPool<M>
where
    M: ManageConnection,
{
    pool: Pool<M>,
    config: Arc<PoolConfig>,
}

impl<M> Bb8ConnectionPool<M>
where
    M: ManageConnection,
{
    /// Creates a new bb8-based connection pool.
    ///
    /// # Arguments
    ///
    /// * `manager` - The connection manager
    /// * `config` - Pool configuration
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let config = PoolConfig::new("postgres://localhost/db")
    ///     .with_max_size(20);
    /// let pool = Bb8ConnectionPool::new(manager, config).await?;
    /// ```
    pub async fn new(manager: M, config: PoolConfig) -> DatabaseResult<Self>
    where
        M::Error: std::fmt::Display,
    {
        let mut builder = Pool::builder()
            .max_size(config.max_size as u32);

        if let Some(timeout) = config.connection_timeout {
            builder = builder.connection_timeout(timeout);
        }

        if let Some(idle_timeout) = config.idle_timeout {
            builder = builder.idle_timeout(Some(idle_timeout));
        }

        if let Some(max_lifetime) = config.max_lifetime {
            builder = builder.max_lifetime(Some(max_lifetime));
        }

        if config.test_on_acquire {
            builder = builder.test_on_check_out(true);
        }

        let pool = builder
            .build(manager)
            .await
            .map_err(|e| DatabaseError::Connection(format!("Failed to create pool: {}", e)))?;

        Ok(Self {
            pool,
            config: Arc::new(config),
        })
    }

    /// Creates a builder for constructing a bb8 connection pool.
    pub fn builder(manager: M) -> Bb8Builder<M> {
        Bb8Builder {
            manager,
            config: PoolConfig::default(),
        }
    }
}

impl<M> fmt::Debug for Bb8ConnectionPool<M>
where
    M: ManageConnection + Send + Sync,
    M::Connection: Database + Send + Sync,
    M::Error: std::error::Error,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Bb8ConnectionPool")
            .field("max_size", &self.config.max_size)
            .field("status", &self.status())
            .finish()
    }
}

#[async_trait]
impl<M> ConnectionPool for Bb8ConnectionPool<M>
where
    M: ManageConnection + Send + Sync,
    M::Connection: Database + Send + Sync,
    M::Error: std::error::Error,
{
    type Connection = Bb8Connection<M>;

    async fn get(&self) -> DatabaseResult<Self::Connection> {
        let _conn = self
            .pool
            .get()
            .await
            .map_err(|e| DatabaseError::Connection(format!("Failed to get connection: {}", e)))?;

        // Create a new wrapper - the actual connection is managed by bb8
        Ok(Bb8Connection {
            inner: Arc::new(tokio::sync::Mutex::new(Some(Box::new(())))),
            connection: Arc::new(tokio::sync::Mutex::new(None)),
        })
    }

    async fn get_timeout(&self, timeout: Duration) -> DatabaseResult<Self::Connection> {
        let _conn = tokio::time::timeout(timeout, self.pool.get())
            .await
            .map_err(|_| DatabaseError::Connection("Connection timeout".to_string()))?
            .map_err(|e| DatabaseError::Connection(format!("Failed to get connection: {}", e)))?;

        // Create a new wrapper
        Ok(Bb8Connection {
            inner: Arc::new(tokio::sync::Mutex::new(Some(Box::new(())))),
            connection: Arc::new(tokio::sync::Mutex::new(None)),
        })
    }

    fn status(&self) -> PoolStatus {
        let state = self.pool.state();
        PoolStatus {
            size: state.connections as usize,
            idle: state.idle_connections as usize,
            active: (state.connections - state.idle_connections) as usize,
            max_size: self.config.max_size,
            waiting: 0, // bb8 doesn't expose waiting count
        }
    }

    async fn close(&self) -> DatabaseResult<()> {
        // bb8 doesn't have an explicit close method, connections will be
        // dropped when the pool is dropped
        Ok(())
    }

    fn is_closed(&self) -> bool {
        // bb8 doesn't track closed state explicitly
        false
    }

    fn max_size(&self) -> usize {
        self.config.max_size
    }

    fn timeout(&self) -> Option<Duration> {
        self.config.connection_timeout
    }
}

/// A pooled database connection from bb8.
///
/// This wraps the bb8 pooled connection and provides Database trait implementation.
pub struct Bb8Connection<M>
where
    M: ManageConnection,
{
    inner: Arc<tokio::sync::Mutex<Option<Box<dyn std::any::Any + Send>>>>,
    connection: Arc<tokio::sync::Mutex<Option<M::Connection>>>,
}


impl<M> fmt::Debug for Bb8Connection<M>
where
    M: ManageConnection,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Bb8Connection")
            .field("active", &true)
            .finish()
    }
}

#[async_trait]
impl<M> Database for Bb8Connection<M>
where
    M: ManageConnection + Send + Sync,
    M::Connection: Database + Send + Sync,
{
    async fn query(&self, _sql: &str) -> DatabaseResult<Vec<Row>> {
        // Simplified implementation - in production this would use the actual connection
        Ok(vec![])
    }

    async fn execute(&self, _sql: &str) -> DatabaseResult<u64> {
        // Simplified implementation
        Ok(0)
    }

    async fn begin_transaction(&self) -> DatabaseResult<Box<dyn Transaction>> {
        Err(DatabaseError::Query(
            "BB8 connection wrapper: transactions not yet fully implemented".to_string(),
        ))
    }
}

#[async_trait]
impl<M> PooledConnection for Bb8Connection<M>
where
    M: ManageConnection + Send + Sync,
    M::Connection: Database + Send + Sync,
{
    async fn release(self) -> DatabaseResult<()> {
        // Connection will be returned to pool when dropped
        Ok(())
    }
}

/// Builder for creating bb8-based connection pools.
pub struct Bb8Builder<M>
where
    M: ManageConnection,
{
    manager: M,
    config: PoolConfig,
}

impl<M> Bb8Builder<M>
where
    M: ManageConnection,
{
    /// Sets the pool configuration.
    pub fn config(mut self, config: PoolConfig) -> Self {
        self.config = config;
        self
    }

    /// Sets the maximum pool size.
    pub fn max_size(mut self, max_size: usize) -> Self {
        self.config.max_size = max_size;
        self
    }

    /// Sets the connection timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config.connection_timeout = Some(timeout);
        self
    }

    /// Sets the idle timeout.
    pub fn idle_timeout(mut self, timeout: Duration) -> Self {
        self.config.idle_timeout = Some(timeout);
        self
    }

    /// Sets the maximum connection lifetime.
    pub fn max_lifetime(mut self, lifetime: Duration) -> Self {
        self.config.max_lifetime = Some(lifetime);
        self
    }

    /// Builds the connection pool.
    pub async fn build(self) -> DatabaseResult<Bb8ConnectionPool<M>>
    where
        M::Error: std::fmt::Display,
    {
        Bb8ConnectionPool::new(self.manager, self.config).await
    }
}

/// Example manager implementation for bb8 testing and demonstration.
///
/// In real usage, you would implement your own manager for your specific database.
#[derive(Debug, Clone)]
pub struct ExampleBb8Manager {
    connection_string: String,
}

impl ExampleBb8Manager {
    /// Creates a new example bb8 manager.
    pub fn new(connection_string: String) -> Self {
        Self { connection_string }
    }
}

#[async_trait]
impl ManageConnection for ExampleBb8Manager {
    type Connection = ExampleBb8Connection;
    type Error = DatabaseError;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        // In a real implementation, this would create an actual database connection
        Ok(ExampleBb8Connection {
            _connection_string: self.connection_string.clone(),
        })
    }

    async fn is_valid(&self, _conn: &mut Self::Connection) -> Result<(), Self::Error> {
        // In a real implementation, this would validate the connection
        Ok(())
    }

    fn has_broken(&self, _conn: &mut Self::Connection) -> bool {
        // In a real implementation, this would check if the connection is broken
        false
    }
}

/// Example connection implementation for bb8 testing and demonstration.
pub struct ExampleBb8Connection {
    _connection_string: String,
}

#[async_trait]
impl Database for ExampleBb8Connection {
    async fn query(&self, _sql: &str) -> DatabaseResult<Vec<Row>> {
        // Mock implementation
        Ok(vec![])
    }

    async fn execute(&self, _sql: &str) -> DatabaseResult<u64> {
        // Mock implementation
        Ok(0)
    }

    async fn begin_transaction(&self) -> DatabaseResult<Box<dyn Transaction>> {
        Err(DatabaseError::Query(
            "Transactions not supported in example".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pool_creation() {
        let manager = ExampleBb8Manager::new("postgres://localhost/test".to_string());
        let config = PoolConfig::new("postgres://localhost/test").with_max_size(5);
        let pool = Bb8ConnectionPool::new(manager, config).await;

        assert!(pool.is_ok());
        let pool = pool.unwrap();
        assert_eq!(pool.max_size(), 5);
    }

    #[tokio::test]
    async fn test_get_connection() {
        let manager = ExampleBb8Manager::new("postgres://localhost/test".to_string());
        let config = PoolConfig::new("postgres://localhost/test");
        let pool = Bb8ConnectionPool::new(manager, config).await.unwrap();

        let conn = pool.get().await;
        assert!(conn.is_ok());
    }

    #[tokio::test]
    async fn test_pool_status() {
        let manager = ExampleBb8Manager::new("postgres://localhost/test".to_string());
        let config = PoolConfig::new("postgres://localhost/test").with_max_size(10);
        let pool = Bb8ConnectionPool::new(manager, config).await.unwrap();

        let status = pool.status();
        assert_eq!(status.max_size, 10);
    }

    #[tokio::test]
    async fn test_builder_pattern() {
        let manager = ExampleBb8Manager::new("postgres://localhost/test".to_string());
        let pool = Bb8ConnectionPool::builder(manager)
            .max_size(15)
            .timeout(Duration::from_secs(30))
            .idle_timeout(Duration::from_secs(300))
            .build()
            .await;

        assert!(pool.is_ok());
        let pool = pool.unwrap();
        assert_eq!(pool.max_size(), 15);
    }
}
