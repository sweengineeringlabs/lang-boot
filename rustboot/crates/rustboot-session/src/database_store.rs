//! Database session store implementation.
//!
//! This is a generic implementation that can work with any database
//! that implements the rustboot Database trait.

use crate::error::SessionResult;
use crate::session_data::SessionData;
use crate::session_id::SessionId;
use crate::store::SessionStore;
use async_trait::async_trait;
use std::sync::Arc;

/// Database session store.
///
/// This store uses a database table for session persistence. It requires
/// a table with the following schema:
///
/// ```sql
/// CREATE TABLE sessions (
///     id VARCHAR(255) PRIMARY KEY,
///     data TEXT NOT NULL,
///     created_at BIGINT NOT NULL,
///     last_accessed BIGINT NOT NULL,
///     expires_at BIGINT
/// );
/// ```
pub struct DatabaseSessionStore<DB>
where
    DB: SessionDatabase,
{
    database: Arc<DB>,
    table_name: String,
}

impl<DB> DatabaseSessionStore<DB>
where
    DB: SessionDatabase,
{
    /// Create a new database session store.
    ///
    /// # Arguments
    ///
    /// * `database` - Database connection implementing SessionDatabase trait
    /// * `table_name` - Name of the sessions table (default: "sessions")
    pub fn new(database: Arc<DB>, table_name: impl Into<String>) -> Self {
        Self {
            database,
            table_name: table_name.into(),
        }
    }

    /// Create a new database session store with default table name.
    pub fn with_defaults(database: Arc<DB>) -> Self {
        Self::new(database, "sessions")
    }

    /// Initialize the sessions table.
    ///
    /// This creates the sessions table if it doesn't exist.
    /// Note: This is a basic implementation and may need to be customized
    /// for specific database systems.
    pub async fn init_table(&self) -> SessionResult<()> {
        self.database.init_sessions_table(&self.table_name).await
    }
}

/// Trait for database operations required by the session store.
///
/// This abstracts the database layer so the session store can work
/// with any database implementation.
#[async_trait]
pub trait SessionDatabase: Send + Sync {
    /// Initialize the sessions table.
    async fn init_sessions_table(&self, table_name: &str) -> SessionResult<()>;

    /// Load session data by ID.
    async fn load_session(
        &self,
        table_name: &str,
        session_id: &SessionId,
    ) -> SessionResult<Option<SessionData>>;

    /// Save session data.
    async fn save_session(
        &self,
        table_name: &str,
        session_id: &SessionId,
        data: &SessionData,
    ) -> SessionResult<()>;

    /// Delete session by ID.
    async fn delete_session(
        &self,
        table_name: &str,
        session_id: &SessionId,
    ) -> SessionResult<()>;

    /// Check if session exists.
    async fn session_exists(
        &self,
        table_name: &str,
        session_id: &SessionId,
    ) -> SessionResult<bool>;

    /// Delete expired sessions.
    async fn cleanup_expired_sessions(&self, table_name: &str) -> SessionResult<usize>;

    /// Count total sessions.
    async fn count_sessions(&self, table_name: &str) -> SessionResult<usize>;

    /// Clear all sessions.
    async fn clear_sessions(&self, table_name: &str) -> SessionResult<()>;
}

