//! Session middleware for request/response processing.
//!
//! This module provides middleware components that can be used with
//! web frameworks to automatically manage sessions.

use crate::error::SessionResult;
use crate::manager::SessionManager;
use crate::session_data::SessionData;
use crate::session_id::SessionId;
use crate::store::SessionStore;
use std::sync::Arc;

/// Session context that can be attached to requests.
///
/// This provides access to session data within request handlers.
#[derive(Clone)]
pub struct SessionContext {
    session_id: SessionId,
    data: SessionData,
    modified: bool,
}

impl SessionContext {
    /// Create a new session context.
    pub fn new(session_id: SessionId, data: SessionData) -> Self {
        Self {
            session_id,
            data,
            modified: false,
        }
    }

    /// Get the session ID.
    pub fn id(&self) -> &SessionId {
        &self.session_id
    }

    /// Get a value from the session.
    pub fn get<T>(&self, key: &str) -> SessionResult<Option<T>>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        self.data.get(key)
    }

    /// Set a value in the session.
    pub fn set<T>(&mut self, key: impl Into<String>, value: T) -> SessionResult<()>
    where
        T: serde::Serialize,
    {
        self.modified = true;
        self.data.set(key, value)
    }

    /// Remove a value from the session.
    pub fn remove(&mut self, key: &str) -> Option<serde_json::Value> {
        self.modified = true;
        self.data.remove(key)
    }

    /// Check if a key exists.
    pub fn contains(&self, key: &str) -> bool {
        self.data.contains(key)
    }

    /// Clear all session data.
    pub fn clear(&mut self) {
        self.modified = true;
        self.data.clear();
    }

    /// Check if the session has been modified.
    pub fn is_modified(&self) -> bool {
        self.modified
    }

    /// Get the underlying session data.
    pub fn data(&self) -> &SessionData {
        &self.data
    }

    /// Get a mutable reference to the session data.
    pub fn data_mut(&mut self) -> &mut SessionData {
        self.modified = true;
        &mut self.data
    }

    /// Consume the context and return the session data.
    pub fn into_data(self) -> (SessionId, SessionData, bool) {
        (self.session_id, self.data, self.modified)
    }
}

/// Session middleware helper.
///
/// This provides utilities for integrating sessions into middleware pipelines.
pub struct SessionMiddleware<S>
where
    S: SessionStore,
{
    manager: Arc<SessionManager<S>>,
}

impl<S> SessionMiddleware<S>
where
    S: SessionStore + 'static,
{
    /// Create a new session middleware.
    pub fn new(manager: SessionManager<S>) -> Self {
        Self {
            manager: Arc::new(manager),
        }
    }

    /// Get the session manager.
    pub fn manager(&self) -> &SessionManager<S> {
        &self.manager
    }

    /// Load or create a session.
    ///
    /// This is typically called at the start of request processing.
    pub async fn load_or_create(
        &self,
        session_id: Option<&SessionId>,
    ) -> SessionResult<SessionContext> {
        let (id, data) = self.manager.get_or_create(session_id).await?;
        Ok(SessionContext::new(id, data))
    }

    /// Save a session if it was modified.
    ///
    /// This is typically called at the end of request processing.
    pub async fn save_if_modified(&self, context: SessionContext) -> SessionResult<SessionId> {
        let (session_id, data, modified) = context.into_data();

        if modified {
            self.manager.save(&session_id, data).await?;
        }

        Ok(session_id)
    }

    /// Extract session ID from a cookie value.
    pub fn extract_session_id(&self, cookie_value: &str) -> SessionResult<SessionId> {
        SessionId::from_string(cookie_value)
    }

    /// Start automatic cleanup.
    pub async fn start_cleanup(&self) {
        self.manager.start_cleanup().await;
    }

    /// Stop automatic cleanup.
    pub async fn stop_cleanup(&self) {
        self.manager.stop_cleanup().await;
    }
}

impl<S> Clone for SessionMiddleware<S>
where
    S: SessionStore,
{
    fn clone(&self) -> Self {
        Self {
            manager: Arc::clone(&self.manager),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory_store::MemorySessionStore;

    #[tokio::test]
    async fn test_session_context_basic() {
        let session_id = SessionId::generate();
        let mut data = SessionData::new();
        data.set("key", "value".to_string()).unwrap();

        let mut context = SessionContext::new(session_id.clone(), data);

        assert_eq!(context.id(), &session_id);
        assert!(!context.is_modified());

        let value: String = context.get("key").unwrap().unwrap();
        assert_eq!(value, "value");

        context.set("new_key", "new_value".to_string()).unwrap();
        assert!(context.is_modified());
    }

    #[tokio::test]
    async fn test_session_context_remove() {
        let session_id = SessionId::generate();
        let mut data = SessionData::new();
        data.set("key", "value".to_string()).unwrap();

        let mut context = SessionContext::new(session_id, data);

        assert!(context.contains("key"));
        context.remove("key");
        assert!(!context.contains("key"));
        assert!(context.is_modified());
    }

    #[tokio::test]
    async fn test_session_middleware_load_or_create() {
        let store = MemorySessionStore::new();
        let manager = SessionManager::with_defaults(store);
        let middleware = SessionMiddleware::new(manager);

        // Create new session
        let context = middleware.load_or_create(None).await.unwrap();
        let id = context.id().clone();

        // Load existing session
        let context2 = middleware.load_or_create(Some(&id)).await.unwrap();
        assert_eq!(context2.id(), &id);
    }

    #[tokio::test]
    async fn test_session_middleware_save() {
        let store = MemorySessionStore::new();
        let manager = SessionManager::with_defaults(store);
        let middleware = SessionMiddleware::new(manager);

        let mut context = middleware.load_or_create(None).await.unwrap();
        let session_id = context.id().clone();

        context.set("user_id", 42u64).unwrap();

        middleware.save_if_modified(context).await.unwrap();

        // Verify saved
        let loaded = middleware.manager().load(&session_id).await.unwrap().unwrap();
        let user_id: u64 = loaded.get("user_id").unwrap().unwrap();
        assert_eq!(user_id, 42);
    }

    #[tokio::test]
    async fn test_session_middleware_no_save_if_not_modified() {
        let store = MemorySessionStore::new();
        let manager = SessionManager::with_defaults(store);
        let middleware = SessionMiddleware::new(manager);

        let context = middleware.load_or_create(None).await.unwrap();

        // Don't modify the session
        assert!(!context.is_modified());

        middleware.save_if_modified(context).await.unwrap();

        // Session should still exist but with no data
        // (this is expected behavior - session is created but not modified)
    }

    #[tokio::test]
    async fn test_extract_session_id() {
        let store = MemorySessionStore::new();
        let manager = SessionManager::with_defaults(store);
        let middleware = SessionMiddleware::new(manager);

        let id = SessionId::generate();
        let cookie_value = id.as_str();

        let extracted = middleware.extract_session_id(cookie_value).unwrap();
        assert_eq!(extracted, id);
    }

    #[tokio::test]
    async fn test_extract_invalid_session_id() {
        let store = MemorySessionStore::new();
        let manager = SessionManager::with_defaults(store);
        let middleware = SessionMiddleware::new(manager);

        let result = middleware.extract_session_id("invalid-id");
        assert!(result.is_err());
    }
}
