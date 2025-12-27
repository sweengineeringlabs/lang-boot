//! Session manager for managing session lifecycle.

use crate::error::{SessionError, SessionResult};
use crate::session_data::SessionData;
use crate::session_id::SessionId;
use crate::store::{SessionConfig, SessionStore};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use tokio::time::interval;

/// Session manager.
///
/// The manager handles session creation, loading, saving, and cleanup.
/// It wraps a SessionStore and provides high-level session operations.
pub struct SessionManager<S>
where
    S: SessionStore,
{
    store: Arc<S>,
    config: SessionConfig,
    cleanup_task: RwLock<Option<JoinHandle<()>>>,
}

impl<S> SessionManager<S>
where
    S: SessionStore + 'static,
{
    /// Create a new session manager.
    pub fn new(store: S, config: SessionConfig) -> Self {
        Self {
            store: Arc::new(store),
            config,
            cleanup_task: RwLock::new(None),
        }
    }

    /// Create a new session manager with default configuration.
    pub fn with_defaults(store: S) -> Self {
        Self::new(store, SessionConfig::default())
    }

    /// Get the session configuration.
    pub fn config(&self) -> &SessionConfig {
        &self.config
    }

    /// Start automatic cleanup of expired sessions.
    ///
    /// This spawns a background task that periodically removes expired sessions.
    pub async fn start_cleanup(&self) {
        if let Some(cleanup_interval) = self.config.cleanup_interval {
            let store = Arc::clone(&self.store);
            let mut interval = interval(cleanup_interval);

            let handle = tokio::spawn(async move {
                loop {
                    interval.tick().await;
                    if let Err(e) = store.cleanup_expired().await {
                        tracing::warn!("Session cleanup error: {}", e);
                    }
                }
            });

            let mut cleanup_task = self.cleanup_task.write().await;
            *cleanup_task = Some(handle);
        }
    }

    /// Stop the automatic cleanup task.
    pub async fn stop_cleanup(&self) {
        let mut cleanup_task = self.cleanup_task.write().await;
        if let Some(handle) = cleanup_task.take() {
            handle.abort();
        }
    }

    /// Create a new session.
    ///
    /// Returns a new session ID and empty session data.
    pub async fn create(&self) -> SessionResult<(SessionId, SessionData)> {
        let session_id = SessionId::generate();
        let data = if let Some(ttl) = self.config.default_ttl {
            SessionData::with_expiration(ttl)
        } else {
            SessionData::new()
        };

        self.store.save(&session_id, data.clone()).await?;

        Ok((session_id, data))
    }

    /// Load an existing session.
    ///
    /// Returns None if the session doesn't exist or has expired.
    pub async fn load(&self, session_id: &SessionId) -> SessionResult<Option<SessionData>> {
        self.store.load(session_id).await
    }

    /// Get a session, creating it if it doesn't exist.
    pub async fn get_or_create(
        &self,
        session_id: Option<&SessionId>,
    ) -> SessionResult<(SessionId, SessionData)> {
        if let Some(id) = session_id {
            if let Some(data) = self.load(id).await? {
                return Ok((id.clone(), data));
            }
        }

        self.create().await
    }

    /// Save session data.
    pub async fn save(&self, session_id: &SessionId, data: SessionData) -> SessionResult<()> {
        self.store.save(session_id, data).await
    }

    /// Update session data with a function.
    ///
    /// Loads the session, applies the function, and saves it back.
    pub async fn update<F>(&self, session_id: &SessionId, f: F) -> SessionResult<()>
    where
        F: FnOnce(&mut SessionData) -> SessionResult<()>,
    {
        let mut data = self
            .load(session_id)
            .await?
            .ok_or(SessionError::NotFound)?;

        f(&mut data)?;

        self.save(session_id, data).await
    }

    /// Touch a session to update its last_accessed timestamp.
    pub async fn touch(&self, session_id: &SessionId) -> SessionResult<()> {
        self.update(session_id, |data| {
            data.touch();
            Ok(())
        })
        .await
    }

    /// Delete a session.
    pub async fn delete(&self, session_id: &SessionId) -> SessionResult<()> {
        self.store.delete(session_id).await
    }

    /// Check if a session exists.
    pub async fn exists(&self, session_id: &SessionId) -> SessionResult<bool> {
        self.store.exists(session_id).await
    }

    /// Regenerate a session ID (for security after login).
    ///
    /// Creates a new session ID with the same data as the old one,
    /// then deletes the old session.
    pub async fn regenerate(&self, old_id: &SessionId) -> SessionResult<SessionId> {
        let data = self.load(old_id).await?.ok_or(SessionError::NotFound)?;

        let new_id = SessionId::generate();
        self.store.save(&new_id, data).await?;
        self.store.delete(old_id).await?;

        Ok(new_id)
    }

    /// Get session count.
    pub async fn count(&self) -> SessionResult<usize> {
        self.store.count().await
    }

    /// Manually trigger cleanup of expired sessions.
    ///
    /// Returns the number of sessions removed.
    pub async fn cleanup_expired(&self) -> SessionResult<usize> {
        self.store.cleanup_expired().await
    }

    /// Clear all sessions.
    pub async fn clear(&self) -> SessionResult<()> {
        self.store.clear().await
    }
}

