//! HTTP Client Builder Example
//!
//! This example demonstrates using the builder pattern to configure the HTTP client.
//!
//! Run with: cargo run --example http_builder --features reqwest

use dev_engineeringlabs_rustboot_http::{HttpClient, ReqwestClient, ReqwestClientBuilder};
use std::collections::HashMap;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Rustboot HTTP Builder Example ===\n");

    // Example 1: Simple builder usage
    println!("1. Simple builder with timeout:");
    let client = ReqwestClient::builder()
        .timeout(Duration::from_secs(60))
        .build()?;

    let response = client.get("https://httpbin.org/get").await?;
    println!("   Status: {}\n", response.status);

    // Example 2: Builder with User-Agent
    println!("2. Custom User-Agent:");
    let client = ReqwestClient::builder()
        .user_agent("RustbootApp/1.0 (https://github.com/example/rustboot)")
        .build()?;

    let response = client.get("https://httpbin.org/user-agent").await?;
    println!("   Response: {}\n", response.text()?);

    // Example 3: Builder with default headers (API client pattern)
    println!("3. API client with default headers:");
    let api_client = ReqwestClient::builder()
        .timeout(Duration::from_secs(30))
        .user_agent("MyAPIClient/2.0")
        .default_header("Accept", "application/json")
        .default_header("X-Api-Version", "2024-01-01")
        .build()?;

    let response = api_client.get("https://httpbin.org/headers").await?;
    println!("   Our headers are echoed back:");
    println!("   {}\n", &response.text()?[..500.min(response.body.len())]);

    // Example 4: Builder with multiple default headers at once
    println!("4. Multiple headers at once:");
    let mut headers = HashMap::new();
    headers.insert("X-Custom-Header-1".to_string(), "value1".to_string());
    headers.insert("X-Custom-Header-2".to_string(), "value2".to_string());
    headers.insert("X-Request-Source".to_string(), "rustboot-example".to_string());

    let client = ReqwestClient::builder()
        .default_headers(headers)
        .build()?;

    let response = client.get("https://httpbin.org/headers").await?;
    println!("   Status: {}", response.status);
    println!("   Custom headers sent successfully\n");

    // Example 5: Builder with connection settings
    println!("5. Connection pool configuration:");
    let pooled_client = ReqwestClient::builder()
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(5))
        .pool_idle_timeout(Duration::from_secs(90))
        .pool_max_idle_per_host(10)
        .build()?;

    // Make multiple requests to demonstrate connection reuse
    for i in 1..=3 {
        let response = pooled_client.get("https://httpbin.org/get").await?;
        println!("   Request {}: Status {}", i, response.status);
    }
    println!();

    // Example 6: Builder with redirect control
    println!("6. Redirect control:");

    // Client that follows redirects (default behavior, limited to 5)
    let redirect_client = ReqwestClient::builder()
        .max_redirects(5)
        .build()?;

    let response = redirect_client
        .get("https://httpbin.org/redirect/2")
        .await?;
    println!("   With redirects enabled: Status {} (followed redirects)", response.status);

    // Client that doesn't follow redirects
    let no_redirect_client = ReqwestClient::builder()
        .no_redirects()
        .build()?;

    let response = no_redirect_client
        .get("https://httpbin.org/redirect/2")
        .await?;
    println!("   With redirects disabled: Status {} (got redirect response)\n", response.status);

    // Example 7: Full production-ready client configuration
    println!("7. Production-ready client configuration:");
    let production_client = ReqwestClient::builder()
        // Timeouts
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(10))
        // Identity
        .user_agent("ProductionApp/1.0.0")
        // Default headers for all requests
        .default_header("Accept", "application/json")
        .default_header("Accept-Encoding", "gzip, deflate")
        // Security: limit redirects to prevent redirect loops
        .max_redirects(5)
        // Connection pooling for performance
        .pool_idle_timeout(Duration::from_secs(60))
        .pool_max_idle_per_host(20)
        .build()?;

    let response = production_client.get("https://httpbin.org/get").await?;
    println!("   Production client status: {}", response.status);
    println!("   Configuration applied successfully\n");

    // Example 8: Using ReqwestClientBuilder directly
    println!("8. Using ReqwestClientBuilder::new() directly:");
    let client = ReqwestClientBuilder::new()
        .timeout(Duration::from_secs(15))
        .user_agent("DirectBuilder/1.0")
        .build()?;

    let response = client.get("https://httpbin.org/get").await?;
    println!("   Status: {}\n", response.status);

    println!("=== Example Complete ===");
    Ok(())
}
