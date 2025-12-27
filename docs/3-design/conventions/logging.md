# Logging Conventions

Idiomatic logging patterns for each language.

---

## 🦀 Rust

### Using `tracing` (Recommended)

```rust
use tracing::{info, warn, error, debug, instrument};

#[instrument]
fn process_order(order_id: i64) {
    info!(order_id, "Processing order");
    
    if let Err(e) = validate() {
        error!(error = %e, "Validation failed");
    }
}

// Setup
use tracing_subscriber;

fn main() {
    tracing_subscriber::fmt::init();
    // or with JSON
    tracing_subscriber::fmt()
        .json()
        .init();
}
```

### Log Levels

| Level | Use |
|-------|-----|
| `error!` | Errors requiring attention |
| `warn!` | Unexpected but handled |
| `info!` | Business events |
| `debug!` | Development details |
| `trace!` | Very verbose |

### Conventions

- Use `tracing` over `log` for async
- Use structured fields: `info!(user_id = 1, action = "login")`
- Use `#[instrument]` for automatic span creation

---

## 🦫 Go

### Using `slog` (Go 1.21+)

```go
import "log/slog"

func processOrder(orderID int64) {
    slog.Info("Processing order", "order_id", orderID)
    
    if err := validate(); err != nil {
        slog.Error("Validation failed", 
            "error", err,
            "order_id", orderID,
        )
    }
}

// Setup
func main() {
    // Text (default)
    slog.SetDefault(slog.New(slog.NewTextHandler(os.Stdout, nil)))
    
    // JSON
    slog.SetDefault(slog.New(slog.NewJSONHandler(os.Stdout, nil)))
}
```

### Using `zerolog`

```go
import "github.com/rs/zerolog/log"

log.Info().
    Int64("order_id", orderID).
    Msg("Processing order")

log.Error().
    Err(err).
    Int64("order_id", orderID).
    Msg("Validation failed")
```

### Conventions

- Use `slog` (stdlib) or `zerolog` (fast)
- Always use structured logging
- Include context in fields, not message
- Use context-aware logging

---

## ☕ Java

### Using SLF4J + Logback

```java
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class OrderService {
    private static final Logger log = LoggerFactory.getLogger(OrderService.class);
    
    public void processOrder(long orderId) {
        log.info("Processing order: {}", orderId);
        
        try {
            validate();
        } catch (ValidationException e) {
            log.error("Validation failed for order {}", orderId, e);
        }
    }
}
```

### Structured Logging (Logstash)

```java
import static net.logstash.logback.argument.StructuredArguments.*;

log.info("Processing order", kv("order_id", orderId));
log.error("Validation failed", kv("order_id", orderId), kv("error", e.getMessage()));
```

### Conventions

- Use SLF4J as facade
- Create logger per class: `LoggerFactory.getLogger(MyClass.class)`
- Use `{}` placeholders, not string concatenation
- Use MDC for request context

```java
MDC.put("request_id", requestId);
try {
    // all logs include request_id
} finally {
    MDC.clear();
}
```

---

## 🐍 Python

### Using `logging` (stdlib)

```python
import logging

logger = logging.getLogger(__name__)

def process_order(order_id: int):
    logger.info("Processing order", extra={"order_id": order_id})
    
    try:
        validate()
    except ValidationError as e:
        logger.error("Validation failed", exc_info=True, extra={"order_id": order_id})
```

### Using `structlog`

```python
import structlog

logger = structlog.get_logger()

def process_order(order_id: int):
    log = logger.bind(order_id=order_id)
    log.info("Processing order")
    
    try:
        validate()
    except ValidationError:
        log.exception("Validation failed")
```

### Setup

```python
# Basic
logging.basicConfig(level=logging.INFO)

# Structured JSON
import structlog
structlog.configure(
    processors=[structlog.processors.JSONRenderer()],
    wrapper_class=structlog.BoundLogger,
)
```

### Conventions

- Use `__name__` for logger name
- Use `structlog` for structured logging
- Use `exc_info=True` or `.exception()` for errors
- Configure once at application startup

---

## Comparison

| Aspect | Rust | Go | Java | Python |
|--------|------|-----|------|--------|
| Library | tracing | slog | SLF4J | logging |
| Structured | ✅ Native | ✅ Native | Plugin | structlog |
| Format | `info!(k=v)` | `slog.Info("m", "k", v)` | `log.info("m", kv("k",v))` | `extra={}` |
| Levels | error/warn/info/debug/trace | Error/Warn/Info/Debug | error/warn/info/debug/trace | ERROR/WARN/INFO/DEBUG |

---

## Best Practices

1. **Always use structured logging** - Key-value pairs, not printf
2. **Include context** - request_id, user_id, order_id
3. **Log at appropriate level** - Don't log everything as INFO
4. **Don't log sensitive data** - Passwords, tokens, PII
5. **Use correlation IDs** - Track requests across services
