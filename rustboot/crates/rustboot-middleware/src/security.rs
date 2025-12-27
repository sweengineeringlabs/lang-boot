//! Security headers middleware (L4: Core - Middleware).
//!
//! Middleware for adding security-related HTTP headers to responses.
//! Helps protect against common web vulnerabilities like XSS, clickjacking,
//! and content type sniffing attacks.
//!
//! # Supported Security Headers
//!
//! - **Content-Security-Policy (CSP)**: Controls which resources can be loaded
//! - **Strict-Transport-Security (HSTS)**: Enforces HTTPS connections
//! - **X-Frame-Options**: Prevents clickjacking attacks
//! - **X-Content-Type-Options**: Prevents MIME type sniffing
//! - **X-XSS-Protection**: Enables browser XSS filtering (legacy)
//! - **Referrer-Policy**: Controls referrer information
//! - **Permissions-Policy**: Controls browser features and APIs
//!
//! # Example
//!
//! ```rust,ignore
//! use rustboot_middleware::security::{SecurityHeadersMiddleware, SecurityHeadersConfig};
//!
//! // Use default secure configuration
//! let middleware = SecurityHeadersMiddleware::default();
//!
//! // Or customize the configuration
//! let config = SecurityHeadersConfig::default()
//!     .with_csp("default-src 'self'; script-src 'self' 'unsafe-inline'")
//!     .with_hsts(31536000, true, true)
//!     .with_frame_options("DENY");
//!
//! let middleware = SecurityHeadersMiddleware::new(config);
//! ```

use super::traits::{Middleware, MiddlewareResult, Next};
use std::pin::Pin;

/// Configuration for security headers.
#[derive(Debug, Clone)]
pub struct SecurityHeadersConfig {
    /// Content-Security-Policy header value
    pub content_security_policy: Option<String>,

    /// Strict-Transport-Security max-age in seconds
    pub hsts_max_age: Option<u64>,

    /// Include subdomains in HSTS
    pub hsts_include_subdomains: bool,

    /// Include preload directive in HSTS
    pub hsts_preload: bool,

    /// X-Frame-Options header value (DENY, SAMEORIGIN, or ALLOW-FROM uri)
    pub x_frame_options: Option<String>,

    /// X-Content-Type-Options header value (typically "nosniff")
    pub x_content_type_options: Option<String>,

    /// X-XSS-Protection header value (legacy, typically "1; mode=block")
    pub x_xss_protection: Option<String>,

    /// Referrer-Policy header value
    pub referrer_policy: Option<String>,

    /// Permissions-Policy header value
    pub permissions_policy: Option<String>,
}

impl Default for SecurityHeadersConfig {
    /// Creates a default configuration with secure settings.
    ///
    /// Default values:
    /// - CSP: `default-src 'self'`
    /// - HSTS: 1 year, includeSubDomains, preload
    /// - X-Frame-Options: `DENY`
    /// - X-Content-Type-Options: `nosniff`
    /// - X-XSS-Protection: `1; mode=block`
    /// - Referrer-Policy: `strict-origin-when-cross-origin`
    /// - Permissions-Policy: restrictive defaults
    fn default() -> Self {
        Self {
            content_security_policy: Some("default-src 'self'".to_string()),
            hsts_max_age: Some(31536000), // 1 year
            hsts_include_subdomains: true,
            hsts_preload: true,
            x_frame_options: Some("DENY".to_string()),
            x_content_type_options: Some("nosniff".to_string()),
            x_xss_protection: Some("1; mode=block".to_string()),
            referrer_policy: Some("strict-origin-when-cross-origin".to_string()),
            permissions_policy: Some(
                "geolocation=(), microphone=(), camera=(), payment=(), usb=(), magnetometer=(), gyroscope=(), accelerometer=()"
                    .to_string(),
            ),
        }
    }
}

impl SecurityHeadersConfig {
    /// Creates a new empty configuration with no headers set.
    pub fn new() -> Self {
        Self {
            content_security_policy: None,
            hsts_max_age: None,
            hsts_include_subdomains: false,
            hsts_preload: false,
            x_frame_options: None,
            x_content_type_options: None,
            x_xss_protection: None,
            referrer_policy: None,
            permissions_policy: None,
        }
    }

