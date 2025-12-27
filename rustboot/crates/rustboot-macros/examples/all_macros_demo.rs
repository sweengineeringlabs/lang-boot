//! Example demonstrating Rustboot macros usage patterns
//!
//! This example demonstrates the syntax for using Rustboot macros.
//! Note: Many macros require additional runtime crates to execute:
//!   - rustboot-observability (for traced, timed, metrics)
//!   - rustboot-resilience (for retry, timeout, circuit_breaker)
//!   - rustboot-cache (for cached)
//!   - rustboot-ratelimit (for rate_limit)
//!   - rustboot-security (for audit, authorized)
//!   - rustboot-database (for transactional)
//!   - rustboot-config (for feature_flag)
//!   - rustboot-validation (for validate_params)
//!   - rustboot-di (for Injectable derive)
//!   - serde (for Event derive)
//!
//! This example focuses on the Builder and Validate derive macros
//! which have complete standalone implementations.

use rustboot_macros::*;

// ============================================================================
// Derive Macros - Builder Pattern
// ============================================================================

/// Example: Builder pattern generation
///
/// The Builder derive macro automatically generates a builder implementation
/// for the struct, allowing ergonomic construction with default values.
#[derive(Builder)]
struct ServerConfig {
    host: String,
    port: u16,
    timeout: Option<u64>,
    max_connections: Option<usize>,
}

#[derive(Builder)]
struct DatabaseConfig {
    url: String,
    pool_size: Option<u32>,
    max_lifetime: Option<u64>,
}

// ============================================================================
// Derive Macros - Validation
// ============================================================================

/// Example: Validation with various constraints
///
/// The Validate derive macro generates validation logic based on
/// field-level attributes.
#[derive(Validate)]
struct UserRegistration {
    #[validate(length(min = 3, max = 50))]
    username: String,

    #[validate(email)]
    email: String,

    #[validate(range(min = 18, max = 120))]
    age: u8,

    #[validate(length(min = 8))]
    password: String,
}

// Note: Validate macro currently doesn't support Option fields
// #[derive(Validate)]
// struct ProfileUpdate {
//     #[validate(length(max = 500))]
//     bio: Option<String>,
//
//     #[validate(length(max = 100))]
//     website: Option<String>,
//
//     #[validate(length(max = 200))]
//     location: Option<String>,
// }

#[derive(Validate)]
struct ProductInfo {
    #[validate(length(min = 1, max = 100))]
    name: String,

    #[validate(length(max = 1000))]
    description: String,

    #[validate(range(min = 0, max = 1000000))]
    price_cents: u32,
}

// ============================================================================
// Examples of other macros (commented out due to missing dependencies)
// ============================================================================

// Injectable - Dependency Injection (requires rustboot-di)
// #[derive(Injectable)]
// struct UserService {
//     database: Arc<Database>,
//     cache: Arc<Cache>,
// }

// Event - Event trait implementation (requires serde)
// #[derive(Event)]
// #[event(type_name = "user_created", version = "1.0")]
// struct UserCreatedEvent {
//     user_id: String,
//     timestamp: u64,
// }

// OpenApiSchema - OpenAPI schema generation
// #[derive(OpenApiSchema)]
// struct ApiUser {
//     id: u64,
//     name: String,
//     email: String,
// }

// ============================================================================
// Attribute Macros - Observability (require rustboot-observability)
// ============================================================================

// #[timed]
// fn expensive_operation() -> u64 {
//     // Execution time is automatically measured
//     42
// }

// #[traced(level = "info")]
// async fn process_request(id: u64) {
//     // Automatically creates tracing span with parameters
// }

// #[metrics_histogram(name = "api_latency")]
// async fn api_call() -> Result<(), String> {
//     // Records latency to Prometheus histogram
//     Ok(())
// }

// ============================================================================
// Attribute Macros - Resilience (require rustboot-resilience)
// ============================================================================

// #[retry(max_attempts = 3, backoff = "exponential", delay = 100)]
// async fn flaky_network_call() -> Result<String, String> {
//     // Automatically retried with exponential backoff
//     Ok("success".to_string())
// }

// #[timeout(duration = 5000)]
// async fn slow_operation() -> Result<(), String> {
//     // Times out after 5 seconds
//     Ok(())
// }

// #[circuit_breaker(failure_threshold = 5, timeout = 60000)]
// async fn protected_call() -> Result<(), String> {
//     // Circuit opens after 5 failures, tries to close after 60s
//     Ok(())
// }

