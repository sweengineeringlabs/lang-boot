# Docker Architecture for Rustboot

This document describes the Docker architecture and design decisions for Rustboot applications.

## Overview

The Docker setup provides multiple deployment options:
1. **Development**: Full stack with hot-reload and management tools
2. **Production**: Optimized, secure, and scalable deployment
3. **Runtime-only**: Minimal images for pre-built binaries

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                     Docker Network                          │
│                   (rustboot-network)                        │
│                                                             │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐ │
│  │              │    │              │    │              │ │
│  │  Rustboot    │───▶│  PostgreSQL  │    │    Redis     │ │
│  │     App      │    │              │    │              │ │
│  │              │───▶│              │    │              │ │
│  │  Port: 8080  │    │  Port: 5432  │    │  Port: 6379  │ │
│  │              │    │              │    │              │ │
│  └──────┬───────┘    └──────────────┘    └──────────────┘ │
│         │                                                   │
│         │            ┌──────────────┐    ┌──────────────┐ │
│         │            │              │    │              │ │
│         └───────────▶│   pgAdmin    │    │Redis Command.│ │
│                      │  Port: 5050  │    │  Port: 8081  │ │
│                      │              │    │              │ │
│                      └──────────────┘    └──────────────┘ │
│                      (dev/tools only)    (dev/tools only) │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
                      ┌──────────────┐
                      │    Nginx     │
                      │  (optional)  │
                      │  Port: 80/443│
                      └──────────────┘
```

## Multi-Stage Build Strategy

### Dockerfile.builder

Uses cargo-chef for optimal dependency caching:

```
Stage 1: Chef         → Install cargo-chef tool
Stage 2: Planner      → Analyze dependencies
Stage 3: Dependencies → Build deps (cached layer)
Stage 4: Builder      → Compile application
Stage 5: Runtime      → Minimal Debian runtime
```

**Benefits:**
- Dependencies cached separately from source
- Faster rebuilds when only code changes
- Smaller final image (only runtime deps)

### Dockerfile.runtime

Three runtime variants for different needs:

| Variant | Size | Security | Debugging | Use Case |
|---------|------|----------|-----------|----------|
| Distroless | ~20MB | Highest | None | Production |
| Alpine | ~15MB | High | Shell | Dev/Debug |
| Debian | ~80MB | Good | Full | Compatibility |

## Image Optimization

### Size Reduction Techniques

1. **Multi-stage builds**: Only runtime artifacts in final image
2. **Cargo stripping**: Remove debug symbols
3. **Distroless base**: No package manager, shell, or utilities
4. **.dockerignore**: Exclude unnecessary files from build context

### Build Caching Strategy

```dockerfile
# Layer 1: Dependencies (changes rarely)
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release --dependencies-only

# Layer 2: Source code (changes frequently)
COPY src ./src
RUN cargo build --release
```

With cargo-chef, this is even more efficient.

## Security Considerations

### Container Security

1. **Non-root user**: All containers run as `appuser` (UID 1000)
2. **Read-only filesystem**: Root filesystem mounted read-only
3. **No new privileges**: `security_opt: no-new-privileges:true`
4. **Minimal attack surface**: Distroless has no shell or package manager
5. **Secret management**: Secrets via environment variables, not baked in

### Network Security

1. **Internal network**: Services communicate via bridge network
2. **Port exposure**: Only necessary ports exposed to host
3. **TLS/SSL**: Nginx handles SSL termination
4. **Rate limiting**: Implemented at both nginx and application level

### Production Hardening

```yaml
services:
  app:
    security_opt:
      - no-new-privileges:true
    read_only: true
    tmpfs:
      - /tmp
    cap_drop:
      - ALL
    cap_add:
      - NET_BIND_SERVICE  # Only if needed
```

## Resource Management

### Resource Limits (Production)

```yaml
deploy:
  resources:
    limits:
      cpus: '2'
      memory: 1G
    reservations:
      cpus: '1'
      memory: 512M
```

### Database Tuning

PostgreSQL is configured for 2GB RAM:
- `shared_buffers`: 512MB (25% of RAM)
- `effective_cache_size`: 1536MB (75% of RAM)
- `maintenance_work_mem`: 128MB
- `max_connections`: 200

Redis is configured with:
- `maxmemory`: 512MB
- `maxmemory-policy`: allkeys-lru
- Persistence: AOF with everysec fsync

## High Availability

### Application Scaling

```yaml
deploy:
  replicas: 3
  update_config:
    parallelism: 1
    delay: 10s
    order: start-first
```

### Health Checks

Three layers of health checks:
1. **Docker health check**: Container-level
2. **Compose depends_on**: Service dependency
3. **Nginx upstream**: Backend health monitoring

### Rolling Updates

```yaml
update_config:
  parallelism: 1        # Update 1 container at a time
  delay: 10s            # Wait 10s between updates
  order: start-first    # Start new before stopping old
```

## Data Persistence

### Volume Strategy

```yaml
volumes:
  postgres-data:       # Database files
  redis-data:          # Redis persistence
  app-logs:            # Application logs
  nginx-cache:         # Nginx cache
