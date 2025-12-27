//! CORS (Cross-Origin Resource Sharing) middleware (L4: Core - Middleware).
//!
//! Handles CORS headers and preflight requests for cross-origin HTTP requests.

use super::http_context::HttpContext;
use super::traits::{Middleware, MiddlewareError, MiddlewareResult, Next};
use regex::Regex;
use std::collections::HashSet;
use std::pin::Pin;
use std::time::Duration;

/// CORS middleware configuration.
#[derive(Clone)]
pub struct CorsConfig {
    /// Allowed origins configuration.
    pub origins: OriginConfig,
    /// Allowed HTTP methods.
    pub methods: HashSet<String>,
    /// Allowed request headers.
    pub headers: HashSet<String>,
    /// Exposed response headers.
    pub expose_headers: HashSet<String>,
    /// Whether to allow credentials (cookies, authorization headers).
    pub allow_credentials: bool,
    /// Max age for preflight cache in seconds.
    pub max_age: Option<Duration>,
}

impl Default for CorsConfig {
    fn default() -> Self {
        Self {
            origins: OriginConfig::Any,
            methods: vec!["GET".to_string(), "POST".to_string(), "PUT".to_string(), "DELETE".to_string(), "PATCH".to_string(), "HEAD".to_string(), "OPTIONS".to_string()]
                .into_iter()
                .collect(),
            headers: vec!["Content-Type".to_string(), "Authorization".to_string()]
                .into_iter()
                .collect(),
            expose_headers: HashSet::new(),
            allow_credentials: false,
            max_age: Some(Duration::from_secs(3600)), // 1 hour default
        }
    }
}

impl CorsConfig {
    /// Create a new CORS configuration with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Allow all origins.
    pub fn allow_all_origins(mut self) -> Self {
        self.origins = OriginConfig::Any;
        self
    }

    /// Allow specific origins.
    pub fn allow_origins(mut self, origins: Vec<String>) -> Self {
        self.origins = OriginConfig::Specific(origins.into_iter().collect());
        self
    }

    /// Allow origins matching a regex pattern.
    pub fn allow_origin_regex(mut self, pattern: &str) -> Result<Self, regex::Error> {
        let regex = Regex::new(pattern)?;
        self.origins = OriginConfig::Regex(regex);
        Ok(self)
    }

    /// Set allowed HTTP methods.
    pub fn allow_methods(mut self, methods: Vec<String>) -> Self {
        self.methods = methods.into_iter().collect();
        self
    }

    /// Set allowed request headers.
    pub fn allow_headers(mut self, headers: Vec<String>) -> Self {
        self.headers = headers.into_iter().collect();
        self
    }

    /// Set exposed response headers.
    pub fn expose_headers(mut self, headers: Vec<String>) -> Self {
        self.expose_headers = headers.into_iter().collect();
        self
    }

    /// Enable or disable credentials support.
    pub fn allow_credentials(mut self, allow: bool) -> Self {
        self.allow_credentials = allow;
        self
    }

    /// Set max age for preflight cache.
    pub fn max_age(mut self, duration: Duration) -> Self {
        self.max_age = Some(duration);
        self
    }

    /// Check if an origin is allowed.
    fn is_origin_allowed(&self, origin: &str) -> bool {
        match &self.origins {
            OriginConfig::Any => true,
            OriginConfig::Specific(origins) => origins.contains(origin),
            OriginConfig::Regex(regex) => regex.is_match(origin),
        }
    }
}

/// Origin configuration for CORS.
#[derive(Clone)]
pub enum OriginConfig {
    /// Allow any origin (*).
    Any,
    /// Allow specific origins.
    Specific(HashSet<String>),
    /// Allow origins matching a regex pattern.
    Regex(Regex),
}

// HttpContext is now imported from http_context module

/// CORS middleware.
pub struct CorsMiddleware {
    config: CorsConfig,
}

impl CorsMiddleware {
    /// Create a new CORS middleware with the given configuration.
    pub fn new(config: CorsConfig) -> Self {
        Self { config }
    }

    /// Create a CORS middleware with default permissive settings.
    pub fn permissive() -> Self {
        Self::new(CorsConfig::default().allow_all_origins())
    }

