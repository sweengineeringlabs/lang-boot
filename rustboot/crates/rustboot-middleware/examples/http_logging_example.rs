//! HTTP Logging Middleware Example
//!
//! This example demonstrates how to use the HTTP logging middleware to log
//! HTTP requests and responses with various configuration options.

use dev_engineeringlabs_rustboot_middleware::{
    HttpLoggingConfig, HttpLoggingContext, HttpLoggingMiddleware, HttpLoggingRequest,
    HttpLoggingResponse, HttpLogLevel, Pipeline,
};

#[tokio::main]
async fn main() {
    // Initialize tracing to see the log output
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    println!("=== Rustboot HTTP Logging Middleware Example ===\n");

    // Example 1: Basic logging with default configuration
    println!("Example 1: Basic HTTP logging with defaults");
    basic_logging_example().await;

    println!("\n---\n");

    // Example 2: Custom configuration with body logging
    println!("Example 2: Custom configuration with request/response body logging");
    custom_config_example().await;

    println!("\n---\n");

    // Example 3: Request ID tracking
    println!("Example 3: Request ID tracking and extraction");
    request_id_example().await;

    println!("\n---\n");

    // Example 4: Different log levels
    println!("Example 4: Different log levels for requests and responses");
    log_levels_example().await;

    println!("\n---\n");

    // Example 5: Large body handling
    println!("Example 5: Large body truncation");
    large_body_example().await;
}

/// Example 1: Basic logging with default configuration
async fn basic_logging_example() {
    // Create middleware with default config
    let logging = HttpLoggingMiddleware::new();
    let pipeline = Pipeline::new().with_middleware(logging);

    // Create a simple GET request
    let request = HttpLoggingRequest::new("GET".to_string(), "/api/users".to_string())
        .with_header("Accept".to_string(), "application/json".to_string())
        .with_header("User-Agent".to_string(), "RustBoot/1.0".to_string());

    let mut ctx = HttpLoggingContext::new(request);

    // Execute the pipeline
    let result = pipeline.execute(ctx.clone()).await;

    if let Ok(result_ctx) = result {
        ctx = result_ctx;
        println!("✓ Request processed");
        println!("  Request ID: {:?}", ctx.request.request_id);

        // Simulate adding a successful response
        ctx = ctx.with_response(
            HttpLoggingResponse::new(200)
                .with_header("Content-Type".to_string(), "application/json".to_string())
                .with_body(b"[{\"id\":1,\"name\":\"Alice\"},{\"id\":2,\"name\":\"Bob\"}]".to_vec()),
        );
        println!("  Response status: {}", ctx.response.as_ref().unwrap().status);
    }
}

/// Example 2: Custom configuration with body logging
async fn custom_config_example() {
    // Create custom configuration
    let config = HttpLoggingConfig::builder()
        .request_level(HttpLogLevel::Info)
        .response_level(HttpLogLevel::Info)
        .log_request_headers(true)
        .log_response_headers(true)
        .log_request_body(true)
        .log_response_body(true)
        .max_body_size(512)
        .track_request_id(true)
        .build();

    let logging = HttpLoggingMiddleware::with_config(config);
    let pipeline = Pipeline::new().with_middleware(logging);

    // Create a POST request with body
    let request = HttpLoggingRequest::new("POST".to_string(), "/api/users".to_string())
        .with_header("Content-Type".to_string(), "application/json".to_string())
        .with_header(
            "Authorization".to_string(),
            "Bearer secret-token".to_string(),
        )
        .with_body(b"{\"name\":\"Charlie\",\"email\":\"charlie@example.com\",\"role\":\"admin\"}".to_vec());

    let mut ctx = HttpLoggingContext::new(request);

    // Execute the pipeline
    let result = pipeline.execute(ctx.clone()).await;

    if let Ok(result_ctx) = result {
        ctx = result_ctx;
        println!("✓ POST request processed with body logging");

        // Simulate adding a created response
        ctx = ctx.with_response(
            HttpLoggingResponse::new(201)
                .with_header("Content-Type".to_string(), "application/json".to_string())
                .with_header("Location".to_string(), "/api/users/3".to_string())
                .with_body(
                    b"{\"id\":3,\"name\":\"Charlie\",\"email\":\"charlie@example.com\",\"role\":\"admin\"}"
                        .to_vec(),
                ),
        );
        println!("  Created with ID: 3");
    }
}

