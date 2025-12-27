# Contributing to Pyboot

Thank you for your interest in contributing to Pyboot!

## How to Contribute

### Reporting Issues

1. Check existing issues to avoid duplicates
2. Use the issue templates
3. Provide a minimal reproducible example

### Pull Requests

1. Fork the repository
2. Create a feature branch
3. Write tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

### Development Setup

```bash
# Clone your fork
git clone https://github.com/YOUR-USERNAME/pyboot
cd pyboot

# Create virtual environment
python -m venv .venv
source .venv/bin/activate  # or .venv\Scripts\activate on Windows

# Install dev dependencies
pip install -e ".[dev]"

# Run tests
python -m pytest src/test -v
```

### Code Style

- Use `ruff` for linting and formatting
- Use type hints for all public APIs
- Write docstrings (Google style)
- Keep functions small and focused

### Commit Messages

Use conventional commits:
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation
- `test:` Tests
- `refactor:` Refactoring

### Testing

- Write tests for all new functionality
- Aim for high coverage on new code
- Use pytest fixtures appropriately

## Questions?

Open an issue or discussion on GitHub.
