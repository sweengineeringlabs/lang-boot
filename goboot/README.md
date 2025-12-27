# Goboot

**Go Infrastructure Framework** - Reusable cross-cutting concerns for production applications.

## Features

- 🔄 **Resilience** - Circuit breakers, retries, timeouts
- ⚡ **Rate Limiting** - Token bucket, sliding window
- 💾 **Caching** - Multi-backend caching
- 🔍 **Observability** - Logging, metrics, tracing
- 💉 **Dependency Injection** - Interface-based DI
- ✅ **Validation** - Input validation framework
- 🔐 **Security** - Authentication, authorization, crypto
- 🌐 **Web** - HTTP middleware, CORS
- 📦 **Storage** - File storage abstractions
- ❤️ **Health** - Health checks, liveness/readiness
- ⏰ **DateTime** - Clocks, date utilities
- 📅 **Scheduler** - Task scheduling, cron jobs
- 🚩 **Feature Flags** - Toggles, percentage rollouts
- 🔀 **State Machine** - FSM with guards
- 📬 **Notifications** - Email, SMS, webhooks
- 🌊 **Streams** - Reactive stream processing

## Quick Start

```go
package main

import (
    "fmt"
    
    "dev.engineeringlabs/goboot/errors"
    "dev.engineeringlabs/goboot/resilience"
)

func main() {
    // Use Result monad for error handling
    result := divide(10, 2)
    if result.IsOk() {
        fmt.Printf("Result: %v\n", result.Unwrap())
    }
    
    // Use retry pattern
    cb := resilience.NewCircuitBreaker("api", resilience.DefaultCircuitBreakerConfig())
    // ...
}

func divide(a, b float64) errors.Result[float64] {
    if b == 0 {
        return errors.Err[float64]("Division by zero")
    }
    return errors.Ok(a / b)
}
```

## Installation

```bash
go get dev.engineeringlabs/goboot
```

## Documentation

See [docs/overview.md](docs/overview.md) for complete documentation.

## Modules (25)

| Category | Modules |
|----------|---------|
| **Core** | errors, stereotypes |
| **Foundation** | config, di, validation |
| **Resilience** | resilience, cache |
| **Web/API** | web, http, session |
| **Data** | database, storage |
| **Messaging** | messaging, streams |
| **Security** | security, crypto |
| **Observability** | observability, health |
| **Utilities** | serialization, datetime, testing |
| **Advanced** | scheduler, featureflags, statemachine, notifications |

## Requirements

- Go 1.21+

## License

Apache-2.0
