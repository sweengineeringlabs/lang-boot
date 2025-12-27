//! Rustboot Session - Session management abstraction
//!
//! This crate provides a comprehensive session management system with:
//!
//! - **Session trait**: Generic abstraction for session storage
//! - **Multiple backends**: In-memory, Redis, and database storage
//! - **Session lifecycle**: Creation, loading, updating, and deletion
//! - **Automatic expiration**: TTL-based session expiration
//! - **Middleware integration**: Easy integration with web frameworks
//! - **Security features**: Session ID regeneration, secure cookies
//!
//! # Examples
//!
//! ## Basic Usage with In-Memory Store
//!
//! ```rust,ignore
//! use rustboot_session::{SessionManager, MemorySessionStore, SessionConfig};
//! use std::time::Duration;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create a session manager with in-memory store
//! let store = MemorySessionStore::new();
//! let config = SessionConfig::default()
//!     .with_ttl(Duration::from_secs(3600));
//! let manager = SessionManager::new(store, config);
//!
//! // Create a new session
//! let (session_id, mut data) = manager.create().await?;
//!
//! // Update the session
//! manager.update(&session_id, |data| {
//!     data.set("user_id", 42u64)?;
//!     data.set("username", "alice".to_string())?;
//!     Ok(())
//! }).await?;
//!
//! // Load the session
//! let loaded = manager.load(&session_id).await?.unwrap();
//! let user_id: u64 = loaded.get("user_id")?.unwrap();
//! assert_eq!(user_id, 42);
//!
//! // Delete the session
//! manager.delete(&session_id).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Session with Middleware
//!
//! ```rust,ignore
//! use rustboot_session::{SessionManager, SessionMiddleware, MemorySessionStore};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let store = MemorySessionStore::new();
//! let manager = SessionManager::with_defaults(store);
//! let middleware = SessionMiddleware::new(manager);
//!
//! // In a request handler:
//! let mut context = middleware.load_or_create(None).await?;
//! context.set("cart_items", vec![1, 2, 3])?;
//! middleware.save_if_modified(context).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Redis Store (requires `redis-store` feature)
//!
//! ```rust,ignore
//! use rustboot_session::{SessionManager, RedisSessionStore};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let store = RedisSessionStore::with_defaults("redis://127.0.0.1:6379").await?;
//! let manager = SessionManager::with_defaults(store);
//!
//! let (session_id, _) = manager.create().await?;
//! # Ok(())
//! # }
//! ```

pub mod database_store;
pub mod error;
pub mod manager;
pub mod memory_store;
pub mod middleware;
pub mod redis_store;
pub mod session_data;
pub mod session_id;
pub mod store;

// Re-exports
pub use database_store::{DatabaseSessionStore, SessionDatabase};
pub use error::{SessionError, SessionResult};
pub use manager::SessionManager;
pub use memory_store::MemorySessionStore;
pub use middleware::{SessionContext, SessionMiddleware};
pub use session_data::SessionData;
pub use session_id::SessionId;
pub use store::{SameSite, SessionConfig, SessionStore};

#[cfg(feature = "redis-store")]
pub use redis_store::RedisSessionStore;
