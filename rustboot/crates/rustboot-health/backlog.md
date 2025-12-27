# Rustboot Health - Backlog

## Completed

- [x] Core health check traits (HealthCheck, LivenessCheck, ReadinessCheck)
- [x] Health status types (Healthy, Degraded, Unhealthy)
- [x] Check result with metadata support
- [x] Health aggregator with sequential execution
- [x] Health aggregator with parallel execution
- [x] JSON serialization for health reports
- [x] Built-in checks (AlwaysHealthy, Function, TCP, Ping, Composite)
- [x] Critical vs non-critical check support
- [x] Comprehensive integration tests
- [x] Basic and advanced examples
- [x] HTTP integration example
- [x] Documentation

## Future Enhancements

### Additional Built-in Checks
- [ ] **Database health check** - Generic database connection check
- [ ] **HTTP endpoint check** - Check if HTTP endpoint is responding
- [ ] **Redis health check** - Redis-specific health check
- [ ] **Disk space check** - Monitor available disk space
- [ ] **Memory usage check** - Monitor memory consumption
- [ ] **CPU usage check** - Monitor CPU load

### Advanced Features
- [ ] **Health check caching** - Cache results for expensive checks
- [ ] **Circuit breaker integration** - Skip checks for known-failing services
- [ ] **Metrics export** - Export health metrics to Prometheus/StatsD
- [ ] **Health check groups** - Group related checks together
- [ ] **Conditional checks** - Only run certain checks in specific environments
- [ ] **Check dependencies** - Express dependencies between checks
- [ ] **Startup checks** - One-time checks run only at startup
- [ ] **Periodic background checks** - Run checks in background on interval

### Integration
- [ ] **Axum middleware** - Ready-to-use health check endpoints for Axum
- [ ] **Actix-web integration** - Health check handlers for Actix
- [ ] **Database integration** - Health checks for sqlx, diesel, etc.
- [ ] **Message queue checks** - RabbitMQ, Kafka, Redis pub/sub health
- [ ] **gRPC health check protocol** - Implement gRPC health checking protocol

### Observability
- [ ] **Structured logging** - Emit structured logs for health checks
- [ ] **Tracing integration** - Add tracing spans for health checks
- [ ] **Alert thresholds** - Define thresholds for degraded/unhealthy states
- [ ] **History tracking** - Track health check history over time

## Notes

- The crate is production-ready for basic use cases
- All core features are implemented and tested
- Follow existing framework patterns for any additions
- Maintain backward compatibility
