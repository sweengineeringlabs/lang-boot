# CI/CD Pipeline Guide

**Audience**: DevOps Engineers, Platform Engineers, Developers, Release Managers

## WHAT: Continuous Integration and Continuous Deployment

This guide defines CI/CD pipeline implementation, automation strategies, and best practices for libraries and frameworks.

**Scope**:
- Continuous Integration (CI) pipeline setup
- Automated testing and quality checks
- Continuous Deployment (CD) automation
- Pipeline optimization and maintenance
- Multi-platform builds and testing
- Security scanning and compliance

**Out of Scope**:
- Manual deployment procedures (see [Deployment Workflow](deployment-workflow.md))
- Package registry publishing details (see [Publishing Guide](publishing.md))
- Infrastructure as Code (separate infrastructure docs)

## WHY: Problems and Motivation

### Problems Addressed

1. **Manual Testing Overhead**
   - Current impact: Developers manually run tests before commits
   - Consequence: Missed tests, broken main branch, slower development

2. **Inconsistent Build Environments**
   - Current impact: "Works on my machine" problems
   - Consequence: Integration failures, deployment issues

3. **Slow Feedback Loops**
   - Current impact: Hours or days to discover broken code
   - Consequence: Expensive debugging, blocked developers

4. **No Quality Gates**
   - Current impact: Low-quality code reaches production
   - Consequence: Bugs, technical debt, security vulnerabilities

### Benefits

- **Fast Feedback**: Know within minutes if code breaks
- **Quality Assurance**: Automated gates prevent bad code
- **Consistency**: Same environment every time
- **Confidence**: Deploy frequently without fear
- **Automation**: Free up human time for valuable work

## HOW: Implementation Guide

### CI/CD Pipeline Overview

```
Code Push → CI Pipeline → Quality Gates → CD Pipeline → Deployment
    ↓           ↓              ↓             ↓            ↓
 GitHub     Build/Test    Checks Pass    Publish      Live
            Lint/Audit    Security OK    Release      
```

### Continuous Integration (CI)

#### Core CI Pipeline

**Triggered on**: Every push, every pull request

**Steps**:
1. **Checkout code**
2. **Setup environment** (Rust toolchain, dependencies)
3. **Build** (debug and release)
4. **Run tests** (unit, integration, doc tests)
5. **Check code quality** (clippy, rustfmt)
6. **Security audit** (cargo audit)
7. **Generate reports** (coverage, benchmarks)
8. **Notify results**

#### GitHub Actions Configuration

**File**: `.github/workflows/ci.yml`

```yaml
name: CI

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always
  RUST_BACKTRACE: 1

jobs:
  test:
    name: Test Suite
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        rust: [stable, beta, nightly]
        exclude:
          # Optional: reduce matrix size
          - os: macos-latest
            rust: beta

    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@master
        with:
          toolchain: ${{ matrix.rust }}
          components: rustfmt, clippy

      - name: Cache cargo registry
        uses: actions/cache@v3
        with:
          path: ~/.cargo/registry
          key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}

      - name: Cache cargo index
        uses: actions/cache@v3
        with:
          path: ~/.cargo/git
          key: ${{ runner.os }}-cargo-index-${{ hashFiles('**/Cargo.lock') }}

      - name: Cache target directory
        uses: actions/cache@v3
        with:
          path: target
          key: ${{ runner.os }}-${{ matrix.rust }}-target-${{ hashFiles('**/Cargo.lock') }}

      - name: Check formatting
        run: cargo fmt --all -- --check
        if: matrix.rust == 'stable' && matrix.os == 'ubuntu-latest'

      - name: Run clippy
        run: cargo clippy --all-targets --all-features -- -D warnings
        if: matrix.rust == 'stable'

      - name: Build
        run: cargo build --verbose --all-features

      - name: Run tests
        run: cargo test --verbose --all-features

      - name: Run doc tests
        run: cargo test --doc --all-features

  security:
    name: Security Audit
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: rustsec/audit-check@v1
        with:
          token: ${{ secrets.GITHUB_TOKEN }}

  coverage:
    name: Code Coverage
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install tarpaulin
        run: cargo install cargo-tarpaulin

      - name: Generate coverage
        run: cargo tarpaulin --out Xml --all-features

      - name: Upload to codecov
        uses: codecov/codecov-action@v3
        with:
          files: ./cobertura.xml
          flags: rust
          name: rustboot-coverage
```

