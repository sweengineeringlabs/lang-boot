//! Integration tests for rustboot-database

use dev_engineeringlabs_rustboot_database::pool::prelude::*;
use dev_engineeringlabs_rustboot_database::Database;

#[cfg(feature = "pool-deadpool")]
mod deadpool_tests {
    use super::*;
    use dev_engineeringlabs_rustboot_database::pool::{DeadpoolConnectionPool, ExampleManager};
    use std::sync::Arc;
    use std::time::Duration;

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

        let conn = conn.unwrap();
        let result = conn.query("SELECT 1").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_multiple_connections() {
        let manager = ExampleManager::new("postgres://localhost/test".to_string());
        let config = PoolConfig::new("postgres://localhost/test").with_max_size(10);
        let pool = DeadpoolConnectionPool::new(manager, config);

        let mut connections = Vec::new();
        for _ in 0..5 {
            let conn = pool.get().await;
            assert!(conn.is_ok());
            connections.push(conn.unwrap());
        }

        let status = pool.status();
        assert_eq!(status.active, 5);
        assert!(status.active <= status.max_size);
    }

    #[tokio::test]
    async fn test_pool_status() {
        let manager = ExampleManager::new("postgres://localhost/test".to_string());
        let config = PoolConfig::new("postgres://localhost/test").with_max_size(10);
        let pool = DeadpoolConnectionPool::new(manager, config);

        let status = pool.status();
        assert_eq!(status.max_size, 10);
        assert!(status.size <= status.max_size);
    }

    #[tokio::test]
    async fn test_connection_timeout() {
        let manager = ExampleManager::new("postgres://localhost/test".to_string());
        let config = PoolConfig::new("postgres://localhost/test")
            .with_connection_timeout(Duration::from_secs(5));
        let pool = DeadpoolConnectionPool::new(manager, config);

        let conn = pool.get_timeout(Duration::from_secs(1)).await;
        assert!(conn.is_ok());
    }

    #[tokio::test]
    async fn test_concurrent_access() {
        let manager = ExampleManager::new("postgres://localhost/test".to_string());
        let config = PoolConfig::new("postgres://localhost/test").with_max_size(20);
        let pool = Arc::new(DeadpoolConnectionPool::new(manager, config));

        let mut handles = vec![];
        for _ in 0..10 {
            let pool_clone = Arc::clone(&pool);
            let handle = tokio::spawn(async move {
                let conn = pool_clone.get().await.unwrap();
                let _result = conn.query("SELECT 1").await.unwrap();
            });
            handles.push(handle);
        }

        for handle in handles {
            assert!(handle.await.is_ok());
        }
    }

    #[tokio::test]
    async fn test_pool_builder() {
        let manager = ExampleManager::new("postgres://localhost/test".to_string());
        let pool = DeadpoolConnectionPool::builder(manager)
            .max_size(15)
            .timeout(Duration::from_secs(30))
            .build();

        assert_eq!(pool.max_size(), 15);
        assert_eq!(pool.timeout(), Some(Duration::from_secs(30)));

        let conn = pool.get().await;
        assert!(conn.is_ok());
    }

    #[tokio::test]
    async fn test_pool_close() {
        let manager = ExampleManager::new("postgres://localhost/test".to_string());
        let config = PoolConfig::new("postgres://localhost/test");
        let pool = DeadpoolConnectionPool::new(manager, config);

        assert!(!pool.is_closed());

        pool.close().await.unwrap();
        assert!(pool.is_closed());
    }

    #[tokio::test]
    async fn test_connection_release() {
        let manager = ExampleManager::new("postgres://localhost/test".to_string());
        let config = PoolConfig::new("postgres://localhost/test").with_max_size(5);
        let pool = DeadpoolConnectionPool::new(manager, config);

        let conn = pool.get().await.unwrap();
        let initial_status = pool.status();

        drop(conn);

        // Give a moment for the connection to be returned
        tokio::time::sleep(Duration::from_millis(10)).await;

        let final_status = pool.status();
        assert!(final_status.idle >= initial_status.idle);
    }
}

#[cfg(feature = "pool-bb8")]
mod bb8_tests {
    use super::*;
    use dev_engineeringlabs_rustboot_database::pool::{Bb8ConnectionPool, ExampleBb8Manager};
    use std::sync::Arc;
    use std::time::Duration;

