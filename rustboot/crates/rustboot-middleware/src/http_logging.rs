//! HTTP request/response logging middleware (L4: Core - Middleware).
//!
//! Provides comprehensive HTTP request and response logging with configurable
//! options for log levels, request ID tracking, timing, and optional body logging.
//!
//! # Features
//!
//! - **Request Logging**: Logs HTTP method, path, headers, and optionally body
//! - **Response Logging**: Logs status code, headers, timing, and optionally body
//! - **Request ID Tracking**: Generates or extracts request IDs for correlation
//! - **Configurable Log Levels**: Different log levels for requests and responses
//! - **Body Logging**: Optional logging with configurable size limits
//! - **Binary Data Handling**: Gracefully handles non-UTF8 body content
//! - **Performance Timing**: Measures and logs request/response duration
//! - **Status-based Logging**: Automatically adjusts log level based on HTTP status
//!
//! # Example
//!
//! ```rust
//! use dev_engineeringlabs_rustboot_middleware::{
//!     HttpLoggingConfig, HttpLoggingContext, HttpLoggingMiddleware,
//!     HttpLoggingRequest, HttpLogLevel, Pipeline,
//! };
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create custom configuration
//! let config = HttpLoggingConfig::builder()
//!     .request_level(HttpLogLevel::Info)
//!     .response_level(HttpLogLevel::Info)
//!     .log_request_headers(true)
//!     .log_response_headers(true)
//!     .log_request_body(true)
//!     .max_body_size(1024)
//!     .track_request_id(true)
//!     .build();
//!
//! // Create middleware
//! let logging = HttpLoggingMiddleware::with_config(config);
//! let pipeline = Pipeline::new().with_middleware(logging);
//!
//! // Create a request
//! let request = HttpLoggingRequest::new("POST".to_string(), "/api/users".to_string())
//!     .with_header("Content-Type".to_string(), "application/json".to_string())
//!     .with_body(b"{\"name\":\"Alice\"}".to_vec());
//!
//! let ctx = HttpLoggingContext::new(request);
//!
//! // Execute the pipeline
//! let result = pipeline.execute(ctx).await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Configuration Options
//!
//! The middleware can be configured using `HttpLoggingConfig`:
//!
//! - `request_level`: Log level for incoming requests (default: Info)
//! - `response_level`: Log level for outgoing responses (default: Info)
//! - `log_request_headers`: Whether to log request headers (default: true)
//! - `log_response_headers`: Whether to log response headers (default: true)
//! - `log_request_body`: Whether to log request body (default: false)
//! - `log_response_body`: Whether to log response body (default: false)
//! - `max_body_size`: Maximum body size to log in bytes (default: 1024)
//! - `track_request_id`: Enable request ID tracking (default: true)
//! - `request_id_header`: Header name for request ID (default: "X-Request-ID")
//!
//! # Request ID Tracking
//!
//! The middleware can either extract request IDs from incoming headers or generate
//! new UUIDs for requests that don't have one. This is useful for request correlation
//! across distributed systems.
//!
//! # Performance
//!
//! The middleware measures the time taken to process each request and includes it
//! in the response log. This helps identify slow endpoints and performance issues.
//!
//! # Status-based Logging
//!
//! Response logging automatically adjusts based on HTTP status:
//! - 5xx errors: Logged at ERROR level regardless of configuration
//! - 4xx errors: Logged at WARN level regardless of configuration
//! - 2xx/3xx: Logged at configured response_level

use super::traits::{Middleware, MiddlewareResult, Next};
use std::collections::HashMap;
use std::pin::Pin;
use std::time::Instant;

/// Log level for HTTP logging.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpLogLevel {
    /// Log errors only.
    Error,
    /// Log warnings and errors.
    Warn,
    /// Log info, warnings, and errors.
    Info,
    /// Log debug info and above.
    Debug,
    /// Log everything including trace.
    Trace,
}

impl HttpLogLevel {
    /// Convert to tracing level.
    fn as_tracing_level(&self) -> tracing::Level {
        match self {
            HttpLogLevel::Error => tracing::Level::ERROR,
            HttpLogLevel::Warn => tracing::Level::WARN,
            HttpLogLevel::Info => tracing::Level::INFO,
            HttpLogLevel::Debug => tracing::Level::DEBUG,
            HttpLogLevel::Trace => tracing::Level::TRACE,
        }
    }
}

