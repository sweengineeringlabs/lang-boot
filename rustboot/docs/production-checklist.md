# Production Readiness Checklist

This comprehensive checklist helps ensure your Rustboot application is ready for production deployment. Each section includes specific items with links to relevant documentation and examples.

## Application Fundamentals

### Error Handling
- [ ] No `unwrap()` or `expect()` in production code paths
  - Use `?` operator or explicit error handling
  - See panic auditing: `cargo clippy -- -W clippy::unwrap_used`
- [ ] Proper error propagation with custom error types
  - Implement `std::error::Error` for domain errors
  - Use `thiserror` or `anyhow` for error handling
- [ ] All errors logged with appropriate context
  - See [rustboot-observability](../crates/rustboot-observability/doc/overview.md)
  - Use structured logging with `log::error!` or `tracing::error!`
- [ ] Graceful degradation for non-critical failures
  - Implement fallback mechanisms
  - Use default values where appropriate

### Input Validation
- [ ] All HTTP endpoint inputs validated
  - Validate request bodies, query parameters, and headers
  - See [rustboot-validation](../crates/rustboot-validation/doc/overview.md)
- [ ] Database inputs sanitized
  - Use parameterized queries (never string concatenation)
  - See [sqlx examples](../crates/rustboot-database/examples/sqlx_postgres.rs)
- [ ] File upload size limits enforced
  - Configure max payload size
  - Validate file types and content
- [ ] URL/path validation against path traversal
  - Sanitize file paths
  - Validate redirect URLs
- [ ] Rate limiting configured for all public endpoints
  - See [rate limiting documentation](../crates/rustboot-middleware/RATELIMIT.md)
  - See [http_ratelimit example](../crates/rustboot-middleware/examples/http_ratelimit.rs)

### Health Checks
- [ ] Liveness probe endpoint implemented (`/health/live`)
  - Returns 200 when application is running
  - See [health_basic example](../crates/rustboot-health/examples/health_basic.rs)
- [ ] Readiness probe endpoint implemented (`/health/ready`)
  - Checks database connectivity
  - Checks external dependencies
  - See [health_advanced example](../crates/rustboot-health/examples/health_advanced.rs)
- [ ] Startup probe configured for slow-starting apps
- [ ] Health checks don't depend on external services for liveness
  - See [health_http_integration example](../crates/rustboot-health/examples/health_http_integration.rs)

## Security

### HTTPS & Transport Security
- [ ] HTTPS enforced in production (HTTP redirects to HTTPS)
- [ ] TLS 1.2+ only (disable TLS 1.0/1.1)
- [ ] Valid SSL/TLS certificates configured
  - Use Let's Encrypt or trusted CA
  - Set up automatic renewal
- [ ] HSTS headers enabled
  - See [security headers documentation](../crates/rustboot-middleware/doc/security_headers.md)
  - Use `SecurityHeadersMiddleware::secure()`

### Security Headers
- [ ] Security headers middleware enabled
  - See [security headers documentation](../crates/rustboot-middleware/doc/security_headers.md)
  - See [security_headers example](../crates/rustboot-middleware/examples/security_headers.rs)
- [ ] Content-Security-Policy configured
  - Start with `default-src 'self'` and add as needed
  - Monitor CSP violations
- [ ] X-Frame-Options set to DENY or SAMEORIGIN
- [ ] X-Content-Type-Options set to nosniff
- [ ] Referrer-Policy configured appropriately
- [ ] Permissions-Policy restricts sensitive features

### CORS Configuration
- [ ] CORS origins explicitly whitelisted (no wildcard in production)
  - See [CORS documentation](../crates/rustboot-middleware/doc/cors.md)
  - See [cors_example](../crates/rustboot-middleware/examples/cors_example.rs)
- [ ] CORS methods restricted to required verbs only
- [ ] CORS headers validated and limited
- [ ] Credentials handling properly configured
  - Never use `allow_all_origins()` with credentials

