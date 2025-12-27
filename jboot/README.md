# JBoot

**Java Infrastructure Framework** - Reusable cross-cutting concerns for production applications.

## Features

- 🔍 **Validation** - Bean validation with custom constraints
- 💾 **Caching** - Multi-backend caching abstraction
- 💉 **DI** - Dependency injection utilities
- 🔄 **State Machines** - Fluent FSM with guards and actions
- 🌐 **HTTP** - HTTP client abstraction
- 📨 **Messaging** - Message queue abstractions
- 🗄️ **Database** - Repository patterns and query builders
- 🔌 **Middleware** - Pipeline pattern
- 📊 **Observability** - Metrics, logging, tracing
- 🔐 **Security** - Authentication, authorization, secrets
- 🛡️ **Resilience** - Retry, circuit breaker, timeout patterns
- ⏱️ **Rate Limiting** - Token bucket, sliding window

## Quick Start

```java
import com.jboot.validation.Validator;
import com.jboot.resilience.CircuitBreaker;
import com.jboot.cache.Cache;

// Validation
var validator = Validator.builder()
    .field("email").notEmpty().email()
    .field("age").range(18, 120)
    .build();

// Caching
var cache = Cache.inMemory();
cache.set("key", "value", Duration.ofMinutes(5));

// Resilience
var cb = CircuitBreaker.builder("myService")
    .failureThreshold(5)
    .timeout(Duration.ofSeconds(30))
    .build();

cb.execute(() -> riskyOperation());
```

## Installation

### Maven
```xml
<dependency>
    <groupId>com.jboot</groupId>
    <artifactId>jboot-core</artifactId>
    <version>0.1.0</version>
</dependency>
```

### Gradle
```groovy
implementation 'com.jboot:jboot-core:0.1.0'
```

## Documentation

See [docs/overview.md](docs/overview.md) for complete documentation.

## Modules

| Category | Modules |
|----------|---------|
| **Core** | jboot-core, jboot-error |
| **Foundation** | jboot-config, jboot-di, jboot-validation |
| **Resilience** | jboot-resilience, jboot-cache, jboot-ratelimit |
| **Web/API** | jboot-web, jboot-http, jboot-session |
| **Data** | jboot-database, jboot-storage |
| **Messaging** | jboot-messaging, jboot-streams |
| **Security** | jboot-security, jboot-crypto |
| **Observability** | jboot-observability, jboot-health |
| **Utilities** | jboot-serialization, jboot-datetime, jboot-testing |
| **Advanced** | jboot-scheduler, jboot-statemachine |

## Architecture

JBoot follows the [SEA (Stratified Encapsulation Architecture)](https://github.com/phdsystems/rustratify) pattern:

- **API Layer**: Public interfaces and contracts
- **Core Layer**: Default implementations
- **SPI Layer**: Extension points for custom implementations

## Requirements

- Java 17+

## License

MIT