/// Configuration for HTTP logging middleware.
#[derive(Debug, Clone)]
pub struct HttpLoggingConfig {
    /// Log level for requests.
    pub request_level: HttpLogLevel,
    /// Log level for responses.
    pub response_level: HttpLogLevel,
    /// Whether to log request headers.
    pub log_request_headers: bool,
    /// Whether to log response headers.
    pub log_response_headers: bool,
    /// Whether to log request body (with size limit).
    pub log_request_body: bool,
    /// Whether to log response body (with size limit).
    pub log_response_body: bool,
    /// Maximum body size to log (in bytes).
    pub max_body_size: usize,
    /// Whether to generate and track request IDs.
    pub track_request_id: bool,
    /// Custom request ID header name (default: "X-Request-ID").
    pub request_id_header: String,
}

impl Default for HttpLoggingConfig {
    fn default() -> Self {
        Self {
            request_level: HttpLogLevel::Info,
            response_level: HttpLogLevel::Info,
            log_request_headers: true,
            log_response_headers: true,
            log_request_body: false,
            log_response_body: false,
            max_body_size: 1024, // 1KB
            track_request_id: true,
            request_id_header: "X-Request-ID".to_string(),
        }
    }
}

impl HttpLoggingConfig {
    /// Create a new builder for HttpLoggingConfig.
    pub fn builder() -> HttpLoggingConfigBuilder {
        HttpLoggingConfigBuilder::default()
    }
}

/// Builder for HttpLoggingConfig.
#[derive(Debug, Default)]
pub struct HttpLoggingConfigBuilder {
    request_level: Option<HttpLogLevel>,
    response_level: Option<HttpLogLevel>,
    log_request_headers: Option<bool>,
    log_response_headers: Option<bool>,
    log_request_body: Option<bool>,
    log_response_body: Option<bool>,
    max_body_size: Option<usize>,
    track_request_id: Option<bool>,
    request_id_header: Option<String>,
}

impl HttpLoggingConfigBuilder {
    /// Set request log level.
    pub fn request_level(mut self, level: HttpLogLevel) -> Self {
        self.request_level = Some(level);
        self
    }

    /// Set response log level.
    pub fn response_level(mut self, level: HttpLogLevel) -> Self {
        self.response_level = Some(level);
        self
    }

    /// Set whether to log request headers.
    pub fn log_request_headers(mut self, log: bool) -> Self {
        self.log_request_headers = Some(log);
        self
    }

    /// Set whether to log response headers.
    pub fn log_response_headers(mut self, log: bool) -> Self {
        self.log_response_headers = Some(log);
        self
    }

    /// Set whether to log request body.
    pub fn log_request_body(mut self, log: bool) -> Self {
        self.log_request_body = Some(log);
        self
    }

    /// Set whether to log response body.
    pub fn log_response_body(mut self, log: bool) -> Self {
        self.log_response_body = Some(log);
        self
    }

    /// Set maximum body size to log.
    pub fn max_body_size(mut self, size: usize) -> Self {
        self.max_body_size = Some(size);
        self
    }

    /// Set whether to track request IDs.
    pub fn track_request_id(mut self, track: bool) -> Self {
        self.track_request_id = Some(track);
        self
    }

    /// Set custom request ID header name.
    pub fn request_id_header(mut self, header: String) -> Self {
        self.request_id_header = Some(header);
        self
    }

    /// Build the configuration.
    pub fn build(self) -> HttpLoggingConfig {
        let defaults = HttpLoggingConfig::default();
        HttpLoggingConfig {
            request_level: self.request_level.unwrap_or(defaults.request_level),
            response_level: self.response_level.unwrap_or(defaults.response_level),
            log_request_headers: self.log_request_headers.unwrap_or(defaults.log_request_headers),
            log_response_headers: self.log_response_headers.unwrap_or(defaults.log_response_headers),
            log_request_body: self.log_request_body.unwrap_or(defaults.log_request_body),
            log_response_body: self.log_response_body.unwrap_or(defaults.log_response_body),
            max_body_size: self.max_body_size.unwrap_or(defaults.max_body_size),
            track_request_id: self.track_request_id.unwrap_or(defaults.track_request_id),
            request_id_header: self.request_id_header.unwrap_or(defaults.request_id_header),
        }
    }
}

