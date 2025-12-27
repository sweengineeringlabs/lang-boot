//! Deadpool-based connection pool implementation.
//!
//! This module provides a connection pool implementation using the `deadpool` crate.

use async_trait::async_trait;
use deadpool::managed::{Manager, Object, Pool, RecycleResult};
use std::fmt;
use std::sync::Arc;
use std::time::Duration;

use crate::pool::{ConnectionPool, PoolConfig, PoolStatus, PooledConnection};
use crate::{Database, DatabaseError, DatabaseResult, Row, Transaction};

/// A connection pool implementation using deadpool.
///
/// This pool manages database connections efficiently, recycling them
/// for reuse and managing connection lifecycle.
pub struct DeadpoolConnectionPool<M>
where
    M: Manager,
{
    pool: Pool<M>,
    config: Arc<PoolConfig>,
}

impl<M> DeadpoolConnectionPool<M>
where
    M: Manager,
{
    /// Creates a new deadpool-based connection pool.
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
    /// let pool = DeadpoolConnectionPool::new(manager, config);
    /// ```
    pub fn new(manager: M, config: PoolConfig) -> Self {
        let pool = Pool::builder(manager)
            .max_size(config.max_size)
            .build()
            .expect("Failed to create pool");

        Self {
            pool,
            config: Arc::new(config),
        }
    }

    /// Creates a builder for constructing a deadpool connection pool.
    pub fn builder(manager: M) -> DeadpoolBuilder<M> {
        DeadpoolBuilder {
            manager,
            config: PoolConfig::default(),
        }
    }
}

impl<M> fmt::Debug for DeadpoolConnectionPool<M>
where
    M: Manager + Send + Sync,
    M::Type: Database + Send + Sync,
    M::Error: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DeadpoolConnectionPool")
            .field("max_size", &self.config.max_size)
            .field("status", &self.status())
            .finish()
    }
}

#[async_trait]
impl<M> ConnectionPool for DeadpoolConnectionPool<M>
where
    M: Manager + Send + Sync,
    M::Type: Database + Send + Sync,
    M::Error: fmt::Display,
{
    type Connection = DeadpoolConnection<M>;

    async fn get(&self) -> DatabaseResult<Self::Connection> {
        let timeout = self.config.connection_timeout;

        if let Some(duration) = timeout {
            self.get_timeout(duration).await
        } else {
            let conn = self.pool
                .get()
                .await
                .map_err(|e| DatabaseError::Connection(format!("Failed to get connection: {}", e)))?;
            Ok(DeadpoolConnection { inner: Some(conn) })
        }
    }

    async fn get_timeout(&self, timeout: Duration) -> DatabaseResult<Self::Connection> {
        let conn = tokio::time::timeout(timeout, self.pool.get())
            .await
            .map_err(|_| DatabaseError::Connection("Connection timeout".to_string()))?
            .map_err(|e| DatabaseError::Connection(format!("Failed to get connection: {}", e)))?;

        Ok(DeadpoolConnection { inner: Some(conn) })
    }

    fn status(&self) -> PoolStatus {
        let status = self.pool.status();
        PoolStatus {
            size: status.size,
            idle: status.available,
            active: status.size - status.available,
            max_size: status.max_size,
            waiting: status.waiting,
        }
    }

    async fn close(&self) -> DatabaseResult<()> {
        self.pool.close();
        Ok(())
    }

    fn is_closed(&self) -> bool {
        self.pool.is_closed()
    }

    fn max_size(&self) -> usize {
        self.config.max_size
    }

    fn timeout(&self) -> Option<Duration> {
        self.config.connection_timeout
    }
}

/// A pooled database connection from deadpool.
pub struct DeadpoolConnection<M>
where
    M: Manager,
{
    inner: Option<Object<M>>,
}

impl<M> fmt::Debug for DeadpoolConnection<M>
where
    M: Manager,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DeadpoolConnection")
            .field("active", &self.inner.is_some())
            .finish()
    }
}

