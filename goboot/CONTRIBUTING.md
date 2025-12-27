# Contributing to Goboot

Thank you for your interest in contributing to Goboot!

## Getting Started

1. Fork the repository
2. Clone your fork
3. Create a feature branch

## Development

```bash
# Run tests
go test ./...

# Run with coverage
go test -coverprofile=coverage.out ./...

# Format code
go fmt ./...

# Lint
golangci-lint run
```

## Module Structure

Each module follows the SEA pattern:

```
module/
├── module.go     # Facade - re-exports API and Core
├── api/          # Public contracts
│   └── types.go  # Interfaces, types
├── core/         # Implementations
│   └── impl.go
└── spi/          # Extension points (optional)
    └── provider.go
```

## Pull Request Process

1. Ensure tests pass
2. Update documentation if needed
3. Follow the coding standards
4. Submit PR with clear description

## Code of Conduct

Be respectful and inclusive in all interactions.
