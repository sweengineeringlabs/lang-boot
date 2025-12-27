//! Rustboot UUID - UUID generation and utilities

pub mod error;
pub mod generate;

pub use error::{UuidError, UuidResult};
pub use generate::{new_v4, new_v7, parse_uuid};
// Re-export uuid type
pub use uuid::Uuid;
