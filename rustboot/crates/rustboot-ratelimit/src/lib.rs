//! Rustboot RateLimit - Rate limiting algorithms
//!
//! Token bucket, leaky bucket, and window-based rate limiting.

pub mod error;
pub mod leaky_bucket;
pub mod token_bucket;
pub mod window;

pub use error::{RateLimitError, RateLimitResult};
pub use leaky_bucket::LeakyBucket;
pub use token_bucket::TokenBucket;
pub use window::{FixedWindow, SlidingWindow};