/// HTTP request context for logging middleware.
#[derive(Debug, Clone)]
pub struct HttpLoggingRequest {
    /// HTTP method (GET, POST, etc.).
    pub method: String,
    /// Request path/URL.
    pub path: String,
    /// Request headers.
    pub headers: HashMap<String, String>,
    /// Request body (optional).
    pub body: Option<Vec<u8>>,
    /// Request ID (generated or from header).
    pub request_id: Option<String>,
}

impl HttpLoggingRequest {
    /// Create a new HTTP request for logging.
    pub fn new(method: String, path: String) -> Self {
        Self {
            method,
            path,
            headers: HashMap::new(),
            body: None,
            request_id: None,
        }
    }

    /// Add a header.
    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }

    /// Set the body.
    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }

    /// Set the request ID.
    pub fn with_request_id(mut self, id: String) -> Self {
        self.request_id = Some(id);
        self
    }

    /// Get body as string (truncated if too long).
    pub fn body_preview(&self, max_size: usize) -> Option<String> {
        self.body.as_ref().and_then(|body| {
            if body.is_empty() {
                return None;
            }
            let preview_size = body.len().min(max_size);
            let preview = &body[..preview_size];
            match String::from_utf8(preview.to_vec()) {
                Ok(s) => {
                    if body.len() > max_size {
                        Some(format!("{}... ({} bytes total)", s, body.len()))
                    } else {
                        Some(s)
                    }
                }
                Err(_) => Some(format!("<binary data, {} bytes>", body.len())),
            }
        })
    }
}

/// HTTP response context for logging middleware.
#[derive(Debug, Clone)]
pub struct HttpLoggingResponse {
    /// HTTP status code.
    pub status: u16,
    /// Response headers.
    pub headers: HashMap<String, String>,
    /// Response body (optional).
    pub body: Option<Vec<u8>>,
}

impl HttpLoggingResponse {
    /// Create a new HTTP response for logging.
    pub fn new(status: u16) -> Self {
        Self {
            status,
            headers: HashMap::new(),
            body: None,
        }
    }

    /// Add a header.
    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }

    /// Set the body.
    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }

    /// Get body as string (truncated if too long).
    pub fn body_preview(&self, max_size: usize) -> Option<String> {
        self.body.as_ref().and_then(|body| {
            if body.is_empty() {
                return None;
            }
            let preview_size = body.len().min(max_size);
            let preview = &body[..preview_size];
            match String::from_utf8(preview.to_vec()) {
                Ok(s) => {
                    if body.len() > max_size {
                        Some(format!("{}... ({} bytes total)", s, body.len()))
                    } else {
                        Some(s)
                    }
                }
                Err(_) => Some(format!("<binary data, {} bytes>", body.len())),
            }
        })
    }

    /// Check if response is successful (2xx).
    pub fn is_success(&self) -> bool {
        self.status >= 200 && self.status < 300
    }
}

/// HTTP context containing both request and response for logging.
#[derive(Debug, Clone)]
pub struct HttpLoggingContext {
    /// HTTP request.
    pub request: HttpLoggingRequest,
    /// HTTP response (populated after processing).
    pub response: Option<HttpLoggingResponse>,
}

impl HttpLoggingContext {
    /// Create a new HTTP logging context with just a request.
    pub fn new(request: HttpLoggingRequest) -> Self {
        Self {
            request,
            response: None,
        }
    }

    /// Set the response.
    pub fn with_response(mut self, response: HttpLoggingResponse) -> Self {
        self.response = Some(response);
        self
    }
}

/// HTTP request/response logging middleware.
pub struct HttpLoggingMiddleware {
    config: HttpLoggingConfig,
}

impl HttpLoggingMiddleware {
    /// Create a new HTTP logging middleware with default configuration.
    pub fn new() -> Self {
        Self {
            config: HttpLoggingConfig::default(),
        }
    }

    /// Create a new HTTP logging middleware with custom configuration.
    pub fn with_config(config: HttpLoggingConfig) -> Self {
        Self { config }
    }

    /// Generate a new request ID.
    fn generate_request_id() -> String {
        uuid::Uuid::new_v4().to_string()
    }