### Secrets Management
- [ ] No hardcoded secrets in code
  - Audit with `cargo clippy` and manual review
- [ ] No secrets in version control
  - Use `.gitignore` for `.env`, `credentials.json`, etc.
  - Review git history for leaked secrets
- [ ] Secrets loaded from environment variables
  - See [rustboot-config](../crates/rustboot-config/doc/overview.md)
  - Use `EnvSource` for sensitive values
- [ ] Secrets not logged or exposed in error messages
  - Mask sensitive values in logs
  - Sanitize error responses
- [ ] Secret rotation strategy in place
  - Document rotation procedures
  - Test rotation without downtime

### Authentication & Authorization
- [ ] Authentication implemented and tested
  - JWT, OAuth2, or session-based auth
  - See [rustboot-crypto](../crates/rustboot-crypto/doc/overview.md)
- [ ] Authorization checks on all protected endpoints
  - Verify user permissions before operations
  - Implement role-based access control (RBAC)
- [ ] Session management secure
  - See [rustboot-session](../crates/rustboot-session/)
  - Use secure, httpOnly cookies
  - Configure appropriate session timeouts
- [ ] Password policies enforced (if applicable)
  - Minimum length, complexity requirements
  - Use bcrypt, argon2, or similar for hashing
- [ ] Multi-factor authentication available for sensitive operations

### Dependency Security
- [ ] Dependencies audited with `cargo audit`
  - Run regularly in CI/CD pipeline
  - Address HIGH and CRITICAL vulnerabilities
- [ ] Dependencies up to date
  - Use `cargo outdated` to check
  - Test updates in staging first
- [ ] No known vulnerabilities in dependencies
  - Subscribe to security advisories
  - Monitor RustSec Advisory Database

## Performance

### Connection Pooling
- [ ] Database connection pooling configured
  - See [pool_basic example](../crates/rustboot-database/examples/pool_basic.rs)
  - See [pool_configuration example](../crates/rustboot-database/examples/pool_configuration.rs)
- [ ] Pool size tuned for workload
  - Start with: `max_connections = (CPU cores * 2) + disk_spindles`
  - Monitor and adjust based on metrics
- [ ] Connection timeouts configured
  - Set `connection_timeout` to prevent indefinite waits
  - See [pool_advanced example](../crates/rustboot-database/examples/pool_advanced.rs)
- [ ] Idle connection cleanup enabled
  - Configure `idle_timeout` and `max_lifetime`
  - See [pool_bb8 example](../crates/rustboot-database/examples/pool_bb8.rs)

### Caching Strategy
- [ ] Caching implemented for expensive operations
  - See [rustboot-cache](../crates/rustboot-cache/doc/overview.md)
  - Cache database queries, API responses, computed results
- [ ] Cache TTL configured appropriately
  - Balance freshness vs performance
  - Document cache invalidation strategy
- [ ] Cache invalidation strategy documented
  - Time-based, event-based, or manual
  - Handle cache stampede scenarios
- [ ] Cache hit rate monitored
  - Track cache effectiveness
  - Adjust strategy based on metrics

### Database Optimization
- [ ] Database queries optimized and indexed
  - Use `EXPLAIN ANALYZE` to review query plans
  - Add indexes for frequently queried columns
- [ ] N+1 queries eliminated
  - Use joins or batch loading
  - Profile queries in staging
- [ ] Query timeouts configured
  - Prevent long-running queries from blocking
  - See [sqlx driver documentation](../crates/rustboot-database/SQLX_DRIVER_README.md)
- [ ] Database migrations tested and reversible
  - See [migration examples](../crates/rustboot-database/examples/)
  - Test rollback procedures
  - See [migration documentation](../crates/rustboot-database/doc/migrations.md)

### Async Operations
- [ ] Async/await used for I/O-bound operations
  - Database queries, HTTP requests, file I/O
  - See [rustboot-async](../crates/rustboot-async/doc/overview.md)
