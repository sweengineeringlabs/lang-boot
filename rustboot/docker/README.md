# Rustboot Docker Support

This directory contains Docker configurations for building and running Rustboot applications in containerized environments.

## Contents

- `Dockerfile.builder` - Multi-stage build for compiling Rust applications with optimal dependency caching
- `Dockerfile.runtime` - Minimal runtime images (distroless, Alpine, or Debian)
- `docker-compose.yml` - Complete stack with application, PostgreSQL, and Redis
- `.dockerignore` - Files to exclude from Docker build context

## Quick Start

### Build and Run with Docker Compose

The easiest way to get started is using Docker Compose:

```bash
# Start all services (app, PostgreSQL, Redis)
docker compose -f docker/docker-compose.yml up -d

# View logs
docker compose -f docker/docker-compose.yml logs -f app

# Stop all services
docker compose -f docker/docker-compose.yml down

# Stop and remove volumes (clean slate)
docker compose -f docker/docker-compose.yml down -v
```

### Build Standalone Application Image

```bash
# Build using the builder Dockerfile
docker build -f docker/Dockerfile.builder -t rustboot-app:latest \
  --build-arg BINARY_NAME=rustboot-app .

# Run the container
docker run -p 8080:8080 --name rustboot-app rustboot-app:latest
```

### Build with Runtime Dockerfile (Pre-built Binary)

```bash
# First build your application
cargo build --release

# Build using runtime Dockerfile (distroless by default)
docker build -f docker/Dockerfile.runtime -t rustboot-app:runtime \
  --build-arg BINARY_NAME=rustboot-app .

# Or use Alpine variant
docker build -f docker/Dockerfile.runtime -t rustboot-app:alpine \
  --target alpine \
  --build-arg BINARY_NAME=rustboot-app .
```

## Architecture

### Multi-Stage Build (Dockerfile.builder)

The builder uses cargo-chef for optimal dependency caching:

1. **Chef Stage**: Installs cargo-chef
2. **Planner Stage**: Analyzes dependencies
3. **Dependencies Stage**: Builds dependencies (cached)
4. **Builder Stage**: Compiles application
5. **Runtime Stage**: Minimal Debian-based runtime

### Runtime Images (Dockerfile.runtime)

Three runtime options are provided:

| Option | Base | Size | Use Case |
|--------|------|------|----------|
| Distroless | gcr.io/distroless/cc-debian12 | ~20MB | Production (most secure) |
| Alpine | alpine:3.19 | ~15MB | When you need shell access |
| Debian | debian:bookworm-slim | ~80MB | Maximum compatibility |

## Configuration

### Environment Variables

The docker-compose.yml includes comprehensive environment variables:

#### Application
- `APP_NAME`: Application name
- `APP_ENV`: Environment (development, production)
- `APP_HOST`: Listen address (0.0.0.0 for containers)
- `APP_PORT`: Port to listen on (default: 8080)

#### Database
- `DATABASE_URL`: PostgreSQL connection string
- `DATABASE_MAX_CONNECTIONS`: Connection pool size
- `DATABASE_MIN_CONNECTIONS`: Minimum idle connections
- `DATABASE_CONNECT_TIMEOUT`: Connection timeout (seconds)
- `DATABASE_IDLE_TIMEOUT`: Idle connection timeout (seconds)
- `DATABASE_MAX_LIFETIME`: Maximum connection lifetime (seconds)

#### Redis
- `REDIS_URL`: Redis connection string
- `REDIS_MAX_CONNECTIONS`: Connection pool size
- `REDIS_CONNECTION_TIMEOUT`: Connection timeout (seconds)

#### Cache
- `CACHE_TTL`: Default cache TTL (seconds)
- `CACHE_ENABLED`: Enable/disable caching

#### Session
- `SESSION_SECRET`: Secret key for session encryption
- `SESSION_TIMEOUT`: Session timeout (seconds)
- `SESSION_COOKIE_SECURE`: Use secure cookies (HTTPS)
- `SESSION_COOKIE_HTTP_ONLY`: HTTP-only cookies

