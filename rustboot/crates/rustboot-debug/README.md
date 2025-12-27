# Rustboot Debug

Development debugging utilities for the Rustboot framework.

## Warning

**This crate is intended for development and testing only.** The utilities may expose sensitive information and have performance overhead. Always disable in production builds by using feature flags or conditional compilation.

## Features

- `http` - HTTP request/response dumping middleware
- `database` - Database query logging utilities
- `state-machine` - State machine visualization (text-based diagrams)
- `di` - DI container introspection
- `config` - Configuration dumping utilities
- `all` - Enable all features

## Installation

Add to your `Cargo.toml`:

```toml
[dev-dependencies]
dev-engineeringlabs-rustboot-debug = { version = "0.1", features = ["all"] }
```

Or enable specific features:

```toml
[dev-dependencies]
dev-engineeringlabs-rustboot-debug = { version = "0.1", features = ["http", "database"] }
```

## Utilities

### 1. Timing Utilities

Profile operation performance with automatic logging:

```rust
use dev_engineeringlabs_rustboot_debug::timing::*;

// Basic timing
{
    let _guard = TimingGuard::new("my_operation");
    // Your code here
} // Timing logged on drop

// With custom thresholds
let thresholds = TimingThresholds::new()
    .with_warn_threshold(Duration::from_secs(1))
    .with_info_threshold(Duration::from_millis(100));

let _guard = TimingGuard::with_thresholds("operation", thresholds);

// With checkpoints
let guard = TimingGuard::new("multi_step");
do_step_1();
guard.checkpoint("step_1_done");
do_step_2();
guard.checkpoint("step_2_done");
```

### 2. HTTP Debugging (requires `http` feature)

Dump HTTP requests and responses:

```rust
use dev_engineeringlabs_rustboot_debug::HttpDumpMiddleware;

// Create middleware with defaults
let middleware = HttpDumpMiddleware::new();

// Or customize
let middleware = HttpDumpMiddleware::with_config(
    HttpDumpConfig::new()
        .with_request_headers(true)
        .with_request_body(true)
        .with_max_body_size(4096)
        .with_pretty_json(true)
        .with_info_level(true) // Use INFO instead of DEBUG
);

// Add to middleware pipeline
pipeline.add(middleware);
```

Output includes:
- HTTP method and URL
- Request/response headers
- Request/response body (with JSON pretty-printing)
- Client IP address
- Configurable body size limits

### 3. Database Query Logging (requires `database` feature)

Track and log database queries:

```rust
use dev_engineeringlabs_rustboot_debug::QueryLogger;
use std::sync::Arc;

let logger = Arc::new(QueryLogger::new());

// Log queries
let start = std::time::Instant::now();
// Execute query...
logger.log_query("SELECT * FROM users WHERE id = ?", &["123"], start);

// Log errors
logger.log_error("INVALID SQL", "Syntax error", start);

// Get statistics
println!("Stats: {}", logger.stats().format_stats());
logger.print_stats();
```

Features:
- Configurable slow query threshold
- Query statistics tracking
- Parameter logging (disabled by default for security)
- Result count logging

### 4. State Machine Visualization (requires `state-machine` feature)

Generate text-based state machine diagrams:

```rust
use dev_engineeringlabs_rustboot_debug::StateMachineVisualizer;

let mut viz = StateMachineVisualizer::new();
viz.set_current_state(State::Running);
viz.add_transition(State::Idle, Event::Start, State::Running);
viz.add_state_description(State::Idle, "Waiting for work");

// Generate different formats
println!("{}", viz.generate_diagram());        // Detailed text
println!("{}", viz.generate_ascii_diagram());  // Simple ASCII
println!("{}", viz.generate_dot_graph());      // Graphviz DOT format
println!("{}", viz.generate_markdown_table()); // Markdown table
```

### 5. DI Container Introspection (requires `di` feature)

Inspect registered services in your DI container:

```rust
use dev_engineeringlabs_rustboot_debug::ContainerIntrospector;

let introspector = ContainerIntrospector::new();

// Record services
introspector.record_service::<MyService>();
introspector.record_service::<AnotherService>();

// List services
let services = introspector.list_services();
println!("Registered: {:?}", services);

// Generate reports
println!("{}", introspector.generate_report());
println!("{}", introspector.generate_markdown());

// Health check
let health = introspector.check_health();
println!("{}", health.format());
```

### 6. Configuration Dumping (requires `config` feature)

Dump and analyze configuration:

```rust
use dev_engineeringlabs_rustboot_debug::ConfigDumper;

// Dump as JSON
let json = ConfigDumper::dump_json(&config)?;
println!("{}", json);

// Redact sensitive fields
let redacted = ConfigDumper::redact_sensitive(&json, &["password", "api_key"]);

// Generate summary
let summary = ConfigDumper::summarize(&config)?;
println!("{}", summary.format());

// Compare configs
ConfigDumper::diff_configs("old", &config1, "new", &config2)?;

// Log to tracing
ConfigDumper::log_config("MyConfig", &config);
```

### 7. Debug Macros

Conditional compilation helpers:

```rust
use dev_engineeringlabs_rustboot_debug::{debug_only, debug_log};

// Only executes in debug builds
debug_only! {
    println!("Debug mode is enabled!");
    expensive_debug_operation();
}

// Log with location info
debug_log!("User logged in", user_id = user.id);
```

## Production Safety

### Using Feature Flags

Only include in dev builds:

```toml
[dev-dependencies]
dev-engineeringlabs-rustboot-debug = { version = "0.1", features = ["all"] }
```

### Conditional Compilation

```rust
#[cfg(debug_assertions)]
use dev_engineeringlabs_rustboot_debug::HttpDumpMiddleware;

#[cfg(debug_assertions)]
{
    pipeline.add(HttpDumpMiddleware::new());
}
```

### Runtime Checks

```rust
if dev_engineeringlabs_rustboot_debug::is_debug_mode() {
    // Debug-only code
}
```

## Example

See `examples/debug_example.rs` for a comprehensive demonstration:

```bash
cargo run --example debug_example --all-features
```

## Best Practices

1. **Never log sensitive data in production** - Use `redact_sensitive()` for configs
2. **Use feature flags** - Keep debug code out of production builds
3. **Set appropriate log levels** - Use DEBUG for verbose output, INFO for important events
4. **Limit body sizes** - Configure `max_body_size` to prevent memory issues
5. **Monitor performance** - Timing utilities help identify bottlenecks

## License

MIT
