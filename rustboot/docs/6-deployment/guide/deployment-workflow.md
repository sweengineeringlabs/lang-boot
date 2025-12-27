# Deployment Workflow Guide

**Audience**: DevOps Engineers, Release Managers, Platform Engineers, Developers

## WHAT: Deployment Process and Strategies

This guide defines the complete deployment workflow for libraries and frameworks, from code commit to production release.

**Scope**:
- Deployment environments and strategies
- Pre-deployment validation and testing
- Deployment execution and rollback procedures
- Post-deployment verification
- Rollback and recovery strategies
- Environment-specific configurations

**Out of Scope**:
- CI/CD pipeline implementation (see [CI/CD Guide](ci-cd.md))
- Package registry publishing (see [Publishing Guide](publishing.md))
- Application-specific deployment (different from library deployment)

## WHY: Problems and Motivation

### Problems Addressed

1. **Manual Deployment Errors**
   - Current impact: Human error during deployment causes outages
   - Consequence: Production incidents, rollbacks, user impact

2. **Inconsistent Environments**
   - Current impact: "Works on my machine" but fails in production
   - Consequence: Deployment failures, debugging difficulties

3. **No Rollback Strategy**
   - Current impact: Broken deployments stay live
   - Consequence: Extended outages, user frustration

4. **Poor Deployment Visibility**
   - Current impact: Can't track what's deployed where
   - Consequence: Configuration drift, security vulnerabilities

### Benefits

- **Reliability**: Automated deployments reduce errors
- **Speed**: Faster time from code to production
- **Confidence**: Tested deployment process
- **Traceability**: Know exactly what's deployed
- **Recovery**: Quick rollback when issues occur

## HOW: Implementation Guide

### Deployment Environments

#### Environment Hierarchy

**For Libraries/Frameworks**:
```
Development
    ↓
Staging/Testing
    ↓
Production (crates.io, npm, PyPI, etc.)
```

**For Applications** (if applicable):
```
Local Development
    ↓
CI/CD Build Environment
    ↓
Staging/QA
    ↓
Pre-Production
    ↓
Production
```

#### Environment Characteristics

**Development**:
- **Purpose**: Active development and experimentation
- **Stability**: Unstable, frequent changes
- **Data**: Mock/test data
- **Access**: All developers
- **Updates**: Continuous, automated from main branch

**Staging/Testing**:
- **Purpose**: Pre-release validation
- **Stability**: Semi-stable, mirrors production
- **Data**: Production-like test data
- **Access**: QA team, select developers
- **Updates**: Manual or scheduled

**Production**:
- **Purpose**: Public releases
- **Stability**: Highly stable, versioned releases
- **Data**: Real users
- **Access**: Release managers only
- **Updates**: Manual, controlled releases

### Deployment Strategies

#### Strategy 1: Direct Deployment (Recommended for Libraries)

**For libraries published to package registries**:

```
Code → Tests Pass → Version Tag → Publish to Registry
```

**Characteristics**:
- Simple, straightforward
- Immutable once published
- No gradual rollout (users control upgrade)
- Cannot unpublish (most registries)

**Best for**:
- Libraries and frameworks
- Packages with semantic versioning
- APIs with backwards compatibility

#### Strategy 2: Blue-Green Deployment (Applications)

**If deploying applications/services**:

```
Blue (current)     Green (new)
    ↓                  ↓
Load Balancer switches traffic
    ↓
Green becomes Blue
```

**Characteristics**:
- Zero downtime
- Instant rollback (switch back)
- Doubles infrastructure temporarily

#### Strategy 3: Canary Deployment (Applications)

**Gradual rollout**:

```
1% traffic → 5% → 25% → 50% → 100%
    ↓         ↓     ↓      ↓      ↓
Monitor   Monitor Check Metrics Complete
```

**Characteristics**:
- Gradual validation
- Early issue detection
- Minimal user impact on failures

### Pre-Deployment Checklist

#### Code Quality Validation

- [ ] **All tests passing**
  - Unit tests: `cargo test --all`
  - Integration tests: `cargo test --features integration`
  - Documentation tests: `cargo test --doc`

- [ ] **Code quality checks**
  - Linting: `cargo clippy -- -D warnings`
  - Formatting: `cargo fmt -- --check`
  - Security audit: `cargo audit`

- [ ] **Dependency updates**
  - Check for outdated dependencies: `cargo outdated`
  - Review security advisories
  - Update critical CVEs

#### Documentation Validation

- [ ] **Documentation complete**
  - API documentation generated: `cargo doc --no-deps`
  - README updated
  - CHANGELOG updated with version
  - Examples tested

- [ ] **Links verified**
  - No broken documentation links
  - External resources accessible
  - Examples work with new version

#### Version Management

- [ ] **Version numbers updated**
  - Cargo.toml version bumped correctly
  - CHANGELOG has version entry
  - Git tag ready (not yet pushed)
  - Version follows SemVer (see [Release Versioning](release-versioning.md))

