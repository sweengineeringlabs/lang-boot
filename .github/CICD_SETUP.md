# CI/CD Setup Guide

This guide walks you through setting up the complete CI/CD pipeline for the rustboot framework.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Initial Setup](#initial-setup)
- [Required Secrets](#required-secrets)
- [GitHub Pages Setup](#github-pages-setup)
- [Codecov Integration](#codecov-integration)
- [Release Process](#release-process)
- [Verification](#verification)
- [Troubleshooting](#troubleshooting)

## Prerequisites

Before setting up CI/CD, ensure you have:

1. Admin access to the GitHub repository
2. A crates.io account with publish permissions
3. (Optional) Codecov account for code coverage reporting

## Initial Setup

### 1. Enable GitHub Actions

GitHub Actions should be enabled by default. Verify in:
- Repository Settings > Actions > General
- Ensure "Allow all actions and reusable workflows" is selected

### 2. Configure Workflow Permissions

Set appropriate permissions for workflows:

1. Go to: Settings > Actions > General
2. Under "Workflow permissions":
   - Select "Read and write permissions"
   - Check "Allow GitHub Actions to create and approve pull requests"
3. Click "Save"

### 3. Enable Security Features

1. Go to: Settings > Security > Code security and analysis
2. Enable:
   - Dependency graph
   - Dependabot alerts
   - Dependabot security updates
   - Secret scanning
   - Push protection

## Required Secrets

### 1. CARGO_REGISTRY_TOKEN (Required for Releases)

This token is required for publishing crates to crates.io.

**Generate Token:**
1. Log in to [crates.io](https://crates.io)
2. Go to Account Settings > API Tokens
3. Click "New Token"
4. Name: "rustboot-github-actions"
5. Permissions: Select "publish-new" and "publish-update"
6. Click "Create"
7. Copy the generated token

**Add to GitHub:**
1. Go to: Repository Settings > Secrets and variables > Actions
2. Click "New repository secret"
3. Name: `CARGO_REGISTRY_TOKEN`
4. Value: Paste the token from crates.io
5. Click "Add secret"

### 2. CODECOV_TOKEN (Optional)

Required only if you want code coverage reports on Codecov.

**Generate Token:**
1. Log in to [Codecov](https://codecov.io)
2. Add your repository
3. Copy the repository upload token

**Add to GitHub:**
1. Go to: Repository Settings > Secrets and variables > Actions
2. Click "New repository secret"
3. Name: `CODECOV_TOKEN`
4. Value: Paste the Codecov token
5. Click "Add secret"

## GitHub Pages Setup

To enable documentation hosting on GitHub Pages:

### 1. Enable Pages

1. Go to: Settings > Pages
2. Under "Source":
   - Select "GitHub Actions"
3. Click "Save"

### 2. Verify Deployment

After the next push to main:
1. The `docs.yml` workflow will run
2. Documentation will be deployed to: `https://<username>.github.io/rustboot/`

## Codecov Integration

### 1. Setup Codecov (Optional)

1. Visit [Codecov](https://codecov.io)
2. Sign in with GitHub
3. Add the rustboot repository
4. Copy the upload token
5. Add as `CODECOV_TOKEN` secret (see above)

### 2. Add Badge to README

Add to your README.md:

```markdown
[![codecov](https://codecov.io/gh/phdsystems/rustboot/branch/main/graph/badge.svg)](https://codecov.io/gh/phdsystems/rustboot)
```

## Release Process

### Automated Release Workflow

The release process is fully automated when you push a version tag.

### 1. Prepare for Release

Before creating a release:

```bash
# Ensure you're on main and up to date
git checkout main
git pull origin main

# Run tests locally
cargo test --workspace --all-features

# Build all crates
cargo build --workspace --all-features

# Check for issues
cargo clippy --workspace --all-features
```

### 2. Update Version

Update the version in `/home/adentic/rustboot/Cargo.toml`:

```toml
[workspace.package]
version = "0.2.0"  # Update this
```

Commit the version change:

```bash
git add Cargo.toml
git commit -m "chore: bump version to 0.2.0"
git push origin main
```

### 3. Create and Push Tag

```bash
# Create annotated tag
git tag -a v0.2.0 -m "Release version 0.2.0"

# Push tag to trigger release workflow
git push origin v0.2.0
```

### 4. Monitor Release

1. Go to: Actions tab in GitHub
2. Watch the "Release" workflow
3. The workflow will:
   - Validate the version
   - Run all tests
   - Publish to crates.io
   - Generate changelog
   - Create GitHub release

### 5. Verify Publication

After release completes:

```bash
# Check on crates.io (wait a few minutes for propagation)
cargo search rustboot

# Or visit directly
# https://crates.io/crates/rustboot
```

## Verification

### Test CI Pipeline

Create a test branch and PR:

```bash
# Create test branch
git checkout -b test-ci

# Make a small change
echo "# Test" >> README.md

# Commit and push
git add README.md
git commit -m "test: verify CI pipeline"
git push origin test-ci
```

Create a PR and verify:
- All CI checks pass (check, fmt, clippy, test, etc.)
- Code coverage is reported (if Codecov is configured)
- No security issues detected

### Verify Individual Workflows

#### CI Workflow
```bash
# Should trigger on every push/PR
git push origin main
```

#### Security Workflow
```bash
# Runs daily automatically, or trigger manually:
# Go to Actions > Security Audit > Run workflow
```

#### Nightly Workflow
```bash
# Runs daily automatically at 02:00 UTC
# Or trigger manually from Actions tab
```

#### Documentation Workflow
```bash
# Triggers on push to main
git push origin main
# Check: https://<username>.github.io/rustboot/
```

## Branch Protection Rules

Recommended branch protection for `main`:

1. Go to: Settings > Branches
2. Add rule for `main`:
   - ✓ Require a pull request before merging
   - ✓ Require approvals (1)
   - ✓ Dismiss stale pull request approvals
   - ✓ Require status checks to pass before merging
     - Add: `All Checks Passed` (from ci.yml)
     - Add: `Cargo Audit` (from security.yml)
   - ✓ Require conversation resolution before merging
   - ✓ Require linear history
   - ✓ Include administrators

## Maintenance

### Weekly Tasks

- Review and merge Dependabot PRs
- Check security audit results
- Review failed nightly builds

### Monthly Tasks

- Review and update MSRV if needed
- Audit dependencies manually
- Review and update workflow versions

### Before Each Release

- Run full test suite locally
- Update CHANGELOG manually if needed
- Verify documentation is up to date
- Check for breaking changes

## Troubleshooting

### CI Workflow Fails

**Problem**: Tests pass locally but fail in CI

**Solutions**:
1. Check platform-specific code (Windows vs. Linux)
2. Verify environment variables
3. Check file path assumptions
4. Review test output in Actions logs

### Release Workflow Fails

**Problem**: Publishing to crates.io fails

**Solutions**:
1. Verify `CARGO_REGISTRY_TOKEN` is set correctly
2. Ensure token has publish permissions
3. Check that version doesn't already exist on crates.io
4. Verify Cargo.toml version matches tag

### Documentation Build Fails

**Problem**: Documentation fails to build

**Solutions**:
1. Run `cargo doc --workspace --all-features` locally
2. Check for missing documentation comments
3. Fix any rustdoc warnings
4. Verify RUSTDOCFLAGS settings

### Security Workflow Creates Too Many Issues

**Problem**: False positives in security scans

**Solutions**:
1. Update `.github/deny.toml` to allow specific advisories
2. Configure Trivy to ignore certain vulnerabilities
3. Add exceptions for known false positives

### Codecov Reports Not Appearing

**Problem**: Code coverage not showing on PRs

**Solutions**:
1. Verify `CODECOV_TOKEN` is set
2. Check that coverage workflow completed
3. Ensure Codecov integration is enabled
4. Review Codecov logs in Actions

## Advanced Configuration

### Custom Benchmark Thresholds

Edit `.github/workflows/benchmark.yml` to add performance regression checks:

```yaml
- name: Check for regressions
  run: |
    # Add custom benchmark comparison logic
```

### Custom Security Policies

Edit `.github/deny.toml` to customize:
- Allowed/denied licenses
- Advisory exceptions
- Duplicate dependency handling
- Source restrictions

### Conditional Workflow Execution

Optimize workflow runs with path filters:

```yaml
on:
  push:
    paths:
      - 'crates/rustboot-http/**'
      - '!**.md'
```

## Getting Help

If you encounter issues:

1. Check workflow logs in Actions tab
2. Review this documentation
3. Consult `.github/workflows/README.md`
4. Open an issue with the `ci/cd` label

## Resources

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Publishing to crates.io](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [cargo-deny Documentation](https://embarkstudios.github.io/cargo-deny/)
- [Codecov Documentation](https://docs.codecov.com/)
- [GitHub Pages Documentation](https://docs.github.com/en/pages)

---

Last updated: 2024-12-24
