//! Integration tests for rustboot-middleware

use dev_engineeringlabs_rustboot_middleware::{
    HttpLoggingConfig, HttpLoggingContext, HttpLoggingMiddleware, HttpLoggingRequest,
    HttpLoggingResponse, HttpLogLevel, Pipeline,
};

#[cfg(feature = "ratelimit")]
use dev_engineeringlabs_rustboot_middleware::{
    CustomKeyExtractor, HeaderKeyExtractor, HttpContext, RateLimitConfig, RateLimitMiddleware,
};

#[cfg(feature = "ratelimit")]
use std::sync::Arc;
#[cfg(feature = "ratelimit")]
use std::time::Duration;

#[tokio::test]
async fn test_http_logging_integration() {
    // Create a logging middleware with custom configuration
    let config = HttpLoggingConfig::builder()
        .request_level(HttpLogLevel::Info)
        .response_level(HttpLogLevel::Info)
        .log_request_headers(true)
        .log_response_headers(true)
        .log_request_body(true)
        .log_response_body(true)
        .max_body_size(2048)
        .track_request_id(true)
        .build();

    let logging = HttpLoggingMiddleware::with_config(config);

    // Create a pipeline with the logging middleware
    let pipeline = Pipeline::new().with_middleware(logging);

    // Create a test request
    let request = HttpLoggingRequest::new("POST".to_string(), "/api/v1/users".to_string())
        .with_header("Content-Type".to_string(), "application/json".to_string())
        .with_header(
            "Authorization".to_string(),
            "Bearer test-token".to_string(),
        )
        .with_body(b"{\"name\":\"Alice\",\"email\":\"alice@example.com\"}".to_vec());

    let ctx = HttpLoggingContext::new(request);

    // Execute the pipeline
    let result = pipeline.execute(ctx).await;
    assert!(result.is_ok());

    let mut ctx = result.unwrap();

    // Verify request ID was generated
    assert!(ctx.request.request_id.is_some());

    // Simulate adding a response
    ctx = ctx.with_response(
        HttpLoggingResponse::new(201)
            .with_header("Content-Type".to_string(), "application/json".to_string())
            .with_header("Location".to_string(), "/api/v1/users/123".to_string())
            .with_body(b"{\"id\":123,\"name\":\"Alice\",\"email\":\"alice@example.com\"}".to_vec()),
    );

    // Verify response was set
    assert!(ctx.response.is_some());
    assert_eq!(ctx.response.as_ref().unwrap().status, 201);
}

#[tokio::test]
async fn test_http_logging_request_id_extraction() {
    let config = HttpLoggingConfig::builder()
        .track_request_id(true)
        .request_id_header("X-Request-ID".to_string())
        .build();

    let logging = HttpLoggingMiddleware::with_config(config);
    let pipeline = Pipeline::new().with_middleware(logging);

    let custom_request_id = "req-12345-abcde";
    let request = HttpLoggingRequest::new("GET".to_string(), "/health".to_string())
        .with_header("X-Request-ID".to_string(), custom_request_id.to_string());

    let ctx = HttpLoggingContext::new(request);
    let result = pipeline.execute(ctx).await;

    assert!(result.is_ok());
    let ctx = result.unwrap();
    assert_eq!(ctx.request.request_id.as_deref(), Some(custom_request_id));
}