- [ ] **Breaking changes documented**
  - Migration guide created if needed
  - Deprecation warnings in place
  - CHANGELOG clearly marks breaking changes

#### Security Review

- [ ] **Security scan completed**
  - `cargo audit` passed
  - No known vulnerabilities
  - Dependencies reviewed

- [ ] **Secrets removed**
  - No API keys in code
  - No credentials committed
  - Environment variables documented

### Deployment Execution

#### Step 1: Final Pre-Deployment Validation

```bash
# Run full test suite
cargo test --all-features

# Check for warnings
cargo build --release --all-features

# Verify package
cargo package --list

# Dry run publish
cargo publish --dry-run
```

**Expected**: All checks pass, no errors or warnings

#### Step 2: Create Release Tag

```bash
# Update CHANGELOG
# Move [Unreleased] items to [X.Y.Z] - YYYY-MM-DD

# Commit version changes
git add Cargo.toml CHANGELOG.md
git commit -m "chore: release version X.Y.Z"

# Create annotated tag
git tag -a vX.Y.Z -m "Release version X.Y.Z

Release notes:
- [Summary of changes]
- [Key features]
- [Bug fixes]
"

# Push commits and tag
git push origin main
git push origin vX.Y.Z
```

#### Step 3: Build and Publish

**For Rust libraries**:
```bash
# Build release artifacts
cargo build --release

# Publish to crates.io
cargo publish

# If workspace with multiple crates, publish in dependency order
cargo publish -p rustboot-common
cargo publish -p rustboot-core
cargo publish -p rustboot
```

**For other languages**: See [Publishing Guide](publishing.md)

#### Step 4: Create GitHub Release

```bash
# Via GitHub CLI
gh release create vX.Y.Z \
  --title "Version X.Y.Z" \
  --notes-file RELEASE_NOTES.md

# Or manually via GitHub web interface
# - Go to Releases → Draft a new release
# - Select tag vX.Y.Z
# - Copy CHANGELOG entries
# - Attach binaries if applicable
# - Publish release
```

#### Step 5: Update Documentation Site

```bash
# If you have a docs site (e.g., docs.rs, GitHub Pages)
# Docs.rs automatically updates for crates.io publications

# For custom documentation sites
cd docs-site
# Update version
# Build and deploy
./deploy.sh
```

### Post-Deployment Verification

#### Immediate Verification (Within 5 minutes)

- [ ] **Registry verification**
  - Package appears on registry (crates.io, npm, etc.)
  - Correct version number shown
  - Documentation generated successfully

- [ ] **Installation test**
  ```bash
  # In a clean directory
  cargo new test-install
  cd test-install
  # Add dependency with new version
  cargo add rustboot@X.Y.Z
  cargo build
  ```

- [ ] **Documentation check**
  - docs.rs built successfully
  - API docs accessible
  - Examples compile

#### Extended Verification (Within 1 hour)

- [ ] **Download metrics**
  - Check registry shows the new version
  - Monitor initial download counts

- [ ] **Community feedback**
  - Monitor GitHub issues
  - Check Discord/Slack/forums
  - Watch for quick bug reports

- [ ] **Automated tests**
  - CI/CD runs against new version
  - Integration tests with dependent projects pass

### Rollback Procedures

#### When to Rollback

Rollback if:
- Critical bug discovered affecting most users
- Security vulnerability introduced
- Breaking change not documented
- Installation failures widespread
- Dependency incompatibilities

#### Rollback for Libraries (Published to Registry)

**Problem**: Most registries don't allow unpublishing

**Solution 1: Yank Version** (Recommended)
```bash
# Yank the broken version
cargo yank --vers X.Y.Z

# Immediately publish patched version
# Update version to X.Y.(Z+1)
cargo publish
```

**What yanking does**:
- Prevents new projects from using that version
- Existing projects with Cargo.lock continue working
- Marks version as problematic

**Solution 2: Publish Hotfix**
```bash
# Create hotfix branch from broken tag
git checkout -b hotfix/X.Y.(Z+1) vX.Y.Z

# Fix the critical issue
git commit -m "fix: critical issue in X.Y.Z"

# Update version
# Update CHANGELOG
git commit -m "chore: release hotfix X.Y.(Z+1)"

# Tag and publish
git tag -a vX.Y.(Z+1) -m "Hotfix release"
cargo publish
```

#### Rollback for Applications

**Blue-Green**: Switch load balancer back to previous version
```bash
# Switch traffic back to blue environment
kubectl set-image deployment/app app=app:previous-version

# Or with load balancer
lb-cli switch-to blue
```

**Canary**: Halt rollout and route 100% to stable
```bash
# Set canary weight to 0%
kubectl patch deployment canary -p '{"spec":{"replicas":0}}'
```

### Deployment Monitoring

#### Metrics to Track

**During Deployment**:
- Build success/failure rate
- Test pass rate
- Deployment duration
- Artifact size

