//! Caching framework (L4: Core).
//!
//! Trait-based caching with TTL support and multiple backends.

pub mod cache;

// Re-export main types
pub use cache::{Cache, CacheError, CacheResult, InMemoryCache};
