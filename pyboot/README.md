# Pyboot

**Python Infrastructure Framework** - Reusable cross-cutting concerns for production applications.

## Features

- 🔄 **Resilience** - Circuit breakers, retries, timeouts
- ⚡ **Rate Limiting** - Token bucket, sliding window
- 💾 **Caching** - Multi-backend caching
- 🔍 **Observability** - Logging, metrics, tracing
- 💉 **Dependency Injection** - Protocol-based DI
- ✅ **Validation** - Input validation framework
- 🔐 **Security** - Authentication, authorization
- 🌐 **Web** - Routing, CORS, middleware

## Quick Start

```python
from dev.engineeringlabs.pyboot.decorators import compose, memoize
from dev.engineeringlabs.pyboot.error import Result, Ok, Err

# Compose decorators
@memoize
def expensive_computation(n: int) -> int:
    return sum(range(n))

# Use Result monad for error handling
def divide(a: float, b: float) -> Result[float, str]:
    if b == 0:
        return Err("Division by zero")
    return Ok(a / b)

result = divide(10, 2)
if result.is_ok:
    print(f"Result: {result.unwrap()}")
```

## Installation

```bash
pip install pyboot

# With optional dependencies
pip install pyboot[full]    # All features
pip install pyboot[redis]   # Redis cache backend
pip install pyboot[http]    # HTTP client
```

## Documentation

See [docs/overview.md](docs/overview.md) for complete documentation.

## Modules (37)

| Category | Modules |
|----------|---------|
| **Core** | error, decorators, toolchain |
| **Foundation** | config, di, validation |
| **Resilience** | resilience, ratelimit, cache |
| **Web/API** | web, http, openapi, middleware |
| **Data** | database, storage, serialization |
| **Observability** | observability, debug, health |
| **Messaging** | messaging, notifications, streams |
| **Security** | security, crypto |
| **Utilities** | async_utils, cli, compress, datetime, fileio, parsing, uuid |
| **Special** | feature_flags, scheduler, session, state_machine, testing |

## Requirements

- Python 3.11+

## License

Apache-2.0
