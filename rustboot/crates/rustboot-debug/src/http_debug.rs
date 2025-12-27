//! HTTP request/response dumping middleware for debugging.

use rustboot_middleware::{
    HttpContext, Middleware, MiddlewareResult,
    traits::Next,
};
use std::pin::Pin;
use tracing::{debug, info};

/// Configuration for HTTP dump middleware.
#[derive(Debug, Clone)]
pub struct HttpDumpConfig {
    /// Whether to dump request headers (default: true).
    pub dump_request_headers: bool,
    /// Whether to dump request body (default: true).
    pub dump_request_body: bool,
    /// Whether to dump response headers (default: true).
    pub dump_response_headers: bool,
    /// Whether to dump response body (default: true).
    pub dump_response_body: bool,
    /// Maximum body size to dump in bytes (default: 4096).
    pub max_body_size: usize,
    /// Whether to pretty-print JSON bodies (default: true).
    pub pretty_json: bool,
    /// Whether to use info level instead of debug (default: false).
    pub use_info_level: bool,
}

impl Default for HttpDumpConfig {
    fn default() -> Self {
        Self {
            dump_request_headers: true,
            dump_request_body: true,
            dump_response_headers: true,
            dump_response_body: true,
            max_body_size: 4096,
            pretty_json: true,
            use_info_level: false,
        }
    }
}

impl HttpDumpConfig {
    /// Create a new config with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set whether to dump request headers.
    pub fn with_request_headers(mut self, enabled: bool) -> Self {
        self.dump_request_headers = enabled;
        self
    }

    /// Set whether to dump request body.
    pub fn with_request_body(mut self, enabled: bool) -> Self {
        self.dump_request_body = enabled;
        self
    }

    /// Set whether to dump response headers.
    pub fn with_response_headers(mut self, enabled: bool) -> Self {
        self.dump_response_headers = enabled;
        self
    }

    /// Set whether to dump response body.
    pub fn with_response_body(mut self, enabled: bool) -> Self {
        self.dump_response_body = enabled;
        self
    }

    /// Set maximum body size to dump.
    pub fn with_max_body_size(mut self, size: usize) -> Self {
        self.max_body_size = size;
        self
    }

    /// Set whether to pretty-print JSON.
    pub fn with_pretty_json(mut self, enabled: bool) -> Self {
        self.pretty_json = enabled;
        self
    }

    /// Use info level logging instead of debug.
    pub fn with_info_level(mut self, enabled: bool) -> Self {
        self.use_info_level = enabled;
        self
    }
}

/// HTTP dump middleware - logs all requests and responses.
pub struct HttpDumpMiddleware {
    config: HttpDumpConfig,
}

impl HttpDumpMiddleware {
    /// Create a new HTTP dump middleware with default config.
    pub fn new() -> Self {
        Self {
            config: HttpDumpConfig::default(),
        }
    }

    /// Create with custom config.
    pub fn with_config(config: HttpDumpConfig) -> Self {
        Self { config }
    }

