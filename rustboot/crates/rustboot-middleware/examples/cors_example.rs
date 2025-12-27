//! CORS Middleware Example
//!
//! This example demonstrates the CORS (Cross-Origin Resource Sharing) middleware
//! with various configurations and use cases.

use dev_engineeringlabs_rustboot_middleware::{
    CorsConfig, CorsMiddleware, HttpContext, Pipeline,
};
use std::collections::HashMap;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("=== Rustboot CORS Middleware Examples ===\n");

    example_1_permissive().await;
    example_2_specific_origins().await;
    example_3_regex_origins().await;
    example_4_preflight_request().await;
    example_5_with_credentials().await;
    example_6_custom_configuration().await;
}

/// Example 1: Permissive CORS - Allow all origins
async fn example_1_permissive() {
    println!("Example 1: Permissive CORS (Allow all origins)");
    println!("-----------------------------------------------");

    let cors = CorsMiddleware::permissive();
    let pipeline = Pipeline::new().with_middleware(cors);

    let mut headers = HashMap::new();
    headers.insert("Origin".to_string(), "https://example.com".to_string());

    let ctx = HttpContext::from_headers("GET".to_string(), headers);
    let result = pipeline.execute(ctx).await;

    match result {
        Ok(ctx) => {
            println!("✓ Request allowed");
            println!("  Access-Control-Allow-Origin: {}",
                ctx.response_headers.get("Access-Control-Allow-Origin").unwrap_or(&"(not set)".to_string()));
        }
        Err(e) => println!("✗ Request rejected: {}", e),
    }
    println!();
}

/// Example 2: Specific allowed origins
async fn example_2_specific_origins() {
    println!("Example 2: Specific Allowed Origins");
    println!("------------------------------------");

    let config = CorsConfig::new().allow_origins(vec![
        "https://example.com".to_string(),
        "https://app.example.com".to_string(),
    ]);
    let cors = CorsMiddleware::new(config);

    // Test allowed origin
    let pipeline1 = Pipeline::new().with_middleware(cors);
    let mut headers1 = HashMap::new();
    headers1.insert("Origin".to_string(), "https://example.com".to_string());
    let ctx1 = HttpContext::from_headers("GET".to_string(), headers1);

    match pipeline1.execute(ctx1).await {
        Ok(ctx) => {
            println!("✓ Allowed origin (https://example.com)");
            println!("  Access-Control-Allow-Origin: {}",
                ctx.response_headers.get("Access-Control-Allow-Origin").unwrap_or(&"(not set)".to_string()));
        }
        Err(e) => println!("✗ Request rejected: {}", e),
    }

    // Test rejected origin
    let config2 = CorsConfig::new().allow_origins(vec![
        "https://example.com".to_string(),
    ]);
    let cors2 = CorsMiddleware::new(config2);
    let pipeline2 = Pipeline::new().with_middleware(cors2);
    let mut headers2 = HashMap::new();
    headers2.insert("Origin".to_string(), "https://evil.com".to_string());
    let ctx2 = HttpContext::from_headers("GET".to_string(), headers2);

    match pipeline2.execute(ctx2).await {
        Ok(_) => println!("✓ Request allowed (unexpected!)"),
        Err(e) => println!("✓ Rejected origin (https://evil.com): {}", e),
    }
    println!();
}

/// Example 3: Regex pattern for origins
async fn example_3_regex_origins() {
    println!("Example 3: Regex Pattern Origins");
    println!("---------------------------------");

    let config = CorsConfig::new()
        .allow_origin_regex(r"^https://.*\.example\.com$")
        .expect("Valid regex");
    let cors = CorsMiddleware::new(config);

    // Test matching subdomain
    let pipeline1 = Pipeline::new().with_middleware(cors);
    let mut headers1 = HashMap::new();
    headers1.insert("Origin".to_string(), "https://api.example.com".to_string());
    let ctx1 = HttpContext::from_headers("GET".to_string(), headers1);

    match pipeline1.execute(ctx1).await {
        Ok(ctx) => {
            println!("✓ Matched regex pattern (https://api.example.com)");
            println!("  Pattern: ^https://.*\\.example\\.com$");
            println!("  Access-Control-Allow-Origin: {}",
                ctx.response_headers.get("Access-Control-Allow-Origin").unwrap_or(&"(not set)".to_string()));
        }
        Err(e) => println!("✗ Request rejected: {}", e),
    }

    // Test non-matching domain
    let config2 = CorsConfig::new()
        .allow_origin_regex(r"^https://.*\.example\.com$")
        .expect("Valid regex");
    let cors2 = CorsMiddleware::new(config2);
    let pipeline2 = Pipeline::new().with_middleware(cors2);
    let mut headers2 = HashMap::new();
    headers2.insert("Origin".to_string(), "https://example.org".to_string());
    let ctx2 = HttpContext::from_headers("GET".to_string(), headers2);

    match pipeline2.execute(ctx2).await {
        Ok(_) => println!("✓ Request allowed (unexpected!)"),
        Err(e) => println!("✓ Did not match pattern (https://example.org): {}", e),
    }
    println!();
}