- [ ] Blocking operations moved to dedicated thread pools
  - Use `tokio::task::spawn_blocking` for CPU-intensive work
  - Don't block async runtime
- [ ] Backpressure handling implemented
  - Use bounded channels
  - Implement flow control
- [ ] Async runtime tuned for workload
  - Configure worker threads: `TOKIO_WORKER_THREADS`
  - Monitor task queue depth

## Resilience

### Fault Tolerance
- [ ] Retry policies implemented for transient failures
  - See [resilience patterns](../crates/rustboot-resilience/doc/overview.md)
  - See [resilience_basic example](../crates/rustboot-resilience/examples/resilience_basic.rs)
- [ ] Circuit breakers protect external dependencies
  - Prevent cascading failures
  - Configure thresholds and timeouts
- [ ] Timeouts configured for all external calls
  - HTTP requests, database queries, API calls
  - Use `with_timeout` from resilience crate
- [ ] Graceful degradation for non-critical services
  - Serve stale cache on failure
  - Provide reduced functionality
  - See [resilience_comprehensive example](../crates/rustboot-resilience/examples/resilience_comprehensive.rs)

### Resource Management
- [ ] File descriptors managed (no leaks)
  - Use RAII patterns (Drop trait)
  - Monitor open file descriptors: `lsof`
- [ ] Memory usage monitored and bounded
  - Set appropriate buffer sizes
  - Use streaming for large payloads
- [ ] Thread/task limits configured
  - Prevent unbounded task spawning
  - Use semaphores or bounded channels
- [ ] Graceful shutdown implemented
  - Handle SIGTERM signal
  - Drain in-flight requests
  - Close connections cleanly

## Observability

### Structured Logging
- [ ] Structured logging configured (JSON format)
  - See [rustboot-observability](../crates/rustboot-observability/doc/overview.md)
  - Use key-value pairs for searchability
- [ ] Log levels properly set (INFO in production)
  - ERROR for actionable issues
  - WARN for degraded states
  - INFO for normal operations
  - DEBUG/TRACE disabled in production
- [ ] Sensitive data not logged
  - Mask passwords, tokens, PII
  - Review logs for data leaks
- [ ] Request IDs included in logs for tracing
  - Generate unique ID per request
  - Propagate through entire request lifecycle
- [ ] HTTP logging middleware enabled
  - See [http_logging documentation](../crates/rustboot-middleware/HTTP_LOGGING.md)
  - See [http_logging_example](../crates/rustboot-middleware/examples/http_logging_example.rs)

### Metrics
- [ ] Key business metrics exposed
  - Request count, error rate, latency
  - Active users, transactions
- [ ] System metrics collected
  - CPU, memory, disk I/O
  - Connection pool stats
  - Cache hit/miss rates
- [ ] Custom metrics for critical paths
  - Payment processing, authentication
  - External API calls
- [ ] Metrics endpoint secured or not publicly accessible
  - Use `/metrics` with authentication
  - Restrict to monitoring systems

### Distributed Tracing
- [ ] Tracing enabled for request flows
  - Use OpenTelemetry or similar
  - Trace cross-service calls
- [ ] Trace sampling configured
  - 100% in staging, 1-10% in production
  - Always trace errors
- [ ] Trace context propagated across services
  - Use W3C Trace Context headers
  - Propagate trace/span IDs

### Alerting
- [ ] Alerts configured for critical errors
  - Service down, database unreachable
  - Error rate spikes
- [ ] SLO/SLI thresholds monitored
  - 99.9% uptime, p95 latency < 200ms
  - Define error budgets
- [ ] Alert fatigue prevented (no noisy alerts)
  - Tune thresholds to reduce false positives
  - Group related alerts
- [ ] On-call runbook linked to alerts
  - Clear investigation steps
  - Link to relevant dashboards

## Infrastructure

### Containerization
- [ ] Container health checks configured
  - Liveness, readiness, startup probes
  - Match health endpoints in application
- [ ] Container runs as non-root user
  - Use `USER` directive in Dockerfile
  - Principle of least privilege
