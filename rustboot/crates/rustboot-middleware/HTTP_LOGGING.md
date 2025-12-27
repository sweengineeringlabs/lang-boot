# HTTP Logging Middleware

Comprehensive HTTP request/response logging middleware for RustBoot framework.

## Overview

The HTTP Logging Middleware provides detailed logging of HTTP requests and responses with configurable options for log levels, request ID tracking, timing, and optional body logging. It integrates seamlessly with the RustBoot middleware pipeline and the rustboot-observability crate.

## Features

- **Request Logging**: Captures HTTP method, path, headers, and optionally the request body
- **Response Logging**: Logs status code, headers, response time, and optionally the response body
- **Request ID Tracking**: Generates or extracts request IDs for correlation across distributed systems
- **Configurable Log Levels**: Set different log levels for requests and responses
- **Body Logging with Size Limits**: Optionally log request/response bodies with configurable truncation
- **Binary Data Handling**: Gracefully handles non-UTF8 binary content
- **Performance Timing**: Automatically measures and logs request processing duration
- **Smart Status-based Logging**: Automatically adjusts log level based on HTTP status codes

## Installation

The HTTP logging middleware is included in the `rustboot-middleware` crate:

```toml
[dependencies]
dev-engineeringlabs-rustboot-middleware = "0.1.0"
```

## Quick Start

### Basic Usage

```rust
use dev_engineeringlabs_rustboot_middleware::{
    HttpLoggingMiddleware, HttpLoggingContext, HttpLoggingRequest, Pipeline,
};

// Create middleware with default configuration
let logging = HttpLoggingMiddleware::new();
let pipeline = Pipeline::new().with_middleware(logging);

// Create a request
let request = HttpLoggingRequest::new("GET".to_string(), "/api/users".to_string())
    .with_header("Accept".to_string(), "application/json".to_string());

let ctx = HttpLoggingContext::new(request);

// Execute the pipeline
let result = pipeline.execute(ctx).await?;
```

### Custom Configuration

```rust
use dev_engineeringlabs_rustboot_middleware::{
    HttpLoggingConfig, HttpLoggingMiddleware, HttpLogLevel,
};

let config = HttpLoggingConfig::builder()
    .request_level(HttpLogLevel::Info)
    .response_level(HttpLogLevel::Debug)
    .log_request_headers(true)
    .log_response_headers(true)
    .log_request_body(true)
    .log_response_body(true)
    .max_body_size(2048)
    .track_request_id(true)
    .request_id_header("X-Request-ID".to_string())
    .build();

let logging = HttpLoggingMiddleware::with_config(config);
```

## Configuration Options

### HttpLoggingConfig

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `request_level` | `HttpLogLevel` | `Info` | Log level for incoming requests |
| `response_level` | `HttpLogLevel` | `Info` | Log level for outgoing responses |
| `log_request_headers` | `bool` | `true` | Whether to log request headers |
| `log_response_headers` | `bool` | `true` | Whether to log response headers |
| `log_request_body` | `bool` | `false` | Whether to log request body |
| `log_response_body` | `bool` | `false` | Whether to log response body |
| `max_body_size` | `usize` | `1024` | Maximum body size to log (bytes) |
| `track_request_id` | `bool` | `true` | Enable request ID tracking |
| `request_id_header` | `String` | `"X-Request-ID"` | Header name for request ID |

### Log Levels

- `HttpLogLevel::Error` - Only errors
- `HttpLogLevel::Warn` - Warnings and errors
- `HttpLogLevel::Info` - Info, warnings, and errors
- `HttpLogLevel::Debug` - Debug and above
- `HttpLogLevel::Trace` - All logs including trace

## Request ID Tracking

The middleware supports automatic request ID tracking for request correlation:

### Extracting from Headers

```rust
let config = HttpLoggingConfig::builder()
    .track_request_id(true)
    .request_id_header("X-Request-ID".to_string())
    .build();

// Request with existing ID
let request = HttpLoggingRequest::new("GET".to_string(), "/api/data".to_string())
    .with_header("X-Request-ID".to_string(), "req-12345".to_string());
```

### Auto-generation

If no request ID is present in the headers, the middleware automatically generates a UUID:

