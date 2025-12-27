//! Basic HTTP Client Example
//!
//! This example demonstrates basic HTTP operations using the ReqwestClient.
//!
//! Run with: cargo run --example http_basic --features reqwest

use dev_engineeringlabs_rustboot_http::{HttpClient, Method, Request, ReqwestClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Rustboot HTTP Basic Example ===\n");

    // Create a new HTTP client with default settings (30s timeout)
    let client = ReqwestClient::new();

    // Example 1: Simple GET request
    println!("1. Simple GET request:");
    let response = client.get("https://httpbin.org/get").await?;
    println!("   Status: {}", response.status);
    println!("   Success: {}", response.is_success());
    println!("   Body length: {} bytes\n", response.body.len());

    // Example 2: GET with query parameters (encoded in URL)
    println!("2. GET with query parameters:");
    let response = client
        .get("https://httpbin.org/get?name=rustboot&version=0.1")
        .await?;
    println!("   Status: {}", response.status);
    println!("   Response: {}\n", &response.text()?[..200.min(response.body.len())]);

    // Example 3: POST request with body
    println!("3. POST request with body:");
    let response = client
        .post("https://httpbin.org/post", b"Hello, World!".to_vec())
        .await?;
    println!("   Status: {}", response.status);
    println!("   Success: {}\n", response.is_success());

    // Example 4: Custom request with headers
    println!("4. Custom request with headers:");
    let request = Request::new(Method::Get, "https://httpbin.org/headers".to_string())
        .header("X-Custom-Header".to_string(), "custom-value".to_string())
        .header("Accept".to_string(), "application/json".to_string());

    let response = client.send(request).await?;
    println!("   Status: {}", response.status);
    println!("   Response shows our custom headers in the echo\n");

    // Example 5: Check response headers
    println!("5. Reading response headers:");
    let response = client.get("https://httpbin.org/response-headers?X-Test=hello").await?;
    println!("   Status: {}", response.status);
    if let Some(content_type) = response.headers.get("content-type") {
        println!("   Content-Type: {}", content_type);
    }

    println!("\n=== Example Complete ===");
    Ok(())
}
