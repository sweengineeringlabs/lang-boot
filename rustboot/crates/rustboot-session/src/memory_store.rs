//! In-memory session store implementation.

use crate::error::SessionResult;
use crate::session_data::SessionData;
use crate::session_id::SessionId;
use crate::store::SessionStore;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// In-memory session store.
///
/// This store keeps all sessions in memory and is suitable for development
/// and single-server deployments. Sessions are lost on restart.
#[derive(Clone)]
pub struct MemorySessionStore {
    sessions: Arc<RwLock<HashMap<SessionId, SessionData>>>,
}

impl MemorySessionStore {
    /// Create a new in-memory session store.
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Get the number of sessions currently in memory (including expired).
    pub async fn size(&self) -> usize {
        self.sessions.read().await.len()
    }
}

impl Default for MemorySessionStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SessionStore for MemorySessionStore {
    async fn load(&self, session_id: &SessionId) -> SessionResult<Option<SessionData>> {
        let sessions = self.sessions.read().await;

        if let Some(data) = sessions.get(session_id) {
            if data.is_expired() {
                // Session exists but is expired
                Ok(None)
            } else {
                Ok(Some(data.clone()))
            }
        } else {
            Ok(None)
        }
    }

    async fn save(&self, session_id: &SessionId, data: SessionData) -> SessionResult<()> {
        let mut sessions = self.sessions.write().await;
        sessions.insert(session_id.clone(), data);
        Ok(())
    }

    async fn delete(&self, session_id: &SessionId) -> SessionResult<()> {
        let mut sessions = self.sessions.write().await;
        sessions.remove(session_id);
        Ok(())
    }

    async fn exists(&self, session_id: &SessionId) -> SessionResult<bool> {
        let sessions = self.sessions.read().await;

        if let Some(data) = sessions.get(session_id) {
            Ok(!data.is_expired())
        } else {
            Ok(false)
        }
    }

    async fn cleanup_expired(&self) -> SessionResult<usize> {
        let mut sessions = self.sessions.write().await;
        let initial_count = sessions.len();

        sessions.retain(|_, data| !data.is_expired());

        let removed = initial_count - sessions.len();
        Ok(removed)
    }

    async fn count(&self) -> SessionResult<usize> {
        let sessions = self.sessions.read().await;
        Ok(sessions.len())
    }

    async fn clear(&self) -> SessionResult<()> {
        let mut sessions = self.sessions.write().await;
        sessions.clear();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_memory_store_basic() {
        let store = MemorySessionStore::new();
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
    }

    #[tokio::test]
    async fn test_memory_store_not_found() {
        let store = MemorySessionStore::new();
        let session_id = SessionId::generate();

        let result = store.load(&session_id).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_memory_store_delete() {
        let store = MemorySessionStore::new();
        let session_id = SessionId::generate();
        let data = SessionData::new();

        store.save(&session_id, data).await.unwrap();
        assert!(store.exists(&session_id).await.unwrap());

        store.delete(&session_id).await.unwrap();
        assert!(!store.exists(&session_id).await.unwrap());
    }

    #[tokio::test]
    async fn test_memory_store_expired_session() {
        let store = MemorySessionStore::new();
        let session_id = SessionId::generate();

        // Create a session that expires immediately
        let data = SessionData::with_expiration(Duration::from_millis(1));

        store.save(&session_id, data).await.unwrap();

        // Wait for expiration
        tokio::time::sleep(Duration::from_millis(10)).await;

        // Session should not be loaded (expired)
        let result = store.load(&session_id).await.unwrap();
        assert!(result.is_none());

        // exists should return false for expired session
        assert!(!store.exists(&session_id).await.unwrap());
    }

    #[tokio::test]
    async fn test_memory_store_cleanup() {
        let store = MemorySessionStore::new();

        // Create some valid sessions
        for i in 0..3 {
            let session_id = SessionId::generate();
            let mut data = SessionData::new();
            data.set("index", i).unwrap();
            store.save(&session_id, data).await.unwrap();
        }

        // Create some expired sessions
        for i in 0..2 {
            let session_id = SessionId::generate();
            let mut data = SessionData::with_expiration(Duration::from_millis(1));
            data.set("index", i).unwrap();
            store.save(&session_id, data).await.unwrap();
        }

        // Wait for expiration
        tokio::time::sleep(Duration::from_millis(10)).await;

        assert_eq!(store.count().await.unwrap(), 5);

        // Cleanup expired sessions
        let removed = store.cleanup_expired().await.unwrap();
        assert_eq!(removed, 2);
        assert_eq!(store.count().await.unwrap(), 3);
    }

    #[tokio::test]
    async fn test_memory_store_clear() {
        let store = MemorySessionStore::new();

        // Create multiple sessions
        for _ in 0..5 {
            let session_id = SessionId::generate();
            let data = SessionData::new();
            store.save(&session_id, data).await.unwrap();
        }

        assert_eq!(store.count().await.unwrap(), 5);

        store.clear().await.unwrap();
        assert_eq!(store.count().await.unwrap(), 0);
    }

    #[tokio::test]
    async fn test_memory_store_concurrent_access() {
        let store = MemorySessionStore::new();
        let store1 = store.clone();
        let store2 = store.clone();

        let session_id = SessionId::generate();
        let session_id1 = session_id.clone();
        let session_id2 = session_id.clone();

        // Concurrent writes
        let handle1 = tokio::spawn(async move {
            let mut data = SessionData::new();
            data.set("thread", 1u64).unwrap();
            store1.save(&session_id1, data).await.unwrap();
        });

        let handle2 = tokio::spawn(async move {
            let mut data = SessionData::new();
            data.set("thread", 2u64).unwrap();
            store2.save(&session_id2, data).await.unwrap();
        });

        handle1.await.unwrap();
        handle2.await.unwrap();

        // Session should exist (one of the writes succeeded)
        assert!(store.exists(&session_id).await.unwrap());
    }
}
