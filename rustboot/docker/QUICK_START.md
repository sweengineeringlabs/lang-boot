# Docker Quick Start Guide

Get your Rustboot application running in Docker in under 5 minutes.

## Prerequisites

- Docker Engine 20.10 or later
- Docker Compose V2
- 4GB RAM available for Docker

## Quick Start

### 1. Start Everything

```bash
cd docker
docker compose up -d
```

This starts:
- Your Rustboot application on http://localhost:8080
- PostgreSQL database on localhost:5432
- Redis cache on localhost:6379

### 2. Check Status

```bash
docker compose ps
```

All services should show as "healthy" or "running".

### 3. View Logs

```bash
# All services
docker compose logs -f

# Just the app
docker compose logs -f app
```

### 4. Test the Application

```bash
# Health check
curl http://localhost:8080/health

# Your API endpoints
curl http://localhost:8080/api/...
```

### 5. Stop Everything

```bash
docker compose down
```

## Development Mode

For development with source code mounting:

```bash
docker compose -f docker-compose.dev.yml up -d
```

This includes:
- pgAdmin at http://localhost:5050 (admin@rustboot.dev / admin)
- Redis Commander at http://localhost:8081
- Live code mounting (modify code without rebuilding)

## Using the Makefile

Even easier commands:

```bash
make up          # Start services
make logs        # View logs
make shell       # Access app container
make shell-db    # Access database
make down        # Stop services
make clean       # Remove everything
make help        # See all commands
```

## Troubleshooting

### Ports Already in Use

```bash
# Change ports in docker-compose.yml
ports:
  - "8081:8080"  # Use different host port
```

### Build Fails

```bash
# Rebuild without cache
docker compose build --no-cache
```

### Database Connection Issues

```bash
# Check database is ready
docker compose exec postgres pg_isready -U rustboot

# View database logs
docker compose logs postgres
```

### Permission Issues

```bash
# On Linux, ensure proper ownership
sudo chown -R $USER:$USER .
```

## Next Steps

1. Read the full [README.md](README.md) for detailed documentation
2. Customize environment variables in `.env` (copy from `.env.example`)
3. Configure your application in `config/app.toml`
4. Add your database migrations to `init-scripts/`

## Common Tasks

### Access Database

```bash
docker compose exec postgres psql -U rustboot -d rustboot_db
```

### Access Redis CLI

```bash
docker compose exec redis redis-cli
```

### View Application Shell

```bash
docker compose exec app sh
```

### Backup Database

```bash
docker compose exec postgres pg_dump -U rustboot rustboot_db > backup.sql
```

### Restore Database

```bash
docker compose exec -T postgres psql -U rustboot rustboot_db < backup.sql
```

## Need Help?

- Check logs: `docker compose logs`
- Inspect container: `docker compose exec app sh`
- View health: `make health`
- Read full docs: [README.md](README.md)