#### Quality Gates

**All checks must pass before merge**:

| Check | Tool | Threshold | Block Merge |
|-------|------|-----------|-------------|
| Build | cargo build | Must succeed | ✅ Yes |
| Tests | cargo test | 100% pass | ✅ Yes |
| Format | cargo fmt | Perfect format | ✅ Yes |
| Lints | cargo clippy | No warnings | ✅ Yes |
| Security | cargo audit | No vulnerabilities | ✅ Yes |
| Coverage | tarpaulin | ≥80% | ⚠️ Warn |
| Docs | cargo doc | Builds successfully | ✅ Yes |

### Continuous Deployment (CD)

#### Automated Release Pipeline

**Triggered on**: Git tag push (vX.Y.Z)

**File**: `.github/workflows/release.yml`

```yaml
name: Release

on:
  push:
    tags:
      - 'v*.*.*'

permissions:
  contents: write

jobs:
  create-release:
    name: Create GitHub Release
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Extract version from tag
        id: version
        run: echo "VERSION=${GITHUB_REF#refs/tags/v}" >> $GITHUB_OUTPUT

      - name: Extract changelog
        id: changelog
        run: |
          # Extract section for this version from CHANGELOG.md
          sed -n "/## \[${{ steps.version.outputs.VERSION }}\]/,/## \[/p" CHANGELOG.md | head -n -1 > release-notes.md

      - name: Create GitHub Release
        uses: softprops/action-gh-release@v1
        with:
          body_path: release-notes.md
          draft: false
          prerelease: false

  publish-crate:
    name: Publish to crates.io
    runs-on: ubuntu-latest
    needs: create-release
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Publish to crates.io
        run: cargo publish --token ${{ secrets.CARGO_TOKEN }}

  build-binaries:
    name: Build Release Binaries
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            artifact_name: rustboot-linux-x86_64
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            artifact_name: rustboot-windows-x86_64.exe
          - os: macos-latest
            target: x86_64-apple-darwin
            artifact_name: rustboot-macos-x86_64

    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}

      - name: Build release binary
        run: cargo build --release --target ${{ matrix.target }}

      - name: Upload binary to release
        uses: softprops/action-gh-release@v1
        with:
          files: target/${{ matrix.target }}/release/${{ matrix.artifact_name }}
```

#### Automated Documentation Deployment

**File**: `.github/workflows/docs.yml`

```yaml
name: Documentation

on:
  push:
    branches: [main]
    tags: ['v*']

jobs:
  deploy-docs:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Build documentation
        run: cargo doc --no-deps --all-features

      - name: Add index.html redirect
        run: echo '<meta http-equiv="refresh" content="0; url=rustboot">' > target/doc/index.html

      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./target/doc
          cname: docs.rustboot.dev  # Optional: custom domain
```

### Multi-Platform Testing

#### Testing Matrix

Test on multiple:
- **Operating Systems**: Linux, Windows, macOS
- **Rust Versions**: stable, beta, nightly
- **Architectures**: x86_64, aarch64 (ARM)

**Why**: Catch platform-specific bugs early

#### Platform-Specific Tests

```yaml
jobs:
  platform-tests:
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            features: default,linux-specific
          - os: windows-latest
            features: default,windows-specific
          - os: macos-latest
            features: default,macos-specific

    steps:
      - name: Run platform tests
        run: cargo test --features ${{ matrix.features }}
```

### Performance and Benchmarking

#### Automated Benchmarks

**File**: `.github/workflows/benchmark.yml`

