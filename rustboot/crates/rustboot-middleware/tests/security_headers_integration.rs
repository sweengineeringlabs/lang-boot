//! Integration tests for security headers middleware

use dev_engineeringlabs_rustboot_middleware::{
    security::{HasHeaders, SecurityHeadersConfig, SecurityHeadersMiddleware},
    Pipeline,
};

#[derive(Debug, Clone)]
struct TestResponse {
    headers: Vec<(String, String)>,
}

impl TestResponse {
    fn new() -> Self {
        Self {
            headers: Vec::new(),
        }
    }

    fn get_header(&self, name: &str) -> Option<String> {
        self.headers
            .iter()
            .find(|(n, _)| n.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.clone())
    }

    fn has_header(&self, name: &str) -> bool {
        self.headers
            .iter()
            .any(|(n, _)| n.eq_ignore_ascii_case(name))
    }

    fn header_count(&self) -> usize {
        self.headers.len()
    }
}

impl HasHeaders for TestResponse {
    fn add_header(&mut self, name: String, value: String) {
        self.headers.push((name, value));
    }
}

#[tokio::test]
async fn test_default_configuration_includes_all_headers() {
    let middleware = SecurityHeadersMiddleware::default();
    let pipeline = Pipeline::new().with_middleware(middleware);

    let response = TestResponse::new();
    let result = pipeline.execute(response).await;

    assert!(result.is_ok());
    let response = result.unwrap();

    // Verify all default headers are present
    assert!(response.has_header("Content-Security-Policy"));
    assert!(response.has_header("Strict-Transport-Security"));
    assert!(response.has_header("X-Frame-Options"));
    assert!(response.has_header("X-Content-Type-Options"));
    assert!(response.has_header("X-XSS-Protection"));
    assert!(response.has_header("Referrer-Policy"));
    assert!(response.has_header("Permissions-Policy"));

    // Should have exactly 7 headers
    assert_eq!(response.header_count(), 7);
}

#[tokio::test]
async fn test_csp_header_format() {
    let config = SecurityHeadersConfig::new()
        .with_csp("default-src 'self'; script-src 'self' https://cdn.example.com; style-src 'self' 'unsafe-inline'");

    let middleware = SecurityHeadersMiddleware::new(config);
    let pipeline = Pipeline::new().with_middleware(middleware);

    let response = TestResponse::new();
    let result = pipeline.execute(response).await;

    assert!(result.is_ok());
    let response = result.unwrap();

    let csp = response.get_header("Content-Security-Policy").unwrap();
    assert!(csp.contains("default-src 'self'"));
    assert!(csp.contains("script-src 'self' https://cdn.example.com"));
    assert!(csp.contains("style-src 'self' 'unsafe-inline'"));
}

#[tokio::test]
async fn test_hsts_header_format() {
    let config = SecurityHeadersConfig::new()
        .with_hsts(31536000, true, true);

    let middleware = SecurityHeadersMiddleware::new(config);
    let pipeline = Pipeline::new().with_middleware(middleware);

    let response = TestResponse::new();
    let result = pipeline.execute(response).await;

    assert!(result.is_ok());
    let response = result.unwrap();

    let hsts = response.get_header("Strict-Transport-Security").unwrap();
    assert_eq!(hsts, "max-age=31536000; includeSubDomains; preload");
}

#[tokio::test]
async fn test_hsts_without_subdomains_and_preload() {
    let config = SecurityHeadersConfig::new()
        .with_hsts(86400, false, false);

    let middleware = SecurityHeadersMiddleware::new(config);
    let pipeline = Pipeline::new().with_middleware(middleware);

    let response = TestResponse::new();
    let result = pipeline.execute(response).await;

    assert!(result.is_ok());
    let response = result.unwrap();

    let hsts = response.get_header("Strict-Transport-Security").unwrap();
    assert_eq!(hsts, "max-age=86400");
    assert!(!hsts.contains("includeSubDomains"));
    assert!(!hsts.contains("preload"));
}

#[tokio::test]
async fn test_hsts_with_subdomains_only() {
    let config = SecurityHeadersConfig::new()
        .with_hsts(31536000, true, false);

    let middleware = SecurityHeadersMiddleware::new(config);
    let pipeline = Pipeline::new().with_middleware(middleware);

    let response = TestResponse::new();
    let result = pipeline.execute(response).await;

    assert!(result.is_ok());
    let response = result.unwrap();

    let hsts = response.get_header("Strict-Transport-Security").unwrap();
    assert_eq!(hsts, "max-age=31536000; includeSubDomains");
}

