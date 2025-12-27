//! Pool abstraction traits.
//!
//! Generic traits for connection pooling that work across different implementations.

use async_trait::async_trait;
use std::fmt::Debug;
use std::time::Duration;

use crate::{Database, DatabaseResult};

/// Trait representing a pooled database connection.
///
/// This trait allows connections to be used generically regardless of the
/// underlying pool implementation.
#[async_trait]
pub trait PooledConnection: Database {
    /// Returns the connection back to the pool.
    ///
    /// This is typically called automatically when the connection is dropped,
    /// but can be called explicitly if needed.
    async fn release(self) -> DatabaseResult<()>;
}

/// Trait for database connection pool implementations.
///
/// This trait provides a generic interface for connection pools,
/// allowing different implementations (deadpool, bb8, etc.) to be used interchangeably.
#[async_trait]
pub trait ConnectionPool: Send + Sync + Debug {
    /// The type of connection this pool manages.
    type Connection: PooledConnection;

    /// Gets a connection from the pool.
    ///
    /// This will wait for a connection to become available if the pool is exhausted,
    /// up to the configured timeout.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The timeout is exceeded while waiting for a connection
    /// - The pool is closed
    /// - The connection fails validation (if configured)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let connection = pool.get().await?;
    /// let rows = connection.query("SELECT * FROM users").await?;
    /// ```
    async fn get(&self) -> DatabaseResult<Self::Connection>;

    /// Gets a connection with a custom timeout.
    ///
    /// # Arguments
    ///
    /// * `timeout` - Maximum time to wait for a connection
    ///
    /// # Errors
    ///
    /// Returns an error if the timeout is exceeded or connection acquisition fails.
    async fn get_timeout(&self, timeout: Duration) -> DatabaseResult<Self::Connection>;

    /// Returns the current pool status.
    ///
    /// This provides information about the pool's current state including
    /// active connections, idle connections, and pool size.
    fn status(&self) -> PoolStatus;

    /// Closes the pool and releases all connections.
    ///
    /// After calling this, no new connections can be acquired.
    /// Existing connections will be closed when they are returned.
    async fn close(&self) -> DatabaseResult<()>;

    /// Returns true if the pool is closed.
    fn is_closed(&self) -> bool;

    /// Returns the maximum size of the pool.
    fn max_size(&self) -> usize;

    /// Returns the configured timeout for acquiring connections.
    fn timeout(&self) -> Option<Duration>;
}

/// Status information about a connection pool.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PoolStatus {
    /// Current number of connections in the pool (both idle and active).
    pub size: usize,

    /// Number of idle connections available for use.
    pub idle: usize,

    /// Number of connections currently in use.
    pub active: usize,

    /// Maximum number of connections the pool can hold.
    pub max_size: usize,

    /// Number of connections waiting to be created.
    pub waiting: usize,
}

impl PoolStatus {
    /// Creates a new pool status.
    pub fn new(size: usize, idle: usize, max_size: usize) -> Self {
        Self {
            size,
            idle,
            active: size.saturating_sub(idle),
            max_size,
            waiting: 0,
        }
    }

    /// Returns true if the pool is at maximum capacity.
    pub fn is_full(&self) -> bool {
        self.size >= self.max_size
    }

    /// Returns true if there are idle connections available.
    pub fn has_idle(&self) -> bool {
        self.idle > 0
    }

    /// Returns the utilization rate as a percentage (0.0 to 1.0).
    pub fn utilization(&self) -> f64 {
        if self.max_size == 0 {
            0.0
        } else {
            self.active as f64 / self.max_size as f64
        }
    }
}

/// Builder trait for constructing connection pools.
///
/// This trait allows pools to be constructed in a generic way.
#[async_trait]
pub trait PoolBuilder: Send + Sync {
    /// The type of pool this builder creates.
    type Pool: ConnectionPool;

    /// Builds the connection pool.
    ///
    /// # Errors
    ///
    /// Returns an error if pool initialization fails.
    async fn build(self) -> DatabaseResult<Self::Pool>;
}

/// Extension trait for Database to provide pool-aware functionality.
#[async_trait]
pub trait PooledDatabase: Database {
    /// Returns a reference to the underlying connection pool.
    fn pool(&self) -> &dyn ConnectionPool<Connection = Self>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_status() {
        let status = PoolStatus::new(5, 3, 10);
        assert_eq!(status.size, 5);
        assert_eq!(status.idle, 3);
        assert_eq!(status.active, 2);
        assert_eq!(status.max_size, 10);
        assert!(!status.is_full());
        assert!(status.has_idle());
    }

    #[test]
    fn test_pool_status_utilization() {
        let status = PoolStatus::new(8, 2, 10);
        assert_eq!(status.active, 6);
        assert_eq!(status.utilization(), 0.6);
    }

    #[test]
    fn test_pool_status_full() {
        let status = PoolStatus::new(10, 0, 10);
        assert!(status.is_full());
        assert!(!status.has_idle());
        assert_eq!(status.utilization(), 1.0);
    }
}
