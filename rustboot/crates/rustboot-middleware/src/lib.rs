//! Rustboot Middleware - Middleware pipeline pattern

pub mod built_in;
pub mod chain;
pub mod cors;
pub mod http;
pub mod http_context;
pub mod http_logging;
pub mod security;
pub mod traits;

pub use built_in::{LoggingMiddleware, MetricsMiddleware, TimingMiddleware};
pub use chain::Pipeline;
pub use cors::{CorsConfig, CorsMiddleware, OriginConfig};
pub use http_context::HttpContext;
pub use http_logging::{
    HttpLoggingConfig, HttpLoggingConfigBuilder, HttpLoggingContext, HttpLoggingMiddleware,
    HttpLoggingRequest, HttpLoggingResponse, HttpLogLevel,
};
pub use security::{HasHeaders, SecurityHeadersConfig, SecurityHeadersMiddleware};
pub use traits::{Middleware, MiddlewareError, MiddlewareResult};

#[cfg(feature = "ratelimit")]
pub use http::ratelimit::{
    CustomKeyExtractor, HeaderKeyExtractor, IpKeyExtractor, KeyExtractor, RateLimitConfig,
    RateLimitMiddleware,
};
