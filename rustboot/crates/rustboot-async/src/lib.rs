//! Rustboot Async - Runtime utilities and helpers
//!
//! Spawn tasks, join multiple futures, and manage async operations.

pub mod error;
pub mod spawn;
pub mod timeout_pool;

pub use error::{AsyncError, AsyncResult};
pub use spawn::{spawn_blocking_task, spawn_task};
pub use timeout_pool::TimeoutPool;
