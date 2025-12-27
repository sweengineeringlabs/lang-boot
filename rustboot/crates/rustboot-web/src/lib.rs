//! Rustboot Web - Web router integration layer
//!
//! This crate provides web router abstractions and integrations with popular web frameworks.
//!
//! # Features
//!
//! - `axum` - Enables the axum integration (enabled by default)
//!
//! # Example
//!
//! ```rust,ignore
//! use rustboot_web::{Router, RouterBuilder};
//!
//! #[tokio::main]
//! async fn main() {
//!     let router = Router::builder()
//!         .route("/", get(handler))
//!         .route("/users/:id", get(get_user))
//!         .build();
//!
//!     router.serve("127.0.0.1:3000").await.unwrap();
//! }
//! ```

pub mod error;
pub mod handler;
pub mod router;
pub mod response;
pub mod extractors;
pub mod middleware_bridge;

#[cfg(feature = "axum")]
pub mod axum_integration;

pub use error::{WebError, WebResult};
pub use handler::{Handler, HandlerContext};
pub use router::{Router, RouterBuilder, RouteMethod};
pub use response::{Response, StatusCode, JsonResponse};
pub use extractors::{Json, Path, Query, Headers};
pub use middleware_bridge::{
    WebMiddleware, MiddlewareChain, RequestLoggingMiddleware,
    RequestTimingMiddleware, CorsMiddleware,
};

#[cfg(feature = "axum")]
pub use axum_integration::{AxumRouterBuilder, IntoAxumRouter};