    /// Creates a permissive configuration suitable for development.
    ///
    /// This configuration is less restrictive and more suitable for
    /// development environments where you may be loading resources
    /// from various sources.
    pub fn permissive() -> Self {
        Self {
            content_security_policy: Some("default-src 'self' 'unsafe-inline' 'unsafe-eval'".to_string()),
            hsts_max_age: None, // Don't enforce HTTPS in development
            hsts_include_subdomains: false,
            hsts_preload: false,
            x_frame_options: Some("SAMEORIGIN".to_string()),
            x_content_type_options: Some("nosniff".to_string()),
            x_xss_protection: Some("1; mode=block".to_string()),
            referrer_policy: Some("no-referrer-when-downgrade".to_string()),
            permissions_policy: None,
        }
    }

    /// Sets the Content-Security-Policy header.
    ///
    /// # Arguments
    ///
    /// * `policy` - The CSP policy string
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use rustboot_middleware::security::SecurityHeadersConfig;
    ///
    /// let config = SecurityHeadersConfig::new()
    ///     .with_csp("default-src 'self'; script-src 'self' https://cdn.example.com");
    /// ```
    pub fn with_csp(mut self, policy: impl Into<String>) -> Self {
        self.content_security_policy = Some(policy.into());
        self
    }

    /// Disables the Content-Security-Policy header.
    pub fn without_csp(mut self) -> Self {
        self.content_security_policy = None;
        self
    }

    /// Sets the Strict-Transport-Security header.
    ///
    /// # Arguments
    ///
    /// * `max_age` - Maximum age in seconds
    /// * `include_subdomains` - Whether to include subdomains
    /// * `preload` - Whether to include the preload directive
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use rustboot_middleware::security::SecurityHeadersConfig;
    ///
    /// // 1 year HSTS with subdomains and preload
    /// let config = SecurityHeadersConfig::new()
    ///     .with_hsts(31536000, true, true);
    /// ```
    pub fn with_hsts(mut self, max_age: u64, include_subdomains: bool, preload: bool) -> Self {
        self.hsts_max_age = Some(max_age);
        self.hsts_include_subdomains = include_subdomains;
        self.hsts_preload = preload;
        self
    }

    /// Disables the Strict-Transport-Security header.
    pub fn without_hsts(mut self) -> Self {
        self.hsts_max_age = None;
        self
    }

    /// Sets the X-Frame-Options header.
    ///
    /// # Arguments
    ///
    /// * `value` - One of "DENY", "SAMEORIGIN", or "ALLOW-FROM uri"
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use rustboot_middleware::security::SecurityHeadersConfig;
    ///
    /// let config = SecurityHeadersConfig::new()
    ///     .with_frame_options("SAMEORIGIN");
    /// ```
    pub fn with_frame_options(mut self, value: impl Into<String>) -> Self {
        self.x_frame_options = Some(value.into());
        self
    }

    /// Disables the X-Frame-Options header.
    pub fn without_frame_options(mut self) -> Self {
        self.x_frame_options = None;
        self
    }

    /// Sets the X-Content-Type-Options header.
    ///
    /// Typically set to "nosniff" to prevent MIME type sniffing.
    pub fn with_content_type_options(mut self, value: impl Into<String>) -> Self {
        self.x_content_type_options = Some(value.into());
        self
    }

    /// Disables the X-Content-Type-Options header.
    pub fn without_content_type_options(mut self) -> Self {
        self.x_content_type_options = None;
        self
    }

    /// Sets the X-XSS-Protection header.
    ///
    /// This is a legacy header. Modern browsers use CSP instead.
    /// Typically set to "1; mode=block".
    pub fn with_xss_protection(mut self, value: impl Into<String>) -> Self {
        self.x_xss_protection = Some(value.into());
        self
    }

    /// Disables the X-XSS-Protection header.
    pub fn without_xss_protection(mut self) -> Self {
        self.x_xss_protection = None;
        self
    }

    /// Sets the Referrer-Policy header.
    ///
    /// # Arguments
    ///
    /// * `policy` - One of the standard referrer policy values:
    ///   - "no-referrer"
    ///   - "no-referrer-when-downgrade"
    ///   - "origin"
    ///   - "origin-when-cross-origin"
    ///   - "same-origin"
    ///   - "strict-origin"
    ///   - "strict-origin-when-cross-origin"
    ///   - "unsafe-url"
    pub fn with_referrer_policy(mut self, policy: impl Into<String>) -> Self {
        self.referrer_policy = Some(policy.into());
        self
    }