```rust
// Request without ID - will be auto-generated
let request = HttpLoggingRequest::new("GET".to_string(), "/api/data".to_string());
// After processing, ctx.request.request_id will contain a generated UUID
```

## Body Logging

Body logging is disabled by default for security and performance reasons. When enabled:

### Size Limits

Bodies larger than `max_body_size` are truncated:

```rust
let config = HttpLoggingConfig::builder()
    .log_request_body(true)
    .max_body_size(512)  // Log up to 512 bytes
    .build();
```

### Binary Data

Binary data is automatically detected and logged as `<binary data, N bytes>`:

```rust
// Binary content is handled gracefully
let request = HttpLoggingRequest::new("POST".to_string(), "/upload".to_string())
    .with_body(vec![0xFF, 0xFE, 0xFD]);
// Logs: "<binary data, 3 bytes>"
```

## Status-based Logging

The middleware automatically adjusts log levels based on HTTP status codes:

- **5xx errors**: Always logged at `ERROR` level
- **4xx errors**: Always logged at `WARN` level
- **2xx/3xx**: Logged at configured `response_level`

This ensures critical issues are always visible regardless of configuration.

## Performance Timing

Every request is automatically timed, and the duration is logged with the response:

```
HTTP Response: method=GET | path=/api/users | status=200 | duration=25.3ms | request_id=abc-123
```

## Integration with rustboot-observability

The middleware uses the `tracing` crate and integrates with rustboot-observability:

```rust
// The middleware automatically logs using tracing
// Configure your tracing subscriber as needed
tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    .init();
```

## Log Format

### Request Log

```
HTTP Request: method=POST | path=/api/users | request_id=abc-123 | headers=[Content-Type=application/json, Authorization=Bearer ***] | body={"name":"Alice"}
```

### Response Log

```
HTTP Response: method=POST | path=/api/users | status=201 | duration=15.2ms | request_id=abc-123 | headers=[Content-Type=application/json] | body={"id":1,"name":"Alice"}
```

## Use Cases

### Development & Debugging

Enable full logging with body content:

```rust
let config = HttpLoggingConfig::builder()
    .request_level(HttpLogLevel::Debug)
    .response_level(HttpLogLevel::Debug)
    .log_request_body(true)
    .log_response_body(true)
    .max_body_size(4096)
    .build();
```

### Production

Minimal logging without bodies:

```rust
let config = HttpLoggingConfig::builder()
    .request_level(HttpLogLevel::Info)
    .response_level(HttpLogLevel::Info)
    .log_request_body(false)
    .log_response_body(false)
    .track_request_id(true)
    .build();
```

### Security-sensitive Endpoints

Disable header logging for sensitive routes:

```rust
let config = HttpLoggingConfig::builder()
    .log_request_headers(false)
    .log_response_headers(false)
    .log_request_body(false)
    .log_response_body(false)
    .build();
```

## Examples

See the [http_logging_example.rs](examples/http_logging_example.rs) file for comprehensive examples including:

- Basic logging with defaults
- Custom configuration with body logging
- Request ID tracking and extraction
- Different log levels
- Large body handling

Run the example:

```bash
cargo run --example http_logging_example
```

## Testing

The middleware includes comprehensive unit and integration tests:

```bash
# Run unit tests
cargo test --lib http_logging

# Run all middleware tests
cargo test -p dev-engineeringlabs-rustboot-middleware
```

## Performance Considerations

- **Body Logging**: Disabled by default as it can impact performance
- **Size Limits**: Use `max_body_size` to prevent logging large payloads
- **Log Levels**: Use appropriate log levels in production (Info or Warn)
- **Header Logging**: Consider security implications when logging headers

## Best Practices

1. **Don't log sensitive data**: Avoid logging authorization tokens, passwords, or PII
2. **Use request IDs**: Enable request ID tracking for distributed tracing
3. **Set appropriate size limits**: Prevent logging of large file uploads
4. **Adjust log levels by environment**: More verbose in dev, minimal in production
5. **Monitor log volume**: High-traffic endpoints can generate significant logs

## Contributing

Contributions are welcome! Please ensure:

- All tests pass
- New features include tests
- Code follows existing patterns
- Documentation is updated

## License

See the main RustBoot framework license.
