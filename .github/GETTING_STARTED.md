# Getting Started with rustboot CI/CD

Welcome to the rustboot framework CI/CD system! This guide will help you get started quickly.

## For First-Time Contributors

### Before You Start

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/rustboot.git
   cd rustboot
   ```

3. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

### Making Your First Contribution

1. **Create a new branch**:
   ```bash
   git checkout -b feature/my-awesome-feature
   ```

2. **Make your changes** to the code

3. **Run local checks** (IMPORTANT - do this before pushing):
   ```bash
   # Format your code
   cargo fmt --all

   # Check for errors
   cargo clippy --workspace --all-features --all-targets

   # Run tests
   cargo test --workspace --all-features
   ```

4. **Commit your changes**:
   ```bash
   git add .
   git commit -m "feat: add my awesome feature"
   ```

5. **Push to your fork**:
   ```bash
   git push origin feature/my-awesome-feature
   ```

6. **Create a Pull Request** on GitHub
   - The PR template will guide you through the process
   - All CI checks will run automatically

### What Happens After You Create a PR?

The CI system will automatically:

1. Check your code formatting
2. Run clippy (linter)
3. Run all tests on multiple platforms
4. Build documentation
5. Check for security vulnerabilities
6. Check for spelling errors

You'll see green checkmarks when everything passes!

## For Regular Contributors

### Daily Workflow

```bash
# 1. Update your local main
git checkout main
git pull upstream main

# 2. Create feature branch
git checkout -b feature/new-thing

# 3. Make changes...

# 4. Run pre-commit checks
cargo fmt --all
cargo clippy --workspace --all-features --all-targets -- -D warnings
cargo test --workspace --all-features

# 5. Commit and push
git add .
git commit -m "feat: add new thing"
git push origin feature/new-thing

# 6. Create PR on GitHub
```

### Quick Commands Reference

| Task | Command |
|------|---------|
| Format code | `cargo fmt --all` |
| Check formatting | `cargo fmt --all -- --check` |
| Run linter | `cargo clippy --workspace --all-features` |
| Run tests | `cargo test --workspace --all-features` |
| Build docs | `cargo doc --workspace --all-features --no-deps` |
| Security audit | `cargo audit` (requires: `cargo install cargo-audit`) |

### Understanding CI Checks

When you create a PR, you'll see these checks:

- **Check** - Verifies the project builds
- **Rustfmt** - Code formatting check
- **Clippy** - Lint checks
- **Test Suite** - Runs all tests (multiple platforms)
- **Documentation** - Ensures docs build without warnings
- **Code Coverage** - Measures test coverage
- **MSRV** - Checks minimum supported Rust version
- **Cargo Deny** - License and security checks
- **Spell Check** - Checks for typos

All must pass before your PR can be merged!

## For Maintainers

### Repository Setup (One-Time)

1. **Configure secrets** (see CICD_SETUP.md for details):
   - Add `CARGO_REGISTRY_TOKEN` for releases
   - Add `CODECOV_TOKEN` for coverage (optional)

2. **Enable GitHub Pages**:
   - Settings > Pages > Source: "GitHub Actions"

3. **Set up branch protection**:
   - Settings > Branches > Add rule for `main`
   - Require PR reviews
   - Require status checks to pass

### Releasing a New Version

```bash
# 1. Ensure you're on main and up to date
git checkout main
git pull origin main

# 2. Update version in Cargo.toml
# Edit: [workspace.package] version = "X.Y.Z"
vim Cargo.toml

# 3. Commit version bump
git add Cargo.toml
git commit -m "chore: bump version to X.Y.Z"
git push origin main

# 4. Create and push tag
git tag -a vX.Y.Z -m "Release version X.Y.Z"
git push origin vX.Y.Z

# 5. Monitor the release workflow
# GitHub Actions will automatically:
# - Run all tests
# - Publish to crates.io
# - Create GitHub release
# - Generate changelog
```

### Monitoring CI/CD

- **View workflow runs**: Actions tab on GitHub
- **Check security alerts**: Security tab on GitHub
- **Review Dependabot PRs**: Pull requests tab
- **Monitor coverage**: Codecov dashboard (if configured)

### Common Maintenance Tasks

#### Update Dependencies
```bash
# Check for updates
cargo outdated

# Update specific package
cargo update -p package-name

# Update all
cargo update
```

#### Review Security Alerts
- Security workflow runs daily
- Check the Security tab for alerts
- Review and merge Dependabot security PRs promptly

#### Respond to Failed Nightly Builds
- Nightly workflow tests against nightly Rust
- Failures may indicate upcoming breaking changes
- Review and fix or document as known issue

## File Guide

Quick reference to important files:

- **QUICK_REFERENCE.md** - Commands and quick tips
- **CICD_SETUP.md** - Complete setup instructions
- **INDEX.md** - File index and descriptions
- **workflows/README.md** - Detailed workflow documentation

## Troubleshooting

### "My PR checks are failing"

1. **Check the logs**: Click on the failed check to see details
2. **Run locally**: Use the commands above to reproduce
3. **Fix the issue**: Update your code
4. **Push again**: CI will re-run automatically

### "Tests pass locally but fail in CI"

Common causes:
- Platform differences (Windows vs Linux)
- Missing environment variables
- Race conditions in async tests
- File path assumptions

### "Clippy is complaining"

```bash
# See all clippy warnings
cargo clippy --workspace --all-features --all-targets

# Auto-fix some issues
cargo clippy --fix --workspace --all-features
```

### "Format check failed"

```bash
# Fix formatting
cargo fmt --all

# Verify it's fixed
cargo fmt --all -- --check
```

## Best Practices

### Commit Messages

Follow conventional commits:
- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `test:` - Test changes
- `refactor:` - Code refactoring
- `chore:` - Maintenance tasks

Examples:
```bash
git commit -m "feat: add HTTP client timeout configuration"
git commit -m "fix: resolve race condition in state machine"
git commit -m "docs: update middleware examples"
```

### Writing Tests

- Add tests for new features
- Ensure tests are deterministic
- Use descriptive test names
- Clean up resources in tests

### Documentation

- Update docs when changing public APIs
- Add examples for new features
- Keep README.md up to date
- Document breaking changes

## Getting Help

### Resources

1. **Quick questions**: Check QUICK_REFERENCE.md
2. **Setup issues**: Read CICD_SETUP.md
3. **Workflow details**: See workflows/README.md
4. **File index**: Consult INDEX.md

### Support Channels

- **GitHub Issues**: Open an issue with the `question` label
- **GitHub Discussions**: For general questions
- **Code Owner**: @elvischidera

## Next Steps

### For Contributors
1. Read QUICK_REFERENCE.md for common commands
2. Make your first contribution!
3. Review the PR template before submitting

### For Maintainers
1. Complete setup steps in CICD_SETUP.md
2. Review workflows/README.md
3. Set up branch protection
4. Configure secrets

## Additional Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [GitHub Actions Docs](https://docs.github.com/en/actions)
- [Rustup Docs](https://rust-lang.github.io/rustup/)

---

**Welcome aboard!** 🚀

If you have questions or suggestions for improving this guide, please open an issue!

**Last Updated**: 2024-12-24
**Maintained by**: @elvischidera