/// Example 3: Request ID tracking
async fn request_id_example() {
    // Configure with custom request ID header
    let config = HttpLoggingConfig::builder()
        .track_request_id(true)
        .request_id_header("X-Request-ID".to_string())
        .build();

    let logging = HttpLoggingMiddleware::with_config(config);
    let pipeline = Pipeline::new().with_middleware(logging);

    // Request with existing request ID
    let custom_id = "req-abc-123-xyz";
    let request = HttpLoggingRequest::new("GET".to_string(), "/api/health".to_string())
        .with_header("X-Request-ID".to_string(), custom_id.to_string());

    let ctx = HttpLoggingContext::new(request);
    let result = pipeline.execute(ctx).await;

    if let Ok(ctx) = result {
        println!("✓ Request ID extracted from header");
        println!("  Request ID: {:?}", ctx.request.request_id);
        assert_eq!(ctx.request.request_id.as_deref(), Some(custom_id));
    }

    // Request without request ID (auto-generated)
    let request2 = HttpLoggingRequest::new("GET".to_string(), "/api/health".to_string());
    let ctx2 = HttpLoggingContext::new(request2);

    let logging2 = HttpLoggingMiddleware::with_config(
        HttpLoggingConfig::builder()
            .track_request_id(true)
            .build(),
    );
    let pipeline2 = Pipeline::new().with_middleware(logging2);

    let result2 = pipeline2.execute(ctx2).await;

    if let Ok(ctx2) = result2 {
        println!("✓ Request ID auto-generated");
        println!("  Request ID: {:?}", ctx2.request.request_id);
        assert!(ctx2.request.request_id.is_some());
    }
}

/// Example 4: Different log levels
async fn log_levels_example() {
    // Configure with different log levels
    let config = HttpLoggingConfig::builder()
        .request_level(HttpLogLevel::Debug)
        .response_level(HttpLogLevel::Trace)
        .build();

    let logging = HttpLoggingMiddleware::with_config(config);
    let pipeline = Pipeline::new().with_middleware(logging);

    // DELETE request
    let request = HttpLoggingRequest::new("DELETE".to_string(), "/api/users/42".to_string())
        .with_header("Authorization".to_string(), "Bearer admin-token".to_string());

    let mut ctx = HttpLoggingContext::new(request);
    let result = pipeline.execute(ctx.clone()).await;

    if let Ok(result_ctx) = result {
        ctx = result_ctx;
        println!("✓ Request logged at DEBUG level");

        // Simulate successful deletion response
        ctx = ctx.with_response(HttpLoggingResponse::new(204));
        println!("  Response logged at TRACE level");
        println!("  Status: 204 No Content");
    }
}

/// Example 5: Large body handling
async fn large_body_example() {
    // Configure with small max body size to demonstrate truncation
    let config = HttpLoggingConfig::builder()
        .log_request_body(true)
        .max_body_size(50)
        .build();

    let logging = HttpLoggingMiddleware::with_config(config);
    let pipeline = Pipeline::new().with_middleware(logging);

    // Create a large request body
    let large_body = "X".repeat(500);
    let request = HttpLoggingRequest::new("POST".to_string(), "/api/upload".to_string())
        .with_header("Content-Type".to_string(), "text/plain".to_string())
        .with_body(large_body.into_bytes());

    let ctx = HttpLoggingContext::new(request.clone());
    let result = pipeline.execute(ctx).await;

    if let Ok(_) = result {
        println!("✓ Large body logged with truncation");
        let preview = request.body_preview(50);
        if let Some(preview_str) = preview {
            println!("  Body preview: {}", preview_str);
        }
    }
}