    #[tokio::test]
    async fn test_bb8_pool_creation() {
        let manager = ExampleBb8Manager::new("postgres://localhost/test".to_string());
        let config = PoolConfig::new("postgres://localhost/test").with_max_size(5);
        let pool = Bb8ConnectionPool::new(manager, config).await;

        assert!(pool.is_ok());
        let pool = pool.unwrap();
        assert_eq!(pool.max_size(), 5);
    }

    #[tokio::test]
    async fn test_bb8_get_connection() {
        let manager = ExampleBb8Manager::new("postgres://localhost/test".to_string());
        let config = PoolConfig::new("postgres://localhost/test");
        let pool = Bb8ConnectionPool::new(manager, config).await.unwrap();

        let conn = pool.get().await;
        assert!(conn.is_ok());

        let conn = conn.unwrap();
        let result = conn.query("SELECT 1").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_bb8_multiple_connections() {
        let manager = ExampleBb8Manager::new("postgres://localhost/test".to_string());
        let config = PoolConfig::new("postgres://localhost/test").with_max_size(10);
        let pool = Bb8ConnectionPool::new(manager, config).await.unwrap();

        let mut connections = Vec::new();
        for _ in 0..5 {
            let conn = pool.get().await;
            assert!(conn.is_ok());
            connections.push(conn.unwrap());
        }

        let status = pool.status();
        assert!(status.active <= status.max_size);
    }

    #[tokio::test]
    async fn test_bb8_concurrent_access() {
        let manager = ExampleBb8Manager::new("postgres://localhost/test".to_string());
        let config = PoolConfig::new("postgres://localhost/test").with_max_size(20);
        let pool = Arc::new(Bb8ConnectionPool::new(manager, config).await.unwrap());

        let mut handles = vec![];
        for _ in 0..10 {
            let pool_clone = Arc::clone(&pool);
            let handle = tokio::spawn(async move {
                let conn = pool_clone.get().await.unwrap();
                let _result = conn.query("SELECT 1").await.unwrap();
            });
            handles.push(handle);
        }

        for handle in handles {
            assert!(handle.await.is_ok());
        }
    }

    #[tokio::test]
    async fn test_bb8_builder() {
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

        let conn = pool.get().await;
        assert!(conn.is_ok());
    }
}

mod pool_config_tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_default_config() {
        let config = PoolConfig::new("postgres://localhost/test");
        assert_eq!(config.max_size, 10);
        assert_eq!(config.min_idle, Some(1));
        assert_eq!(config.connection_timeout, Some(Duration::from_secs(30)));
        assert!(!config.test_on_acquire);
        assert!(!config.test_on_release);
    }

    #[test]
    fn test_config_builder() {
        let config = PoolConfig::new("postgres://localhost/test")
            .with_max_size(20)
            .with_min_idle(5)
            .with_connection_timeout(Duration::from_secs(60))
            .with_max_lifetime(Duration::from_secs(3600))
            .with_idle_timeout(Duration::from_secs(300))
            .with_test_on_acquire(true)
            .with_test_on_release(true);

        assert_eq!(config.max_size, 20);
        assert_eq!(config.min_idle, Some(5));
        assert_eq!(config.connection_timeout, Some(Duration::from_secs(60)));
        assert_eq!(config.max_lifetime, Some(Duration::from_secs(3600)));
        assert_eq!(config.idle_timeout, Some(Duration::from_secs(300)));
        assert!(config.test_on_acquire);
        assert!(config.test_on_release);
    }

    #[test]
    fn test_pool_status() {
        let status = PoolStatus::new(5, 3, 10);
        assert_eq!(status.size, 5);
        assert_eq!(status.idle, 3);
        assert_eq!(status.active, 2);
        assert_eq!(status.max_size, 10);
        assert!(!status.is_full());
        assert!(status.has_idle());
        assert_eq!(status.utilization(), 0.2);
    }

    #[test]
    fn test_pool_status_full() {
        let status = PoolStatus::new(10, 0, 10);
        assert!(status.is_full());
        assert!(!status.has_idle());
        assert_eq!(status.utilization(), 1.0);
    }

    #[test]
    fn test_pool_status_empty() {
        let status = PoolStatus::new(0, 0, 10);
        assert!(!status.is_full());
        assert!(!status.has_idle());
        assert_eq!(status.utilization(), 0.0);
    }
}

#[test]
fn test_crud() {
    // Placeholder test
}
