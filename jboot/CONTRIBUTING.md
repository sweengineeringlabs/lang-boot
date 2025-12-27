# Contributing to JBoot

Thank you for your interest in contributing to JBoot!

## Development Setup

### Prerequisites
- Java 17+
- Maven 3.8+

### Building
```bash
mvn clean install
```

### Running Tests
```bash
mvn test
```

## Coding Standards

### Architecture (SEA Pattern)

Each module follows the Stratified Encapsulation Architecture:

```
module/
├── src/main/java/com/jboot/{module}/
│   ├── api/       # Public interfaces and contracts
│   ├── core/      # Default implementations
│   └── spi/       # Extension points
├── src/test/java/
└── doc/
    └── overview.md
```

### Layers

- **API Layer**: Public interfaces, DTOs, exceptions
- **Core Layer**: Default implementations of API interfaces
- **SPI Layer**: Extension points for custom implementations

### Guidelines

1. **Interfaces First**: Define API interfaces before implementations
2. **Immutability**: Prefer immutable objects (records)
3. **Fluent APIs**: Use builders for complex object construction
4. **Functional Style**: Use lambdas, streams, and functional interfaces
5. **Documentation**: Add Javadoc to all public APIs
6. **Testing**: Write tests for all implementations

## Pull Request Process

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes
4. Write/update tests
5. Run `mvn verify`
6. Submit a pull request

## Code Style

- Follow standard Java conventions
- Use meaningful names
- Keep methods focused and small
- Add Javadoc to public APIs

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