**Post-Deployment**:
- Download/install counts
- Error rates from users
- GitHub issues/bug reports
- Support ticket volume

#### Alerting

Set up alerts for:
- Failed builds
- Failed tests
- Security vulnerabilities
- High error rates post-release
- Unusual download patterns

### Environment-Specific Configuration

#### Configuration Management

**Best Practices**:
```
config/
├── default.toml       # Default settings
├── development.toml   # Dev overrides
├── staging.toml       # Staging overrides
└── production.toml    # Production overrides
```

**Loading configuration**:
```rust
use config::{Config, Environment, File};

let mut cfg = Config::builder()
    .add_source(File::with_name("config/default"))
    .add_source(File::with_name(&format!("config/{}", env)))
    .add_source(Environment::with_prefix("APP"))
    .build()?;
```

#### Secrets Management

**Never commit secrets**:
- Use environment variables
- Use secret management tools (Vault, AWS Secrets Manager)
- Encrypt sensitive config files

**Example**:
```bash
# .env.example (committed)
DATABASE_URL=postgres://localhost/dev
API_KEY=your_key_here

# .env (not committed, in .gitignore)
DATABASE_URL=postgres://real-server/prod
API_KEY=actual_secret_key
```

### Deployment Workflows by Project Type

#### Library/Framework Deployment

```
1. Code merged to main
2. CI/CD runs tests
3. Developer tags release
4. CI/CD publishes to registry
5. Documentation auto-updates
6. GitHub release created
7. Community notified
```

#### Application Deployment

```
1. Code merged to main
2. CI/CD builds and tests
3. Deploy to staging
4. Run smoke tests
5. Deploy to production (blue-green or canary)
6. Monitor metrics
7. Gradual rollout or switch
8. Post-deployment verification
```

### Common Deployment Issues

#### Issue: "Package won't publish"

**Symptoms**: `cargo publish` fails

**Solutions**:
```bash
# Check you're logged in
cargo login [token]

# Verify package contents
cargo package --list

# Check for uncommitted changes
git status

# Ensure version isn't already published
cargo search [crate-name]
```

#### Issue: "Documentation build fails"

**Symptoms**: docs.rs shows build failure

**Solutions**:
```bash
# Test docs locally
cargo doc --no-deps --document-private-items

# Check for broken doc links
cargo rustdoc -- -D warnings

# Review docs.rs build logs
# Visit docs.rs/crate/[name]/[version]/builds
```

#### Issue: "Dependencies conflict"

**Symptoms**: Users can't install due to dependency conflicts

**Solutions**:
- Review dependency version ranges
- Use `cargo tree` to check dependency resolution
- Consider loosening version requirements
- Test installation in clean environment

### Best Practices

#### ✅ DO:

- **Automate everything possible** - Manual steps introduce errors
- **Test in production-like environment** - Catch issues early
- **Have rollback plan ready** - Before every deployment
- **Document deployment steps** - Even automated ones
- **Monitor post-deployment** - First hour is critical
- **Tag releases properly** - Version control is crucial
- **Keep CHANGELOG updated** - Users need to know what changed
- **Verify before announcing** - Test thoroughly

#### ❌ DON'T:

- **Deploy on Fridays** - Weekend support is hard
- **Skip testing** - Even for "small" changes
- **Deploy multiple changes at once** - Hard to diagnose issues
- **Ignore warnings** - They become errors in production
- **Deploy without communication** - Team should know
- **Forget to update docs** - Outdated docs confuse users
- **Rush deployments** - Take time to verify

## Summary

Successful deployments require preparation, automation, and monitoring. Follow a consistent process with pre-deployment validation, careful execution, post-deployment verification, and ready rollback procedures. Libraries use registry publishing, applications use strategies like blue-green or canary deployments.

**Key Takeaways**:
1. **Automate deployment** - Reduce human error
2. **Test thoroughly** - Catch issues before production
3. **Have rollback ready** - Issues happen, be prepared
4. **Monitor everything** - Know when problems occur
5. **Document process** - Consistency is key

---

**Related Documentation**:
- [Release Versioning Guide](release-versioning.md) - Version management and SemVer
- [CI/CD Guide](ci-cd.md) - Continuous integration and deployment pipelines
- [Publishing Guide](publishing.md) - Publishing to package registries
- [Repository Governance](../../4-development/guide/repository-governance.md) - SECURITY.md and CHANGELOG

**External Resources**:
- [Deployment Strategies](https://www.martinfowler.com/bliki/BlueGreenDeployment.html) - Blue-Green deployment
- [Cargo Book - Publishing](https://doc.rust-lang.org/cargo/reference/publishing.html) - Publishing to crates.io
- [Semantic Versioning](https://semver.org/) - Version numbering
- [Keep a Changelog](https://keepachangelog.com/) - CHANGELOG format

**Last Updated**: 2025-12-22  
**Version**: 1.0  
**Next Review**: 2026-03-22