    /// Log the HTTP request.
    fn log_request(&self, ctx: &HttpLoggingContext) {
        let req = &ctx.request;
        let level = self.config.request_level.as_tracing_level();

        let request_id = req.request_id.as_deref().unwrap_or("<none>");

        // Build log message
        let mut parts = vec![
            format!("method={}", req.method),
            format!("path={}", req.path),
            format!("request_id={}", request_id),
        ];

        if self.config.log_request_headers && !req.headers.is_empty() {
            let headers: Vec<String> = req
                .headers
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            parts.push(format!("headers=[{}]", headers.join(", ")));
        }

        if self.config.log_request_body {
            if let Some(body) = req.body_preview(self.config.max_body_size) {
                parts.push(format!("body={}", body));
            }
        }

        let message = format!("HTTP Request: {}", parts.join(" | "));

        match level {
            tracing::Level::ERROR => tracing::error!("{}", message),
            tracing::Level::WARN => tracing::warn!("{}", message),
            tracing::Level::INFO => tracing::info!("{}", message),
            tracing::Level::DEBUG => tracing::debug!("{}", message),
            tracing::Level::TRACE => tracing::trace!("{}", message),
        }
    }

    /// Log the HTTP response.
    fn log_response(&self, ctx: &HttpLoggingContext, duration: std::time::Duration) {
        let req = &ctx.request;
        let request_id = req.request_id.as_deref().unwrap_or("<none>");

        if let Some(resp) = &ctx.response {
            let level = self.config.response_level.as_tracing_level();

            // Build log message
            let mut parts = vec![
                format!("method={}", req.method),
                format!("path={}", req.path),
                format!("status={}", resp.status),
                format!("duration={:?}", duration),
                format!("request_id={}", request_id),
            ];

            if self.config.log_response_headers && !resp.headers.is_empty() {
                let headers: Vec<String> = resp
                    .headers
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect();
                parts.push(format!("headers=[{}]", headers.join(", ")));
            }

            if self.config.log_response_body {
                if let Some(body) = resp.body_preview(self.config.max_body_size) {
                    parts.push(format!("body={}", body));
                }
            }

            let message = format!("HTTP Response: {}", parts.join(" | "));

            // Use different log levels based on status code
            if resp.status >= 500 {
                tracing::error!("{}", message);
            } else if resp.status >= 400 {
                tracing::warn!("{}", message);
            } else {
                match level {
                    tracing::Level::ERROR => tracing::error!("{}", message),
                    tracing::Level::WARN => tracing::warn!("{}", message),
                    tracing::Level::INFO => tracing::info!("{}", message),
                    tracing::Level::DEBUG => tracing::debug!("{}", message),
                    tracing::Level::TRACE => tracing::trace!("{}", message),
                }
            }
        }
    }
}

impl Default for HttpLoggingMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