```yaml
name: Benchmark

on:
  push:
    branches: [main]
  pull_request:

jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Run benchmarks
        run: cargo bench --all-features | tee benchmark-results.txt

      - name: Store benchmark result
        uses: benchmark-action/github-action-benchmark@v1
        with:
          tool: 'cargo'
          output-file-path: benchmark-results.txt
          github-token: ${{ secrets.GITHUB_TOKEN }}
          auto-push: true
          # Alert if performance degrades
          alert-threshold: '150%'
          comment-on-alert: true
          fail-on-alert: false
```

### Security and Compliance

#### Automated Security Scanning

**1. Dependency Scanning**
```yaml
- name: Check dependencies
  run: cargo audit

- name: Check for outdated deps
  run: |
    cargo install cargo-outdated
    cargo outdated --exit-code 1 --depth 1
```

**2. License Compliance**
```yaml
- name: Check licenses
  run: |
    cargo install cargo-license
    cargo license --avoid-dev-deps --avoid-build-deps
```

**3. Secret Scanning**
```yaml
- name: Scan for secrets
  uses: trufflesecurity/trufflehog@main
  with:
    path: ./
    base: main
```

**4. SAST (Static Analysis)**
```yaml
- name: Run Clippy pedantic
  run: cargo clippy --all-targets --all-features -- -W clippy::pedantic
```

#### Compliance Checks

```yaml
name: Compliance

on: [push, pull_request]

jobs:
  compliance:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Check SPDX headers
        run: |
          # Verify all source files have SPDX license identifier
          find src -name "*.rs" -exec grep -L "SPDX-License-Identifier" {} \; | tee missing-license.txt
          [ ! -s missing-license.txt ]

      - name: Verify CHANGELOG
        run: |
          # Check CHANGELOG has Unreleased section
          grep -q "## \[Unreleased\]" CHANGELOG.md

      - name: Check required files
        run: |
          # Verify governance files exist
          test -f CODE_OF_CONDUCT.md
          test -f SECURITY.md
          test -f CONTRIBUTING.md
```

### Pipeline Optimization

#### Caching Strategy

**Cache these to speed up builds**:
```yaml
- name: Cache dependencies
  uses: actions/cache@v3
  with:
    path: |
      ~/.cargo/bin/
      ~/.cargo/registry/index/
      ~/.cargo/registry/cache/
      ~/.cargo/git/db/
      target/
    key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
    restore-keys: |
      ${{ runner.os }}-cargo-
```

**Benefits**:
- 5-10x faster builds
- Reduced GitHub Actions minutes
- Faster feedback

#### Conditional Execution

**Skip unnecessary jobs**:
```yaml
jobs:
  docs:
    # Only run docs on main branch
    if: github.ref == 'refs/heads/main'

  deploy:
    # Only deploy on tags
    if: startsWith(github.ref, 'refs/tags/v')
```

#### Parallel Execution

**Run independent jobs in parallel**:
```yaml
jobs:
  test:
    # Runs in parallel
  lint:
    # Runs in parallel
  security:
    # Runs in parallel
  
  deploy:
    needs: [test, lint, security]  # Waits for all
```

### Notification and Reporting

#### Slack Notifications

```yaml
- name: Notify Slack on failure
  if: failure()
  uses: slackapi/slack-github-action@v1
  with:
    webhook-url: ${{ secrets.SLACK_WEBHOOK }}
    payload: |
      {
        "text": "❌ Build failed: ${{ github.repository }}/${{ github.ref }}"
      }
```

#### GitHub Status Checks

```yaml
- name: Report status
  uses: actions/github-script@v6
  if: always()
  with:
    script: |
      github.rest.repos.createCommitStatus({
        owner: context.repo.owner,
        repo: context.repo.repo,
        sha: context.sha,
        state: '${{ job.status }}',
        context: 'CI Pipeline',
        description: 'Complete'
      })
```

### Branch Protection Rules

**Configure in GitHub Settings → Branches**:

**For `main` branch**:
- ✅ Require pull request reviews (at least 1)
- ✅ Require status checks to pass before merging
  - CI / Test Suite (ubuntu-latest, stable)
  - CI / Security Audit
  - CI / Check formatting
  - CI / Run clippy
