# Makefile for Rustboot Docker operations
# Simplifies common Docker commands

.PHONY: help build up down logs clean restart ps shell test build-prod push

# Default target
help:
	@echo "Rustboot Docker Commands:"
	@echo "  make build         - Build Docker images"
	@echo "  make build-prod    - Build production image (distroless)"
	@echo "  make up            - Start all services"
	@echo "  make up-tools      - Start all services including management tools"
	@echo "  make down          - Stop all services"
	@echo "  make restart       - Restart all services"
	@echo "  make logs          - View logs (all services)"
	@echo "  make logs-app      - View application logs"
	@echo "  make logs-db       - View database logs"
	@echo "  make logs-redis    - View Redis logs"
	@echo "  make ps            - List running containers"
	@echo "  make shell         - Access application container shell"
	@echo "  make shell-db      - Access database shell"
	@echo "  make shell-redis   - Access Redis CLI"
	@echo "  make clean         - Stop and remove containers, networks, volumes"
	@echo "  make test          - Run tests in Docker"
	@echo "  make db-migrate    - Run database migrations"
	@echo "  make db-backup     - Backup PostgreSQL database"
	@echo "  make db-restore    - Restore PostgreSQL database"
	@echo "  make push          - Push Docker image to registry"

# Configuration
COMPOSE_FILE := docker-compose.yml
COMPOSE := docker compose -f $(COMPOSE_FILE)
IMAGE_NAME ?= rustboot-app
IMAGE_TAG ?= latest
BINARY_NAME ?= rustboot-app

# Build Docker images
build:
	$(COMPOSE) build

# Build production image with distroless
build-prod:
	docker build -f Dockerfile.runtime --target distroless \
		--build-arg BINARY_NAME=$(BINARY_NAME) \
		-t $(IMAGE_NAME):$(IMAGE_TAG) ..

# Start all services
up:
	$(COMPOSE) up -d

# Start all services including management tools
up-tools:
	$(COMPOSE) --profile tools up -d

# Stop all services
down:
	$(COMPOSE) down

# Restart all services
restart:
	$(COMPOSE) restart

# View logs for all services
logs:
	$(COMPOSE) logs -f

# View application logs
logs-app:
	$(COMPOSE) logs -f app

# View database logs
logs-db:
	$(COMPOSE) logs -f postgres

# View Redis logs
logs-redis:
	$(COMPOSE) logs -f redis

# List running containers
ps:
	$(COMPOSE) ps

# Access application container shell (Debian-based)
shell:
	$(COMPOSE) exec app sh

# Access database shell
shell-db:
	$(COMPOSE) exec postgres psql -U rustboot -d rustboot_db

# Access Redis CLI
shell-redis:
	$(COMPOSE) exec redis redis-cli

# Clean up everything (containers, networks, volumes)
clean:
	$(COMPOSE) down -v
	docker system prune -f

# Run tests in Docker
test:
	docker build -f Dockerfile.builder --target builder \
		--build-arg BINARY_NAME=$(BINARY_NAME) \
		-t $(IMAGE_NAME):test ..
	docker run --rm $(IMAGE_NAME):test cargo test

# Run database migrations (requires your migration tool)
db-migrate:
	$(COMPOSE) exec app /app/app migrate

# Backup PostgreSQL database
db-backup:
	@mkdir -p backups
	$(COMPOSE) exec postgres pg_dump -U rustboot rustboot_db > backups/backup-$$(date +%Y%m%d-%H%M%S).sql
	@echo "Backup created in backups/"

# Restore PostgreSQL database (set BACKUP_FILE variable)
db-restore:
	@if [ -z "$(BACKUP_FILE)" ]; then \
		echo "Error: BACKUP_FILE not set. Usage: make db-restore BACKUP_FILE=backups/backup.sql"; \
		exit 1; \
	fi
	$(COMPOSE) exec -T postgres psql -U rustboot rustboot_db < $(BACKUP_FILE)

# Push image to registry (set REGISTRY variable)
push:
	@if [ -z "$(REGISTRY)" ]; then \
		echo "Error: REGISTRY not set. Usage: make push REGISTRY=myregistry.com/myorg"; \
		exit 1; \
	fi
	docker tag $(IMAGE_NAME):$(IMAGE_TAG) $(REGISTRY)/$(IMAGE_NAME):$(IMAGE_TAG)
	docker push $(REGISTRY)/$(IMAGE_NAME):$(IMAGE_TAG)

# Health check all services
health:
	@echo "Checking service health..."
	@$(COMPOSE) ps
	@echo "\nApplication health:"
	@curl -f http://localhost:8080/health || echo "Application not healthy"
	@echo "\nPostgreSQL health:"
	@$(COMPOSE) exec postgres pg_isready -U rustboot || echo "PostgreSQL not ready"
	@echo "\nRedis health:"
	@$(COMPOSE) exec redis redis-cli ping || echo "Redis not responding"

# Watch logs (requires watch command)
watch-logs:
	watch -n 1 'docker compose -f $(COMPOSE_FILE) logs --tail=50'

# Stop and remove only app container
reset-app:
	$(COMPOSE) stop app
	$(COMPOSE) rm -f app
	$(COMPOSE) up -d app
