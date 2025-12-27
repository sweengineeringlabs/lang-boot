//! Comprehensive example demonstrating all debug utilities.

use dev_engineeringlabs_rustboot_debug::{
    debug_log, debug_only, is_debug_mode, timing::*, ConfigDumper, ContainerIntrospector,
    HttpDumpConfig, HttpDumpMiddleware, QueryLogger, QueryLoggerConfig, StateMachineVisualizer,
};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

// For HTTP debugging
#[cfg(feature = "http")]
use rustboot_middleware::{HttpContext, Middleware, traits::Next};

// For config debugging
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AppConfig {
    server: ServerConfig,
    database: DatabaseConfig,
    features: Features,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
    workers: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DatabaseConfig {
    url: String,
    max_connections: u32,
    password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Features {
    caching: bool,
    rate_limiting: bool,
    compression: bool,
}

// For state machine visualization
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum OrderState {
    Created,
    PaymentPending,
    PaymentConfirmed,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum OrderEvent {
    SubmitPayment,
    ConfirmPayment,
    StartProcessing,
    Ship,
    Deliver,
    Cancel,
}

// For DI introspection
#[derive(Clone)]
struct UserService {
    name: String,
}

#[derive(Clone)]
struct EmailService {
    smtp_host: String,
}

#[derive(Clone)]
struct PaymentService {
    api_key: String,
}

fn main() {
    // Initialize tracing for better log output
    tracing_subscriber::fmt()
        .with_target(true)
        .with_max_level(tracing::Level::DEBUG)
        .init();

    println!("=== Rustboot Debug Utilities Demo ===\n");

    // 1. Debug mode check
    demo_debug_mode();

    // 2. Timing utilities
    demo_timing_utilities();

    // 3. Database query logging
    demo_database_debugging();

    // 4. State machine visualization
    demo_state_machine_viz();

    // 5. DI container introspection
    demo_di_introspection();

    // 6. Configuration dumping
    demo_config_dump();

    // 7. HTTP debugging (if feature enabled)
    #[cfg(feature = "http")]
    demo_http_debugging();

    println!("\n=== Demo Complete ===");
}

fn demo_debug_mode() {
    println!("\n--- 1. Debug Mode Detection ---");
    println!("Debug mode enabled: {}", is_debug_mode());

    debug_only! {
        println!("This message only appears in debug builds!");
    }

    debug_log!("This is a debug log message with location info");
}

fn demo_timing_utilities() {
    println!("\n--- 2. Timing Utilities ---");

    // Basic timing guard
    {
        let _guard = TimingGuard::new("example_operation");
        simulate_work(50);
    } // Timing logged on drop

    // Timing with custom thresholds
    {
        let thresholds = TimingThresholds::new()
            .with_warn_threshold(Duration::from_millis(200))
            .with_info_threshold(Duration::from_millis(100));

        let _guard = TimingGuard::with_thresholds("custom_threshold_operation", thresholds);
        simulate_work(150);
    }

    // Timing with checkpoints
    {
        let guard = TimingGuard::new("multi_step_operation");
        simulate_work(30);
        guard.checkpoint("step_1_complete");
        simulate_work(30);
        guard.checkpoint("step_2_complete");
        simulate_work(30);
    }

    // Synchronous timing helper
    let result = time_sync("sync_calculation", || {
        simulate_work(20);
        42
    });
    println!("Sync result: {}", result);

    // Async timing (would need tokio runtime in real usage)
    println!("Timing utilities demonstrated!");
}

fn demo_database_debugging() {
    println!("\n--- 3. Database Query Logging ---");

    let config = QueryLoggerConfig::new()
        .with_log_all(true)
        .with_slow_threshold(Duration::from_millis(50))
        .with_log_result_count(true);

    let logger = Arc::new(QueryLogger::with_config(config));

    // Simulate some queries
    let queries = vec![
        ("SELECT * FROM users WHERE id = ?", 10),
        ("SELECT * FROM orders WHERE status = ?", 100),
        ("UPDATE products SET price = ? WHERE id = ?", 5),
    ];

    for (sql, delay_ms) in queries {
        let timer = std::time::Instant::now();
        simulate_work(delay_ms);
        logger.log_query(sql, &["param1"], timer);
    }

    // Simulate a slow query
    let timer = std::time::Instant::now();
    simulate_work(120);
    logger.log_query("SELECT * FROM large_table JOIN another_table", &[], timer);

    // Simulate a failed query
    let timer = std::time::Instant::now();
    simulate_work(10);
    logger.log_error("INVALID SQL SYNTAX", "Syntax error near 'INVALID'", timer);

    // Print statistics
    logger.print_stats();
    println!("Stats: {}", logger.stats().format_stats());
}

fn demo_state_machine_viz() {
    println!("\n--- 4. State Machine Visualization ---");

    let mut viz = StateMachineVisualizer::new();

    // Set current state
    viz.set_current_state(OrderState::PaymentPending);

    // Add state descriptions
    viz.add_state_description(OrderState::Created, "Order has been created");
    viz.add_state_description(
        OrderState::PaymentPending,
        "Waiting for payment confirmation",
    );
    viz.add_state_description(OrderState::PaymentConfirmed, "Payment received");
    viz.add_state_description(OrderState::Processing, "Order is being processed");
    viz.add_state_description(OrderState::Shipped, "Order has been shipped");
    viz.add_state_description(OrderState::Delivered, "Order delivered to customer");
    viz.add_state_description(OrderState::Cancelled, "Order was cancelled");

    // Add transitions
    viz.add_transition(
        OrderState::Created,
        OrderEvent::SubmitPayment,
        OrderState::PaymentPending,
    );
    viz.add_transition(
        OrderState::PaymentPending,
        OrderEvent::ConfirmPayment,
        OrderState::PaymentConfirmed,
    );
    viz.add_transition(
        OrderState::PaymentConfirmed,
        OrderEvent::StartProcessing,
        OrderState::Processing,
    );
    viz.add_transition(OrderState::Processing, OrderEvent::Ship, OrderState::Shipped);
    viz.add_transition(OrderState::Shipped, OrderEvent::Deliver, OrderState::Delivered);
    viz.add_transition(OrderState::Created, OrderEvent::Cancel, OrderState::Cancelled);
    viz.add_transition(
        OrderState::PaymentPending,
        OrderEvent::Cancel,
        OrderState::Cancelled,
    );
    viz.add_transition(
        OrderState::PaymentConfirmed,
        OrderEvent::Cancel,
        OrderState::Cancelled,
    );

    // Generate different visualizations
    println!("\n{}", viz.generate_diagram());
    println!("\nASCII Diagram:");
    println!("{}", viz.generate_ascii_diagram());
    println!("\nMarkdown Table:");
    println!("{}", viz.generate_markdown_table());
    println!("\nDOT Graph (for Graphviz):");
    println!("{}", viz.generate_dot_graph());
}

fn demo_di_introspection() {
    println!("\n--- 5. DI Container Introspection ---");

    let introspector = ContainerIntrospector::new();

    // Simulate service registrations
    introspector.record_service::<UserService>();
    introspector.record_service::<EmailService>();
    introspector.record_service::<PaymentService>();

    // Check if services are registered
    println!(
        "UserService registered: {}",
        introspector.is_registered::<UserService>()
    );
    println!(
        "EmailService registered: {}",
        introspector.is_registered::<EmailService>()
    );

    // Get service info
    let user_service_info = introspector.get_service_info::<UserService>();
    println!("UserService type: {}", user_service_info.type_name);

    // List all services
    println!("\nRegistered services:");
    for service in introspector.list_services() {
        println!("  - {}", service);
    }

    // Generate reports
    println!("\n{}", introspector.generate_report());
    println!("{}", introspector.generate_markdown());

    // Health check
    let health = introspector.check_health();
    println!("{}", health.format());

    introspector.print_services();
}

fn demo_config_dump() {
    println!("\n--- 6. Configuration Dumping ---");

    let config = AppConfig {
        server: ServerConfig {
            host: "localhost".to_string(),
            port: 8080,
            workers: 4,
        },
        database: DatabaseConfig {
            url: "postgres://localhost/mydb".to_string(),
            max_connections: 20,
            password: "super_secret_password".to_string(),
        },
        features: Features {
            caching: true,
            rate_limiting: true,
            compression: false,
        },
    };

    // Dump as JSON
    match ConfigDumper::dump_json(&config) {
        Ok(json) => {
            println!("\nConfiguration (JSON):");
            println!("{}", json);
        }
        Err(e) => println!("Error dumping config: {}", e),
    }

    // Redact sensitive fields
    if let Ok(json) = ConfigDumper::dump_json(&config) {
        let redacted = ConfigDumper::redact_sensitive(&json, &["password"]);
        println!("\nRedacted Configuration:");
        println!("{}", redacted);
    }

    // Generate summary
    match ConfigDumper::summarize(&config) {
        Ok(summary) => {
            println!("\n{}", summary.format());
        }
        Err(e) => println!("Error generating summary: {}", e),
    }

    // Log configuration
    ConfigDumper::log_config("AppConfig", &config);

    // Compare configs
    let mut config2 = config.clone();
    config2.server.port = 9090;

    if let Ok(diff) = ConfigDumper::diff_configs("original", &config, "modified", &config2) {
        println!("\n{}", diff);
    }
}

#[cfg(feature = "http")]
fn demo_http_debugging() {
    println!("\n--- 7. HTTP Request/Response Debugging ---");

    // Create HTTP dump middleware
    let _middleware = HttpDumpMiddleware::new();

    // Example with custom config
    let _custom_middleware = HttpDumpMiddleware::with_config(
        HttpDumpConfig::new()
            .with_request_headers(true)
            .with_request_body(true)
            .with_response_headers(true)
            .with_response_body(true)
            .with_max_body_size(1024)
            .with_pretty_json(true),
    );

    println!("HTTP debugging middleware created!");
    println!("In real usage, this would be added to your middleware pipeline.");

    // Example: Creating a test context (would normally come from HTTP server)
    let mut ctx = HttpContext::new("POST".to_string(), "/api/users".to_string());
    ctx.headers.insert("Content-Type".to_string(), "application/json".to_string());
    ctx.headers.insert("User-Agent".to_string(), "RustbootClient/1.0".to_string());
    ctx.body = Some(r#"{"name":"John Doe","email":"john@example.com"}"#.as_bytes().to_vec());
    ctx.client_ip = Some("192.168.1.100".to_string());

    println!("\nExample HTTP context created:");
    println!("  Method: {}", ctx.method);
    println!("  URL: {}", ctx.url);
    println!("  Headers: {:?}", ctx.headers);
    println!("  Client IP: {:?}", ctx.client_ip);

    // In a real application, you would use this in a middleware pipeline:
    // pipeline.add(middleware);
}

fn simulate_work(millis: u64) {
    std::thread::sleep(Duration::from_millis(millis));
}