- [ ] Multi-stage builds used for smaller images
  - Separate build and runtime stages
  - Remove build dependencies from final image
- [ ] Base images regularly updated
  - Track upstream security patches
  - Rebuild images regularly
- [ ] Image scanning for vulnerabilities
  - Use tools like Trivy, Clair
  - Fail builds on HIGH/CRITICAL vulnerabilities

### Resource Limits
- [ ] CPU limits configured
  - Prevent resource starvation
  - Set requests and limits in Kubernetes
- [ ] Memory limits configured
  - Prevent OOM kills
  - Leave headroom for spikes (request * 1.5 = limit)
- [ ] Disk space monitored
  - Alert on low disk space
  - Implement log rotation
- [ ] Network bandwidth limits considered
  - Rate limit external API calls
  - Consider egress costs

### Horizontal Scaling
- [ ] Application is stateless (or uses external state)
  - Session data in Redis/database
  - No local file storage for user data
- [ ] Load balancing configured
  - Health check-based routing
  - Session affinity if needed
- [ ] Auto-scaling rules defined
  - Scale on CPU, memory, or custom metrics
  - Set min/max replicas
- [ ] Startup/shutdown handled gracefully
  - Warm-up period for caches
  - Graceful termination of requests

### Data Persistence
- [ ] Database backups automated
  - Daily full backups minimum
  - Point-in-time recovery enabled
- [ ] Backup retention policy defined
  - 30 days minimum, 90 days recommended
  - Comply with regulatory requirements
- [ ] Backup restoration tested
  - Test quarterly minimum
  - Document restoration procedures
- [ ] Data replication configured (if applicable)
  - Multi-region for critical data
  - Monitor replication lag

### Disaster Recovery
- [ ] Disaster recovery plan documented
  - RTO (Recovery Time Objective) defined
  - RPO (Recovery Point Objective) defined
- [ ] Failover procedures tested
  - Database failover, region failover
  - Test annually minimum
- [ ] Multi-region deployment (for critical services)
  - Active-active or active-passive
  - Data consistency strategy defined
- [ ] Data restoration tested end-to-end
  - Restore to separate environment
  - Validate data integrity

## Operations

### Documentation
- [ ] Runbook created and up to date
  - Common issues and resolutions
  - Escalation procedures
  - Dependency map
- [ ] Architecture diagrams current
  - System architecture, data flow
  - Network topology
- [ ] API documentation published
  - OpenAPI/Swagger specs
  - See [rustboot-openapi](../crates/rustboot-openapi/)
- [ ] Configuration documented
  - All environment variables
  - Configuration file formats
  - Default values and valid ranges
- [ ] Incident response procedures defined
  - Who to contact, when to escalate
  - Communication templates

### Deployment
- [ ] Deployment process automated
  - CI/CD pipeline configured
  - No manual deployment steps
- [ ] Blue-green or canary deployments configured
  - Zero-downtime deployments
  - Traffic shifting strategy
- [ ] Database migrations automated
  - See [migration examples](../crates/rustboot-database/examples/)
  - Run before application deployment
  - Test rollback procedures
- [ ] Configuration changes tracked
  - Version control for config
  - Audit trail for changes
- [ ] Deployment rollback tested
  - One-command rollback
  - Test quarterly

### Monitoring & Maintenance
- [ ] Monitoring dashboards created
  - System health, application metrics
  - Business KPIs
- [ ] Log aggregation configured
  - Centralized logging (ELK, Splunk, CloudWatch)
  - Retention policy defined
- [ ] Performance baseline established
  - Normal CPU, memory, latency ranges
  - Compare against baseline to detect anomalies
- [ ] Regular security scans scheduled
  - Vulnerability scanning
  - Penetration testing annually
- [ ] Dependency updates scheduled
  - Monthly review of updates
  - Quarterly major version upgrades

### Team Readiness
- [ ] On-call rotation established
  - 24/7 coverage for critical services
  - Documented on-call procedures
