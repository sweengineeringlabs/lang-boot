//! HTTP Rate Limiting Middleware Example
//!
//! This example demonstrates how to use the HTTP rate limiting middleware
//! with various algorithms and key extraction strategies.

use dev_engineeringlabs_rustboot_middleware::{
    HeaderKeyExtractor, HttpContext, Pipeline, RateLimitConfig, RateLimitMiddleware,
};
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("=== Rustboot HTTP Rate Limiting Example ===\n");

    // Example 1: IP-based rate limiting with Fixed Window
    println!("Example 1: IP-based rate limiting with Fixed Window");
    ip_based_rate_limiting().await;

    // Example 2: API key-based rate limiting with Token Bucket
    println!("\nExample 2: API key-based rate limiting with Token Bucket");
    api_key_rate_limiting().await;

    // Example 3: Custom key extraction with Sliding Window
    println!("\nExample 3: Custom key extraction with Sliding Window");
    custom_key_rate_limiting().await;

    // Example 4: Leaky Bucket algorithm
    println!("\nExample 4: Leaky Bucket algorithm");
    leaky_bucket_rate_limiting().await;
}

async fn ip_based_rate_limiting() {
    // Create a Fixed Window rate limiter: 5 requests per minute
    let config = RateLimitConfig::FixedWindow {
        max_requests: 5,
        window_size: Duration::from_secs(60),
    };

    let middleware = Arc::new(RateLimitMiddleware::new(config));

    println!("  Config: 5 requests per 60 seconds (Fixed Window)");
    println!("  Key extraction: Client IP address");

    let client_ip = "192.168.1.100";

    // Simulate 7 requests from the same IP
    for i in 1..=7 {
        let ctx = HttpContext::new("GET".to_string(), "/api/data".to_string())
            .with_client_ip(client_ip.to_string());

        let pipeline = Pipeline::new().with_middleware(Arc::clone(&middleware));
        let result = pipeline.execute(ctx).await;

        match result {
            Ok(ctx) => {
                println!(
                    "  Request {}: Allowed (Remaining: {})",
                    i,
                    ctx.response_headers
                        .get("X-RateLimit-Remaining")
                        .unwrap_or(&"?".to_string())
                );
            }
            Err(e) => {
                println!("  Request {}: Rejected - {}", i, e);
            }
        }
    }
}

async fn api_key_rate_limiting() {
    // Create a Token Bucket rate limiter: 10 tokens, refill 2 per second
    let config = RateLimitConfig::TokenBucket {
        capacity: 10,
        refill_rate: 2,
        refill_interval: Duration::from_secs(1),
    };

    let extractor = Arc::new(HeaderKeyExtractor::new("X-API-Key"));
    let middleware = Arc::new(RateLimitMiddleware::with_key_extractor(config, extractor));

    println!("  Config: 10 tokens, refill 2 per second (Token Bucket)");
    println!("  Key extraction: X-API-Key header");

    let api_key_1 = "key-abc123";
    let api_key_2 = "key-xyz789";

    // Simulate requests from two different API keys
    println!("  Requests from API key: {}", api_key_1);
    for i in 1..=5 {
        let ctx = HttpContext::new("POST".to_string(), "/api/create".to_string())
            .with_header("X-API-Key".to_string(), api_key_1.to_string());

        let pipeline = Pipeline::new().with_middleware(Arc::clone(&middleware));
        let result = pipeline.execute(ctx).await;

        match result {
            Ok(_) => println!("    Request {}: Allowed", i),
            Err(e) => println!("    Request {}: Rejected - {}", i, e),
        }
    }

    // Requests from different API key should have separate limit
    println!("  Requests from API key: {}", api_key_2);
    for i in 1..=5 {
        let ctx = HttpContext::new("POST".to_string(), "/api/create".to_string())
            .with_header("X-API-Key".to_string(), api_key_2.to_string());

        let pipeline = Pipeline::new().with_middleware(Arc::clone(&middleware));
        let result = pipeline.execute(ctx).await;

        match result {
            Ok(_) => println!("    Request {}: Allowed", i),
            Err(e) => println!("    Request {}: Rejected - {}", i, e),
        }
    }
}

async fn custom_key_rate_limiting() {
    use dev_engineeringlabs_rustboot_middleware::CustomKeyExtractor;

    // Create a Sliding Window rate limiter: 3 requests per 10 seconds
    let config = RateLimitConfig::SlidingWindow {
        max_requests: 3,
        window_size: Duration::from_secs(10),
    };

    // Custom extractor: rate limit per user ID from URL path
    let extractor = Arc::new(CustomKeyExtractor::new(|ctx: &HttpContext| {
        // Extract user ID from path like /users/123/profile
        ctx.url
            .split('/')
            .nth(2)
            .map(|id| format!("user:{}", id))
    }));

    let middleware = Arc::new(RateLimitMiddleware::with_key_extractor(config, extractor));

    println!("  Config: 3 requests per 10 seconds (Sliding Window)");
    println!("  Key extraction: User ID from URL path");

    // Simulate requests for different users
    let users = vec!["123", "456"];

    for user_id in users {
        println!("  Requests for user {}:", user_id);
        for i in 1..=4 {
            let ctx =
                HttpContext::new("GET".to_string(), format!("/users/{}/profile", user_id));

            let pipeline = Pipeline::new().with_middleware(Arc::clone(&middleware));
            let result = pipeline.execute(ctx).await;

            match result {
                Ok(_) => println!("    Request {}: Allowed", i),
                Err(e) => println!("    Request {}: Rejected - {}", i, e),
            }
        }
    }
}

async fn leaky_bucket_rate_limiting() {
    // Create a Leaky Bucket rate limiter: capacity 5, leaks 1 per second
    let config = RateLimitConfig::LeakyBucket {
        capacity: 5,
        leak_rate: 1,
        leak_interval: Duration::from_secs(1),
    };

    let middleware = Arc::new(RateLimitMiddleware::new(config));

    println!("  Config: capacity 5, leaks 1 per second (Leaky Bucket)");
    println!("  Key extraction: Client IP address");

    let client_ip = "10.0.0.50";

    // Simulate burst of requests
    println!("  Burst of 8 requests:");
    for i in 1..=8 {
        let ctx = HttpContext::new("GET".to_string(), "/api/status".to_string())
            .with_client_ip(client_ip.to_string());

        let pipeline = Pipeline::new().with_middleware(Arc::clone(&middleware));
        let result = pipeline.execute(ctx).await;

        match result {
            Ok(_) => println!("    Request {}: Allowed", i),
            Err(e) => println!("    Request {}: Rejected - {}", i, e),
        }
    }

    // Wait for bucket to drain
    println!("  Waiting 2 seconds for bucket to drain...");
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Try again after waiting
    println!("  Requests after waiting:");
    for i in 1..=3 {
        let ctx = HttpContext::new("GET".to_string(), "/api/status".to_string())
            .with_client_ip(client_ip.to_string());

        let pipeline = Pipeline::new().with_middleware(Arc::clone(&middleware));
        let result = pipeline.execute(ctx).await;

        match result {
            Ok(_) => println!("    Request {}: Allowed", i),
            Err(e) => println!("    Request {}: Rejected - {}", i, e),
        }
    }
}
