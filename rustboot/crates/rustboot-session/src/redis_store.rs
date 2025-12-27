//! Redis session store implementation.

#[cfg(feature = "redis-store")]
use crate::error::{SessionError, SessionResult};
#[cfg(feature = "redis-store")]
use crate::session_data::SessionData;
#[cfg(feature = "redis-store")]
use crate::session_id::SessionId;
#[cfg(feature = "redis-store")]
use crate::store::SessionStore;
#[cfg(feature = "redis-store")]
use async_trait::async_trait;
#[cfg(feature = "redis-store")]
use redis::{aio::ConnectionManager, AsyncCommands, RedisError};

#[cfg(feature = "redis-store")]
/// Redis session store.
///
/// This store uses Redis for session persistence, suitable for distributed
/// deployments and high-traffic applications.
#[derive(Clone)]
pub struct RedisSessionStore {
    connection: ConnectionManager,
    key_prefix: String,
}

#[cfg(feature = "redis-store")]
impl RedisSessionStore {
    /// Create a new Redis session store.
    ///
    /// # Arguments
    ///
    /// * `redis_url` - Redis connection URL (e.g., "redis://127.0.0.1:6379")
    /// * `key_prefix` - Prefix for session keys (e.g., "session:")
    pub async fn new(redis_url: &str, key_prefix: impl Into<String>) -> SessionResult<Self> {
        let client = redis::Client::open(redis_url)
            .map_err(|e| SessionError::Storage(format!("Failed to create Redis client: {}", e)))?;

        let connection = ConnectionManager::new(client)
            .await
            .map_err(|e| SessionError::Storage(format!("Failed to connect to Redis: {}", e)))?;

        Ok(Self {
            connection,
            key_prefix: key_prefix.into(),
        })
    }

    /// Create a new Redis session store with default key prefix.
    pub async fn with_defaults(redis_url: &str) -> SessionResult<Self> {
        Self::new(redis_url, "session:").await
    }

    fn make_key(&self, session_id: &SessionId) -> String {
        format!("{}{}", self.key_prefix, session_id.as_str())
    }

    fn extract_session_id(&self, key: &str) -> Option<String> {
        key.strip_prefix(&self.key_prefix).map(|s| s.to_string())
    }
}

#[cfg(feature = "redis-store")]
#[async_trait]
impl SessionStore for RedisSessionStore {
    async fn load(&self, session_id: &SessionId) -> SessionResult<Option<SessionData>> {
        let key = self.make_key(session_id);
        let mut conn = self.connection.clone();

        let result: Option<String> = conn
            .get(&key)
            .await
            .map_err(|e: RedisError| SessionError::Storage(format!("Redis GET error: {}", e)))?;

        match result {
            Some(json) => {
                let data = SessionData::from_json(&json)?;

                // Check if expired
                if data.is_expired() {
                    // Delete expired session
                    let _: () = conn
                        .del(&key)
                        .await
                        .map_err(|e: RedisError| SessionError::Storage(format!("Redis DEL error: {}", e)))?;
                    Ok(None)
                } else {
                    Ok(Some(data))
                }
            }
            None => Ok(None),
        }
    }

    async fn save(&self, session_id: &SessionId, data: SessionData) -> SessionResult<()> {
        let key = self.make_key(session_id);
        let json = data.to_json()?;
        let mut conn = self.connection.clone();

        // Set with expiration if configured
        if let Some(ttl) = data.expires_in() {
            let _: () = conn
                .set_ex(&key, json, ttl.as_secs() as u64)
                .await
                .map_err(|e: RedisError| SessionError::Storage(format!("Redis SETEX error: {}", e)))?;
        } else {
            let _: () = conn
                .set(&key, json)
                .await
                .map_err(|e: RedisError| SessionError::Storage(format!("Redis SET error: {}", e)))?;
        }

        Ok(())
    }

    async fn delete(&self, session_id: &SessionId) -> SessionResult<()> {
        let key = self.make_key(session_id);
        let mut conn = self.connection.clone();

        let _: () = conn
            .del(&key)
            .await
            .map_err(|e: RedisError| SessionError::Storage(format!("Redis DEL error: {}", e)))?;

        Ok(())
    }