    /// Create a CORS middleware with restrictive settings.
    pub fn restrictive(allowed_origins: Vec<String>) -> Self {
        Self::new(
            CorsConfig::default()
                .allow_origins(allowed_origins)
                .allow_credentials(true),
        )
    }
}

impl Middleware<HttpContext> for CorsMiddleware {
    fn handle(
        &self,
        mut ctx: HttpContext,
        next: Next<HttpContext>,
    ) -> Pin<Box<dyn std::future::Future<Output = MiddlewareResult<HttpContext>> + Send>> {
        let config = self.config.clone();

        Box::pin(async move {
            // Get the Origin header
            let origin = ctx.get_header("Origin");

            // If no Origin header, this is not a CORS request - pass through
            if origin.is_none() {
                tracing::trace!("No Origin header present, skipping CORS");
                return next(ctx).await;
            }

            let origin = origin.unwrap().clone();

            // Check if origin is allowed
            if !config.is_origin_allowed(&origin) {
                tracing::warn!("Origin not allowed: {}", origin);
                return Err(MiddlewareError::Rejected(format!(
                    "Origin '{}' is not allowed",
                    origin
                )));
            }

            // Set CORS headers based on configuration
            if config.allow_credentials {
                // When credentials are allowed, we must echo the specific origin
                ctx.set_response_header("Access-Control-Allow-Origin".to_string(), origin.clone());
                ctx.set_response_header("Access-Control-Allow-Credentials".to_string(), "true".to_string());
                ctx.set_response_header("Vary".to_string(), "Origin".to_string());
            } else {
                // Without credentials, we can use wildcard or specific origin
                match &config.origins {
                    OriginConfig::Any => {
                        ctx.set_response_header("Access-Control-Allow-Origin".to_string(), "*".to_string());
                    }
                    _ => {
                        ctx.set_response_header("Access-Control-Allow-Origin".to_string(), origin.clone());
                        ctx.set_response_header("Vary".to_string(), "Origin".to_string());
                    }
                }
            }

            // Handle preflight requests
            if ctx.is_preflight {
                tracing::debug!("Handling CORS preflight request from origin: {}", origin);

                // Validate requested method
                if let Some(requested_method) = ctx.get_header("Access-Control-Request-Method") {
                    if !config.methods.contains(&requested_method.to_uppercase()) {
                        return Err(MiddlewareError::Rejected(format!(
                            "Method '{}' is not allowed",
                            requested_method
                        )));
                    }
                }

                // Validate requested headers
                if let Some(requested_headers) = ctx.get_header("Access-Control-Request-Headers") {
                    let headers: Vec<&str> = requested_headers.split(',').map(|s| s.trim()).collect();
                    for header in headers {
                        let header_lower = header.to_lowercase();
                        if !config.headers.iter().any(|h| h.to_lowercase() == header_lower) {
                            return Err(MiddlewareError::Rejected(format!(
                                "Header '{}' is not allowed",
                                header
                            )));
                        }
                    }
                }

                // Set preflight response headers
                ctx.set_response_header(
                    "Access-Control-Allow-Methods".to_string(),
                    config.methods.iter().cloned().collect::<Vec<_>>().join(", "),
                );

                ctx.set_response_header(
                    "Access-Control-Allow-Headers".to_string(),
                    config.headers.iter().cloned().collect::<Vec<_>>().join(", "),
                );

                if let Some(max_age) = config.max_age {
                    ctx.set_response_header(
                        "Access-Control-Max-Age".to_string(),
                        max_age.as_secs().to_string(),
                    );
                }

                // Set status to 204 No Content for preflight
                ctx.status = Some(204);

                // Return immediately for preflight - don't call next
                return Ok(ctx);
            }

            // For actual requests, set expose headers if configured
            if !config.expose_headers.is_empty() {
                ctx.set_response_header(
                    "Access-Control-Expose-Headers".to_string(),
                    config.expose_headers.iter().cloned().collect::<Vec<_>>().join(", "),
                );
            }

            // Continue to next middleware
            next(ctx).await
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chain::Pipeline;
    use std::collections::HashMap;

    fn create_test_context(method: &str, origin: Option<&str>) -> HttpContext {
        let mut headers = HashMap::new();
        if let Some(origin) = origin {
            headers.insert("Origin".to_string(), origin.to_string());
        }
        HttpContext::from_headers(method.to_string(), headers)
    }

    #[tokio::test]
    async fn test_cors_no_origin_header() {
        let cors = CorsMiddleware::permissive();
        let pipeline = Pipeline::new().with_middleware(cors);

        let ctx = create_test_context("GET", None);
        let result = pipeline.execute(ctx).await;

        assert!(result.is_ok());
        let ctx = result.unwrap();
        assert!(!ctx.response_headers.contains_key("Access-Control-Allow-Origin"));
    }

    #[tokio::test]
    async fn test_cors_permissive_any_origin() {
        let cors = CorsMiddleware::permissive();
        let pipeline = Pipeline::new().with_middleware(cors);

        let ctx = create_test_context("GET", Some("https://example.com"));
        let result = pipeline.execute(ctx).await;

        assert!(result.is_ok());
        let ctx = result.unwrap();
        assert_eq!(
            ctx.response_headers.get("Access-Control-Allow-Origin"),
            Some(&"*".to_string())
        );
    }

    #[tokio::test]
    async fn test_cors_specific_origin_allowed() {
        let config = CorsConfig::new()
            .allow_origins(vec!["https://example.com".to_string()]);
        let cors = CorsMiddleware::new(config);
        let pipeline = Pipeline::new().with_middleware(cors);

        let ctx = create_test_context("GET", Some("https://example.com"));
        let result = pipeline.execute(ctx).await;

        assert!(result.is_ok());
        let ctx = result.unwrap();
        assert_eq!(
            ctx.response_headers.get("Access-Control-Allow-Origin"),
            Some(&"https://example.com".to_string())
        );
        assert_eq!(
            ctx.response_headers.get("Vary"),
            Some(&"Origin".to_string())
        );
    }

    #[tokio::test]
    async fn test_cors_specific_origin_rejected() {
        let config = CorsConfig::new()
            .allow_origins(vec!["https://example.com".to_string()]);
        let cors = CorsMiddleware::new(config);
        let pipeline = Pipeline::new().with_middleware(cors);

        let ctx = create_test_context("GET", Some("https://evil.com"));
        let result = pipeline.execute(ctx).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            MiddlewareError::Rejected(msg) => {
                assert!(msg.contains("not allowed"));
            }
            _ => panic!("Expected Rejected error"),
        }
    }

    #[tokio::test]
    async fn test_cors_regex_origin() {
        let config = CorsConfig::new()
            .allow_origin_regex(r"^https://.*\.example\.com$")
            .unwrap();
        let cors = CorsMiddleware::new(config);
        let pipeline = Pipeline::new().with_middleware(cors);

        // Should allow subdomain
        let ctx = create_test_context("GET", Some("https://api.example.com"));
        let result = pipeline.execute(ctx).await;
        assert!(result.is_ok());

        // Should reject non-matching domain
        let cors2 = CorsMiddleware::new(
            CorsConfig::new()
                .allow_origin_regex(r"^https://.*\.example\.com$")
                .unwrap(),
        );
        let pipeline2 = Pipeline::new().with_middleware(cors2);
        let ctx2 = create_test_context("GET", Some("https://example.org"));
        let result2 = pipeline2.execute(ctx2).await;
        assert!(result2.is_err());
    }

    #[tokio::test]
    async fn test_cors_preflight_success() {
        let config = CorsConfig::new()
            .allow_origins(vec!["https://example.com".to_string()])
            .allow_methods(vec!["GET".to_string(), "POST".to_string()])
            .allow_headers(vec!["Content-Type".to_string()])
            .max_age(Duration::from_secs(7200));
        let cors = CorsMiddleware::new(config);
        let pipeline = Pipeline::new().with_middleware(cors);

        let mut headers = HashMap::new();
        headers.insert("Origin".to_string(), "https://example.com".to_string());
        headers.insert("Access-Control-Request-Method".to_string(), "POST".to_string());
        headers.insert("Access-Control-Request-Headers".to_string(), "Content-Type".to_string());

        let ctx = HttpContext::from_headers("OPTIONS".to_string(), headers);
        let result = pipeline.execute(ctx).await;

        assert!(result.is_ok());
        let ctx = result.unwrap();
        assert_eq!(ctx.status, Some(204));
        assert_eq!(
            ctx.response_headers.get("Access-Control-Allow-Origin"),
            Some(&"https://example.com".to_string())
        );
        assert!(ctx.response_headers.get("Access-Control-Allow-Methods").is_some());
        assert!(ctx.response_headers.get("Access-Control-Allow-Headers").is_some());
        assert_eq!(
            ctx.response_headers.get("Access-Control-Max-Age"),
            Some(&"7200".to_string())
        );
    }

    #[tokio::test]
    async fn test_cors_preflight_method_rejected() {
        let config = CorsConfig::new()
            .allow_origins(vec!["https://example.com".to_string()])
            .allow_methods(vec!["GET".to_string()]);
        let cors = CorsMiddleware::new(config);
        let pipeline = Pipeline::new().with_middleware(cors);

        let mut headers = HashMap::new();
        headers.insert("Origin".to_string(), "https://example.com".to_string());
        headers.insert("Access-Control-Request-Method".to_string(), "DELETE".to_string());

        let ctx = HttpContext::from_headers("OPTIONS".to_string(), headers);
        let result = pipeline.execute(ctx).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_cors_preflight_header_rejected() {
        let config = CorsConfig::new()
            .allow_origins(vec!["https://example.com".to_string()])
            .allow_headers(vec!["Content-Type".to_string()]);
        let cors = CorsMiddleware::new(config);
        let pipeline = Pipeline::new().with_middleware(cors);

        let mut headers = HashMap::new();
        headers.insert("Origin".to_string(), "https://example.com".to_string());
        headers.insert("Access-Control-Request-Method".to_string(), "GET".to_string());
        headers.insert("Access-Control-Request-Headers".to_string(), "X-Custom-Header".to_string());

        let ctx = HttpContext::from_headers("OPTIONS".to_string(), headers);
        let result = pipeline.execute(ctx).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_cors_with_credentials() {
        let config = CorsConfig::new()
            .allow_origins(vec!["https://example.com".to_string()])
            .allow_credentials(true);
        let cors = CorsMiddleware::new(config);
        let pipeline = Pipeline::new().with_middleware(cors);

        let ctx = create_test_context("GET", Some("https://example.com"));
        let result = pipeline.execute(ctx).await;

        assert!(result.is_ok());
        let ctx = result.unwrap();
        assert_eq!(
            ctx.response_headers.get("Access-Control-Allow-Credentials"),
            Some(&"true".to_string())
        );
        assert_eq!(
            ctx.response_headers.get("Access-Control-Allow-Origin"),
            Some(&"https://example.com".to_string())
        );
    }

    #[tokio::test]
    async fn test_cors_expose_headers() {
        let config = CorsConfig::new()
            .allow_origins(vec!["https://example.com".to_string()])
            .expose_headers(vec!["X-Custom-Header".to_string(), "X-Another-Header".to_string()]);
        let cors = CorsMiddleware::new(config);
        let pipeline = Pipeline::new().with_middleware(cors);

        let ctx = create_test_context("GET", Some("https://example.com"));
        let result = pipeline.execute(ctx).await;

        assert!(result.is_ok());
        let ctx = result.unwrap();
        let expose_headers = ctx.response_headers.get("Access-Control-Expose-Headers");
        assert!(expose_headers.is_some());
        let expose_headers = expose_headers.unwrap();
        assert!(expose_headers.contains("X-Custom-Header"));
        assert!(expose_headers.contains("X-Another-Header"));
    }

    #[tokio::test]
    async fn test_cors_restrictive_constructor() {
        let cors = CorsMiddleware::restrictive(vec!["https://example.com".to_string()]);
        let pipeline = Pipeline::new().with_middleware(cors);

        let ctx = create_test_context("GET", Some("https://example.com"));
        let result = pipeline.execute(ctx).await;

        assert!(result.is_ok());
        let ctx = result.unwrap();
        assert_eq!(
            ctx.response_headers.get("Access-Control-Allow-Credentials"),
            Some(&"true".to_string())
        );
    }
}