#[async_trait]
impl<M> Database for DeadpoolConnection<M>
where
    M: Manager + Send + Sync,
    M::Type: Database + Send + Sync,
{
    async fn query(&self, sql: &str) -> DatabaseResult<Vec<Row>> {
        match &self.inner {
            Some(conn) => conn.query(sql).await,
            None => Err(DatabaseError::Connection(
                "Connection has been released".to_string(),
            )),
        }
    }

    async fn execute(&self, sql: &str) -> DatabaseResult<u64> {
        match &self.inner {
            Some(conn) => conn.execute(sql).await,
            None => Err(DatabaseError::Connection(
                "Connection has been released".to_string(),
            )),
        }
    }

    async fn begin_transaction(&self) -> DatabaseResult<Box<dyn Transaction>> {
        match &self.inner {
            Some(conn) => conn.begin_transaction().await,
            None => Err(DatabaseError::Connection(
                "Connection has been released".to_string(),
            )),
        }
    }
}

#[async_trait]
impl<M> PooledConnection for DeadpoolConnection<M>
where
    M: Manager + Send + Sync,
    M::Type: Database + Send + Sync,
{
    async fn release(mut self) -> DatabaseResult<()> {
        self.inner = None;
        Ok(())
    }
}

/// Builder for creating deadpool-based connection pools.
pub struct DeadpoolBuilder<M>
where
    M: Manager,
{
    manager: M,
    config: PoolConfig,
}

impl<M> DeadpoolBuilder<M>
where
    M: Manager,
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

    /// Builds the connection pool.
    pub fn build(self) -> DeadpoolConnectionPool<M> {
        DeadpoolConnectionPool::new(self.manager, self.config)
    }
}

/// Example manager implementation for testing and demonstration.
///
/// In real usage, you would implement your own manager for your specific database.
#[derive(Debug)]
pub struct ExampleManager {
    connection_string: String,
}

impl ExampleManager {
    /// Creates a new example manager.
    pub fn new(connection_string: String) -> Self {
        Self { connection_string }
    }
}

impl Manager for ExampleManager {
    type Type = ExampleConnection;
    type Error = DatabaseError;

    fn create(&self) -> impl std::future::Future<Output = Result<Self::Type, Self::Error>> + Send {
        let connection_string = self.connection_string.clone();
        async move {
            // In a real implementation, this would create an actual database connection
            Ok(ExampleConnection {
                _connection_string: connection_string,
            })
        }
    }

    fn recycle(
        &self,
        _conn: &mut Self::Type,
        _metrics: &deadpool::managed::Metrics,
    ) -> impl std::future::Future<Output = RecycleResult<Self::Error>> + Send {
        async move {
            // In a real implementation, this would validate the connection
            Ok(())
        }
    }
}

/// Example connection implementation for testing and demonstration.
pub struct ExampleConnection {
    _connection_string: String,
}

#[async_trait]
impl Database for ExampleConnection {
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
        let manager = ExampleManager::new("postgres://localhost/test".to_string());
        let config = PoolConfig::new("postgres://localhost/test").with_max_size(5);
        let pool = DeadpoolConnectionPool::new(manager, config);

        assert_eq!(pool.max_size(), 5);
        assert!(!pool.is_closed());
    }

    #[tokio::test]
    async fn test_get_connection() {
        let manager = ExampleManager::new("postgres://localhost/test".to_string());
        let config = PoolConfig::new("postgres://localhost/test");
        let pool = DeadpoolConnectionPool::new(manager, config);

        let conn = pool.get().await;
        assert!(conn.is_ok());
    }

    #[tokio::test]
    async fn test_pool_status() {
        let manager = ExampleManager::new("postgres://localhost/test".to_string());
        let config = PoolConfig::new("postgres://localhost/test").with_max_size(10);
        let pool = DeadpoolConnectionPool::new(manager, config);

        let status = pool.status();
        assert_eq!(status.max_size, 10);
    }

    #[tokio::test]
    async fn test_builder_pattern() {
        let manager = ExampleManager::new("postgres://localhost/test".to_string());
        let pool = DeadpoolConnectionPool::builder(manager)
            .max_size(15)
            .timeout(Duration::from_secs(30))
            .build();

        assert_eq!(pool.max_size(), 15);
    }
}
