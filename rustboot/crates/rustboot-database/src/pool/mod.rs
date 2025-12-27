//! Database connection pooling abstraction.
//!
//! This module provides a generic connection pooling abstraction that works
//! across different pool implementations (deadpool, bb8, etc.).
//!
//! # Features
//!
//! - **Generic Pool Trait**: `ConnectionPool` trait for implementation-agnostic code
//! - **Multiple Implementations**: Support for deadpool (default) and bb8 (optional)
//! - **Configuration**: Rich configuration options via `PoolConfig`
//! - **Pool Statistics**: Real-time pool status and metrics
//!
//! # Examples
//!
//! ## Using deadpool (default feature)
//!
//! ```ignore
//! use dev_engineeringlabs_rustboot_database::pool::{PoolConfig, DeadpoolConnectionPool};
//!
//! let config = PoolConfig::new("postgres://localhost/mydb")
//!     .with_max_size(20)
//!     .with_min_idle(5);
//!
//! let pool = DeadpoolConnectionPool::new(manager, config);
//! let conn = pool.get().await?;
//! let rows = conn.query("SELECT * FROM users").await?;
//! ```
//!
//! ## Using bb8 (requires `pool-bb8` feature)
//!
//! ```ignore
//! use dev_engineeringlabs_rustboot_database::pool::{PoolConfig, Bb8ConnectionPool};
//!
//! let config = PoolConfig::new("postgres://localhost/mydb")
//!     .with_max_size(20)
//!     .with_idle_timeout(Duration::from_secs(300));
//!
//! let pool = Bb8ConnectionPool::new(manager, config).await?;
//! let conn = pool.get().await?;
//! ```

pub mod config;
pub mod traits;

// Export core types
pub use config::PoolConfig;
pub use traits::{ConnectionPool, PoolBuilder, PoolStatus, PooledConnection, PooledDatabase};

// Deadpool implementation (default feature)
#[cfg(feature = "pool-deadpool")]
pub mod deadpool_impl;

#[cfg(feature = "pool-deadpool")]
pub use deadpool_impl::{DeadpoolBuilder, DeadpoolConnection, DeadpoolConnectionPool, ExampleManager};

// BB8 implementation (optional feature)
#[cfg(feature = "pool-bb8")]
pub mod bb8_impl;

#[cfg(feature = "pool-bb8")]
pub use bb8_impl::{Bb8Builder, Bb8Connection, Bb8ConnectionPool, ExampleBb8Manager};

/// Prelude module for convenient imports.
///
/// # Examples
///
/// ```
/// use dev_engineeringlabs_rustboot_database::pool::prelude::*;
/// ```
pub mod prelude {
    pub use super::config::PoolConfig;
    pub use super::traits::{ConnectionPool, PoolStatus, PooledConnection};

    #[cfg(feature = "pool-deadpool")]
    pub use super::deadpool_impl::{DeadpoolConnection, DeadpoolConnectionPool};

    #[cfg(feature = "pool-bb8")]
    pub use super::bb8_impl::{Bb8Connection, Bb8ConnectionPool};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_config_defaults() {
        let config = PoolConfig::new("postgres://localhost/test");
        assert_eq!(config.max_size, 10);
        assert_eq!(config.min_idle, Some(1));
    }

    #[test]
    fn test_pool_status() {
        let status = PoolStatus::new(5, 3, 10);
        assert_eq!(status.active, 2);
        assert!(!status.is_full());
        assert!(status.has_idle());
    }
}