#[async_trait]
impl<DB> SessionStore for DatabaseSessionStore<DB>
where
    DB: SessionDatabase,
{
    async fn load(&self, session_id: &SessionId) -> SessionResult<Option<SessionData>> {
        let result = self
            .database
            .load_session(&self.table_name, session_id)
            .await?;

        // Check if expired
        if let Some(data) = result {
            if data.is_expired() {
                // Delete expired session
                self.database
                    .delete_session(&self.table_name, session_id)
                    .await?;
                Ok(None)
            } else {
                Ok(Some(data))
            }
        } else {
            Ok(None)
        }
    }

    async fn save(&self, session_id: &SessionId, data: SessionData) -> SessionResult<()> {
        self.database
            .save_session(&self.table_name, session_id, &data)
            .await
    }

    async fn delete(&self, session_id: &SessionId) -> SessionResult<()> {
        self.database
            .delete_session(&self.table_name, session_id)
            .await
    }

    async fn exists(&self, session_id: &SessionId) -> SessionResult<bool> {
        let exists = self
            .database
            .session_exists(&self.table_name, session_id)
            .await?;

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
        self.database
            .cleanup_expired_sessions(&self.table_name)
            .await
    }

    async fn count(&self) -> SessionResult<usize> {
        self.database.count_sessions(&self.table_name).await
    }

    async fn clear(&self) -> SessionResult<()> {
        self.database.clear_sessions(&self.table_name).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Mutex;
    use std::time::Duration;

    // Mock database for testing
    struct MockDatabase {
        sessions: Mutex<HashMap<String, SessionData>>,
    }

    impl MockDatabase {
        fn new() -> Self {
            Self {
                sessions: Mutex::new(HashMap::new()),
            }
        }
    }

    #[async_trait]
    impl SessionDatabase for MockDatabase {
        async fn init_sessions_table(&self, _table_name: &str) -> SessionResult<()> {
            Ok(())
        }

        async fn load_session(
            &self,
            _table_name: &str,
            session_id: &SessionId,
        ) -> SessionResult<Option<SessionData>> {
            let sessions = self.sessions.lock().unwrap();
            Ok(sessions.get(session_id.as_str()).cloned())
        }

        async fn save_session(
            &self,
            _table_name: &str,
            session_id: &SessionId,
            data: &SessionData,
        ) -> SessionResult<()> {
            let mut sessions = self.sessions.lock().unwrap();
            sessions.insert(session_id.as_str().to_string(), data.clone());
            Ok(())
        }

        async fn delete_session(
            &self,
            _table_name: &str,
            session_id: &SessionId,
        ) -> SessionResult<()> {
            let mut sessions = self.sessions.lock().unwrap();
            sessions.remove(session_id.as_str());
            Ok(())
        }

        async fn session_exists(
            &self,
            _table_name: &str,
            session_id: &SessionId,
        ) -> SessionResult<bool> {
            let sessions = self.sessions.lock().unwrap();
            Ok(sessions.contains_key(session_id.as_str()))
        }

        async fn cleanup_expired_sessions(&self, _table_name: &str) -> SessionResult<usize> {
            let mut sessions = self.sessions.lock().unwrap();
            let initial_count = sessions.len();
            sessions.retain(|_, data| !data.is_expired());
            Ok(initial_count - sessions.len())
        }

        async fn count_sessions(&self, _table_name: &str) -> SessionResult<usize> {
            let sessions = self.sessions.lock().unwrap();
            Ok(sessions.len())
        }

        async fn clear_sessions(&self, _table_name: &str) -> SessionResult<()> {
            let mut sessions = self.sessions.lock().unwrap();
            sessions.clear();
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_database_store_basic() {
        let db = Arc::new(MockDatabase::new());
        let store = DatabaseSessionStore::with_defaults(db);

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
    async fn test_database_store_delete() {
        let db = Arc::new(MockDatabase::new());
        let store = DatabaseSessionStore::with_defaults(db);

        let session_id = SessionId::generate();
        let data = SessionData::new();

        store.save(&session_id, data).await.unwrap();
        assert!(store.exists(&session_id).await.unwrap());

        store.delete(&session_id).await.unwrap();
        assert!(!store.exists(&session_id).await.unwrap());
    }

    #[tokio::test]
    async fn test_database_store_expired() {
        let db = Arc::new(MockDatabase::new());
        let store = DatabaseSessionStore::with_defaults(db);

        let session_id = SessionId::generate();
        let data = SessionData::with_expiration(Duration::from_millis(1));

        store.save(&session_id, data).await.unwrap();

        // Wait for expiration
        tokio::time::sleep(Duration::from_millis(10)).await;

        // Session should not be loaded (expired)
        let result = store.load(&session_id).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_database_store_cleanup() {
        let db = Arc::new(MockDatabase::new());
        let store = DatabaseSessionStore::with_defaults(db);

        // Create valid sessions
        for i in 0..3 {
            let session_id = SessionId::generate();
            let mut data = SessionData::new();
            data.set("index", i).unwrap();
            store.save(&session_id, data).await.unwrap();
        }

        // Create expired sessions
        for _ in 0..2 {
            let session_id = SessionId::generate();
            let data = SessionData::with_expiration(Duration::from_millis(1));
            store.save(&session_id, data).await.unwrap();
        }

        tokio::time::sleep(Duration::from_millis(10)).await;

        let removed = store.cleanup_expired().await.unwrap();
        assert_eq!(removed, 2);
        assert_eq!(store.count().await.unwrap(), 3);
    }
}