#[tokio::test]
async fn test_http_logging_without_body_logging() {
    let config = HttpLoggingConfig::builder()
        .log_request_body(false)
        .log_response_body(false)
        .build();

    let logging = HttpLoggingMiddleware::with_config(config);
    let pipeline = Pipeline::new().with_middleware(logging);

    let request = HttpLoggingRequest::new("POST".to_string(), "/api/data".to_string())
        .with_body(b"sensitive data that should not be logged".to_vec());

    let ctx = HttpLoggingContext::new(request);
    let result = pipeline.execute(ctx).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_http_logging_large_body_truncation() {
    let config = HttpLoggingConfig::builder()
        .log_request_body(true)
        .max_body_size(50)
        .build();

    let logging = HttpLoggingMiddleware::with_config(config);
    let pipeline = Pipeline::new().with_middleware(logging);

    let large_body = vec![b'X'; 1000];
    let request =
        HttpLoggingRequest::new("POST".to_string(), "/upload".to_string()).with_body(large_body);

    let ctx = HttpLoggingContext::new(request.clone());
    let result = pipeline.execute(ctx).await;

    assert!(result.is_ok());

    // Verify body preview is truncated
    let preview = request.body_preview(50);
    assert!(preview.is_some());
    let preview_str = preview.unwrap();
    assert!(preview_str.contains("1000 bytes"));
}

#[tokio::test]
async fn test_http_logging_different_log_levels() {
    let config = HttpLoggingConfig::builder()
        .request_level(HttpLogLevel::Debug)
        .response_level(HttpLogLevel::Trace)
        .build();

    let logging = HttpLoggingMiddleware::with_config(config);
    let pipeline = Pipeline::new().with_middleware(logging);

    let request = HttpLoggingRequest::new("DELETE".to_string(), "/api/items/42".to_string());
    let ctx = HttpLoggingContext::new(request);

    let result = pipeline.execute(ctx).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_http_logging_multiple_headers() {
    let logging = HttpLoggingMiddleware::new();
    let pipeline = Pipeline::new().with_middleware(logging);

    let request = HttpLoggingRequest::new("GET".to_string(), "/api/products".to_string())
        .with_header("Accept".to_string(), "application/json".to_string())
        .with_header("User-Agent".to_string(), "RustBoot/1.0".to_string())
        .with_header("Accept-Language".to_string(), "en-US".to_string())
        .with_header("Cache-Control".to_string(), "no-cache".to_string());

    let ctx = HttpLoggingContext::new(request);
    let result = pipeline.execute(ctx).await;

    assert!(result.is_ok());
    let ctx = result.unwrap();
    assert_eq!(ctx.request.headers.len(), 4);
}

#[tokio::test]
async fn test_http_logging_binary_body() {
    let config = HttpLoggingConfig::builder()
        .log_request_body(true)
        .max_body_size(100)
        .build();

    let logging = HttpLoggingMiddleware::with_config(config);
    let pipeline = Pipeline::new().with_middleware(logging);

    // Binary data that can't be converted to UTF-8
    let binary_body = vec![0xFF, 0xFE, 0xFD, 0xFC, 0xFB];
    let request =
        HttpLoggingRequest::new("POST".to_string(), "/upload/binary".to_string())
            .with_body(binary_body.clone());

    let ctx = HttpLoggingContext::new(request.clone());
    let result = pipeline.execute(ctx).await;

    assert!(result.is_ok());

    // Verify binary data is handled properly
    let preview = request.body_preview(100);
    assert!(preview.is_some());
    let preview_str = preview.unwrap();
    assert!(preview_str.contains("binary data"));
}

// Rate Limiting Integration Tests
#[cfg(feature = "ratelimit")]
#[tokio::test]
async fn test_rate_limit_fixed_window_integration() {
    let config = RateLimitConfig::FixedWindow {
        max_requests: 3,
        window_size: Duration::from_secs(60),
    };
    let middleware = Arc::new(RateLimitMiddleware::new(config).with_headers(true));

    let client_ip = "10.0.0.1";

    // First 3 requests should succeed
    for i in 1..=3 {
        let ctx = HttpContext::new("GET".to_string(), "/api/data".to_string())
            .with_client_ip(client_ip.to_string());
        let pipeline = Pipeline::new().with_middleware(Arc::clone(&middleware));
        let result = pipeline.execute(ctx).await;
        assert!(result.is_ok(), "Request {} should succeed", i);

        let ctx = result.unwrap();
        assert!(ctx.response_headers.contains_key("X-RateLimit-Limit"));
        assert_eq!(
            ctx.response_headers.get("X-RateLimit-Limit"),
            Some(&"3".to_string())
        );
    }

    // 4th request should fail
    let ctx = HttpContext::new("GET".to_string(), "/api/data".to_string())
        .with_client_ip(client_ip.to_string());
    let pipeline = Pipeline::new().with_middleware(Arc::clone(&middleware));
    let result = pipeline.execute(ctx).await;
    assert!(result.is_err(), "4th request should be rate limited");
}

#[cfg(feature = "ratelimit")]
#[tokio::test]
async fn test_rate_limit_token_bucket_integration() {
    let config = RateLimitConfig::TokenBucket {
        capacity: 5,
        refill_rate: 1,
        refill_interval: Duration::from_millis(100),
    };
    let middleware = Arc::new(RateLimitMiddleware::new(config));

    let client_ip = "10.0.0.2";

    // Use all 5 tokens
    for i in 1..=5 {
        let ctx = HttpContext::new("POST".to_string(), "/api/create".to_string())
            .with_client_ip(client_ip.to_string());
        let pipeline = Pipeline::new().with_middleware(Arc::clone(&middleware));
        let result = pipeline.execute(ctx).await;
        assert!(result.is_ok(), "Request {} should succeed", i);
    }

    // 6th request should fail (no tokens left)
    let ctx = HttpContext::new("POST".to_string(), "/api/create".to_string())
        .with_client_ip(client_ip.to_string());
    let pipeline = Pipeline::new().with_middleware(Arc::clone(&middleware));
    let result = pipeline.execute(ctx).await;
    assert!(result.is_err(), "6th request should be rate limited");

    // Wait for refill
    tokio::time::sleep(Duration::from_millis(150)).await;

    // Should work again after refill
    let ctx = HttpContext::new("POST".to_string(), "/api/create".to_string())
        .with_client_ip(client_ip.to_string());
    let pipeline = Pipeline::new().with_middleware(Arc::clone(&middleware));
    let result = pipeline.execute(ctx).await;
    assert!(
        result.is_ok(),
        "Request after refill should succeed"
    );
}

#[cfg(feature = "ratelimit")]
#[tokio::test]
async fn test_rate_limit_api_key_integration() {
    let config = RateLimitConfig::FixedWindow {
        max_requests: 2,
        window_size: Duration::from_secs(60),
    };

    let extractor = Arc::new(HeaderKeyExtractor::new("X-API-Key"));
    let middleware = Arc::new(RateLimitMiddleware::with_key_extractor(config, extractor));

    // API Key 1 - use both requests
    for i in 1..=2 {
        let ctx = HttpContext::new("GET".to_string(), "/api/v1/users".to_string())
            .with_header("X-API-Key".to_string(), "key-abc-123".to_string());
        let pipeline = Pipeline::new().with_middleware(Arc::clone(&middleware));
        let result = pipeline.execute(ctx).await;
        assert!(result.is_ok(), "API key 1 request {} should succeed", i);
    }

    // API Key 1 - 3rd request should fail
    let ctx = HttpContext::new("GET".to_string(), "/api/v1/users".to_string())
        .with_header("X-API-Key".to_string(), "key-abc-123".to_string());
    let pipeline = Pipeline::new().with_middleware(Arc::clone(&middleware));
    let result = pipeline.execute(ctx).await;
    assert!(result.is_err(), "API key 1 should be rate limited");

    // API Key 2 - should still work (separate limit)
    let ctx = HttpContext::new("GET".to_string(), "/api/v1/users".to_string())
        .with_header("X-API-Key".to_string(), "key-xyz-789".to_string());
    let pipeline = Pipeline::new().with_middleware(Arc::clone(&middleware));
    let result = pipeline.execute(ctx).await;
    assert!(
        result.is_ok(),
        "Different API key should have separate limit"
    );
}

#[cfg(feature = "ratelimit")]
#[tokio::test]
async fn test_rate_limit_custom_extractor_integration() {
    let config = RateLimitConfig::SlidingWindow {
        max_requests: 2,
        window_size: Duration::from_secs(60),
    };

    // Rate limit by user ID from URL
    let extractor = Arc::new(CustomKeyExtractor::new(|ctx: &HttpContext| {
        ctx.url
            .split('/')
            .nth(3)
            .map(|id| format!("user:{}", id))
    }));

    let middleware = Arc::new(RateLimitMiddleware::with_key_extractor(config, extractor));

    // User 42 - use both requests
    for i in 1..=2 {
        let ctx = HttpContext::new(
            "GET".to_string(),
            "/api/users/42/profile".to_string(),
        );
        let pipeline = Pipeline::new().with_middleware(Arc::clone(&middleware));
        let result = pipeline.execute(ctx).await;
        assert!(result.is_ok(), "User 42 request {} should succeed", i);
    }

    // User 42 - 3rd request should fail
    let ctx = HttpContext::new(
        "GET".to_string(),
        "/api/users/42/profile".to_string(),
    );
    let pipeline = Pipeline::new().with_middleware(Arc::clone(&middleware));
    let result = pipeline.execute(ctx).await;
    assert!(result.is_err(), "User 42 should be rate limited");

    // User 99 - should still work (separate limit)
    let ctx = HttpContext::new(
        "GET".to_string(),
        "/api/users/99/profile".to_string(),
    );
    let pipeline = Pipeline::new().with_middleware(Arc::clone(&middleware));
    let result = pipeline.execute(ctx).await;
    assert!(
        result.is_ok(),
        "Different user should have separate limit"
    );
}

#[cfg(feature = "ratelimit")]
#[tokio::test]
async fn test_rate_limit_no_key_rejection() {
    let config = RateLimitConfig::FixedWindow {
        max_requests: 10,
        window_size: Duration::from_secs(60),
    };
    let middleware = RateLimitMiddleware::new(config);
    let pipeline = Pipeline::new().with_middleware(middleware);

    // Request without client IP should be rejected
    let ctx = HttpContext::new("GET".to_string(), "/api/data".to_string());
    let result = pipeline.execute(ctx).await;
    assert!(
        result.is_err(),
        "Request without rate limit key should be rejected"
    );
}

#[cfg(feature = "ratelimit")]
#[tokio::test]
async fn test_rate_limit_headers_included() {
    let config = RateLimitConfig::FixedWindow {
        max_requests: 5,
        window_size: Duration::from_secs(60),
    };
    let middleware = Arc::new(RateLimitMiddleware::new(config).with_headers(true));

    let ctx = HttpContext::new("GET".to_string(), "/test".to_string())
        .with_client_ip("192.168.1.1".to_string());
    let pipeline = Pipeline::new().with_middleware(Arc::clone(&middleware));
    let result = pipeline.execute(ctx).await;

    assert!(result.is_ok());
    let ctx = result.unwrap();

    // Verify rate limit headers are present
    assert!(ctx.response_headers.contains_key("X-RateLimit-Limit"));
    assert!(ctx.response_headers.contains_key("X-RateLimit-Remaining"));
}

#[cfg(feature = "ratelimit")]
#[tokio::test]
async fn test_rate_limit_headers_disabled() {
    let config = RateLimitConfig::FixedWindow {
        max_requests: 5,
        window_size: Duration::from_secs(60),
    };
    let middleware = Arc::new(RateLimitMiddleware::new(config).with_headers(false));

    let ctx = HttpContext::new("GET".to_string(), "/test".to_string())
        .with_client_ip("192.168.1.1".to_string());
    let pipeline = Pipeline::new().with_middleware(Arc::clone(&middleware));
    let result = pipeline.execute(ctx).await;

    assert!(result.is_ok());
    let ctx = result.unwrap();

    // Verify rate limit headers are NOT present
    assert!(!ctx.response_headers.contains_key("X-RateLimit-Limit"));
    assert!(!ctx.response_headers.contains_key("X-RateLimit-Remaining"));
}