- ✅ Require branches to be up to date before merging
- ✅ Require signed commits (optional but recommended)
- ✅ Include administrators (enforce for everyone)
- ✅ Restrict who can push to matching branches
- ❌ Allow force pushes (never allow)
- ❌ Allow deletions (never allow)

### CI/CD for Different Project Types

#### Library/Framework

```yaml
on:
  push:
    branches: [main]
  pull_request:
  release:
    types: [published]

jobs:
  test → lint → audit → publish (on release)
```

#### Application/Service

```yaml
on:
  push:
    branches: [main]
  pull_request:

jobs:
  test → build → deploy-staging → integration-tests → deploy-production
```

### Troubleshooting Common CI/CD Issues

#### Issue: "CI runs too slow"

**Solutions**:
- Add caching for dependencies
- Use `cargo clippy --all-targets` instead of separate runs
- Run expensive jobs (benchmarks, coverage) only on main branch
- Reduce testing matrix (fewer OS/Rust version combinations)

#### Issue: "Flaky tests"

**Symptoms**: Tests pass locally but fail in CI randomly

**Solutions**:
```rust
// Add retry logic for flaky network tests
#[tokio::test(flavor = "multi_thread")]
#[cfg_attr(not(feature = "flaky-tests"), ignore)]
async fn test_network_call() {
    // Retry logic
}

// Use deterministic random seeds
use rand::SeedableRng;
let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(42);
```

#### Issue: "Out of disk space"

**Solutions**:
```yaml
- name: Free disk space
  run: |
    sudo rm -rf /usr/local/lib/android
    sudo rm -rf /usr/share/dotnet
    df -h

- name: Limit target size
  run: cargo clean -p rustboot
```

### Best Practices

#### ✅ DO:

- **Run CI on every commit** - Catch issues early
- **Make CI fast** - Developers won't wait >10 minutes
- **Cache aggressively** - Speed up builds
- **Fail fast** - Stop on first error
- **Test on multiple platforms** - Catch platform bugs
- **Automate everything** - Manual steps get skipped
- **Monitor pipeline health** - Track success rates
- **Keep secrets secret** - Use GitHub Secrets

#### ❌ DON'T:

- **Skip tests in CI** - Even for "small" changes
- **Ignore warnings** - They become errors later
- **Commit credentials** - Use secrets management
- **Have flaky tests** - Fix or mark as flaky
- **Make CI optional** - Required status checks
- **Cache everything** - Only cache dependencies
- **Deploy directly from CI** - Use CD pipeline
- **Ignore failed pipelines** - Fix immediately

## Summary

CI/CD pipelines automate testing, building, and deployment, providing fast feedback and consistent quality. Use GitHub Actions for Rust projects with multi-platform testing, security scanning, and automated releases. Implement caching, parallel execution, and quality gates to optimize pipelines.

**Key Takeaways**:
1. **Automate all checks** - Testing, linting, security
2. **Multi-platform testing** - Linux, Windows, macOS
3. **Fast feedback** - Optimize with caching and parallelism
4. **Quality gates** - Block merges on failures
5. **Secure pipelines** - Protect secrets, scan for vulnerabilities

---

**Related Documentation**:
- [Deployment Workflow Guide](deployment-workflow.md) - Manual deployment procedures
- [Publishing Guide](publishing.md) - Publishing to registries
- [Release Versioning Guide](release-versioning.md) - Version management
- [Repository Governance](../../4-development/guide/repository-governance.md) - Required files

**External Resources**:
- [GitHub Actions Documentation](https://docs.github.com/en/actions) - Official docs
- [Rust CI/CD Examples](https://github.com/actions-rs) - Community actions
- [Cargo Book](https://doc.rust-lang.org/cargo/) - Cargo documentation
- [Security Best Practices](https://cheatsheetseries.owasp.org/cheatsheets/CI_CD_Security_Cheat_Sheet.html) - OWASP CI/CD security

**Last Updated**: 2025-12-22  
**Version**: 1.0  
**Next Review**: 2026-03-22