    /// Format body for display.
    fn format_body(&self, body: &[u8]) -> String {
        if body.is_empty() {
            return "(empty)".to_string();
        }

        let truncated = if body.len() > self.config.max_body_size {
            &body[..self.config.max_body_size]
        } else {
            body
        };

        match String::from_utf8(truncated.to_vec()) {
            Ok(text) => {
                // Try to parse as JSON if pretty-printing is enabled
                if self.config.pretty_json {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
                        if let Ok(pretty) = serde_json::to_string_pretty(&json) {
                            return if body.len() > self.config.max_body_size {
                                format!("{}\n... (truncated)", pretty)
                            } else {
                                pretty
                            };
                        }
                    }
                }

                if body.len() > self.config.max_body_size {
                    format!("{}... (truncated)", text)
                } else {
                    text
                }
            }
            Err(_) => {
                // Binary data
                format!("(binary data, {} bytes)", body.len())
            }
        }
    }

    /// Log the request.
    fn log_request(&self, ctx: &HttpContext) {
        let method = &ctx.method;
        let url = &ctx.url;

        if self.config.use_info_level {
            info!(
                target: "rustboot::debug::http",
                method = %method,
                url = %url,
                "HTTP Request"
            );

            if self.config.dump_request_headers && !ctx.headers.is_empty() {
                info!(
                    target: "rustboot::debug::http",
                    "Request Headers:"
                );
                for (key, value) in &ctx.headers {
                    info!(
                        target: "rustboot::debug::http",
                        "  {}: {}", key, value
                    );
                }
            }

            if self.config.dump_request_body {
                if let Some(body) = &ctx.body {
                    let formatted = self.format_body(body);
                    info!(
                        target: "rustboot::debug::http",
                        "Request Body:\n{}", formatted
                    );
                }
            }

            if let Some(client_ip) = &ctx.client_ip {
                info!(
                    target: "rustboot::debug::http",
                    client_ip = %client_ip,
                    "Client IP"
                );
            }
        } else {
            debug!(
                target: "rustboot::debug::http",
                method = %method,
                url = %url,
                "HTTP Request"
            );

            if self.config.dump_request_headers && !ctx.headers.is_empty() {
                debug!(
                    target: "rustboot::debug::http",
                    "Request Headers:"
                );
                for (key, value) in &ctx.headers {
                    debug!(
                        target: "rustboot::debug::http",
                        "  {}: {}", key, value
                    );
                }
            }

            if self.config.dump_request_body {
                if let Some(body) = &ctx.body {
                    let formatted = self.format_body(body);
                    debug!(
                        target: "rustboot::debug::http",
                        "Request Body:\n{}", formatted
                    );
                }
            }

            if let Some(client_ip) = &ctx.client_ip {
                debug!(
                    target: "rustboot::debug::http",
                    client_ip = %client_ip,
                    "Client IP"
                );
            }
        }
    }

    /// Log the response.
    fn log_response(&self, ctx: &HttpContext) {
        if self.config.use_info_level {
            if let Some(status) = ctx.status {
                info!(
                    target: "rustboot::debug::http",
                    status = status,
                    "HTTP Response"
                );
            }

            if self.config.dump_response_headers && !ctx.response_headers.is_empty() {
                info!(
                    target: "rustboot::debug::http",
                    "Response Headers:"
                );
                for (key, value) in &ctx.response_headers {
                    info!(
                        target: "rustboot::debug::http",
                        "  {}: {}", key, value
                    );
                }
            }

            if self.config.dump_response_body {
                if let Some(body) = &ctx.response_body {
                    let formatted = self.format_body(body);
                    info!(
                        target: "rustboot::debug::http",
                        "Response Body:\n{}", formatted
                    );
                }
            }
        } else {
            if let Some(status) = ctx.status {
                debug!(
                    target: "rustboot::debug::http",
                    status = status,
                    "HTTP Response"
                );
            }

            if self.config.dump_response_headers && !ctx.response_headers.is_empty() {
                debug!(
                    target: "rustboot::debug::http",
                    "Response Headers:"
                );
                for (key, value) in &ctx.response_headers {
                    debug!(
                        target: "rustboot::debug::http",
                        "  {}: {}", key, value
                    );
                }
            }

            if self.config.dump_response_body {
                if let Some(body) = &ctx.response_body {
                    let formatted = self.format_body(body);
                    debug!(
                        target: "rustboot::debug::http",
                        "Response Body:\n{}", formatted
                    );
                }
            }
        }
    }
}

impl Default for HttpDumpMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

impl Middleware<HttpContext> for HttpDumpMiddleware {
    fn handle(
        &self,
        ctx: HttpContext,
        next: Next<HttpContext>,
    ) -> Pin<Box<dyn std::future::Future<Output = MiddlewareResult<HttpContext>> + Send>> {
        let config = self.config.clone();

        Box::pin(async move {
            // Log request
            let middleware = HttpDumpMiddleware { config: config.clone() };
            middleware.log_request(&ctx);

            // Call next middleware
            let result = next(ctx).await;

            // Log response
            if let Ok(ref ctx) = result {
                middleware.log_response(ctx);
            }

            result
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_http_dump_middleware() {
        let middleware = HttpDumpMiddleware::new();

        let mut ctx = HttpContext::new("GET".to_string(), "/test".to_string());
        ctx.headers.insert("User-Agent".to_string(), "TestAgent".to_string());
        ctx.body = Some(b"test body".to_vec());

        let next: Next<HttpContext> = Arc::new(|mut ctx| {
            Box::pin(async move {
                ctx.status = Some(200);
                ctx.response_body = Some(b"response body".to_vec());
                Ok(ctx)
            })
        });

        let result = middleware.handle(ctx, next).await;
        assert!(result.is_ok());
        let ctx = result.unwrap();
        assert_eq!(ctx.status, Some(200));
    }

    #[tokio::test]
    async fn test_http_dump_with_json() {
        let middleware = HttpDumpMiddleware::with_config(
            HttpDumpConfig::new().with_pretty_json(true)
        );

        let json_body = r#"{"name":"test","value":42}"#;
        let mut ctx = HttpContext::new("POST".to_string(), "/api/data".to_string());
        ctx.body = Some(json_body.as_bytes().to_vec());

        let next: Next<HttpContext> = Arc::new(|ctx| Box::pin(async move { Ok(ctx) }));

        let result = middleware.handle(ctx, next).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_body_truncation() {
        let middleware = HttpDumpMiddleware::with_config(
            HttpDumpConfig::new().with_max_body_size(10)
        );

        let body = b"This is a very long body that should be truncated";
        let formatted = middleware.format_body(body);
        assert!(formatted.contains("truncated"));
    }

    #[test]
    fn test_format_body_binary() {
        let middleware = HttpDumpMiddleware::new();
        let body = vec![0xFF, 0xFE, 0xFD, 0xFC];
        let formatted = middleware.format_body(&body);
        assert!(formatted.contains("binary data"));
    }
}