impl Middleware<HttpLoggingContext> for HttpLoggingMiddleware {
    fn handle(
        &self,
        mut ctx: HttpLoggingContext,
        next: Next<HttpLoggingContext>,
    ) -> Pin<Box<dyn std::future::Future<Output = MiddlewareResult<HttpLoggingContext>> + Send>> {
        let config = self.config.clone();

        Box::pin(async move {
            // Generate or extract request ID
            if config.track_request_id && ctx.request.request_id.is_none() {
                let request_id = ctx
                    .request
                    .headers
                    .get(&config.request_id_header)
                    .cloned()
                    .unwrap_or_else(Self::generate_request_id);
                ctx.request.request_id = Some(request_id);
            }

            // Log the request
            let middleware = Self::with_config(config.clone());
            middleware.log_request(&ctx);

            // Execute the rest of the pipeline and measure time
            let start = Instant::now();
            let result = next(ctx).await;
            let duration = start.elapsed();

            // Log the response
            match &result {
                Ok(ctx) => {
                    middleware.log_response(ctx, duration);
                }
                Err(e) => {
                    tracing::error!(
                        "HTTP Request failed: error={} duration={:?}",
                        e,
                        duration
                    );
                }
            }

            result
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chain::Pipeline;

    #[tokio::test]
    async fn test_http_logging_middleware() {
        let middleware = HttpLoggingMiddleware::new();
        let pipeline = Pipeline::new().with_middleware(middleware);

        let request = HttpLoggingRequest::new("GET".to_string(), "/api/users".to_string())
            .with_header("Content-Type".to_string(), "application/json".to_string());

        let ctx = HttpLoggingContext::new(request);
        let result = pipeline.execute(ctx).await;

        assert!(result.is_ok());
        let ctx = result.unwrap();
        assert!(ctx.request.request_id.is_some());
    }

    #[tokio::test]
    async fn test_http_logging_with_response() {
        let middleware = HttpLoggingMiddleware::new();
        let pipeline = Pipeline::new().with_middleware(middleware);

        let request = HttpLoggingRequest::new("POST".to_string(), "/api/users".to_string())
            .with_body(b"{\"name\":\"John\"}".to_vec());

        let mut ctx = HttpLoggingContext::new(request);

        // Simulate adding a response in the pipeline
        let result = pipeline.execute(ctx.clone()).await;
        assert!(result.is_ok());

        ctx = result.unwrap();
        ctx = ctx.with_response(
            HttpLoggingResponse::new(201)
                .with_header("Content-Type".to_string(), "application/json".to_string())
                .with_body(b"{\"id\":1,\"name\":\"John\"}".to_vec()),
        );

        assert!(ctx.response.is_some());
        assert_eq!(ctx.response.as_ref().unwrap().status, 201);
    }

    #[tokio::test]
    async fn test_custom_config() {
        let config = HttpLoggingConfig::builder()
            .request_level(HttpLogLevel::Debug)
            .response_level(HttpLogLevel::Info)
            .log_request_body(true)
            .log_response_body(true)
            .max_body_size(512)
            .build();

        let middleware = HttpLoggingMiddleware::with_config(config);
        let pipeline = Pipeline::new().with_middleware(middleware);

        let request = HttpLoggingRequest::new("GET".to_string(), "/api/data".to_string());
        let ctx = HttpLoggingContext::new(request);

        let result = pipeline.execute(ctx).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_request_id_from_header() {
        let config = HttpLoggingConfig::builder()
            .track_request_id(true)
            .request_id_header("X-Request-ID".to_string())
            .build();

        let middleware = HttpLoggingMiddleware::with_config(config);
        let pipeline = Pipeline::new().with_middleware(middleware);

        let custom_id = "custom-request-id-123";
        let request = HttpLoggingRequest::new("GET".to_string(), "/test".to_string())
            .with_header("X-Request-ID".to_string(), custom_id.to_string());

        let ctx = HttpLoggingContext::new(request);
        let result = pipeline.execute(ctx).await;

        assert!(result.is_ok());
        let ctx = result.unwrap();
        assert_eq!(ctx.request.request_id.as_deref(), Some(custom_id));
    }

    #[tokio::test]
    async fn test_body_preview_truncation() {
        let large_body = vec![b'A'; 2000];
        let request = HttpLoggingRequest::new("POST".to_string(), "/upload".to_string())
            .with_body(large_body);

        let preview = request.body_preview(100);
        assert!(preview.is_some());
        assert!(preview.unwrap().contains("2000 bytes total"));
    }

    #[tokio::test]
    async fn test_response_success_check() {
        let response_200 = HttpLoggingResponse::new(200);
        assert!(response_200.is_success());

        let response_404 = HttpLoggingResponse::new(404);
        assert!(!response_404.is_success());

        let response_500 = HttpLoggingResponse::new(500);
        assert!(!response_500.is_success());
    }

    #[test]
    fn test_config_builder() {
        let config = HttpLoggingConfig::builder()
            .request_level(HttpLogLevel::Trace)
            .response_level(HttpLogLevel::Debug)
            .log_request_headers(false)
            .log_response_headers(true)
            .log_request_body(true)
            .log_response_body(false)
            .max_body_size(2048)
            .track_request_id(false)
            .request_id_header("X-Custom-ID".to_string())
            .build();

        assert_eq!(config.request_level, HttpLogLevel::Trace);
        assert_eq!(config.response_level, HttpLogLevel::Debug);
        assert!(!config.log_request_headers);
        assert!(config.log_response_headers);
        assert!(config.log_request_body);
        assert!(!config.log_response_body);
        assert_eq!(config.max_body_size, 2048);
        assert!(!config.track_request_id);
        assert_eq!(config.request_id_header, "X-Custom-ID");
    }
}
