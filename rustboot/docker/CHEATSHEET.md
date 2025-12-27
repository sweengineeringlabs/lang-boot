# Docker Commands Cheatsheet

Quick reference for common Docker operations with Rustboot.

## Quick Start

```bash
# Standard deployment
docker compose up -d

# Development mode
docker compose -f docker-compose.dev.yml up -d

# Production mode
docker compose -f docker-compose.prod.yml up -d

# With management tools
docker compose --profile tools up -d
```

## Makefile Commands

```bash
make help          # Show all available commands
make build         # Build Docker images
make up            # Start all services
make down          # Stop all services
make restart       # Restart services
make logs          # View all logs
make logs-app      # View app logs
make logs-db       # View database logs
make ps            # List containers
make shell         # Access app shell
make shell-db      # Access database shell
make clean         # Remove everything
make health        # Check service health
```

## Docker Compose Commands

### Service Management

```bash
# Start services
docker compose up -d

# Stop services
docker compose down

# Restart services
docker compose restart

# Restart specific service
docker compose restart app

# Remove all (including volumes)
docker compose down -v
```

### Logs

```bash
# View all logs
docker compose logs -f

# View specific service
docker compose logs -f app

# Last 100 lines
docker compose logs --tail=100 app

# Since timestamp
docker compose logs --since 2024-01-01T00:00:00
```

### Building

```bash
# Build all images
docker compose build

# Build specific service
docker compose build app

# Build without cache
docker compose build --no-cache

# Pull latest images
docker compose pull
```

### Scaling

```bash
# Scale to 3 instances
docker compose up -d --scale app=3

# Production scaling
docker compose -f docker-compose.prod.yml up -d --scale app=5
```

## Container Management

### Access Containers

```bash
# App container shell
docker compose exec app sh

# Database shell
docker compose exec postgres psql -U rustboot -d rustboot_db

# Redis CLI
docker compose exec redis redis-cli

# Run one-off command
docker compose run --rm app cargo test
```

### Inspect Containers

```bash
# List containers
docker compose ps

# Detailed info
docker compose ps -a

# Container details
docker inspect rustboot-app

# Health status
docker inspect rustboot-app | jq '.[0].State.Health'
```

## Database Operations

### PostgreSQL

```bash
# Access database
docker compose exec postgres psql -U rustboot -d rustboot_db

# Backup database
docker compose exec postgres pg_dump -U rustboot rustboot_db > backup.sql

# Restore database
docker compose exec -T postgres psql -U rustboot rustboot_db < backup.sql

# Check connection
docker compose exec postgres pg_isready -U rustboot

# View tables
docker compose exec postgres psql -U rustboot -d rustboot_db -c "\dt app.*"

# Run SQL file
docker compose exec -T postgres psql -U rustboot -d rustboot_db < script.sql
```

### Redis

```bash
# Access Redis CLI
docker compose exec redis redis-cli

# Check connection
docker compose exec redis redis-cli ping

# Get all keys
docker compose exec redis redis-cli KEYS '*'

# Flush all data (careful!)
docker compose exec redis redis-cli FLUSHALL

# Save snapshot
docker compose exec redis redis-cli SAVE

# Get info
docker compose exec redis redis-cli INFO
```

## Volume Management

```bash
# List volumes
docker volume ls

# Inspect volume
docker volume inspect rustboot_postgres-data

# Remove unused volumes
docker volume prune

# Backup volume
docker run --rm -v rustboot_postgres-data:/data -v $(pwd):/backup \
  alpine tar czf /backup/postgres-backup.tar.gz /data

# Restore volume
docker run --rm -v rustboot_postgres-data:/data -v $(pwd):/backup \
  alpine tar xzf /backup/postgres-backup.tar.gz -C /
```

## Network Operations

```bash
# List networks
docker network ls

# Inspect network
docker network inspect rustboot_rustboot-network

# Remove unused networks
docker network prune
```

## Image Management

### Building Images

```bash
# Build with builder Dockerfile
docker build -f docker/Dockerfile.builder \
  --build-arg BINARY_NAME=rustboot-app \
  -t rustboot-app:latest .

# Build runtime image
docker build -f docker/Dockerfile.runtime \
  --target distroless \
  -t rustboot-app:runtime .

# Build with BuildKit
DOCKER_BUILDKIT=1 docker build -f docker/Dockerfile.builder .
```

### Image Operations

```bash
# List images
docker images

# Remove image
docker rmi rustboot-app:latest

# Tag image
docker tag rustboot-app:latest myregistry.com/rustboot-app:v1.0.0

# Push to registry
docker push myregistry.com/rustboot-app:v1.0.0

# Pull from registry
docker pull myregistry.com/rustboot-app:v1.0.0

# Save image to file
docker save rustboot-app:latest > rustboot-app.tar

# Load image from file
docker load < rustboot-app.tar
```

