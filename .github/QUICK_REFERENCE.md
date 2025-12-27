# CI/CD Quick Reference

Quick reference guide for developers working with the rustboot CI/CD pipeline.

## Local Pre-Commit Checks

Run these before pushing to ensure CI passes:

```bash
# Format code
cargo fmt --all

# Check for errors
cargo clippy --workspace --all-features --all-targets -- -D warnings

# Run tests
cargo test --workspace --all-features

# Build docs
cargo doc --workspace --all-features --no-deps

# Security audit
cargo install cargo-audit && cargo audit

# Check licenses
cargo install cargo-deny && cargo deny check
```

## Common Commands

### Run Full CI Suite Locally
```bash
# Install required tools
cargo install cargo-nextest cargo-llvm-cov cargo-audit cargo-deny

# Run everything
cargo fmt --all -- --check && \
cargo clippy --workspace --all-features --all-targets -- -D warnings && \
cargo nextest run --workspace --all-features && \
cargo doc --workspace --all-features --no-deps && \
cargo audit && \
cargo deny check
```

### Update Dependencies
```bash
# Check for outdated
cargo install cargo-outdated && cargo outdated

# Update Cargo.lock
cargo update

# Update specific package
cargo update -p <package-name>
```

### Release Workflow

```bash
# 1. Update version in Cargo.toml
vim Cargo.toml  # Update [workspace.package] version

# 2. Commit version bump
git add Cargo.toml
git commit -m "chore: bump version to X.Y.Z"
git push origin main

# 3. Create and push tag
git tag -a vX.Y.Z -m "Release version X.Y.Z"
git push origin vX.Y.Z

# 4. Monitor release in GitHub Actions
# Visit: https://github.com/phdsystems/rustboot/actions
```

## Workflow Status

Check workflow status at: https://github.com/phdsystems/rustboot/actions

### CI Workflow (ci.yml)
- **Triggers**: Push/PR to main
- **Duration**: ~15-20 minutes
- **Key Jobs**: check, fmt, clippy, test (matrix), doc, coverage

### Release Workflow (release.yml)
- **Triggers**: Tags matching `v*.*.*`
- **Duration**: ~20-30 minutes
- **Key Jobs**: validate, test, publish, changelog, release

### Security Workflow (security.yml)
- **Triggers**: Daily at 00:00 UTC, Cargo file changes
- **Duration**: ~10-15 minutes
- **Key Jobs**: audit, deny, trivy, semgrep

## Debugging Failed Workflows

### CI Failures

**Tests fail in CI but pass locally:**
- Check platform-specific issues (Windows path separators, etc.)
- Verify environment variables
- Check for race conditions in async tests

**Clippy fails in CI:**
```bash
# Run with same flags as CI
cargo clippy --workspace --all-features --all-targets -- -D warnings
```

**Format check fails:**
```bash
# Check formatting
cargo fmt --all -- --check

# Auto-fix
cargo fmt --all
```

### Release Failures

**Version mismatch:**
- Ensure tag version matches Cargo.toml version
- Tag: `v0.1.0` → Cargo.toml: `version = "0.1.0"`

**Publishing fails:**
- Verify CARGO_REGISTRY_TOKEN is set
- Check token permissions
- Ensure version doesn't exist on crates.io

## GitHub Actions Secrets

Required secrets in Repository Settings > Secrets > Actions:

1. **CARGO_REGISTRY_TOKEN** (required for releases)
   - Generate: https://crates.io/settings/tokens
   - Permissions: publish-new, publish-update

2. **CODECOV_TOKEN** (optional)
   - Generate: https://codecov.io
   - Used for: Code coverage reports

## PR Checklist

Before opening a PR, ensure:

- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] All tests pass (`cargo test --workspace --all-features`)
- [ ] Documentation is updated
- [ ] New tests added for new features
- [ ] No security vulnerabilities (`cargo audit`)
- [ ] PR template is filled out

## Branch Protection

The `main` branch has these protections:

- Requires PR reviews (1 approval)
- Requires status checks to pass:
  - All Checks Passed
  - Cargo Audit
- Requires conversation resolution
- Linear history enforced

## Badges

Add to README.md:

```markdown
[![CI](https://github.com/phdsystems/rustboot/actions/workflows/ci.yml/badge.svg)](https://github.com/phdsystems/rustboot/actions/workflows/ci.yml)
[![Security](https://github.com/phdsystems/rustboot/actions/workflows/security.yml/badge.svg)](https://github.com/phdsystems/rustboot/actions/workflows/security.yml)
[![codecov](https://codecov.io/gh/phdsystems/rustboot/branch/main/graph/badge.svg)](https://codecov.io/gh/phdsystems/rustboot)
```

## Helpful Links

- **Repository**: https://github.com/phdsystems/rustboot
- **Actions**: https://github.com/phdsystems/rustboot/actions
- **Crates.io**: https://crates.io/crates/rustboot
- **Documentation**: https://phdsystems.github.io/rustboot/
- **Issues**: https://github.com/phdsystems/rustboot/issues

## Troubleshooting

### "CI is slow"
- Workflows use caching (Swatinem/rust-cache)
- First run is slower, subsequent runs are faster
- Check cache is being restored in workflow logs

### "Dependabot creating too many PRs"
- Adjust schedule in `.github/dependabot.yml`
- Use grouped updates (already configured)
- Set `open-pull-requests-limit`

### "Security alerts for false positives"
- Add to ignore list in `.github/deny.toml`
- Document reason for exception
- Review periodically

## Getting Help

1. Check workflow logs in Actions tab
2. Review documentation:
   - `.github/workflows/README.md` - Detailed workflow docs
   - `.github/CICD_SETUP.md` - Setup guide
   - This file - Quick reference
3. Open issue with `ci/cd` label
4. Contact: @elvischidera

## Advanced

### Run workflows manually
```bash
# Install GitHub CLI
gh auth login

# Trigger workflow
gh workflow run ci.yml
gh workflow run security.yml
gh workflow run docs.yml

# List workflow runs
gh run list
```

### View logs
```bash
# List runs for a workflow
gh run list --workflow=ci.yml

# View specific run
gh run view <run-id>

# Download logs
gh run download <run-id>
```

---

**Last Updated**: 2024-12-24
**Maintainer**: @elvischidera