#[tokio::test]
async fn test_frame_options_deny() {
    let config = SecurityHeadersConfig::new()
        .with_frame_options("DENY");

    let middleware = SecurityHeadersMiddleware::new(config);
    let pipeline = Pipeline::new().with_middleware(middleware);

    let response = TestResponse::new();
    let result = pipeline.execute(response).await;

    assert!(result.is_ok());
    let response = result.unwrap();

    assert_eq!(response.get_header("X-Frame-Options"), Some("DENY".to_string()));
}

#[tokio::test]
async fn test_frame_options_sameorigin() {
    let config = SecurityHeadersConfig::new()
        .with_frame_options("SAMEORIGIN");

    let middleware = SecurityHeadersMiddleware::new(config);
    let pipeline = Pipeline::new().with_middleware(middleware);

    let response = TestResponse::new();
    let result = pipeline.execute(response).await;

    assert!(result.is_ok());
    let response = result.unwrap();

    assert_eq!(
        response.get_header("X-Frame-Options"),
        Some("SAMEORIGIN".to_string())
    );
}

#[tokio::test]
async fn test_referrer_policy_values() {
    let policies = vec![
        "no-referrer",
        "no-referrer-when-downgrade",
        "origin",
        "origin-when-cross-origin",
        "same-origin",
        "strict-origin",
        "strict-origin-when-cross-origin",
        "unsafe-url",
    ];

    for policy in policies {
        let config = SecurityHeadersConfig::new()
            .with_referrer_policy(policy);

        let middleware = SecurityHeadersMiddleware::new(config);
        let pipeline = Pipeline::new().with_middleware(middleware);

        let response = TestResponse::new();
        let result = pipeline.execute(response).await;

        assert!(result.is_ok());
        let response = result.unwrap();

        assert_eq!(
            response.get_header("Referrer-Policy"),
            Some(policy.to_string())
        );
    }
}

#[tokio::test]
async fn test_permissions_policy_format() {
    let config = SecurityHeadersConfig::new()
        .with_permissions_policy("geolocation=(self), microphone=(), camera=(self \"https://example.com\")");

    let middleware = SecurityHeadersMiddleware::new(config);
    let pipeline = Pipeline::new().with_middleware(middleware);

    let response = TestResponse::new();
    let result = pipeline.execute(response).await;

    assert!(result.is_ok());
    let response = result.unwrap();

    let permissions = response.get_header("Permissions-Policy").unwrap();
    assert!(permissions.contains("geolocation=(self)"));
    assert!(permissions.contains("microphone=()"));
    assert!(permissions.contains("camera=(self \"https://example.com\")"));
}

#[tokio::test]
async fn test_disabling_headers() {
    let config = SecurityHeadersConfig::default()
        .without_csp()
        .without_hsts()
        .without_frame_options()
        .without_xss_protection()
        .without_referrer_policy()
        .without_permissions_policy();

    let middleware = SecurityHeadersMiddleware::new(config);
    let pipeline = Pipeline::new().with_middleware(middleware);

    let response = TestResponse::new();
    let result = pipeline.execute(response).await;

    assert!(result.is_ok());
    let response = result.unwrap();

    // Only X-Content-Type-Options should remain from default
    assert!(!response.has_header("Content-Security-Policy"));
    assert!(!response.has_header("Strict-Transport-Security"));
    assert!(!response.has_header("X-Frame-Options"));
    assert!(response.has_header("X-Content-Type-Options"));
    assert!(!response.has_header("X-XSS-Protection"));
    assert!(!response.has_header("Referrer-Policy"));
    assert!(!response.has_header("Permissions-Policy"));

    assert_eq!(response.header_count(), 1);
}

#[tokio::test]
async fn test_permissive_configuration() {
    let middleware = SecurityHeadersMiddleware::permissive();
    let pipeline = Pipeline::new().with_middleware(middleware);

    let response = TestResponse::new();
    let result = pipeline.execute(response).await;

    assert!(result.is_ok());
    let response = result.unwrap();

    // Should have CSP with unsafe directives
    let csp = response.get_header("Content-Security-Policy").unwrap();
    assert!(csp.contains("unsafe-inline"));
    assert!(csp.contains("unsafe-eval"));

    // Should NOT have HSTS in development mode
    assert!(!response.has_header("Strict-Transport-Security"));

    // Frame options should be SAMEORIGIN
    assert_eq!(
        response.get_header("X-Frame-Options"),
        Some("SAMEORIGIN".to_string())
    );

    // Should NOT have Permissions-Policy
    assert!(!response.has_header("Permissions-Policy"));
}

