//! Rustboot HTTP - HTTP client abstraction
//!
//! This crate provides HTTP client abstractions and implementations.
//!
//! # Features
//!
//! - `reqwest` - Enables the `ReqwestClient` implementation using the reqwest library
//!
//! # Example
//!
//! ```ignore
//! use rustboot_http::{HttpClient, ReqwestClient};
//!
//! let client = ReqwestClient::new();
//! let response = client.get("https://httpbin.org/get").await?;
//! println!("Status: {}", response.status);
//! ```

pub mod client;

#[cfg(feature = "reqwest")]
pub mod reqwest_client;

pub use client::{HttpClient, HttpError, HttpResult, Method, Request, Response};

#[cfg(feature = "reqwest")]
pub use reqwest_client::{ReqwestClient, ReqwestClientBuilder};
