# Docker Directory Index

Complete index of all Docker-related files for Rustboot applications.

## Quick Navigation

### Getting Started
- [QUICK_START.md](QUICK_START.md) - Get running in under 5 minutes
- [README.md](README.md) - Comprehensive documentation
- [.env.example](.env.example) - Environment variables template

### Docker Files
- [Dockerfile.builder](Dockerfile.builder) - Multi-stage build with cargo-chef
- [Dockerfile.runtime](Dockerfile.runtime) - Minimal runtime images
- [.dockerignore](.dockerignore) - Build context exclusions

### Docker Compose Files
- [docker-compose.yml](docker-compose.yml) - Standard production-ready stack
- [docker-compose.dev.yml](docker-compose.dev.yml) - Development with hot-reload
- [docker-compose.prod.yml](docker-compose.prod.yml) - Production with resource limits

### Configuration
- [config/app.toml](config/app.toml) - Application configuration example
- [.env.example](.env.example) - Environment variables documentation
- [nginx.conf](nginx.conf) - Nginx reverse proxy configuration

### Database
- [init-scripts/01-init.sql](init-scripts/01-init.sql) - Database initialization

### Utilities
- [Makefile](Makefile) - Simplified Docker commands
- [healthcheck.sh](healthcheck.sh) - Custom health check script

### Documentation
- [README.md](README.md) - Main documentation (comprehensive)
- [QUICK_START.md](QUICK_START.md) - Quick start guide
- [ARCHITECTURE.md](ARCHITECTURE.md) - Architecture and design decisions

### CI/CD
- [.github-workflows-example.yml](.github-workflows-example.yml) - GitHub Actions example

## File Structure

```
docker/
├── README.md                      # Main documentation
├── QUICK_START.md                 # Quick start guide
├── ARCHITECTURE.md                # Architecture documentation
├── INDEX.md                       # This file
│
├── Dockerfile.builder             # Multi-stage build Dockerfile
├── Dockerfile.runtime             # Minimal runtime Dockerfile
├── .dockerignore                  # Build context exclusions
│
├── docker-compose.yml             # Standard compose file
├── docker-compose.dev.yml         # Development compose file
├── docker-compose.prod.yml        # Production compose file
│
├── Makefile                       # Convenience commands
├── .env.example                   # Environment variables template
├── nginx.conf                     # Nginx configuration
├── healthcheck.sh                 # Health check script
├── .github-workflows-example.yml  # CI/CD example
│
├── config/
│   └── app.toml                   # Application configuration
│
└── init-scripts/
    └── 01-init.sql                # Database initialization
```

## Usage by Use Case

### First Time User
1. Read [QUICK_START.md](QUICK_START.md)
2. Copy [.env.example](.env.example) to `.env`
3. Run `docker compose up -d`

### Developer
1. Read [README.md](README.md)
2. Use [docker-compose.dev.yml](docker-compose.dev.yml)
3. Check [Makefile](Makefile) for commands
4. Customize [config/app.toml](config/app.toml)

### DevOps Engineer
1. Review [ARCHITECTURE.md](ARCHITECTURE.md)
2. Use [docker-compose.prod.yml](docker-compose.prod.yml)
3. Configure [nginx.conf](nginx.conf)
4. Set up CI/CD with [.github-workflows-example.yml](.github-workflows-example.yml)

### Understanding the System
1. [ARCHITECTURE.md](ARCHITECTURE.md) - System design
2. [README.md](README.md) - Detailed usage
3. [Dockerfile.builder](Dockerfile.builder) - Build process
4. [docker-compose.yml](docker-compose.yml) - Service configuration

## File Purposes

### Core Files (Required)

| File | Purpose | Required |
|------|---------|----------|
| Dockerfile.builder | Build Rust applications | Yes |
| docker-compose.yml | Service orchestration | Yes |
| .dockerignore | Optimize build context | Yes |

### Runtime Files (Choose One)