#[tokio::test]
async fn test_builder_pattern_chaining() {
    let config = SecurityHeadersConfig::new()
        .with_csp("default-src 'self'")
        .with_hsts(31536000, true, true)
        .with_frame_options("DENY")
        .with_content_type_options("nosniff")
        .with_xss_protection("1; mode=block")
        .with_referrer_policy("strict-origin")
        .with_permissions_policy("geolocation=()");

    let middleware = SecurityHeadersMiddleware::new(config);
    let pipeline = Pipeline::new().with_middleware(middleware);

    let response = TestResponse::new();
    let result = pipeline.execute(response).await;

    assert!(result.is_ok());
    let response = result.unwrap();

    assert_eq!(response.header_count(), 7);
    assert_eq!(
        response.get_header("Content-Security-Policy"),
        Some("default-src 'self'".to_string())
    );
    assert_eq!(
        response.get_header("X-Frame-Options"),
        Some("DENY".to_string())
    );
}

#[tokio::test]
async fn test_multiple_pipeline_executions() {
    // Execute multiple times to ensure middleware is reusable
    for _ in 0..5 {
        let middleware = SecurityHeadersMiddleware::default();
        let pipeline = Pipeline::new().with_middleware(middleware);

        let response = TestResponse::new();
        let result = pipeline.execute(response).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.header_count(), 7);
    }
}

#[tokio::test]
async fn test_empty_configuration() {
    let config = SecurityHeadersConfig::new();
    let middleware = SecurityHeadersMiddleware::new(config);
    let pipeline = Pipeline::new().with_middleware(middleware);

    let response = TestResponse::new();
    let result = pipeline.execute(response).await;

    assert!(result.is_ok());
    let response = result.unwrap();

    // No headers should be added
    assert_eq!(response.header_count(), 0);
}

#[tokio::test]
async fn test_production_ready_configuration() {
    // A realistic production configuration
    let config = SecurityHeadersConfig::new()
        .with_csp(
            "default-src 'self'; \
             script-src 'self' https://cdn.example.com; \
             style-src 'self' https://cdn.example.com; \
             img-src 'self' data: https:; \
             font-src 'self' https://fonts.gstatic.com; \
             connect-src 'self' https://api.example.com; \
             frame-ancestors 'none'; \
             base-uri 'self'; \
             form-action 'self'"
        )
        .with_hsts(63072000, true, true) // 2 years
        .with_frame_options("DENY")
        .with_content_type_options("nosniff")
        .without_xss_protection() // Modern apps use CSP
        .with_referrer_policy("strict-origin-when-cross-origin")
        .with_permissions_policy(
            "geolocation=(), \
             microphone=(), \
             camera=(), \
             payment=(), \
             usb=(), \
             magnetometer=(), \
             gyroscope=(), \
             accelerometer=(), \
             interest-cohort=()"
        );

    let middleware = SecurityHeadersMiddleware::new(config);
    let pipeline = Pipeline::new().with_middleware(middleware);

    let response = TestResponse::new();
    let result = pipeline.execute(response).await;

    assert!(result.is_ok());
    let response = result.unwrap();

    // Should have 6 headers (no X-XSS-Protection)
    assert_eq!(response.header_count(), 6);

    // Verify critical headers
    assert!(response.has_header("Content-Security-Policy"));
    assert!(response.has_header("Strict-Transport-Security"));
    assert!(!response.has_header("X-XSS-Protection"));

    // Verify HSTS is 2 years
    let hsts = response.get_header("Strict-Transport-Security").unwrap();
    assert!(hsts.contains("max-age=63072000"));
}

#[tokio::test]
async fn test_api_endpoint_configuration() {
    // Minimal headers for API endpoints
    let config = SecurityHeadersConfig::new()
        .with_hsts(31536000, true, true)
        .with_content_type_options("nosniff")
        .with_frame_options("DENY")
        .with_referrer_policy("no-referrer");

    let middleware = SecurityHeadersMiddleware::new(config);
    let pipeline = Pipeline::new().with_middleware(middleware);

    let response = TestResponse::new();
    let result = pipeline.execute(response).await;

    assert!(result.is_ok());
    let response = result.unwrap();

    // Should have 4 headers
    assert_eq!(response.header_count(), 4);

    // No CSP needed for APIs typically
    assert!(!response.has_header("Content-Security-Policy"));
}

#[tokio::test]
async fn test_case_insensitive_header_lookup() {
    let middleware = SecurityHeadersMiddleware::default();
    let pipeline = Pipeline::new().with_middleware(middleware);

    let response = TestResponse::new();
    let result = pipeline.execute(response).await;

    assert!(result.is_ok());
    let response = result.unwrap();

    // Test case-insensitive lookups
    assert!(response.has_header("content-security-policy"));
    assert!(response.has_header("CONTENT-SECURITY-POLICY"));
    assert!(response.has_header("Content-Security-Policy"));
}