    /// Disables the Referrer-Policy header.
    pub fn without_referrer_policy(mut self) -> Self {
        self.referrer_policy = None;
        self
    }

    /// Sets the Permissions-Policy header.
    ///
    /// # Arguments
    ///
    /// * `policy` - The permissions policy string
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use rustboot_middleware::security::SecurityHeadersConfig;
    ///
    /// let config = SecurityHeadersConfig::new()
    ///     .with_permissions_policy("geolocation=(), microphone=(), camera=()");
    /// ```
    pub fn with_permissions_policy(mut self, policy: impl Into<String>) -> Self {
        self.permissions_policy = Some(policy.into());
        self
    }

    /// Disables the Permissions-Policy header.
    pub fn without_permissions_policy(mut self) -> Self {
        self.permissions_policy = None;
        self
    }

    /// Builds the HSTS header value from the configuration.
    fn build_hsts_header(&self) -> Option<String> {
        self.hsts_max_age.map(|max_age| {
            let mut value = format!("max-age={}", max_age);
            if self.hsts_include_subdomains {
                value.push_str("; includeSubDomains");
            }
            if self.hsts_preload {
                value.push_str("; preload");
            }
            value
        })
    }
}

/// Context trait for types that can have HTTP headers added.
///
/// This trait must be implemented by any context type that will be used
/// with the SecurityHeadersMiddleware.
pub trait HasHeaders {
    /// Adds a header to the context.
    ///
    /// # Arguments
    ///
    /// * `name` - The header name
    /// * `value` - The header value
    fn add_header(&mut self, name: String, value: String);
}

/// Middleware that adds security headers to HTTP responses.
///
/// This middleware adds various security-related headers based on the
/// provided configuration. It can be used to protect against common
/// web vulnerabilities.
///
/// # Example
///
/// ```rust,ignore
/// use rustboot_middleware::security::{SecurityHeadersMiddleware, SecurityHeadersConfig};
/// use rustboot_middleware::Pipeline;
///
/// # #[derive(Debug, Clone)]
/// # struct MyContext { headers: Vec<(String, String)> }
/// # impl rustboot_middleware::security::HasHeaders for MyContext {
/// #     fn add_header(&mut self, name: String, value: String) {
/// #         self.headers.push((name, value));
/// #     }
/// # }
/// # async fn example() {
/// let config = SecurityHeadersConfig::default();
/// let middleware = SecurityHeadersMiddleware::new(config);
///
/// let pipeline = Pipeline::new()
///     .with_middleware(middleware);
///
/// let ctx = MyContext { headers: vec![] };
/// let result = pipeline.execute(ctx).await;
/// # }
/// ```
pub struct SecurityHeadersMiddleware {
    config: SecurityHeadersConfig,
}

impl SecurityHeadersMiddleware {
    /// Creates a new security headers middleware with the given configuration.
    pub fn new(config: SecurityHeadersConfig) -> Self {
        Self { config }
    }

    /// Creates a middleware with default secure configuration.
    pub fn secure() -> Self {
        Self::new(SecurityHeadersConfig::default())
    }

    /// Creates a middleware with permissive configuration for development.
    pub fn permissive() -> Self {
        Self::new(SecurityHeadersConfig::permissive())
    }

    /// Applies the configured headers to the context.
    fn apply_headers<Ctx>(&self, ctx: &mut Ctx)
    where
        Ctx: HasHeaders,
    {
        // Content-Security-Policy
        if let Some(ref csp) = self.config.content_security_policy {
            ctx.add_header("Content-Security-Policy".to_string(), csp.clone());
        }

        // Strict-Transport-Security
        if let Some(hsts) = self.config.build_hsts_header() {
            ctx.add_header("Strict-Transport-Security".to_string(), hsts);
        }

        // X-Frame-Options
        if let Some(ref frame_options) = self.config.x_frame_options {
            ctx.add_header("X-Frame-Options".to_string(), frame_options.clone());
        }

        // X-Content-Type-Options
        if let Some(ref content_type_options) = self.config.x_content_type_options {
            ctx.add_header("X-Content-Type-Options".to_string(), content_type_options.clone());
        }

        // X-XSS-Protection
        if let Some(ref xss_protection) = self.config.x_xss_protection {
            ctx.add_header("X-XSS-Protection".to_string(), xss_protection.clone());
        }

        // Referrer-Policy
        if let Some(ref referrer_policy) = self.config.referrer_policy {
            ctx.add_header("Referrer-Policy".to_string(), referrer_policy.clone());
        }

        // Permissions-Policy
        if let Some(ref permissions_policy) = self.config.permissions_policy {
            ctx.add_header("Permissions-Policy".to_string(), permissions_policy.clone());
        }
    }
}

