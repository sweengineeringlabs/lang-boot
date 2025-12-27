//! HTTP client abstraction (L4: Core).
//!
//! Generic HTTP client for making requests.

pub mod client;

// Re-export main types
pub use client::{HttpClient, HttpError, HttpResult, Method, Request, Response};
