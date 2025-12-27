//! Dependency injection (L4: Core).
//!
//! Service locator pattern for dependency management.

pub mod container;

// Re-export main types
pub use container::{Container, Injectable};