impl Default for SecurityHeadersMiddleware {
    fn default() -> Self {
        Self::secure()
    }
}

impl<Ctx> Middleware<Ctx> for SecurityHeadersMiddleware
where
    Ctx: HasHeaders + Send + 'static,
{
    fn handle(
        &self,
        mut ctx: Ctx,
        next: Next<Ctx>,
    ) -> Pin<Box<dyn std::future::Future<Output = MiddlewareResult<Ctx>> + Send>> {
        // Apply headers before processing
        self.apply_headers(&mut ctx);

        Box::pin(async move {
            // Continue with the next middleware
            next(ctx).await
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chain::Pipeline;

    #[derive(Debug, Clone)]
    struct TestContext {
        headers: Vec<(String, String)>,
    }

    impl HasHeaders for TestContext {
        fn add_header(&mut self, name: String, value: String) {
            self.headers.push((name, value));
        }
    }

    impl TestContext {
        fn new() -> Self {
            Self {
                headers: Vec::new(),
            }
        }

        fn has_header(&self, name: &str) -> bool {
            self.headers.iter().any(|(n, _)| n == name)
        }

        fn get_header(&self, name: &str) -> Option<String> {
            self.headers
                .iter()
                .find(|(n, _)| n == name)
                .map(|(_, v)| v.clone())
        }
    }

    #[tokio::test]
    async fn test_default_security_headers() {
        let middleware = SecurityHeadersMiddleware::default();
        let pipeline = Pipeline::new().with_middleware(middleware);

        let ctx = TestContext::new();
        let result = pipeline.execute(ctx).await;

        assert!(result.is_ok());
        let ctx = result.unwrap();

        // Verify all default headers are present
        assert!(ctx.has_header("Content-Security-Policy"));
        assert!(ctx.has_header("Strict-Transport-Security"));
        assert!(ctx.has_header("X-Frame-Options"));
        assert!(ctx.has_header("X-Content-Type-Options"));
        assert!(ctx.has_header("X-XSS-Protection"));
        assert!(ctx.has_header("Referrer-Policy"));
        assert!(ctx.has_header("Permissions-Policy"));
    }

    #[tokio::test]
    async fn test_custom_csp() {
        let config = SecurityHeadersConfig::new()
            .with_csp("default-src 'self'; script-src 'self' https://cdn.example.com");

        let middleware = SecurityHeadersMiddleware::new(config);
        let pipeline = Pipeline::new().with_middleware(middleware);

        let ctx = TestContext::new();
        let result = pipeline.execute(ctx).await;

        assert!(result.is_ok());
        let ctx = result.unwrap();

        assert_eq!(
            ctx.get_header("Content-Security-Policy"),
            Some("default-src 'self'; script-src 'self' https://cdn.example.com".to_string())
        );
    }

    #[tokio::test]
    async fn test_hsts_configuration() {
        let config = SecurityHeadersConfig::new()
            .with_hsts(63072000, true, true);

        let middleware = SecurityHeadersMiddleware::new(config);
        let pipeline = Pipeline::new().with_middleware(middleware);

        let ctx = TestContext::new();
        let result = pipeline.execute(ctx).await;

        assert!(result.is_ok());
        let ctx = result.unwrap();

        let hsts = ctx.get_header("Strict-Transport-Security").unwrap();
        assert!(hsts.contains("max-age=63072000"));
        assert!(hsts.contains("includeSubDomains"));
        assert!(hsts.contains("preload"));
    }

    #[tokio::test]
    async fn test_frame_options() {
        let config = SecurityHeadersConfig::new()
            .with_frame_options("SAMEORIGIN");

        let middleware = SecurityHeadersMiddleware::new(config);
        let pipeline = Pipeline::new().with_middleware(middleware);

        let ctx = TestContext::new();
        let result = pipeline.execute(ctx).await;

        assert!(result.is_ok());
        let ctx = result.unwrap();

        assert_eq!(
            ctx.get_header("X-Frame-Options"),
            Some("SAMEORIGIN".to_string())
        );
    }

    #[tokio::test]
    async fn test_disabled_headers() {
        let config = SecurityHeadersConfig::default()
            .without_csp()
            .without_hsts()
            .without_permissions_policy();

        let middleware = SecurityHeadersMiddleware::new(config);
        let pipeline = Pipeline::new().with_middleware(middleware);

        let ctx = TestContext::new();
        let result = pipeline.execute(ctx).await;

        assert!(result.is_ok());
        let ctx = result.unwrap();

        assert!(!ctx.has_header("Content-Security-Policy"));
        assert!(!ctx.has_header("Strict-Transport-Security"));
        assert!(!ctx.has_header("Permissions-Policy"));

        // These should still be present
        assert!(ctx.has_header("X-Frame-Options"));
        assert!(ctx.has_header("X-Content-Type-Options"));
    }

    #[tokio::test]
    async fn test_permissive_config() {
        let middleware = SecurityHeadersMiddleware::permissive();
        let pipeline = Pipeline::new().with_middleware(middleware);

        let ctx = TestContext::new();
        let result = pipeline.execute(ctx).await;

        assert!(result.is_ok());
        let ctx = result.unwrap();

        let csp = ctx.get_header("Content-Security-Policy").unwrap();
        assert!(csp.contains("unsafe-inline"));
        assert!(csp.contains("unsafe-eval"));

        // HSTS should not be present in permissive mode
        assert!(!ctx.has_header("Strict-Transport-Security"));
    }

    #[tokio::test]
    async fn test_referrer_policy() {
        let config = SecurityHeadersConfig::new()
            .with_referrer_policy("no-referrer");

        let middleware = SecurityHeadersMiddleware::new(config);
        let pipeline = Pipeline::new().with_middleware(middleware);

        let ctx = TestContext::new();
        let result = pipeline.execute(ctx).await;

        assert!(result.is_ok());
        let ctx = result.unwrap();

        assert_eq!(
            ctx.get_header("Referrer-Policy"),
            Some("no-referrer".to_string())
        );
    }

    #[tokio::test]
    async fn test_permissions_policy() {
        let config = SecurityHeadersConfig::new()
            .with_permissions_policy("geolocation=(self), microphone=()");

        let middleware = SecurityHeadersMiddleware::new(config);
        let pipeline = Pipeline::new().with_middleware(middleware);

        let ctx = TestContext::new();
        let result = pipeline.execute(ctx).await;

        assert!(result.is_ok());
        let ctx = result.unwrap();

        assert_eq!(
            ctx.get_header("Permissions-Policy"),
            Some("geolocation=(self), microphone=()".to_string())
        );
    }

    #[tokio::test]
    async fn test_empty_config() {
        let config = SecurityHeadersConfig::new();
        let middleware = SecurityHeadersMiddleware::new(config);
        let pipeline = Pipeline::new().with_middleware(middleware);

        let ctx = TestContext::new();
        let result = pipeline.execute(ctx).await;

        assert!(result.is_ok());
        let ctx = result.unwrap();

        // No headers should be added
        assert_eq!(ctx.headers.len(), 0);
    }

    #[tokio::test]
    async fn test_multiple_middleware_in_pipeline() {
        use crate::traits::FnMiddleware;

        let security_middleware = SecurityHeadersMiddleware::default();
        let custom_middleware = FnMiddleware::new(|mut ctx: TestContext, next: Next<TestContext>| async move {
            ctx.add_header("X-Custom-Header".to_string(), "CustomValue".to_string());
            next(ctx).await
        });

        let pipeline = Pipeline::new()
            .with_middleware(security_middleware)
            .with_middleware(custom_middleware);

        let ctx = TestContext::new();
        let result = pipeline.execute(ctx).await;

        assert!(result.is_ok());
        let ctx = result.unwrap();

        // Both security headers and custom header should be present
        assert!(ctx.has_header("Content-Security-Policy"));
        assert!(ctx.has_header("X-Custom-Header"));
    }
}