    async fn exists(&self, session_id: &SessionId) -> SessionResult<bool> {
        let key = self.make_key(session_id);
        let mut conn = self.connection.clone();

        let exists: bool = conn
            .exists(&key)
            .await
            .map_err(|e: RedisError| SessionError::Storage(format!("Redis EXISTS error: {}", e)))?;

        if exists {
            // Verify not expired
            if let Some(data) = self.load(session_id).await? {
                Ok(!data.is_expired())
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    async fn cleanup_expired(&self) -> SessionResult<usize> {
        // Redis handles expiration automatically with TTL
        // We can scan for sessions and check manually, but it's not necessary
        // This is a no-op for Redis as TTL handles cleanup
        Ok(0)
    }

    async fn count(&self) -> SessionResult<usize> {
        let mut conn = self.connection.clone();
        let pattern = format!("{}*", self.key_prefix);

        let keys: Vec<String> = conn
            .keys(&pattern)
            .await
            .map_err(|e: RedisError| SessionError::Storage(format!("Redis KEYS error: {}", e)))?;

        Ok(keys.len())
    }

    async fn clear(&self) -> SessionResult<()> {
        let mut conn = self.connection.clone();
        let pattern = format!("{}*", self.key_prefix);

        let keys: Vec<String> = conn
            .keys(&pattern)
            .await
            .map_err(|e: RedisError| SessionError::Storage(format!("Redis KEYS error: {}", e)))?;

        if !keys.is_empty() {
            let _: () = conn
                .del(&keys)
                .await
                .map_err(|e: RedisError| SessionError::Storage(format!("Redis DEL error: {}", e)))?;
        }

        Ok(())
    }
}

#[cfg(all(test, feature = "redis-store"))]
mod tests {
    use super::*;
    use std::time::Duration;

    async fn create_store() -> RedisSessionStore {
        // This assumes Redis is running on localhost:6379
        // In CI/CD, you'd use a Docker container or mock
        RedisSessionStore::with_defaults("redis://127.0.0.1:6379")
            .await
            .expect("Failed to connect to Redis")
    }

    #[tokio::test]
    #[ignore] // Requires Redis to be running
    async fn test_redis_store_basic() {
        let store = create_store().await;
        let session_id = SessionId::generate();
        let mut data = SessionData::new();

        data.set("user_id", 42u64).unwrap();

        // Save session
        store.save(&session_id, data.clone()).await.unwrap();

        // Load session
        let loaded = store.load(&session_id).await.unwrap();
        assert!(loaded.is_some());

        let loaded_data = loaded.unwrap();
        let user_id: u64 = loaded_data.get("user_id").unwrap().unwrap();
        assert_eq!(user_id, 42);

        // Cleanup
        store.delete(&session_id).await.unwrap();
    }

    #[tokio::test]
    #[ignore] // Requires Redis to be running
    async fn test_redis_store_expiration() {
        let store = create_store().await;
        let session_id = SessionId::generate();

        let data = SessionData::with_expiration(Duration::from_secs(2));
        store.save(&session_id, data).await.unwrap();

        // Session should exist
        assert!(store.exists(&session_id).await.unwrap());

        // Wait for expiration
        tokio::time::sleep(Duration::from_secs(3)).await;

        // Session should be expired
        let result = store.load(&session_id).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    #[ignore] // Requires Redis to be running
    async fn test_redis_store_delete() {
        let store = create_store().await;
        let session_id = SessionId::generate();
        let data = SessionData::new();

        store.save(&session_id, data).await.unwrap();
        assert!(store.exists(&session_id).await.unwrap());

        store.delete(&session_id).await.unwrap();
        assert!(!store.exists(&session_id).await.unwrap());
    }

    #[tokio::test]
    #[ignore] // Requires Redis to be running
    async fn test_redis_store_count() {
        let store = create_store().await;

        // Clear any existing sessions
        store.clear().await.unwrap();

        // Create some sessions
        for _ in 0..3 {
            let session_id = SessionId::generate();
            let data = SessionData::new();
            store.save(&session_id, data).await.unwrap();
        }

        assert_eq!(store.count().await.unwrap(), 3);

        // Cleanup
        store.clear().await.unwrap();
    }
}
