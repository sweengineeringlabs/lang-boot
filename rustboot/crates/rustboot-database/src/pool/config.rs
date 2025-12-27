//! Pool configuration module.
//!
//! Provides configuration options for database connection pools.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Configuration for database connection pools.
///
/// This struct provides common configuration options that work across
/// different pool implementations (deadpool, bb8, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolConfig {
    /// Maximum number of connections in the pool.
    ///
    /// Default: 10
    pub max_size: usize,

    /// Minimum number of idle connections to maintain.
    ///
    /// Default: 1
    pub min_idle: Option<usize>,

    /// Maximum time to wait for a connection from the pool.
    ///
    /// If None, will wait indefinitely.
    /// Default: 30 seconds
    pub connection_timeout: Option<Duration>,

    /// Maximum lifetime of a connection before it's recycled.
    ///
    /// If None, connections will never be recycled based on age.
    /// Default: 30 minutes
    pub max_lifetime: Option<Duration>,

    /// Maximum idle time before a connection is closed.
    ///
    /// If None, idle connections won't be closed.
    /// Default: 10 minutes
    pub idle_timeout: Option<Duration>,

    /// Test connections when acquired from the pool.
    ///
    /// When true, connections will be validated before being returned.
    /// Default: false
    pub test_on_acquire: bool,

    /// Test connections when returned to the pool.
    ///
    /// When true, connections will be validated before being returned to the pool.
    /// Default: false
    pub test_on_release: bool,

    /// Custom connection string or database URL.
    ///
    /// The format depends on the database driver being used.
    pub connection_string: String,
}

impl PoolConfig {
    /// Creates a new pool configuration with the given connection string.
    ///
    /// # Arguments
    ///
    /// * `connection_string` - The database connection string
    ///
    /// # Examples
    ///
    /// ```
    /// use dev_engineeringlabs_rustboot_database::pool::PoolConfig;
    ///
    /// let config = PoolConfig::new("postgres://user:pass@localhost/db");
    /// assert_eq!(config.max_size, 10);
    /// ```
    pub fn new(connection_string: impl Into<String>) -> Self {
        Self {
            max_size: 10,
            min_idle: Some(1),
            connection_timeout: Some(Duration::from_secs(30)),
            max_lifetime: Some(Duration::from_secs(30 * 60)),
            idle_timeout: Some(Duration::from_secs(10 * 60)),
            test_on_acquire: false,
            test_on_release: false,
            connection_string: connection_string.into(),
        }
    }

    /// Sets the maximum pool size.
    ///
    /// # Examples
    ///
    /// ```
    /// use dev_engineeringlabs_rustboot_database::pool::PoolConfig;
    ///
    /// let config = PoolConfig::new("postgres://localhost/db")
    ///     .with_max_size(20);
    /// assert_eq!(config.max_size, 20);
    /// ```
    pub fn with_max_size(mut self, max_size: usize) -> Self {
        self.max_size = max_size;
        self
    }

    /// Sets the minimum idle connections.
    ///
    /// # Examples
    ///
    /// ```
    /// use dev_engineeringlabs_rustboot_database::pool::PoolConfig;
    ///
    /// let config = PoolConfig::new("postgres://localhost/db")
    ///     .with_min_idle(5);
    /// assert_eq!(config.min_idle, Some(5));
    /// ```
    pub fn with_min_idle(mut self, min_idle: usize) -> Self {
        self.min_idle = Some(min_idle);
        self
    }

    /// Sets the connection timeout.
    ///
    /// # Examples
    ///
    /// ```
    /// use dev_engineeringlabs_rustboot_database::pool::PoolConfig;
    /// use std::time::Duration;
    ///
    /// let config = PoolConfig::new("postgres://localhost/db")
    ///     .with_connection_timeout(Duration::from_secs(60));
    /// assert_eq!(config.connection_timeout, Some(Duration::from_secs(60)));
    /// ```
    pub fn with_connection_timeout(mut self, timeout: Duration) -> Self {
        self.connection_timeout = Some(timeout);
        self
    }

    /// Sets the maximum lifetime for connections.
    ///
    /// # Examples
    ///
    /// ```
    /// use dev_engineeringlabs_rustboot_database::pool::PoolConfig;
    /// use std::time::Duration;
    ///
    /// let config = PoolConfig::new("postgres://localhost/db")
    ///     .with_max_lifetime(Duration::from_secs(3600));
    /// assert_eq!(config.max_lifetime, Some(Duration::from_secs(3600)));
    /// ```
    pub fn with_max_lifetime(mut self, lifetime: Duration) -> Self {
        self.max_lifetime = Some(lifetime);
        self
    }

    /// Sets the idle timeout for connections.
    ///
    /// # Examples
    ///
    /// ```
    /// use dev_engineeringlabs_rustboot_database::pool::PoolConfig;
    /// use std::time::Duration;
    ///
    /// let config = PoolConfig::new("postgres://localhost/db")
    ///     .with_idle_timeout(Duration::from_secs(300));
    /// assert_eq!(config.idle_timeout, Some(Duration::from_secs(300)));
    /// ```
    pub fn with_idle_timeout(mut self, timeout: Duration) -> Self {
        self.idle_timeout = Some(timeout);
        self
    }

    /// Enables connection testing on acquire.
    ///
    /// # Examples
    ///
    /// ```
    /// use dev_engineeringlabs_rustboot_database::pool::PoolConfig;
    ///
    /// let config = PoolConfig::new("postgres://localhost/db")
    ///     .with_test_on_acquire(true);
    /// assert_eq!(config.test_on_acquire, true);
    /// ```
    pub fn with_test_on_acquire(mut self, test: bool) -> Self {
        self.test_on_acquire = test;
        self
    }

    /// Enables connection testing on release.
    ///
    /// # Examples
    ///
    /// ```
    /// use dev_engineeringlabs_rustboot_database::pool::PoolConfig;
    ///
    /// let config = PoolConfig::new("postgres://localhost/db")
    ///     .with_test_on_release(true);
    /// assert_eq!(config.test_on_release, true);
    /// ```
    pub fn with_test_on_release(mut self, test: bool) -> Self {
        self.test_on_release = test;
        self
    }
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self::new("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = PoolConfig::new("postgres://localhost/test");
        assert_eq!(config.max_size, 10);
        assert_eq!(config.min_idle, Some(1));
        assert_eq!(config.connection_timeout, Some(Duration::from_secs(30)));
        assert_eq!(config.test_on_acquire, false);
        assert_eq!(config.test_on_release, false);
    }

    #[test]
    fn test_builder_pattern() {
        let config = PoolConfig::new("postgres://localhost/test")
            .with_max_size(20)
            .with_min_idle(5)
            .with_connection_timeout(Duration::from_secs(60))
            .with_test_on_acquire(true);

        assert_eq!(config.max_size, 20);
        assert_eq!(config.min_idle, Some(5));
        assert_eq!(config.connection_timeout, Some(Duration::from_secs(60)));
        assert_eq!(config.test_on_acquire, true);
    }
}