- [ ] Team trained on runbook procedures
  - Regular training sessions
  - New team member onboarding
- [ ] Post-incident review process defined
  - Blameless postmortems
  - Action items tracked to completion
- [ ] Communication channels established
  - Incident Slack channel
  - Escalation email lists
  - Status page for customers

## Testing

### Test Coverage
- [ ] Unit tests for critical business logic
  - See [test organization guide](../docs/4-development/guide/rust-test-organization.md)
  - Target 80%+ coverage for core logic
- [ ] Integration tests for API endpoints
  - See integration test examples across crates
  - Test happy path and error cases
- [ ] Load testing performed
  - Simulate expected traffic + 50%
  - Identify bottlenecks
- [ ] Security testing completed
  - OWASP Top 10 tested
  - SQL injection, XSS, CSRF tests
- [ ] Chaos engineering experiments run
  - Service failures, network partitions
  - Database failover scenarios

### Environment Parity
- [ ] Staging environment matches production
  - Same infrastructure, configuration
  - Similar data volume
- [ ] All changes tested in staging first
  - No direct-to-production deployments
  - Staging sign-off required
- [ ] Production data NOT used in staging/dev
  - Use anonymized or synthetic data
  - Comply with privacy regulations

## Compliance & Legal

### Data Protection
- [ ] GDPR compliance verified (if applicable)
  - Right to deletion, data export
  - Cookie consent
- [ ] Data encryption at rest
  - Database encryption
  - Encrypted backups
- [ ] Data encryption in transit
  - TLS for all external communication
  - mTLS for internal services
- [ ] PII handling documented
  - Data classification
  - Retention policies

### Audit Trail
- [ ] Critical actions logged
  - User authentication, authorization changes
  - Data modifications
- [ ] Logs tamper-proof
  - Immutable logging (append-only)
  - Off-site log storage
- [ ] Audit log retention compliant with regulations
  - Minimum retention period
  - Secure deletion after retention

## Pre-Launch Checklist

### Final Verification
- [ ] All HIGH/CRITICAL items above completed
- [ ] Load testing passed with margin
  - Can handle 2x expected peak traffic
- [ ] Security review completed
  - Internal review minimum
  - External audit for sensitive applications
- [ ] Disaster recovery tested
  - Full restoration drill completed
- [ ] Team trained and ready
  - Runbook review session held
  - On-call schedule published
- [ ] Monitoring and alerting verified
  - Test alerts firing correctly
  - Dashboards populated with data
- [ ] Rollback procedure tested
  - Can rollback within defined RTO
- [ ] Go-live communication plan ready
  - Customer notifications
  - Internal announcements
  - Status page prepared

---

## Additional Resources

### Rustboot Framework Documentation
- [Framework Overview](./overview.md)
- [Development Guides](./4-development/)
- [Deployment Guides](./6-deployment/)

### Crate-Specific Documentation
- [Database](../crates/rustboot-database/doc/overview.md) - Connection pooling, migrations
- [Resilience](../crates/rustboot-resilience/doc/overview.md) - Retry, circuit breakers, timeouts
- [Middleware](../crates/rustboot-middleware/doc/overview.md) - CORS, security headers, rate limiting
- [Observability](../crates/rustboot-observability/doc/overview.md) - Logging, metrics, tracing
- [Cache](../crates/rustboot-cache/doc/overview.md) - Caching strategies
- [Config](../crates/rustboot-config/doc/overview.md) - Configuration management

### External Resources
- [Rust Security Best Practices](https://anssi-fr.github.io/rust-guide/)
- [OWASP Application Security](https://owasp.org/www-project-top-ten/)
- [Twelve-Factor App](https://12factor.net/)
- [Site Reliability Engineering](https://sre.google/)

---

**Last Updated**: 2025-12-24
**Version**: 1.0
**Maintainer**: Rustboot Team

Use this checklist systematically before each production deployment. Track completion in your project management tool and require sign-off for critical items.