// ============================================================================
// Attribute Macros - Caching (require rustboot-cache)
// ============================================================================

// Note: These macros only work with standalone functions (not methods in impl blocks)
// due to Rust's restriction on static items in associated contexts

// #[cached(ttl = 300, capacity = 1000)]
// fn expensive_computation(input: i32) -> i32 {
//     // Result cached for 5 minutes
//     input * 2
// }

// #[memoize]
// fn fibonacci(n: u64) -> u64 {
//     // Permanently memoized for performance
//     if n <= 1 { n } else { fibonacci(n-1) + fibonacci(n-2) }
// }

// ============================================================================
// Attribute Macros - Rate Limiting (requires rustboot-ratelimit)
// ============================================================================

// #[rate_limit(requests = 100, window = 60)]
// fn api_endpoint() -> Result<String, String> {
//     // Limited to 100 requests per 60 seconds
//     Ok("response".to_string())
// }

// ============================================================================
// Attribute Macros - Security (require rustboot-security)
// ============================================================================

// #[audit(action = "user_login", resource = "auth")]
// fn login(username: &str) -> Result<(), String> {
//     // Automatically logged to audit system
//     Ok(())
// }

// #[authorized(role = "admin")]
// async fn admin_function() -> Result<(), String> {
//     // Only admins can execute
//     Ok(())
// }

// ============================================================================
// Attribute Macros - Database (requires rustboot-database)
// ============================================================================

// #[transactional]
// async fn create_user(user: User) -> Result<User, String> {
//     // Automatically wrapped in database transaction
//     Ok(user)
// }

// ============================================================================
// Attribute Macros - Feature Management (requires rustboot-config)
// ============================================================================

// #[feature_flag(flag = "new_algorithm")]
// fn experimental_feature() -> String {
//     // Only executes if feature flag is enabled
//     "new implementation".to_string()
// }

// ============================================================================
// Attribute Macros - Validation (requires rustboot-validation)
// ============================================================================

// #[validate_params]
// fn validate_inputs(
//     #[validate(range(min = 0, max = 100))] percentage: i32,
//     #[validate(length(min = 1))] name: String,
// ) -> Result<(), String> {
//     // Parameters automatically validated
//     Ok(())
// }

// ============================================================================
// Attribute Macros - HTTP (requires rustboot-http)
// ============================================================================

// struct ApiClient {
//     base_url: String,
//     client: HttpClient,
// }
//
// impl ApiClient {
//     #[http_request(method = "GET", path = "/users/{id}")]
//     async fn get_user(&self, id: u64) -> Result<User, String> {}
//
//     #[http_request(method = "POST", path = "/users", body)]
//     async fn create_user(&self, user: CreateUser) -> Result<User, String> {}
// }

fn main() {
    println!("=== Rustboot Macros - Usage Examples ===");
    println!();

    // Example 1: Builder pattern
    println!("1. Builder Pattern:");
    println!("   let config = ServerConfig::builder()");
    println!("       .host(\"localhost\".to_string())");
    println!("       .port(8080)");
    println!("       .timeout(Some(30))");
    println!("       .build()?;");
    println!();

    // Example 2: Validation
    println!("2. Validation:");
    println!("   let user = UserRegistration {{");
    println!("       username: \"alice\".to_string(),");
    println!("       email: \"alice@example.com\".to_string(),");
    println!("       age: 25,");
    println!("       password: \"secure123\".to_string(),");
    println!("   }};");
    println!("   user.validate()?;");
    println!();

    println!("Available Derive Macros:");
    println!("  - Builder: Generate builder pattern");
    println!("  - Injectable: Dependency injection (requires rustboot-di)");
    println!("  - Validate: Automatic validation");
    println!("  - Event: Event trait (requires serde)");
    println!("  - OpenApiSchema: OpenAPI schema generation");
    println!();

    println!("Available Attribute Macros:");
    println!("  Observability:");
    println!("    - timed, traced, metrics_histogram, audit");
    println!("  Resilience:");
    println!("    - retry, timeout, circuit_breaker");
    println!("  Security:");
    println!("    - authorized, transactional");
    println!("  Caching:");
    println!("    - cached, memoize");
    println!("  Other:");
    println!("    - rate_limit, feature_flag, validate_params");
    println!("    - http_request");
    println!();

    println!("Note: Most attribute macros require additional runtime crates.");
    println!("See the source code comments for details.");
}