#### Security
- `CORS_ALLOWED_ORIGINS`: Comma-separated allowed origins
- `CORS_ALLOWED_METHODS`: Allowed HTTP methods
- `CORS_ALLOWED_HEADERS`: Allowed headers
- `RATE_LIMIT_REQUESTS`: Requests per window
- `RATE_LIMIT_WINDOW`: Rate limit window (seconds)

#### Logging
- `RUST_LOG`: Logging level (info, debug, trace)
- `LOG_FORMAT`: Log format (json, pretty)

### Build Arguments

Both Dockerfiles accept build arguments:

```bash
docker build --build-arg BINARY_NAME=your-app-name ...
```

## Advanced Usage

### Run with Management Tools

Start pgAdmin and Redis Commander for database/cache management:

```bash
docker compose -f docker/docker-compose.yml --profile tools up -d
```

Access:
- pgAdmin: http://localhost:5050 (admin@rustboot.dev / admin)
- Redis Commander: http://localhost:8081

### Custom Binary Names

If your application binary has a different name:

```bash
# In docker-compose.yml, modify:
services:
  app:
    build:
      args:
        BINARY_NAME: your-binary-name
```

### Multi-Application Setup

For workspace with multiple binaries:

```bash
# Build specific workspace member
docker build -f docker/Dockerfile.builder \
  --build-arg BINARY_NAME=example-http-server \
  -t rustboot-http-example:latest .
```

### Development Mode

For development with hot-reload:

```yaml
# Add to docker-compose.yml under app service:
volumes:
  - ..:/build
  - cargo-cache:/usr/local/cargo/registry
command: cargo watch -x 'run --bin your-app'
```

### Production Deployment

#### Best Practices

1. **Use distroless runtime** for production:
```bash
docker build -f docker/Dockerfile.runtime --target distroless ...
```

2. **Set secure secrets**:
```bash
# Use Docker secrets or environment files
docker compose --env-file .env.production up -d
```

3. **Enable health checks**:
The compose file includes health checks for all services.

4. **Resource limits**:
```yaml
# Add to services in docker-compose.yml:
services:
  app:
    deploy:
      resources:
        limits:
          cpus: '1'
          memory: 512M
        reservations:
          cpus: '0.5'
          memory: 256M
```

5. **Logging**:
```yaml
services:
  app:
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"
```

## Optimization Tips

### Reduce Build Time

1. **Use cargo-chef** (already included in Dockerfile.builder)
2. **Layer caching**: Dependencies are cached separately
3. **BuildKit**: Enable Docker BuildKit
   ```bash
   DOCKER_BUILDKIT=1 docker build ...
   ```

### Reduce Image Size

1. **Use distroless** for smallest size
2. **Strip binaries** in Cargo.toml:
   ```toml
   [profile.release]
   strip = true
   lto = true
   codegen-units = 1
   ```
3. **Remove debug info**:
   ```toml
   [profile.release]
   debug = false
   ```

### Improve Startup Time

1. **Optimize Cargo.toml**:
   ```toml
   [profile.release]
   opt-level = 3
   ```
2. **Reduce dependencies**: Only include needed features
3. **Lazy initialization**: Initialize heavy resources on-demand

## Networking

### Container Communication

Services communicate via the `rustboot-network` bridge network:

- App → PostgreSQL: `postgres:5432`
- App → Redis: `redis:6379`

### External Access

Exposed ports:
- Application: `8080`
- PostgreSQL: `5432`
- Redis: `6379`
- pgAdmin: `5050` (with `--profile tools`)
- Redis Commander: `8081` (with `--profile tools`)

## Data Persistence

Volumes for persistent data:
- `postgres-data`: PostgreSQL database files
- `redis-data`: Redis persistence files
- `pgadmin-data`: pgAdmin configuration
- `app-logs`: Application logs

### Backup and Restore

#### PostgreSQL

