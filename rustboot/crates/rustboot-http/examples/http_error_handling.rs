//! HTTP Error Handling Example
//!
//! This example demonstrates proper error handling with the HTTP client.
//!
//! Run with: cargo run --example http_error_handling --features reqwest

use dev_engineeringlabs_rustboot_http::{HttpClient, HttpError, ReqwestClient};
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("=== Rustboot HTTP Error Handling Example ===\n");

    // Example 1: Handle HTTP error status codes
    println!("1. Handling HTTP error status codes:");
    let client = ReqwestClient::new();

    let response = client.get("https://httpbin.org/status/404").await;
    match response {
        Ok(resp) => {
            if resp.is_success() {
                println!("   Success: {}", resp.status);
            } else {
                println!("   HTTP Error: {} (not a network error, just a 4xx/5xx response)", resp.status);
            }
        }
        Err(e) => println!("   Network Error: {}", e),
    }
    println!();

    // Example 2: Handle different status codes
    println!("2. Checking various status codes:");
    for status in [200, 201, 301, 400, 401, 403, 404, 500, 503] {
        let url = format!("https://httpbin.org/status/{}", status);
        match client.get(&url).await {
            Ok(resp) => {
                let status_type = match resp.status {
                    200..=299 => "SUCCESS",
                    300..=399 => "REDIRECT",
                    400..=499 => "CLIENT ERROR",
                    500..=599 => "SERVER ERROR",
                    _ => "UNKNOWN",
                };
                println!("   {} -> {} ({})", status, resp.status, status_type);
            }
            Err(e) => println!("   {} -> Error: {}", status, e),
        }
    }
    println!();

    // Example 3: Handle connection errors
    println!("3. Handling connection errors:");
    let result = client.get("http://localhost:59999/nonexistent").await;
    match result {
        Ok(_) => println!("   Unexpected success"),
        Err(HttpError::Connection(msg)) => println!("   Connection error (expected): {}", msg),
        Err(HttpError::Timeout) => println!("   Timeout error"),
        Err(HttpError::Request(msg)) => println!("   Request error: {}", msg),
    }
    println!();

    // Example 4: Handle timeouts with short timeout
    println!("4. Handling timeouts:");
    let short_timeout_client = ReqwestClient::with_timeout(Duration::from_millis(1));
    let result = short_timeout_client.get("https://httpbin.org/delay/5").await;
    match result {
        Ok(_) => println!("   Unexpected success"),
        Err(HttpError::Timeout) => println!("   Timeout error (expected with 1ms timeout)"),
        Err(e) => println!("   Other error: {}", e),
    }
    println!();

    // Example 5: Graceful error recovery pattern
    println!("5. Graceful error recovery pattern:");
    async fn fetch_with_fallback(client: &ReqwestClient) -> String {
        // Try primary endpoint
        match client.get("https://httpbin.org/status/500").await {
            Ok(resp) if resp.is_success() => {
                return resp.text().unwrap_or_else(|_| "Success".to_string());
            }
            Ok(resp) => {
                println!("   Primary failed with status {}, trying fallback...", resp.status);
            }
            Err(e) => {
                println!("   Primary failed with error: {}, trying fallback...", e);
            }
        }

        // Try fallback endpoint
        match client.get("https://httpbin.org/get").await {
            Ok(resp) if resp.is_success() => {
                println!("   Fallback succeeded!");
                "Fallback response".to_string()
            }
            Ok(resp) => format!("Fallback also failed: {}", resp.status),
            Err(e) => format!("Fallback error: {}", e),
        }
    }

    let result = fetch_with_fallback(&client).await;
    println!("   Final result: {}\n", result);

    // Example 6: Retry pattern
    println!("6. Simple retry pattern:");
    async fn fetch_with_retry(client: &ReqwestClient, url: &str, max_retries: u32) -> Result<String, String> {
        let mut last_error = String::new();

        for attempt in 1..=max_retries {
            println!("   Attempt {}/{}...", attempt, max_retries);

            match client.get(url).await {
                Ok(resp) if resp.is_success() => {
                    return Ok(format!("Success on attempt {}", attempt));
                }
                Ok(resp) => {
                    last_error = format!("HTTP {}", resp.status);
                }
                Err(e) => {
                    last_error = e.to_string();
                }
            }

            if attempt < max_retries {
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        }

        Err(format!("All {} attempts failed. Last error: {}", max_retries, last_error))
    }

    // This will fail all retries (503 Service Unavailable)
    match fetch_with_retry(&client, "https://httpbin.org/status/503", 3).await {
        Ok(msg) => println!("   {}", msg),
        Err(msg) => println!("   {}", msg),
    }

    println!("\n=== Example Complete ===");
}