```

### Backup Strategy

**PostgreSQL:**
```bash
docker compose exec postgres pg_dump -U rustboot rustboot_db > backup.sql
```

**Redis:**
```bash
docker compose exec redis redis-cli SAVE
docker cp rustboot-redis:/data/dump.rdb backup.rdb
```

## Monitoring and Observability

### Built-in Observability

1. **Health endpoints**: `/health` for liveness/readiness
2. **Metrics endpoint**: `/metrics` for Prometheus
3. **Structured logging**: JSON format for log aggregation
4. **Tracing**: OpenTelemetry support

### Log Management

```yaml
logging:
  driver: "json-file"
  options:
    max-size: "10m"
    max-file: "5"
```

For production, consider:
- Fluentd/Fluent Bit for log collection
- Elasticsearch for log storage
- Kibana for log visualization

### Metrics Collection

Expose metrics to Prometheus:

```yaml
# prometheus.yml
scrape_configs:
  - job_name: 'rustboot-app'
    static_configs:
      - targets: ['app:8080']
```

## Development Workflow

### Local Development

```bash
# Start development stack
docker compose -f docker/docker-compose.dev.yml up -d

# View logs
make logs-app

# Access container
make shell

# Run tests
docker compose exec app cargo test
```

### Hot Reload (Optional)

Mount source code as volume:

```yaml
volumes:
  - ../:/build
command: cargo watch -x run
```

## CI/CD Integration

### Build Pipeline

1. Checkout code
2. Build Docker image
3. Run tests in container
4. Security scanning (Trivy)
5. Push to registry
6. Deploy to environment

### Image Tagging Strategy

- `latest`: Latest build from main branch
- `v1.2.3`: Semantic version tags
- `main-abc123`: Branch name + commit SHA
- `pr-123`: Pull request number

## Network Architecture

### Internal Communication

Services use Docker DNS for service discovery:
- App → PostgreSQL: `postgres:5432`
- App → Redis: `redis:6379`

### External Access

| Port | Service | Purpose |
|------|---------|---------|
| 8080 | App | HTTP API |
| 5432 | PostgreSQL | Database (dev only) |
| 6379 | Redis | Cache (dev only) |
| 5050 | pgAdmin | DB Management |
| 8081 | Redis Commander | Cache Management |
| 80/443 | Nginx | Reverse Proxy |

## Environment Configuration

### Configuration Hierarchy

1. Default values in `config/app.toml`
2. Environment-specific in `.env.{environment}`
3. Runtime overrides via environment variables
4. Secrets via Docker secrets or external vault

### Environment Files

- `.env.example`: Template with all options
- `.env`: Local development (gitignored)
- `.env.production`: Production config (encrypted)
- `.env.test`: Test environment config

## Troubleshooting

### Common Issues

1. **Port conflicts**: Change host port in compose file
2. **Build failures**: Clear build cache with `docker builder prune`
3. **Connection refused**: Check service health with `docker compose ps`
4. **Out of memory**: Increase Docker memory limit

### Debugging Tools

```bash
# View detailed logs
docker compose logs --tail=100 -f app

# Inspect container
docker inspect rustboot-app

# Check resource usage
docker stats

# Access shell (if available)
docker compose exec app sh

# Run one-off commands
docker compose run --rm app cargo test
```

## Performance Optimization

### Build Performance

1. **Enable BuildKit**: `DOCKER_BUILDKIT=1`
2. **Use cargo-chef**: Already configured
3. **Parallel builds**: Multiple build stages in parallel
4. **Layer caching**: Optimize Dockerfile layer order

### Runtime Performance

1. **Release builds**: Always use `--release` for production
2. **LTO optimization**: Enable in Cargo.toml
3. **CPU pinning**: Pin containers to specific CPUs
4. **Connection pooling**: Configure appropriate pool sizes

### Database Performance

1. **Indexes**: Add indexes for frequently queried columns
2. **Connection pooling**: Match pool size to workload
3. **Query optimization**: Use EXPLAIN for slow queries
4. **Vacuum**: Regular maintenance tasks

## Disaster Recovery

### Backup Procedures

**Daily backups:**
```bash
# Automated backup script
docker compose exec postgres pg_dump -Fc > backup-$(date +%Y%m%d).dump
```

**Point-in-time recovery:**
Enable WAL archiving for PostgreSQL.

### Recovery Procedures

```bash
# Restore from backup
docker compose exec -T postgres pg_restore -d rustboot_db < backup.dump

# Verify data integrity
docker compose exec postgres psql -U rustboot -d rustboot_db -c "SELECT COUNT(*) FROM app.users;"
```

## References

- [Docker Best Practices](https://docs.docker.com/develop/dev-best-practices/)
- [Dockerfile Best Practices](https://docs.docker.com/develop/develop-images/dockerfile_best-practices/)
- [Docker Security](https://docs.docker.com/engine/security/)
- [cargo-chef](https://github.com/LukeMathWalker/cargo-chef)
- [Distroless Images](https://github.com/GoogleContainerTools/distroless)

## License

MIT License - See LICENSE file for details.
