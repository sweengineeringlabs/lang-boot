//! Comprehensive example usage of Rustboot macros
//!
//! This file demonstrates practical usage patterns for the Rustboot macro library.
//! Note: Many macros require runtime dependencies to execute. This example focuses
//! on demonstrating the syntax and patterns for using these macros.

#![allow(dead_code, unused_imports, unused_variables)]

use rustboot_macros::{Builder, Validate};
use std::sync::Arc;

// ============================================================================
// Derive Macros
// ============================================================================

/// Example: Dependency Injection (requires rustboot-di)
///
/// The Injectable derive macro would generate code to automatically resolve
/// dependencies from a dependency injection container.
//
// #[derive(Injectable)]
// struct UserService {
//     repository: Arc<dyn UserRepository>,
//     cache: Arc<dyn Cache>,
//     logger: Arc<Logger>,
// }
//
// trait UserRepository: Send + Sync {}
// trait Cache: Send + Sync {}
// struct Logger;

/// Example: Validation
///
/// The Validate derive macro automatically generates validation logic based on
/// field-level validation attributes.
#[derive(Validate)]
struct CreateUserRequest {
    #[validate(length(min = 3, max = 50))]
    username: String,

    #[validate(email)]
    email: String,

    #[validate(range(min = 18, max = 120))]
    age: u8,

    #[validate(length(min = 8))]
    password: String,
}

#[derive(Validate)]
struct UpdateUserRequest {
    #[validate(length(min = 1, max = 100))]
    display_name: String,

    #[validate(length(min = 10, max = 200))]
    bio: String,
}

/// Example: Builder pattern
///
/// The Builder derive macro generates a builder implementation for ergonomic
/// struct construction with optional fields.
#[derive(Builder)]
struct ServerConfig {
    host: String,
    port: u16,
    timeout: Option<u64>,
    max_connections: Option<usize>,
    enable_tls: Option<bool>,
}

#[derive(Builder)]
struct DatabaseConfig {
    url: String,
    pool_size: Option<u32>,
    max_lifetime: Option<u64>,
    min_idle: Option<u32>,
}

// ============================================================================
// Attribute Macros - Examples (Most require runtime dependencies)
// ============================================================================

struct ProductService {
    db: Arc<Database>,
}

struct Database;
struct Product { id: u64 }
struct Order;
struct ExternalApi;

#[derive(Debug)]
struct Error;

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "error")
    }
}

impl ProductService {
    /// Example: Regular method (caching shown below as standalone function)
    async fn get_product(&self, id: u64) -> Result<Product, Error> {
        // This would fetch from database
        Ok(Product { id })
    }

    // Tracing example (requires rustboot-observability)
    // #[traced(level = "info")]
    // async fn process_order(&self, order: Order) -> Result<(), Error> {
    //     // Automatically creates tracing span with:
    //     // - Function name
    //     // - Parameters
    //     // - Execution duration
    //     // - Result status
    //     Ok(())
    // }

    // Retry example (requires rustboot-resilience)
    // #[retry(max_attempts = 3, backoff = "exponential", delay = 100)]
    // async fn call_external_api(&self) -> Result<ApiResponse, Error> {
    //     // Automatically retried up to 3 times with exponential backoff
    //     Ok(ApiResponse)
    // }

    // Performance timing example (requires rustboot-observability)
    // #[timed(name = "product_search", slow_threshold = 100)]
    // async fn search_products(&self, query: &str) -> Result<Vec<Product>, Error> {
    //     // Execution time is:
    //     // - Recorded to metrics system as "product_search"
    //     // - Logged as warning if > 100ms
    //     Ok(vec![])
    // }

    // Circuit breaker example (requires rustboot-resilience)
    // #[circuit_breaker(failure_threshold = 5, timeout = 60)]
    // async fn call_unreliable_service(&self) -> Result<Response, Error> {
    //     // Circuit opens after 5 consecutive failures
    //     // Tries to close after 60 seconds
    //     Ok(Response)
    // }

    // Audit logging example (requires rustboot-security)
    // #[audit(action = "delete_product", severity = "high")]
    // async fn delete_product(&self, id: u64, admin_id: u64) -> Result<(), Error> {
    //     // Automatically logged to audit system
    //     Ok(())
    // }
}

// ============================================================================
// Standalone Functions (for cached macro demonstration)
// ============================================================================

// Caching example (requires rustboot-cache)
// Note: cached and memoize macros only work with standalone functions,
// not methods in impl blocks, due to Rust's static item restrictions
//
// #[cached(ttl = 600, capacity = 1000)]
// fn get_product_cached(id: u64) -> Result<Product, Error> {
//     // This result is cached for 10 minutes
//     // Second call with same ID returns cached value
//     Ok(Product { id })
// }

