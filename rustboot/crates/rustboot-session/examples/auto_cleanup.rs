//! Automatic session cleanup example.

use dev_engineeringlabs_rustboot_session::{
    MemorySessionStore, SessionConfig, SessionManager,
};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    println!("=== Automatic Session Cleanup Example ===\n");

    // Create a session manager with short TTL and cleanup interval
    let store = MemorySessionStore::new();
    let config = SessionConfig::default()
        .with_ttl(Duration::from_secs(2)) // Sessions expire after 2 seconds
        .with_cleanup_interval(Duration::from_secs(1)); // Cleanup runs every second

    let manager = SessionManager::new(store, config);

    // Start automatic cleanup
    println!("1. Starting automatic cleanup...");
    manager.start_cleanup().await;
    println!("   Cleanup task started\n");

    // Create some sessions
    println!("2. Creating 5 sessions...");
    let mut session_ids = Vec::new();
    for i in 0..5 {
        let (id, _) = manager.create().await?;
        manager
            .update(&id, |data| {
                data.set("index", i)?;
                Ok(())
            })
            .await?;
        session_ids.push(id);
        println!("   Created session {}: {}", i + 1, session_ids[i]);
    }
    println!("   Total sessions: {}\n", manager.count().await?);

    // Wait a bit
    println!("3. Waiting 1 second (sessions still valid)...");
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("   Total sessions: {}\n", manager.count().await?);

    // Wait for expiration
    println!("4. Waiting 2 more seconds (sessions should expire)...");
    tokio::time::sleep(Duration::from_secs(2)).await;
    println!("   Total sessions: {}", manager.count().await?);
    println!("   Cleanup automatically removed expired sessions\n");

    // Create new sessions
    println!("5. Creating 3 new sessions...");
    for i in 0..3 {
        let (id, _) = manager.create().await?;
        println!("   Created session {}: {}", i + 1, id);
    }
    println!("   Total sessions: {}\n", manager.count().await?);

    // Stop cleanup
    println!("6. Stopping automatic cleanup...");
    manager.stop_cleanup().await;
    println!("   Cleanup task stopped\n");

    // Wait for expiration
    println!("7. Waiting for sessions to expire (cleanup is disabled)...");
    tokio::time::sleep(Duration::from_secs(3)).await;
    println!("   Total sessions (includes expired): {}", manager.count().await?);
    println!("   Note: Expired sessions not removed because cleanup is disabled\n");

    // Manual cleanup
    println!("8. Running manual cleanup...");
    let removed = manager.cleanup_expired().await?;
    println!("   Removed {} expired sessions", removed);
    println!("   Total sessions: {}\n", manager.count().await?);

    println!("=== Example completed successfully ===");

    Ok(())
}
