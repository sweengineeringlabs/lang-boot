//! Middleware integration example.

use dev_engineeringlabs_rustboot_session::{
    MemorySessionStore, SessionConfig, SessionManager, SessionMiddleware,
};
use std::time::Duration;

/// Simulate a request with optional session cookie
async fn handle_request(
    middleware: &SessionMiddleware<MemorySessionStore>,
    session_cookie: Option<&str>,
    action: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    println!("  [Request] Action: {}", action);

    // Extract session ID from cookie
    let session_id = session_cookie
        .and_then(|cookie| middleware.extract_session_id(cookie).ok());

    println!("  [Request] Session cookie: {:?}", session_cookie);

    // Load or create session
    let mut context = middleware
        .load_or_create(session_id.as_ref())
        .await?;

    println!("  [Request] Session ID: {}", context.id());

    // Process request based on action
    match action {
        "login" => {
            context.set("user_id", 42u64)?;
            context.set("username", "alice".to_string())?;
            context.set("authenticated", true)?;
            println!("  [Request] User logged in");
        }
        "view_profile" => {
            if let Some(username) = context.get::<String>("username")? {
                println!("  [Request] Viewing profile for: {}", username);
            } else {
                println!("  [Request] Not authenticated");
            }
        }
        "add_to_cart" => {
            let mut cart: Vec<i32> = context.get("cart")?.unwrap_or_default();
            cart.push(123);
            context.set("cart", cart)?;
            println!("  [Request] Added item to cart");
        }
        "logout" => {
            context.clear();
            println!("  [Request] User logged out, session cleared");
        }
        _ => {}
    }

    // Save session if modified
    let final_session_id = middleware.save_if_modified(context).await?;

    println!("  [Request] Session saved: {}", final_session_id);

    Ok(final_session_id.to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    println!("=== Session Middleware Integration Example ===\n");

    // Setup
    let store = MemorySessionStore::new();
    let config = SessionConfig::default()
        .with_ttl(Duration::from_secs(3600))
        .with_cookie_name("session_id");

    let manager = SessionManager::new(store, config);
    let middleware = SessionMiddleware::new(manager);

    // Request 1: Initial visit (no session)
    println!("1. First request - no session cookie:");
    let session_cookie = handle_request(&middleware, None, "view_profile").await?;
    println!();

    // Request 2: Login
    println!("2. Second request - login:");
    let session_cookie = handle_request(
        &middleware,
        Some(&session_cookie),
        "login",
    )
    .await?;
    println!();

    // Request 3: View profile (authenticated)
    println!("3. Third request - view profile (authenticated):");
    handle_request(
        &middleware,
        Some(&session_cookie),
        "view_profile",
    )
    .await?;
    println!();

    // Request 4: Add to cart
    println!("4. Fourth request - add to cart:");
    handle_request(
        &middleware,
        Some(&session_cookie),
        "add_to_cart",
    )
    .await?;
    println!();

    // Request 5: Logout
    println!("5. Fifth request - logout:");
    handle_request(&middleware, Some(&session_cookie), "logout").await?;
    println!();

    // Request 6: After logout
    println!("6. Sixth request - after logout:");
    handle_request(
        &middleware,
        Some(&session_cookie),
        "view_profile",
    )
    .await?;
    println!();

    // Show session statistics
    println!("7. Session statistics:");
    println!("   Total sessions: {}", middleware.manager().count().await?);
    println!();

    println!("=== Example completed successfully ===");

    Ok(())
}