// ============================================================================
// Combining Multiple Macros
// ============================================================================

struct AnalyticsService {
    external_api: Arc<ExternalApi>,
}

impl AnalyticsService {
    // Example: Combining multiple cross-cutting concerns
    // Note: Most of these require runtime dependencies
    //
    // #[traced(level = "info")]           // Trace execution (rustboot-observability)
    // #[timed(slow_threshold = 500)]      // Monitor performance (rustboot-observability)
    // #[retry(max_attempts = 3)]          // Retry on failure (rustboot-resilience)
    // async fn get_analytics_report(&self, user_id: u64) -> Result<Report, Error> {
    //     // This function benefits from:
    //     // 1. Detailed tracing for debugging
    //     // 2. Performance monitoring
    //     // 3. Automatic retries for resilience
    //
    //     // The macros execute in order (outside-in):
    //     // traced -> timed -> retry -> actual function
    //     Ok(Report)
    // }

    // Example: Complex operation with full observability
    //
    // #[traced(level = "info", name = "user_journey_analysis")]
    // #[timed(name = "journey_analysis")]
    // #[retry(max_attempts = 2, backoff = "fixed", delay = 200)]
    // #[circuit_breaker(failure_threshold = 10)]
    // #[audit(action = "analyze_user_journey", severity = "medium")]
    // async fn analyze_user_journey(&self, user_id: u64) -> Result<Journey, Error> {
    //     // Fully instrumented with:
    //     // - Tracing (custom span name)
    //     // - Performance metrics
    //     // - Retry logic
    //     // - Circuit breaker protection
    //     // - Audit logging
    //     Ok(Journey)
    // }
}

// ============================================================================
// Type Definitions
// ============================================================================

struct Session;
struct ApiResponse;
struct Data;
struct Response;
struct Report;
struct Journey;

// ============================================================================
// Usage Examples
// ============================================================================

fn main() {
    println!("=== Rustboot Macros Usage Examples ===\n");

    // Example 1: Builder Pattern
    println!("1. Builder Pattern\n");
    println!("   Create configurations with optional fields:");
    println!("   ```rust");
    println!("   let config = ServerConfig::builder()");
    println!("       .host(\"0.0.0.0\".to_string())");
    println!("       .port(8080)");
    println!("       .timeout(Some(30))");
    println!("       .max_connections(Some(100))");
    println!("       .build()?;");
    println!("   ```\n");

    // Example 2: Validation
    println!("2. Validation\n");
    println!("   Validate user input automatically:");
    println!("   ```rust");
    println!("   let request = CreateUserRequest {{");
    println!("       username: \"john_doe\".to_string(),");
    println!("       email: \"john@example.com\".to_string(),");
    println!("       age: 25,");
    println!("       password: \"secure_password\".to_string(),");
    println!("   }};");
    println!("   ");
    println!("   match request.validate() {{");
    println!("       Ok(()) => println!(\"Validation passed!\"),");
    println!("       Err(errors) => println!(\"Errors: {{:?}}\", errors),");
    println!("   }}");
    println!("   ```\n");

    // Example 3: Dependency Injection (commented out)
    println!("3. Dependency Injection (requires rustboot-di)\n");
    println!("   ```rust");
    println!("   #[derive(Injectable)]");
    println!("   struct UserService {{");
    println!("       repository: Arc<dyn UserRepository>,");
    println!("       cache: Arc<dyn Cache>,");
    println!("   }}");
    println!("   ");
    println!("   let container = create_di_container();");
    println!("   let service = UserService::from_container(&container)?;");
    println!("   ```\n");

    println!("Available Macros:\n");

    println!("Derive Macros:");
    println!("  - Builder: Generate builder pattern implementation");
    println!("  - Injectable: Dependency injection (requires rustboot-di)");
    println!("  - Validate: Automatic validation");
    println!("  - Event: Event trait (requires serde)");
    println!("  - OpenApiSchema: OpenAPI schema generation");
    println!();

    println!("Attribute Macros:");
    println!("  Observability (rustboot-observability):");
    println!("    - timed, traced, metrics_histogram, audit");
    println!("  Resilience (rustboot-resilience):");
    println!("    - retry, timeout, circuit_breaker");
    println!("  Security (rustboot-security):");
    println!("    - authorized, transactional");
    println!("  Caching (rustboot-cache):");
    println!("    - cached, memoize");
    println!("  Other:");
    println!("    - rate_limit (rustboot-ratelimit)");
    println!("    - feature_flag (rustboot-config)");
    println!("    - validate_params (rustboot-validation)");
    println!("    - http_request (rustboot-http)");
    println!();

    println!("Note: Most attribute macros require additional runtime crates.");
    println!("See the source code for detailed examples of each macro's usage.");
}
