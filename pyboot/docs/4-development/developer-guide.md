# Pyboot Developer Guide

**Audience**: Developers, Contributors

## WHAT: Development Documentation

This guide covers development practices for working with and contributing to Pyboot.

## WHY: Development Standards

Consistent practices ensure:
- Code quality across modules
- Easy onboarding for contributors
- Maintainable codebase

## HOW: Development Practices

### Project Structure

```
pyboot/
├── src/
│   ├── dev/engineeringlabs/pyboot/    # Main package
│   │   ├── cache/                      # Module
│   │   │   ├── __init__.py            # Public exports
│   │   │   ├── api/                   # Public interfaces
│   │   │   └── core/                  # Implementations
│   │   └── ...                        # Other modules
│   └── test/                          # Tests
├── examples/                          # Usage examples
├── docs/                              # Documentation
└── pyproject.toml                     # Project config
```

### Setting Up Development Environment

```bash
# Clone repository
git clone https://github.com/phd-systems/pyboot
cd pyboot

# Create virtual environment
python -m venv .venv
source .venv/bin/activate  # or .venv\Scripts\activate on Windows

# Install with dev dependencies
pip install -e ".[dev]"
```

### Running Tests

```bash
# Run all tests
python -m pytest src/test -v

# Run with coverage
python -m pytest src/test --cov=src/dev/engineeringlabs/pyboot

# Run specific test file
python -m pytest src/test/test_decorators.py -v
```

### Running Examples

```bash
# Set PYTHONPATH (or install package)
export PYTHONPATH=src  # or $env:PYTHONPATH="src" on PowerShell

# Run example
python examples/decorators_example.py
```

### Code Style

We use:
- **ruff** - Linting and formatting
- **mypy** - Type checking
- **black** - Code formatting

```bash
# Format code
ruff format src

# Lint code
ruff check src

# Type check
mypy src/dev
```

### Creating a New Module

1. Create directory structure:
```bash
mkdir -p src/dev/engineeringlabs/pyboot/mymodule/{api,core}
```

2. Create `__init__.py` files:
```python
# mymodule/__init__.py
from dev.engineeringlabs.pyboot.mymodule.api import MyClass
from dev.engineeringlabs.pyboot.mymodule.core import my_function

__all__ = ["MyClass", "my_function"]
```

3. Add tests:
```bash
touch src/test/test_mymodule.py
```

4. Add example:
```bash
touch examples/mymodule_example.py
```

### Writing Tests

```python
# src/test/test_mymodule.py
import pytest
from dev.engineeringlabs.pyboot.mymodule import MyClass

class TestMyClass:
    def test_basic_usage(self):
        instance = MyClass()
        assert instance.method() == expected_value
    
    @pytest.mark.asyncio
    async def test_async_method(self):
        result = await instance.async_method()
        assert result is not None
```

### Documentation Standards

All modules should have:
1. **Docstrings** - Google style
2. **Type hints** - Full typing
3. **Examples** - In examples/ directory
4. **Tests** - In src/test/ directory

## Development Guides

- [Python Test Organization](guide/python-test-organization.md)
- [Adding New Modules](guide/adding-modules.md)

## Related Documentation

- [Architecture](../3-design/architecture.md) - Design overview
- [Overview](../overview.md) - Module index

---

**Questions?** Open an issue on GitHub.