```bash
# Backup
docker compose -f docker/docker-compose.yml exec postgres \
  pg_dump -U rustboot rustboot_db > backup.sql

# Restore
docker compose -f docker/docker-compose.yml exec -T postgres \
  psql -U rustboot rustboot_db < backup.sql
```

#### Redis

```bash
# Backup
docker compose -f docker/docker-compose.yml exec redis \
  redis-cli SAVE
docker cp rustboot-redis:/data/dump.rdb ./redis-backup.rdb

# Restore
docker cp ./redis-backup.rdb rustboot-redis:/data/dump.rdb
docker compose -f docker/docker-compose.yml restart redis
```

## Troubleshooting

### Common Issues

1. **Port already in use**:
   ```bash
   # Check what's using the port
   sudo lsof -i :8080
   # Change port in docker-compose.yml
   ```

2. **Permission denied**:
   ```bash
   # Ensure files are owned by current user
   sudo chown -R $USER:$USER .
   ```

3. **Out of memory**:
   ```bash
   # Increase Docker memory limit
   # Docker Desktop: Settings → Resources → Memory
   ```

4. **Build fails**:
   ```bash
   # Clear Docker cache
   docker builder prune -a
   # Rebuild without cache
   docker compose build --no-cache
   ```

5. **Database connection fails**:
   ```bash
   # Check if PostgreSQL is ready
   docker compose logs postgres
   # Wait for healthy status
   docker compose ps
   ```

### Debugging

#### Access container shell

```bash
# For Debian/Alpine based images
docker compose exec app sh

# For distroless (no shell, use debug variant)
docker build --target debian -f docker/Dockerfile.runtime ...
```

#### View logs

```bash
# All services
docker compose logs -f

# Specific service
docker compose logs -f app

# Last 100 lines
docker compose logs --tail=100 app
```

#### Check health status

```bash
docker compose ps
docker inspect rustboot-app | jq '.[0].State.Health'
```

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Build and Push Docker Image

on:
  push:
    branches: [main]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Build Docker image
        run: |
          docker build -f docker/Dockerfile.builder \
            -t myorg/rustboot-app:${{ github.sha }} \
            -t myorg/rustboot-app:latest .

      - name: Login to Docker Hub
        run: echo "${{ secrets.DOCKER_PASSWORD }}" | \
          docker login -u "${{ secrets.DOCKER_USERNAME }}" --password-stdin

      - name: Push Docker image
        run: |
          docker push myorg/rustboot-app:${{ github.sha }}
          docker push myorg/rustboot-app:latest
```

### GitLab CI Example

```yaml
build:
  image: docker:latest
  services:
    - docker:dind
  script:
    - docker build -f docker/Dockerfile.builder -t $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA .
    - docker push $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA
```

## Security Considerations

1. **Non-root user**: All images run as non-root by default
2. **Minimal attack surface**: Distroless has no shell or package manager
3. **Security scanning**: Use tools like Trivy:
   ```bash
   docker run --rm -v /var/run/docker.sock:/var/run/docker.sock \
     aquasec/trivy image rustboot-app:latest
   ```
4. **Secrets management**: Never hardcode secrets in Dockerfiles
5. **Network isolation**: Use Docker networks for service isolation
6. **Regular updates**: Keep base images updated

## Performance Monitoring

### Resource Usage

```bash
# Monitor resource usage
docker stats

# Specific container
docker stats rustboot-app
```

### Application Metrics

The application exposes metrics at `/metrics` (if observability is enabled).

Access metrics:
```bash
curl http://localhost:8080/metrics
```

## References

- [Rustboot Documentation](https://github.com/phdsystems/rustboot)
- [Docker Best Practices](https://docs.docker.com/develop/dev-best-practices/)
- [Docker Compose Reference](https://docs.docker.com/compose/compose-file/)
- [cargo-chef](https://github.com/LukeMathWalker/cargo-chef)

## License

This Docker configuration is part of the Rustboot project and follows the same MIT license.