/// Example 4: Preflight request handling
async fn example_4_preflight_request() {
    println!("Example 4: Preflight Request (OPTIONS)");
    println!("---------------------------------------");

    let config = CorsConfig::new()
        .allow_origins(vec!["https://example.com".to_string()])
        .allow_methods(vec!["GET".to_string(), "POST".to_string(), "DELETE".to_string()])
        .allow_headers(vec!["Content-Type".to_string(), "Authorization".to_string()])
        .max_age(Duration::from_secs(7200));
    let cors = CorsMiddleware::new(config);
    let pipeline = Pipeline::new().with_middleware(cors);

    let mut headers = HashMap::new();
    headers.insert("Origin".to_string(), "https://example.com".to_string());
    headers.insert("Access-Control-Request-Method".to_string(), "POST".to_string());
    headers.insert("Access-Control-Request-Headers".to_string(), "Content-Type, Authorization".to_string());

    let ctx = HttpContext::from_headers("OPTIONS".to_string(), headers);
    let result = pipeline.execute(ctx).await;

    match result {
        Ok(ctx) => {
            println!("✓ Preflight request successful");
            println!("  Status: {:?}", ctx.status);
            println!("  Access-Control-Allow-Origin: {}",
                ctx.response_headers.get("Access-Control-Allow-Origin").unwrap_or(&"(not set)".to_string()));
            println!("  Access-Control-Allow-Methods: {}",
                ctx.response_headers.get("Access-Control-Allow-Methods").unwrap_or(&"(not set)".to_string()));
            println!("  Access-Control-Allow-Headers: {}",
                ctx.response_headers.get("Access-Control-Allow-Headers").unwrap_or(&"(not set)".to_string()));
            println!("  Access-Control-Max-Age: {}",
                ctx.response_headers.get("Access-Control-Max-Age").unwrap_or(&"(not set)".to_string()));
        }
        Err(e) => println!("✗ Request rejected: {}", e),
    }
    println!();
}

/// Example 5: CORS with credentials support
async fn example_5_with_credentials() {
    println!("Example 5: CORS with Credentials");
    println!("---------------------------------");

    let config = CorsConfig::new()
        .allow_origins(vec!["https://example.com".to_string()])
        .allow_credentials(true);
    let cors = CorsMiddleware::new(config);
    let pipeline = Pipeline::new().with_middleware(cors);

    let mut headers = HashMap::new();
    headers.insert("Origin".to_string(), "https://example.com".to_string());

    let ctx = HttpContext::from_headers("GET".to_string(), headers);
    let result = pipeline.execute(ctx).await;

    match result {
        Ok(ctx) => {
            println!("✓ Request with credentials support");
            println!("  Access-Control-Allow-Origin: {}",
                ctx.response_headers.get("Access-Control-Allow-Origin").unwrap_or(&"(not set)".to_string()));
            println!("  Access-Control-Allow-Credentials: {}",
                ctx.response_headers.get("Access-Control-Allow-Credentials").unwrap_or(&"(not set)".to_string()));
            println!("  Vary: {}",
                ctx.response_headers.get("Vary").unwrap_or(&"(not set)".to_string()));
        }
        Err(e) => println!("✗ Request rejected: {}", e),
    }
    println!();
}

/// Example 6: Custom comprehensive configuration
async fn example_6_custom_configuration() {
    println!("Example 6: Custom Comprehensive Configuration");
    println!("----------------------------------------------");

    let config = CorsConfig::new()
        .allow_origins(vec!["https://example.com".to_string()])
        .allow_methods(vec![
            "GET".to_string(),
            "POST".to_string(),
            "PUT".to_string(),
            "DELETE".to_string(),
        ])
        .allow_headers(vec![
            "Content-Type".to_string(),
            "Authorization".to_string(),
            "X-Custom-Header".to_string(),
        ])
        .expose_headers(vec![
            "X-Total-Count".to_string(),
            "X-Request-Id".to_string(),
        ])
        .allow_credentials(true)
        .max_age(Duration::from_secs(86400)); // 24 hours

    let cors = CorsMiddleware::new(config);
    let pipeline = Pipeline::new().with_middleware(cors);

    let mut headers = HashMap::new();
    headers.insert("Origin".to_string(), "https://example.com".to_string());

    let ctx = HttpContext::from_headers("POST".to_string(), headers);
    let result = pipeline.execute(ctx).await;

    match result {
        Ok(ctx) => {
            println!("✓ Custom configuration applied successfully");
            println!("  Configuration details:");
            println!("    - Allowed origins: https://example.com");
            println!("    - Allowed methods: GET, POST, PUT, DELETE");
            println!("    - Allowed headers: Content-Type, Authorization, X-Custom-Header");
            println!("    - Exposed headers: X-Total-Count, X-Request-Id");
            println!("    - Credentials: enabled");
            println!("    - Max age: 86400 seconds (24 hours)");
            println!("  Response headers:");
            println!("    Access-Control-Allow-Origin: {}",
                ctx.response_headers.get("Access-Control-Allow-Origin").unwrap_or(&"(not set)".to_string()));
            println!("    Access-Control-Allow-Credentials: {}",
                ctx.response_headers.get("Access-Control-Allow-Credentials").unwrap_or(&"(not set)".to_string()));
            println!("    Access-Control-Expose-Headers: {}",
                ctx.response_headers.get("Access-Control-Expose-Headers").unwrap_or(&"(not set)".to_string()));
        }
        Err(e) => println!("✗ Request rejected: {}", e),
    }
    println!();
}
