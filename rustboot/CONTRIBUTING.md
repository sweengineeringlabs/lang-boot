# Contributing to Rustboot

Thank you for your interest in contributing to Rustboot! This document provides guidelines and instructions for contributing.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Coding Standards](#coding-standards)
- [Documentation](#documentation)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)

## Code of Conduct

This project adheres to a [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## Getting Started

### Prerequisites

- Rust 1.70+ (stable)
- Cargo (comes with Rust)
- Git

### Setup

```bash
# Clone the repository
git clone https://github.com/phdsystems/rustboot.git
cd rustboot

# Build all crates
cargo build --all

# Run tests
cargo test --all
```

## Development Workflow

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-number-description
```

### 2. Make Your Changes

- Write clean, idiomatic Rust code
- Follow the project's coding standards
- Add tests for new functionality
- Update documentation

### 3. Test Your Changes

```bash
# Run all tests
cargo test --all

# Run tests for specific crate
cargo test -p rustboot-validation

# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --all -- -D warnings
```

### 4. Commit Your Changes

Use [Conventional Commits](https://www.conventionalcommits.org/):

```bash
# Feature
git commit -m "feat(validation): add email validation rule"

# Bug fix
git commit -m "fix(cache): resolve TTL expiration issue"

# Documentation
git commit -m "docs(security): update authentication examples"

# Tests
git commit -m "test(http): add integration tests for client"
```

**Commit Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `test`: Test additions/changes
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `chore`: Maintenance tasks

## Coding Standards

### Rust Code Style

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Maximum line length: 100 characters

### Code Organization

- **Keep modules focused**: Each module should have a single responsibility
- **Use inline tests**: Unit tests go in `#[cfg(test)]` modules
- **Integration tests**: Put in `tests/` directory
- **Examples**: Create in `examples/` directory

### Example Code Structure

```rust
//! Module documentation

// Public API
pub struct MyStruct {
    // fields
}

impl MyStruct {
    pub fn new() -> Self {
        // implementation
    }
}

// Private helpers
fn helper_function() {
    // implementation
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_my_struct() {
        let instance = MyStruct::new();
        assert!(/* assertion */);
    }
}
```

## Documentation

### Code Documentation

- Add doc comments to all public items
- Use `///` for item documentation
- Use `//!` for module documentation
- Include examples in doc comments

```rust
/// Validates an email address.
///
/// # Examples
///
/// ```
/// use rustboot_validation::*;
///
/// let validator = StringValidationBuilder::new("email")
///     .email()
///     .build();
/// assert!(validator.validate(&"user@example.com".to_string()).is_ok());
/// ```
pub fn validate_email(email: &str) -> Result<()> {
    // implementation
}
```

### Crate Documentation

Every crate must have `doc/overview.md` with:
- WHAT-WHY-HOW structure
- Code examples
- Links to `examples/` and `tests/`

See [documentation templates](docs/templates/) for guidance.

## Testing

### Requirements

- **Unit tests**: For all public functions (inline with `#[cfg(test)]`)
- **Integration tests**: At minimum `tests/integration.rs`
- **Examples**: At minimum `examples/basic.rs`

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // Group tests by functionality
    mod validation_tests {
        use super::*;
        
        #[test]
        fn test_valid_input() {
            // Arrange
            let validator = create_validator();
            
            // Act
            let result = validator.validate(&valid_input());
            
            // Assert
            assert!(result.is_ok());
        }
        
        #[test]
        fn test_invalid_input() {
            let validator = create_validator();
            let result = validator.validate(&invalid_input());
            assert!(result.is_err());
        }
    }
}
```

### Test Coverage

Aim for:
- 80%+ test coverage for new features
- 100% coverage for critical paths (security, validation)

## Submitting Changes

### Before Submitting

Checklist:
- [ ] All tests pass (`cargo test --all`)
- [ ] Code is formatted (`cargo fmt --all`)
- [ ] No clippy warnings (`cargo clippy --all -- -D warnings`)
- [ ] Documentation is updated
- [ ] Examples are added/updated
- [ ] CHANGELOG.md is updated

### Pull Request Process

1. **Push your branch**:
   ```bash
   git push origin feature/your-feature-name
   ```

2. **Create Pull Request**:
   - Go to GitHub repository
   - Click "New Pull Request"
   - Select your branch
   - Fill in the PR template

3. **PR Template**:
   ```markdown
   ## Description
   Brief description of changes
   
   ## Type of Change
   - [ ] Bug fix
   - [ ] New feature
   - [ ] Breaking change
   - [ ] Documentation update
   
   ## Testing
   Describe testing done
   
   ## Checklist
   - [ ] Tests added/updated
   - [ ] Documentation updated
   - [ ] CHANGELOG.md updated
   ```

4. **Code Review**:
   - Address reviewer feedback
   - Keep commits clean and organized
   - Squash commits if requested

5. **Merge**:
   - Maintainer will merge after approval
   - Branch will be deleted after merge

## Project Structure

```
rustboot/
├── crates/              # Individual crates
│   └── rustboot-*/
│       ├── src/         # Source code
│       ├── tests/       # Integration tests
│       ├── examples/    # Usage examples
│       └── doc/         # Crate documentation
├── docs/                # Framework documentation
│   ├── 3-design/       # Architecture docs
│   ├── 4-development/  # Development guides
│   └── templates/      # Doc templates
├── README.md           # Project overview
├── CONTRIBUTING.md     # This file
└── LICENSE             # MIT license
```

## Getting Help

- **Documentation**: See [docs/overview.md](docs/overview.md)
- **Issues**: [GitHub Issues](https://github.com/phdsystems/rustboot/issues)
- **Discussions**: [GitHub Discussions](https://github.com/phdsystems/rustboot/discussions)

## Recognition

Contributors will be recognized in:
- [AUTHORS.md](AUTHORS.md)
- GitHub contributors page
- Release notes

Thank you for contributing to Rustboot! 🦀
