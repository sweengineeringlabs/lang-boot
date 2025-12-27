//! Integration tests for rustboot-session.

use dev_engineeringlabs_rustboot_session::{
    MemorySessionStore, SessionConfig, SessionData, SessionId, SessionManager, SessionMiddleware,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct User {
    id: u64,
    username: String,
    email: String,
}

#[tokio::test]
async fn test_session_lifecycle() {
    let store = MemorySessionStore::new();
    let manager = SessionManager::with_defaults(store);

    // Create session
    let (session_id, data) = manager.create().await.unwrap();
    assert!(data.is_empty());

    // Update session
    manager
        .update(&session_id, |data| {
            data.set("user_id", 123u64)?;
            data.set("logged_in", true)?;
            Ok(())
        })
        .await
        .unwrap();

    // Load and verify
    let loaded = manager.load(&session_id).await.unwrap().unwrap();
    let user_id: u64 = loaded.get("user_id").unwrap().unwrap();
    let logged_in: bool = loaded.get("logged_in").unwrap().unwrap();
    assert_eq!(user_id, 123);
    assert!(logged_in);

    // Delete session
    manager.delete(&session_id).await.unwrap();
    assert!(!manager.exists(&session_id).await.unwrap());
}

#[tokio::test]
async fn test_session_expiration_workflow() {
    let store = MemorySessionStore::new();
    let config = SessionConfig::default().with_ttl(Duration::from_secs(2)); // 2 seconds TTL
    let manager = SessionManager::new(store, config);

    // Create multiple sessions with data
    let mut session_ids = Vec::new();
    for i in 0..5 {
        let (id, _) = manager.create().await.unwrap();
        manager
            .update(&id, |data| {
                data.set("index", i)?;
                Ok(())
            })
            .await
            .unwrap();
        session_ids.push(id);
    }

    assert_eq!(manager.count().await.unwrap(), 5);

    // Verify sessions exist before expiration
    for id in &session_ids {
        assert!(manager.exists(id).await.unwrap());
    }

    // Wait for expiration
    tokio::time::sleep(Duration::from_secs(3)).await;

    // Sessions should be expired (load returns None)
    for id in &session_ids {
        let result = manager.load(id).await.unwrap();
        assert!(result.is_none(), "Session should be expired");
    }

    // Cleanup expired sessions
    let removed = manager.cleanup_expired().await.unwrap();
    assert_eq!(removed, 5);
    assert_eq!(manager.count().await.unwrap(), 0);
}

#[tokio::test]
async fn test_complex_data_types() {
    let store = MemorySessionStore::new();
    let manager = SessionManager::with_defaults(store);

    let user = User {
        id: 1,
        username: "alice".to_string(),
        email: "alice@example.com".to_string(),
    };

    let (session_id, _) = manager.create().await.unwrap();

    manager
        .update(&session_id, |data| {
            data.set("user", user.clone())?;
            data.set("cart", vec![1, 2, 3, 4, 5])?;
            data.set(
                "preferences",
                serde_json::json!({
                    "theme": "dark",
                    "notifications": true
                }),
            )?;
            Ok(())
        })
        .await
        .unwrap();

    let loaded = manager.load(&session_id).await.unwrap().unwrap();

    let loaded_user: User = loaded.get("user").unwrap().unwrap();
    assert_eq!(loaded_user, user);

    let cart: Vec<i32> = loaded.get("cart").unwrap().unwrap();
    assert_eq!(cart, vec![1, 2, 3, 4, 5]);

    let preferences: serde_json::Value = loaded.get("preferences").unwrap().unwrap();
    assert_eq!(preferences["theme"], "dark");
    assert_eq!(preferences["notifications"], true);
}

#[tokio::test]
async fn test_session_regeneration() {
    let store = MemorySessionStore::new();
    let manager = SessionManager::with_defaults(store);

    // Create session with sensitive data
    let (old_id, _) = manager.create().await.unwrap();
    manager
        .update(&old_id, |data| {
            data.set("user_id", 42u64)?;
            data.set("role", "user".to_string())?;
            Ok(())
        })
        .await
        .unwrap();

    // Regenerate (e.g., after login to prevent session fixation)
    let new_id = manager.regenerate(&old_id).await.unwrap();

    // Old session should not exist
    assert!(!manager.exists(&old_id).await.unwrap());

    // New session should have same data
    let new_data = manager.load(&new_id).await.unwrap().unwrap();
    let user_id: u64 = new_data.get("user_id").unwrap().unwrap();
    let role: String = new_data.get("role").unwrap().unwrap();
    assert_eq!(user_id, 42);
    assert_eq!(role, "user");
}

#[tokio::test]
async fn test_middleware_integration() {
    let store = MemorySessionStore::new();
    let manager = SessionManager::with_defaults(store);
    let middleware = SessionMiddleware::new(manager);

    // Simulate request without session
    let mut context = middleware.load_or_create(None).await.unwrap();
    let session_id = context.id().clone();

    // Add data to session
    context.set("user_id", 100u64).unwrap();
    context.set("authenticated", true).unwrap();

    // Save session
    middleware.save_if_modified(context).await.unwrap();

    // Simulate next request with session
    let context2 = middleware.load_or_create(Some(&session_id)).await.unwrap();
    let user_id: u64 = context2.get("user_id").unwrap().unwrap();
    let authenticated: bool = context2.get("authenticated").unwrap().unwrap();

    assert_eq!(user_id, 100);
    assert!(authenticated);
}

#[tokio::test]
async fn test_concurrent_session_access() {
    use std::sync::Arc;

    let store = MemorySessionStore::new();
    let manager = Arc::new(SessionManager::with_defaults(store));

    let (session_id, _) = manager.create().await.unwrap();

    // Simulate concurrent access from multiple requests
    let mut handles = vec![];

    for i in 0..10 {
        let manager_clone = Arc::clone(&manager);
        let id_clone = session_id.clone();

        let handle = tokio::spawn(async move {
            manager_clone
                .update(&id_clone, |data| {
                    data.set(format!("key_{}", i), i)?;
                    Ok(())
                })
                .await
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap().unwrap();
    }

    // Verify all updates
    let data = manager.load(&session_id).await.unwrap().unwrap();
    assert_eq!(data.len(), 10);

    for i in 0..10 {
        let value: i32 = data.get(&format!("key_{}", i)).unwrap().unwrap();
        assert_eq!(value, i);
    }
}

#[tokio::test]
async fn test_session_touch() {
    let store = MemorySessionStore::new();
    let manager = SessionManager::with_defaults(store);

    let (session_id, data) = manager.create().await.unwrap();
    let initial_accessed = data.last_accessed();

    // Sleep for more than 1 second to ensure timestamp changes (timestamps are in seconds)
    tokio::time::sleep(Duration::from_millis(1100)).await;

    manager.touch(&session_id).await.unwrap();

    let updated = manager.load(&session_id).await.unwrap().unwrap();
    assert!(updated.last_accessed() > initial_accessed);
}

#[tokio::test]
async fn test_session_clear_data() {
    let store = MemorySessionStore::new();
    let manager = SessionManager::with_defaults(store);

    let (session_id, _) = manager.create().await.unwrap();

    manager
        .update(&session_id, |data| {
            data.set("key1", "value1".to_string())?;
            data.set("key2", "value2".to_string())?;
            Ok(())
        })
        .await
        .unwrap();

    manager
        .update(&session_id, |data| {
            data.clear();
            Ok(())
        })
        .await
        .unwrap();

    let data = manager.load(&session_id).await.unwrap().unwrap();
    assert!(data.is_empty());
}

#[tokio::test]
async fn test_auto_cleanup() {
    let store = MemorySessionStore::new();
    let config = SessionConfig::default()
        .with_ttl(Duration::from_millis(50))
        .with_cleanup_interval(Duration::from_millis(100));
    let manager = SessionManager::new(store, config);

    // Start automatic cleanup
    manager.start_cleanup().await;

    // Create sessions
    for _ in 0..5 {
        manager.create().await.unwrap();
    }

    assert_eq!(manager.count().await.unwrap(), 5);

    // Wait for expiration and automatic cleanup
    tokio::time::sleep(Duration::from_millis(200)).await;

    // Sessions should be cleaned up
    assert_eq!(manager.count().await.unwrap(), 0);

    manager.stop_cleanup().await;
}

#[tokio::test]
async fn test_session_serialization() {
    let mut data = SessionData::new();
    data.set("user_id", 42u64).unwrap();
    data.set("username", "alice".to_string()).unwrap();

    // Serialize
    let json = data.to_json().unwrap();

    // Deserialize
    let deserialized = SessionData::from_json(&json).unwrap();

    let user_id: u64 = deserialized.get("user_id").unwrap().unwrap();
    let username: String = deserialized.get("username").unwrap().unwrap();

    assert_eq!(user_id, 42);
    assert_eq!(username, "alice");
}

#[tokio::test]
async fn test_session_id_validation() {
    use dev_engineeringlabs_rustboot_session::SessionError;

    // Valid UUID v4
    let valid_id = SessionId::generate();
    let result = SessionId::from_string(valid_id.as_str());
    assert!(result.is_ok());

    // Invalid session ID
    let result = SessionId::from_string("not-a-valid-uuid");
    assert!(result.is_err());

    match result {
        Err(SessionError::InvalidSessionId(_)) => {}
        _ => panic!("Expected InvalidSessionId error"),
    }
}

#[tokio::test]
async fn test_middleware_extract_session_id() {
    let store = MemorySessionStore::new();
    let manager = SessionManager::with_defaults(store);
    let middleware = SessionMiddleware::new(manager);

    let session_id = SessionId::generate();
    let cookie_value = session_id.as_str();

    let extracted = middleware.extract_session_id(cookie_value).unwrap();
    assert_eq!(extracted, session_id);
}

#[tokio::test]
async fn test_session_config_builder() {
    let config = SessionConfig::new()
        .with_ttl(Duration::from_secs(7200))
        .with_cookie_name("my_session")
        .with_cookie_domain("example.com")
        .with_cookie_secure(false)
        .with_cleanup_interval(Duration::from_secs(1800));

    assert_eq!(config.default_ttl, Some(Duration::from_secs(7200)));
    assert_eq!(config.cookie_name, "my_session");
    assert_eq!(config.cookie_domain, Some("example.com".to_string()));
    assert!(!config.cookie_secure);
}

