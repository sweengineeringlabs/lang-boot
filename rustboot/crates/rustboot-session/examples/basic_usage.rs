//! Basic session management example.

use dev_engineeringlabs_rustboot_session::{
    MemorySessionStore, SessionConfig, SessionManager,
};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("=== Basic Session Management Example ===\n");

    // Create a session manager with in-memory store
    let store = MemorySessionStore::new();
    let config = SessionConfig::default()
        .with_ttl(Duration::from_secs(3600)) // 1 hour
        .with_cookie_name("my_session")
        .with_cookie_secure(true);

    let manager = SessionManager::new(store, config);

    // Create a new session
    println!("1. Creating a new session...");
    let (session_id, data) = manager.create().await?;
    println!("   Session ID: {}", session_id);
    println!("   Created at: {}", data.created_at());
    println!();

    // Add data to the session
    println!("2. Adding data to session...");
    manager
        .update(&session_id, |data| {
            data.set("user_id", 42u64)?;
            data.set("username", "alice".to_string())?;
            data.set("email", "alice@example.com".to_string())?;
            data.set("roles", vec!["user", "admin"])?;
            Ok(())
        })
        .await?;
    println!("   Data added successfully");
    println!();

    // Load and display session data
    println!("3. Loading session data...");
    let loaded = manager.load(&session_id).await?.unwrap();
    let user_id: u64 = loaded.get("user_id")?.unwrap();
    let username: String = loaded.get("username")?.unwrap();
    let email: String = loaded.get("email")?.unwrap();
    let roles: Vec<String> = loaded.get("roles")?.unwrap();

    println!("   User ID: {}", user_id);
    println!("   Username: {}", username);
    println!("   Email: {}", email);
    println!("   Roles: {:?}", roles);
    println!("   Last accessed: {}", loaded.last_accessed());
    println!();

    // Update session data
    println!("4. Updating session data...");
    manager
        .update(&session_id, |data| {
            data.set("last_action", "viewed_dashboard".to_string())?;
            Ok(())
        })
        .await?;
    println!("   Session updated");
    println!();

    // Touch the session (update last_accessed)
    println!("5. Touching session...");
    tokio::time::sleep(Duration::from_millis(100)).await;
    manager.touch(&session_id).await?;
    let touched = manager.load(&session_id).await?.unwrap();
    println!("   Last accessed updated: {}", touched.last_accessed());
    println!();

    // Regenerate session ID (security feature)
    println!("6. Regenerating session ID (security)...");
    let new_session_id = manager.regenerate(&session_id).await?;
    println!("   Old ID: {}", session_id);
    println!("   New ID: {}", new_session_id);
    println!("   Old session exists: {}", manager.exists(&session_id).await?);
    println!("   New session exists: {}", manager.exists(&new_session_id).await?);
    println!();

    // Get session count
    println!("7. Session statistics...");
    let count = manager.count().await?;
    println!("   Total sessions: {}", count);
    println!();

    // Delete the session
    println!("8. Deleting session...");
    manager.delete(&new_session_id).await?;
    println!("   Session deleted");
    println!("   Session exists: {}", manager.exists(&new_session_id).await?);
    println!();

    println!("=== Example completed successfully ===");

    Ok(())
}
