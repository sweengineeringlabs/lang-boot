//! Rustboot Cache - Caching abstraction with TTL support

pub mod cache;

pub use cache::{Cache, CacheError, CacheResult, InMemoryCache};