## Monitoring & Debugging

### Resource Usage

```bash
# Real-time stats
docker stats

# Specific container
docker stats rustboot-app

# No streaming
docker stats --no-stream
```

### Health Checks

```bash
# Check app health
curl http://localhost:8080/health

# Check database
docker compose exec postgres pg_isready -U rustboot

# Check Redis
docker compose exec redis redis-cli ping

# All services
make health
```

### Debugging

```bash
# View container processes
docker compose top app

# View container events
docker compose events

# View container changes
docker diff rustboot-app

# Export container filesystem
docker export rustboot-app > container.tar

# View container resource limits
docker inspect rustboot-app | jq '.[0].HostConfig.Memory'
```

## Cleanup

```bash
# Stop and remove containers
docker compose down

# Remove with volumes
docker compose down -v

# Remove images too
docker compose down --rmi all

# System-wide cleanup
docker system prune

# Aggressive cleanup (careful!)
docker system prune -a --volumes

# Remove specific items
docker container prune  # Unused containers
docker image prune      # Unused images
docker volume prune     # Unused volumes
docker network prune    # Unused networks
```

## Environment Variables

```bash
# Use specific env file
docker compose --env-file .env.production up -d

# Override env var
DATABASE_URL=postgres://... docker compose up -d

# View environment
docker compose exec app env

# Set env for one command
docker compose exec -e DEBUG=true app cargo test
```

## Configuration

### Update Configuration

```bash
# Edit compose file
vim docker-compose.yml

# Validate compose file
docker compose config

# View merged configuration
docker compose -f docker-compose.yml -f docker-compose.prod.yml config
```

### Secrets Management

```bash
# Using Docker secrets (Swarm mode)
echo "my-secret" | docker secret create db_password -

# Using environment files
cp .env.example .env
# Edit .env with secrets
docker compose --env-file .env up -d
```

## CI/CD Integration

### GitHub Actions

```bash
# Copy workflow template
cp docker/.github-workflows-example.yml .github/workflows/docker.yml

# Trigger workflow
git push origin main
```

### Manual Deploy

```bash
# Build
docker build -f docker/Dockerfile.builder -t myapp:latest .

# Tag
docker tag myapp:latest registry.com/myapp:latest

# Push
docker push registry.com/myapp:latest

# Deploy
ssh production "docker pull registry.com/myapp:latest && \
  docker compose up -d"
```

## Troubleshooting

### Common Issues

```bash
# Port already in use
sudo lsof -i :8080
# Kill process or change port in compose file

# Permission denied
sudo chown -R $USER:$USER .

# Out of disk space
docker system df
docker system prune -a

# Container won't start
docker compose logs app
docker inspect rustboot-app

# Network issues
docker network inspect rustboot_rustboot-network
docker compose down && docker compose up -d
```

### Reset Everything

```bash
# Complete reset (DANGER: loses all data)
docker compose down -v
docker system prune -a --volumes
docker compose up -d
```

## Performance Optimization

```bash
# Enable BuildKit
export DOCKER_BUILDKIT=1

# Parallel builds
docker compose build --parallel

# Use build cache
docker compose build --build-arg BUILDKIT_INLINE_CACHE=1

# Prune build cache
docker builder prune
```

## Security

```bash
# Scan image for vulnerabilities
docker scan rustboot-app:latest

# Using Trivy
docker run --rm -v /var/run/docker.sock:/var/run/docker.sock \
  aquasec/trivy image rustboot-app:latest

# Check for outdated images
docker images --filter "dangling=true"

# Update base images
docker compose pull
docker compose up -d
```

## Tips & Tricks

```bash
# Follow logs of multiple services
docker compose logs -f app postgres redis

# Execute command in running container
docker compose exec app ls -la

# Copy files from container
docker cp rustboot-app:/app/logs/app.log ./app.log

# Copy files to container
docker cp config.toml rustboot-app:/app/config/

# Run with resource limits
docker run --cpus=1 --memory=512m rustboot-app:latest

# Run with custom network
docker run --network rustboot_rustboot-network rustboot-app:latest
```

## References

- [README.md](README.md) - Full documentation
- [QUICK_START.md](QUICK_START.md) - Quick start guide
- [ARCHITECTURE.md](ARCHITECTURE.md) - Architecture details
- [Makefile](Makefile) - Available make commands

## Getting Help

```bash
# Docker help
docker --help
docker compose --help

# Command-specific help
docker compose up --help

# Show version
docker --version
docker compose version
```
