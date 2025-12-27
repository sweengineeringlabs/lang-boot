"""
Pyboot - Python Infrastructure Framework

A collection of independent, production-ready infrastructure modules for Python applications.
Each module is standalone with no internal dependencies.

Modules:
    - di: Dependency injection container
    - config: Configuration management (env, files, layered)
    - resilience: Circuit breakers, retries, timeouts, bulkheads
    - observability: Logging, metrics, tracing, telemetry
    - health: Health checks and readiness probes
    - http: HTTP client infrastructure
    - cache: Multi-backend caching
    - storage: File storage abstractions
    - messaging: Async messaging and event buses
    - database: Database connections
    - scheduler: Task scheduling
    - validation: Input validation
    - security: Authentication, authorization, encryption
    - serialization: JSON, MessagePack, YAML, Pickle serialization
    - crypto: AES, RSA encryption, digital signatures
    - testing: Test fixtures, mocking, assertions
    - feature_flags: Feature toggles and gradual rollouts
    - notifications: Email, push, SMS, webhook notifications
    - state_machine: State transitions with guards
    - middleware: Pipeline pattern for request/response
    - streams: Reactive stream processing with parallel execution
    - openapi: OpenAPI 3.0 documentation generation
    - session: HTTP session management
    - datetime: Enhanced datetime utilities

Note: Each module is independent. Import from the specific module you need:
    from dev.engineeringlabs.pyboot.resilience import retryable, circuit_breaker
    from dev.engineeringlabs.pyboot.di import Container, Inject
    from dev.engineeringlabs.pyboot.config import ConfigLoader
"""


__version__ = "0.1.0"

__all__ = ["__version__"]