impl<S> Drop for SessionManager<S>
where
    S: SessionStore,
{
    fn drop(&mut self) {
        // Abort cleanup task when manager is dropped
        if let Ok(mut task) = self.cleanup_task.try_write() {
            if let Some(handle) = task.take() {
                handle.abort();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory_store::MemorySessionStore;
    use std::time::Duration;

    #[tokio::test]
    async fn test_manager_create() {
        let store = MemorySessionStore::new();
        let manager = SessionManager::with_defaults(store);

        let (session_id, data) = manager.create().await.unwrap();

        assert!(manager.exists(&session_id).await.unwrap());
        assert!(data.is_empty());
    }

    #[tokio::test]
    async fn test_manager_load() {
        let store = MemorySessionStore::new();
        let manager = SessionManager::with_defaults(store);

        let (session_id, _) = manager.create().await.unwrap();

        let loaded = manager.load(&session_id).await.unwrap();
        assert!(loaded.is_some());

        let nonexistent_id = SessionId::generate();
        let result = manager.load(&nonexistent_id).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_manager_update() {
        let store = MemorySessionStore::new();
        let manager = SessionManager::with_defaults(store);

        let (session_id, _) = manager.create().await.unwrap();

        manager
            .update(&session_id, |data| {
                data.set("user_id", 42u64).unwrap();
                Ok(())
            })
            .await
            .unwrap();

        let data = manager.load(&session_id).await.unwrap().unwrap();
        let user_id: u64 = data.get("user_id").unwrap().unwrap();
        assert_eq!(user_id, 42);
    }

    #[tokio::test]
    async fn test_manager_delete() {
        let store = MemorySessionStore::new();
        let manager = SessionManager::with_defaults(store);

        let (session_id, _) = manager.create().await.unwrap();

        assert!(manager.exists(&session_id).await.unwrap());

        manager.delete(&session_id).await.unwrap();

        assert!(!manager.exists(&session_id).await.unwrap());
    }

    #[tokio::test]
    async fn test_manager_regenerate() {
        let store = MemorySessionStore::new();
        let manager = SessionManager::with_defaults(store);

        let (old_id, _) = manager.create().await.unwrap();

        manager
            .update(&old_id, |data| {
                data.set("user_id", 42u64).unwrap();
                Ok(())
            })
            .await
            .unwrap();

        let new_id = manager.regenerate(&old_id).await.unwrap();

        // Old session should be deleted
        assert!(!manager.exists(&old_id).await.unwrap());

        // New session should exist with same data
        let data = manager.load(&new_id).await.unwrap().unwrap();
        let user_id: u64 = data.get("user_id").unwrap().unwrap();
        assert_eq!(user_id, 42);
    }

    #[tokio::test]
    async fn test_manager_get_or_create() {
        let store = MemorySessionStore::new();
        let manager = SessionManager::with_defaults(store);

        // Without session ID - creates new
        let (id1, _) = manager.get_or_create(None).await.unwrap();

        // With existing session ID - returns existing
        let (id2, _) = manager.get_or_create(Some(&id1)).await.unwrap();
        assert_eq!(id1, id2);

        // With nonexistent session ID - creates new
        let nonexistent = SessionId::generate();
        let (id3, _) = manager.get_or_create(Some(&nonexistent)).await.unwrap();
        assert_ne!(id3, nonexistent);
    }

    #[tokio::test]
    async fn test_manager_cleanup() {
        let store = MemorySessionStore::new();
        let config = SessionConfig::default().with_ttl(Duration::from_millis(50));
        let manager = SessionManager::new(store, config);

        // Create sessions
        for _ in 0..3 {
            manager.create().await.unwrap();
        }

        // Wait for expiration
        tokio::time::sleep(Duration::from_millis(100)).await;

        let removed = manager.cleanup_expired().await.unwrap();
        assert_eq!(removed, 3);
        assert_eq!(manager.count().await.unwrap(), 0);
    }

    #[tokio::test]
    async fn test_manager_auto_cleanup() {
        let store = MemorySessionStore::new();
        let config = SessionConfig::default()
            .with_ttl(Duration::from_millis(50))
            .with_cleanup_interval(Duration::from_millis(100));
        let manager = SessionManager::new(store, config);

        manager.start_cleanup().await;

        // Create sessions
        for _ in 0..3 {
            manager.create().await.unwrap();
        }

        // Wait for expiration and cleanup
        tokio::time::sleep(Duration::from_millis(200)).await;

        // Sessions should be cleaned up automatically
        assert_eq!(manager.count().await.unwrap(), 0);

        manager.stop_cleanup().await;
    }
}