| File | Purpose | Use When |
|------|---------|----------|
| Dockerfile.runtime | Minimal runtime images | Pre-built binaries |
| Dockerfile.builder | Combined build+runtime | Building from source |

### Environment Files (Configure One)

| File | Purpose | Environment |
|------|---------|-------------|
| docker-compose.yml | Standard setup | Production/Staging |
| docker-compose.dev.yml | Development tools | Local development |
| docker-compose.prod.yml | Production optimized | Production with scaling |

### Configuration Files (Optional)

| File | Purpose | Optional |
|------|---------|----------|
| config/app.toml | App configuration | Can use env vars |
| .env | Environment variables | Can use defaults |
| nginx.conf | Reverse proxy | Only if using nginx |
| init-scripts/01-init.sql | Database setup | For PostgreSQL init |

### Utility Files (Helpful)

| File | Purpose | Benefit |
|------|---------|---------|
| Makefile | Simplified commands | Easier operations |
| healthcheck.sh | Health monitoring | Better health checks |
| .github-workflows-example.yml | CI/CD automation | Automated builds |

## Documentation Files

| File | Length | Purpose | Audience |
|------|--------|---------|----------|
| QUICK_START.md | Short | Get started fast | Everyone |
| README.md | Long | Complete guide | Developers |
| ARCHITECTURE.md | Long | System design | DevOps/Architects |
| INDEX.md | Medium | File navigation | Everyone |

## Common Workflows

### Development Workflow
```bash
# Read these files:
- QUICK_START.md
- docker-compose.dev.yml
- Makefile

# Use these commands:
make up        # or: docker compose -f docker-compose.dev.yml up
make logs
make shell
```

### Production Deployment
```bash
# Read these files:
- ARCHITECTURE.md
- docker-compose.prod.yml
- nginx.conf

# Configure:
- .env (from .env.example)
- nginx.conf (SSL certificates)

# Deploy:
docker compose -f docker-compose.prod.yml up -d
```

### CI/CD Setup
```bash
# Read these files:
- .github-workflows-example.yml
- Dockerfile.builder

# Copy workflow:
cp docker/.github-workflows-example.yml .github/workflows/docker.yml

# Configure secrets in GitHub
```

### Troubleshooting
```bash
# Read these sections:
- README.md#troubleshooting
- ARCHITECTURE.md#troubleshooting

# Use these tools:
- Makefile (health, logs, shell)
- healthcheck.sh
```

## File Dependencies

```
docker-compose.yml
├── requires: Dockerfile.builder
├── uses: .env (optional)
├── uses: config/app.toml (optional)
├── uses: init-scripts/01-init.sql (optional)
└── uses: healthcheck.sh (optional)

docker-compose.prod.yml
├── requires: nginx.conf (if using nginx)
├── requires: .env (for secrets)
└── uses: all above

Makefile
└── requires: docker-compose.yml

.github-workflows-example.yml
└── requires: Dockerfile.builder
```

## Maintenance

### Regular Updates
- Keep base images updated (Rust, PostgreSQL, Redis)
- Update dependencies in Dockerfile
- Review security advisories
- Update documentation

### Version Control
- Commit all files except `.env`
- Include `.env.example` as template
- Tag Docker images with semantic versions
- Document breaking changes

### Monitoring
- Check health endpoints regularly
- Monitor resource usage
- Review logs for errors
- Track build times

## Contributing

When adding new files:
1. Update this INDEX.md
2. Add to appropriate section in README.md
3. Include file purpose and usage
4. Add to .dockerignore if temporary/generated

## Additional Resources

- [Docker Documentation](https://docs.docker.com/)
- [Docker Compose Reference](https://docs.docker.com/compose/)
- [Rust Docker Best Practices](https://docs.docker.com/language/rust/)
- [cargo-chef](https://github.com/LukeMathWalker/cargo-chef)

## License

All Docker configuration files are part of the Rustboot project and licensed under MIT.
