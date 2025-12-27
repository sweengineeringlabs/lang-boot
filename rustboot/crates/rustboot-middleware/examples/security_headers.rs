//! Security Headers Middleware Example
//!
//! This example demonstrates how to use the SecurityHeadersMiddleware to add
//! security-related HTTP headers to responses.
//!
//! Run with: cargo run --example security_headers

use dev_engineeringlabs_rustboot_middleware::{
    security::{HasHeaders, SecurityHeadersConfig, SecurityHeadersMiddleware},
    Pipeline,
};

/// Example HTTP response context that can have headers added.
#[derive(Debug, Clone)]
struct HttpResponse {
    status: u16,
    body: String,
    headers: Vec<(String, String)>,
}

impl HttpResponse {
    fn new(status: u16, body: impl Into<String>) -> Self {
        Self {
            status,
            body: body.into(),
            headers: Vec::new(),
        }
    }

    fn print_headers(&self) {
        println!("\nHTTP Response Headers:");
        println!("Status: {}", self.status);
        for (name, value) in &self.headers {
            println!("{}: {}", name, value);
        }
        println!("\nBody: {}", self.body);
    }
}

impl HasHeaders for HttpResponse {
    fn add_header(&mut self, name: String, value: String) {
        self.headers.push((name, value));
    }
}

#[tokio::main]
async fn main() {
    println!("=== Rustboot Security Headers Middleware Example ===\n");

    // Example 1: Default secure configuration
    println!("Example 1: Default Secure Configuration");
    println!("----------------------------------------");
    let middleware = SecurityHeadersMiddleware::default();
    let pipeline = Pipeline::new().with_middleware(middleware);

    let response = HttpResponse::new(200, "Hello, World!");
    let result = pipeline.execute(response).await;

    if let Ok(response) = result {
        response.print_headers();
    }

    println!("\n");

    // Example 2: Permissive configuration for development
    println!("Example 2: Permissive Configuration (Development)");
    println!("--------------------------------------------------");
    let middleware = SecurityHeadersMiddleware::permissive();
    let pipeline = Pipeline::new().with_middleware(middleware);

    let response = HttpResponse::new(200, "Development Response");
    let result = pipeline.execute(response).await;

    if let Ok(response) = result {
        response.print_headers();
    }

    println!("\n");

    // Example 3: Custom configuration
    println!("Example 3: Custom Configuration");
    println!("--------------------------------");
    let config = SecurityHeadersConfig::new()
        .with_csp("default-src 'self'; script-src 'self' https://cdn.example.com; style-src 'self' 'unsafe-inline'")
        .with_hsts(63072000, true, false) // 2 years, with subdomains, no preload
        .with_frame_options("SAMEORIGIN")
        .with_content_type_options("nosniff")
        .with_referrer_policy("strict-origin-when-cross-origin")
        .with_permissions_policy("geolocation=(self), microphone=(), camera=(), payment=()");

    let middleware = SecurityHeadersMiddleware::new(config);
    let pipeline = Pipeline::new().with_middleware(middleware);

    let response = HttpResponse::new(200, "Custom Security Headers");
    let result = pipeline.execute(response).await;

    if let Ok(response) = result {
        response.print_headers();
    }

    println!("\n");

    // Example 4: Selective headers (disable some headers)
    println!("Example 4: Selective Headers");
    println!("-----------------------------");
    let config = SecurityHeadersConfig::default()
        .without_xss_protection() // X-XSS-Protection is deprecated
        .without_permissions_policy()
        .with_csp("default-src 'self'; img-src * data:; script-src 'self' 'unsafe-inline'");

    let middleware = SecurityHeadersMiddleware::new(config);
    let pipeline = Pipeline::new().with_middleware(middleware);

    let response = HttpResponse::new(200, "Selective Headers Response");
    let result = pipeline.execute(response).await;

    if let Ok(response) = result {
        response.print_headers();
    }

    println!("\n");

    // Example 5: API endpoint configuration (minimal headers)
    println!("Example 5: API Endpoint Configuration");
    println!("--------------------------------------");
    let config = SecurityHeadersConfig::new()
        .with_hsts(31536000, true, true)
        .with_content_type_options("nosniff")
        .with_frame_options("DENY")
        .with_referrer_policy("no-referrer");

    let middleware = SecurityHeadersMiddleware::new(config);
    let pipeline = Pipeline::new().with_middleware(middleware);

    let response = HttpResponse::new(200, r#"{"status": "ok", "data": []}"#);
    let result = pipeline.execute(response).await;

    if let Ok(response) = result {
        response.print_headers();
    }

    println!("\n");

    // Example 6: Combining with other middleware
    println!("Example 6: Combined with Timing Middleware");
    println!("-------------------------------------------");
    use dev_engineeringlabs_rustboot_middleware::TimingMiddleware;

    let security = SecurityHeadersMiddleware::default();
    let timing = TimingMiddleware::new("security-example");

    let pipeline = Pipeline::new()
        .with_middleware(timing)
        .with_middleware(security);

    let response = HttpResponse::new(200, "Combined Middleware Response");
    let result = pipeline.execute(response).await;

    if let Ok(response) = result {
        response.print_headers();
    }

    println!("\n=== Security Headers Best Practices ===");
    println!("\n1. Content-Security-Policy (CSP):");
    println!("   - Prevents XSS attacks by controlling which resources can be loaded");
    println!("   - Start strict ('self') and gradually add exceptions as needed");
    println!("   - Avoid 'unsafe-inline' and 'unsafe-eval' in production");

    println!("\n2. Strict-Transport-Security (HSTS):");
    println!("   - Forces HTTPS connections for improved security");
    println!("   - Use long max-age (1-2 years) for production");
    println!("   - Only enable after confirming HTTPS works correctly");

    println!("\n3. X-Frame-Options:");
    println!("   - Prevents clickjacking attacks");
    println!("   - Use 'DENY' unless you need iframe support");
    println!("   - Use 'SAMEORIGIN' if you need same-origin iframes");

    println!("\n4. X-Content-Type-Options:");
    println!("   - Always set to 'nosniff' to prevent MIME sniffing");
    println!("   - Prevents browsers from interpreting files as different types");

    println!("\n5. Referrer-Policy:");
    println!("   - Controls how much referrer information is sent");
    println!("   - 'strict-origin-when-cross-origin' is a good default");
    println!("   - Use 'no-referrer' for maximum privacy");

    println!("\n6. Permissions-Policy:");
    println!("   - Controls access to browser features (camera, microphone, etc.)");
    println!("   - Disable features you don't need");
    println!("   - Helps prevent malicious code from accessing sensitive APIs");

    println!("\n7. X-XSS-Protection:");
    println!("   - Legacy header, modern browsers use CSP instead");
    println!("   - Safe to disable if you have a strong CSP");
    println!("   - Keep enabled for older browser compatibility");
}
