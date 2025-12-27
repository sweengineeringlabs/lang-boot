# Publishing Guide

**Audience**: Library Maintainers, Release Managers, Package Publishers

## WHAT: Package Registry Publishing

This guide covers publishing libraries and frameworks to package registries (crates.io, npm, PyPI, Maven Central, etc.) including preparation, execution, and post-publication tasks.

**Scope**:
- Package preparation and validation
- Registry-specific publishing procedures
- Metadata and documentation publishing
- Post-publication verification
- Package maintenance and yanking
- Multi-registry publishing strategies

**Out of Scope**:
- Versioning strategy (see [Release Versioning](release-versioning.md))
- CI/CD automation (see [CI/CD Guide](ci-cd.md))
- Application deployment (see [Deployment Workflow](deployment-workflow.md))

## WHY: Problems and Motivation

### Problems Addressed

1. **Manual Publishing Errors**
   - Current impact: Missing files, wrong versions, broken packages
   - Consequence: Unusable releases, frustrated users

2. **Incomplete Package Metadata**
   - Current impact: Poor discoverability, unclear licensing
   - Consequence: Low adoption, legal issues

3. **No Publishing Checklist**
   - Current impact: Inconsistent releases, forgotten steps
   - Consequence: Quality issues, support burden

4. **Multi-Registry Coordination**
   - Current impact: Packages out of sync across registries
   - Consequence: Version confusion, user frustration

### Benefits

- **Discoverability**: Good metadata helps users find your package
- **Trust**: Complete information builds confidence
- **Automation**: Repeatable process reduces errors
- **Quality**: Validation ensures package works
- **Reach**: Multi-registry publishing increases adoption

## HOW: Implementation Guide

### Pre-Publication Checklist

#### Package Metadata

**Cargo.toml** (Rust):
```toml
[package]
name = "rustboot"                   # Unique package name
version = "1.4.0"                   # SemVer version
edition = "2021"                    # Rust edition
authors = ["Your Name <email@example.com>"]
license = "MIT OR Apache-2.0"       # SPDX license identifier
description = "A robust framework for building Rust applications"
documentation = "https://docs.rs/rustboot"
homepage = "https://rustboot.dev"
repository = "https://github.com/org/rustboot"
readme = "README.md"
keywords = ["framework", "web", "async", "server"]  # Max 5
categories = ["web-programming", "asynchronous"]    # From crates.io list

# What files to include
include = [
    "src/**/*",
    "Cargo.toml",
    "README.md",
    "LICENSE-*",
    "CHANGELOG.md",
]

# What to exclude (overrides include)
exclude = [
    "target/",
    "tests/fixtures/",
    ".github/",
]

[dependencies]
# Pin dependencies appropriately
tokio = { version = "1.0", features = ["full"] }
```

**package.json** (JavaScript/TypeScript):
```json
{
  "name": "@org/package-name",
  "version": "1.4.0",
  "description": "Package description",
  "keywords": ["keyword1", "keyword2"],
  "author": "Your Name <email@example.com>",
  "license": "MIT",
  "homepage": "https://github.com/org/repo#readme",
  "repository": {
    "type": "git",
    "url": "git+https://github.com/org/repo.git"
  },
  "bugs": {
    "url": "https://github.com/org/repo/issues"
  },
  "main": "dist/index.js",
  "types": "dist/index.d.ts",
  "files": [
    "dist/",
    "README.md",
    "LICENSE"
  ],
  "engines": {
    "node": ">=16.0.0"
  }
}
```

**setup.py** / **pyproject.toml** (Python):
```toml
[project]
name = "package-name"
version = "1.4.0"
description = "Package description"
readme = "README.md"
requires-python = ">=3.8"
license = {text = "MIT"}
keywords = ["keyword1", "keyword2"]
authors = [
  {name = "Your Name", email = "email@example.com"}
]
classifiers = [
    "Development Status :: 4 - Beta",
    "Intended Audience :: Developers",
    "License :: OSI Approved :: MIT License",
    "Programming Language :: Python :: 3",
    "Programming Language :: Python :: 3.8",
    "Programming Language :: Python :: 3.9",
    "Programming Language :: Python :: 3.10",
]

[project.urls]
Homepage = "https://github.com/org/repo"
Documentation = "https://docs.example.com"
Repository = "https://github.com/org/repo"
```

#### Content Validation

**Checklist**:
- [ ] README.md is clear and complete
- [ ] LICENSE file(s) present and correct
- [ ] CHANGELOG.md updated for this version
- [ ] Examples are included and work
- [ ] Documentation is up to date
- [ ] No sensitive data (keys, credentials)
- [ ] No unnecessary files (tests, CI configs)

**Verify package contents**:
```bash
# Rust
cargo package --list | less
cargo package --allow-dirty  # Creates .crate file

# JavaScript
npm pack --dry-run
npm pack  # Creates .tgz file

# Python
python -m build --sdist
python -m build --wheel
```

#### Documentation Validation

- [ ] **API docs build successfully**
  ```bash
  # Rust
  cargo doc --no-deps --all-features
  
  # JavaScript
  npm run docs
  
  # Python
  sphinx-build -b html docs/ docs/_build/
  ```

- [ ] **Examples compile and run**
  ```bash
  # Rust
  cargo build --examples
  cargo run --example basic
  
  # JavaScript
  node examples/basic.js
  
  # Python
  python examples/basic.py
  ```

- [ ] **Broken links check**
  - Use tools like `markdown-link-check`
  - Verify external URLs accessible

### Publishing to crates.io (Rust)

#### Initial Setup

**One-time setup**:
``bash
# 1. Create crates.io account at https://crates.io

# 2. Generate API token
# Go to https://crates.io/me → Account Settings → API Tokens

# 3. Login with token
cargo login [your-token-here]

# Token stored in ~/.cargo/credentials.toml
```

#### Publishing Steps

**Step 1: Dry Run**
```bash
# Test package creation without publishing
cargo publish --dry-run

# Check warnings
cargo package 2>&1 | grep warning
```

**Step 2: Publish**
```bash
# Publish to crates.io
cargo publish

# For workspace with multiple crates, publish in dependency order
cargo publish -p rustboot-common
cargo publish -p rustboot-core  
cargo publish -p rustboot
```

**Step 3: Verify**
```bash
# Check package appears
cargo search rustboot

# Try installing
cargo install rustboot --version 1.4.0

# Visit crates.io page
# https://crates.io/crates/rustboot
```

#### Workspace Publishing

**For multi-crate workspaces**:

```bash
#!/bin/bash
# publish-all.sh

set -e

# Publish in dependency order
CRATES=(
    "rustboot-common"
    "rustboot-config"
    "rustboot-error"
    "rustboot-core"
    "rustboot-web"
    "rustboot"
)

for crate in "${CRATES[@]}"; do
    echo "Publishing $crate..."
    cargo publish -p "$crate"
    echo "Waiting 30s for crates.io to update..."
    sleep 30
done

echo "All crates published!"
```

### Publishing to npm (JavaScript/TypeScript)

#### Initial Setup

```bash
# 1. Create npm account at https://www.npmjs.com

# 2. Login
npm login

# 3. Verify login
npm whoami

# 4. For scoped packages (@org/package)
# Verify organization membership
```

#### Publishing Steps

**Step 1: Build**
```bash
# Build production bundle
npm run build

# Run tests
npm test

# Check package contents
npm pack --dry-run
```

**Step 2: Publish**
```bash
# Publish to npm
npm publish

# For scoped packages (@org/package)
npm publish --access public  # or --access restricted

# For beta/pre-release versions
npm publish --tag beta
```

**Step 3: Verify**
```bash
# Check on npmjs.com
npm view @org/package-name

# Test installation
npm install @org/package-name@1.4.0
```

### Publishing to PyPI (Python)

#### Initial Setup

```bash
# 1. Create PyPI account at https://pypi.org

# 2. Create API token
# Go to Account Settings → API tokens

# 3. Create ~/.pypirc
cat > ~/.pypirc << 'EOF'
[pypi]
  username = __token__
  password = pypi-YOUR-TOKEN-HERE
EOF

chmod 600 ~/.pypirc
```

#### Publishing Steps

**Step 1: Build**
```bash
# Install build tools
pip install build twine

# Build distribution
python -m build

# Creates:
# dist/package-name-1.4.0.tar.gz (source)
# dist/package_name-1.4.0-py3-none-any.whl (wheel)
```

**Step 2: Check**
```bash
# Validate distribution
twine check dist/*

# Upload to TestPyPI first
twine upload --repository testpypi dist/*

# Test install from TestPyPI
pip install --index-url https://test.pypi.org/simple/ package-name
```

**Step 3: Publish**
```bash
# Upload to PyPI
twine upload dist/*
```

**Step 4: Verify**
```bash
# View on PyPI
pip show package-name

# Test installation
pip install package-name==1.4.0
```

### Publishing to Maven Central (Java)

#### Initial Setup

**One-time setup** (complex):
1. Create Sonatype JIRA account
2. Create ticket to claim group ID
3. Setup GPG signing
4. Configure `settings.xml`

**settings.xml**:
```xml
<settings>
  <servers>
    <server>
      <id>ossrh</id>
      <username>your-jira-username</username>
      <password>your-jira-password</password>
    </server>
  </servers>
  <profiles>
    <profile>
      <id>ossrh</id>
      <activation>
        <activeByDefault>true</activeByDefault>
      </activation>
      <properties>
        <gpg.executable>gpg</gpg.executable>
        <gpg.passphrase>your-gpg-passphrase</gpg.passphrase>
      </properties>
    </profile>
  </profiles>
</settings>
```

#### Publishing Steps

**Using Maven**:
```bash
# Deploy to staging
mvn clean deploy -P release

# Release from staging (via Nexus UI or plugin)
mvn nexus-staging:release
```

**Using Gradle**:
```gradle
// build.gradle
plugins {
    id 'maven-publish'
    id 'signing'
}

publishing {
    publications {
        mavenJava(MavenPublication) {
            from components.java
        }
    }
    repositories {
        maven {
            url "https://s01.oss.sonatype.org/service/local/staging/deploy/maven2/"
            credentials {
                username ossrhUsername
                password ossrhPassword
            }
        }
    }
}

signing {
    sign publishing.publications.mavenJava
}
```

```bash
# Publish
./gradlew publishToSonatype closeAndReleaseSonatypeStagingRepository
```

### Post-Publication Tasks

#### Immediate Verification (Within 5 minutes)

- [ ] **Package appears on registry**
  - Search shows new version
  - Version number is correct
  - Description/metadata correct

- [ ] **Installation test**
  ```bash
  # Create clean test environment
  # Install your package
  # Verify it works
  ```

- [ ] **Documentation check**
  - docs.rs built successfully (Rust)
  - npm docs page looks good (JavaScript)
  - PyPI page renders correctly (Python)

#### Extended Verification (Within 1 hour)

- [ ] **GitHub release created**
  - Tag matches published version
  - Release notes from CHANGELOG
  - Binaries attached if applicable

- [ ] **Documentation site updated**
  - Latest version docs available
  - Version switcher works
  - Examples updated

- [ ] **Communication**
  - Announce on project blog/social media
  - Post in relevant communities
  - Update status page/badges

#### Monitoring

**Track**:
- Download counts
- GitHub issues/bug reports
- Community feedback (Discord, forums)
- Registry health (uptime, API status)

### Package Maintenance

#### Updating Published Package

**Minor fixes (PATCH)**:
1. Fix the issue
2. Bump patch version
3. Update CHANGELOG
4. Publish new version

**You CANNOT**:
- Republish same version number
- Modify published package contents
- Unpublish without good reason

#### Yanking/Deprecating Versions

**Rust (crates.io)**:
```bash
# Yank broken version
cargo yank --vers 1.4.0

# Unyank if mistake
cargo yank --vers 1.4.0 --undo
```

**JavaScript (npm)**:
```bash
# Deprecate version
npm deprecate package-name@1.4.0 "Critical bug, use 1.4.1 instead"

# Unpublish (only within 72 hours)
npm unpublish package-name@1.4.0
```

**Python (PyPI)**:
```bash
# Cannot unpublish
# Must yank or publish corrected version
# Contact PyPI support for true emergencies
```

#### When to Yank

**Yank if**:
- Critical security vulnerability
- Data corruption bug
- Installation fails for most users
- Major functionality broken

**Don't yank if**:
- Minor bugs (publish patch instead)
- Performance issues (publish improvement)
- You just want to clean up old versions

### Multi-Registry Publishing

#### Coordinated Releases

**For packages published to multiple registries**:

```bash
#!/bin/bash
# Release to all registries

VERSION="1.4.0"

# 1. Test all packages
cargo test --all
npm test
python -m pytest

# 2. Build all packages
cargo build --release
npm run build
python -m build

# 3. Publish in order
echo "Publishing to crates.io..."
cargo publish

echo "Waiting for crates.io to update..."
sleep 60

echo "Publishing to npm..."
npm publish

echo "Publishing to PyPI..."
twine upload dist/*

echo "All registries published successfully!"
```

#### Version Consistency

**Keep versions synchronized**:
- Same version number across all registries
- Publish to all registries on same day
- Update all changelogs together
- Coordinate GitHub release

### Registry-Specific Best Practices

#### crates.io

**✅ DO**:
- Use broad dependency version ranges (`1.0` not `=1.0.0`)
- Keep dependencies minimal
- Include all necessary files
- Use feature flags appropriately

**❌ DON'T**:
- Publish with `[patch]` or `[replace]` dependencies
- Include huge files (>10MB total)
- Spam with frequent updates
- Yank without good reason

#### npm

**✅ DO**:
- Use scoped package names (`@org/package`)
- Set access level explicitly
- Include type definitions
- Test on multiple Node versions

**❌ DON'T**:
- Include `node_modules/`
- Publish without `.npmignore` or `files` field
- Use loose version ranges for dependencies
- Publish broken packages (test first!)

#### PyPI

**✅ DO**:
- Build both source and wheel distributions
- Include classifiers for discoverability
- Test on TestPyPI first
- Document Python version support

**❌ DON'T**:
- Publish untested wheels
- Include unnecessary files
- Use deprecated `setup.py` (use pyproject.toml)
- Skip version history in CHANGELOG

### Troubleshooting

#### Issue: "Package name already taken"

**Solutions**:
- Choose different name
- Request ownership transfer (if abandoned)
- Use scoped/namespaced name (`@org/name`)
- Add suffix (`name-rs`, `node-name`)

#### Issue: "Package too large"

**Registry limits**:
- crates.io: 10 MB uncompressed
- npm: 10 MB (can request increase)
- PyPI: 100 MB per file

**Solutions**:
```bash
# Find large files
cargo package --list | xargs ls -lh | sort -k5 -h

# Exclude unnecessary files
# Add to Cargo.toml:
exclude = [
    "tests/fixtures/*.bin",
    "docs/images/*.png",
]

# For npm, use .npmignore
# For Python, use MANIFEST.in
```

#### Issue: "Version conflict during publish"

**Symptoms**: "version 1.4.0 already exists"

**Solutions**:
- Verify you bumped version number
- Check if someone else published
- Ensure git tag matches package version

### Best Practices

#### ✅ DO:

- **Test before publishing** - Always dry run
- **Complete metadata** - Help users find your package
- **Include documentation** - README, examples, API docs
- **Follow SemVer strictly** - Users depend on it
- **Publish release notes** - Communicate changes
- **Keep packages lean** - Only necessary files
- **Automate publishing** - Reduce human error
- **Monitor post-publication** - Catch issues early

#### ❌ DON'T:

- **Publish without testing** - Even small changes
- **Skip CHANGELOG** - Users need to know what changed
- **Include build artifacts** - Source only
- **Publish with TODOs** - Complete the work first
- **Rush publication** - Take time to verify
- **Ignore warnings** - Fix before publishing
- **Publish and disappear** - Monitor for issues
- **Yank frivolously** - Only for serious issues

## Summary

Publishing packages to registries requires careful preparation, validation, and verification. Complete all metadata, test thoroughly, and follow registry-specific guidelines. Coordinate multi-registry releases and monitor post-publication for issues.

**Key Takeaways**:
1. **Prepare thoroughly** - Metadata, docs, tests complete
2. **Validate before publishing** - Dry run and verify
3. **Follow registry rules** - Each has specific requirements
4. **Monitor after publishing** - Catch issues early
5. **Maintain published packages** - Yank only when necessary

---

**Related Documentation**:
- [Release Versioning Guide](release-versioning.md) - Version management and SemVer
- [Deployment Workflow](deployment-workflow.md) - Overall deployment process
- [CI/CD Guide](ci-cd.md) - Automated publishing pipelines
- [Repository Governance](../../4-development/guide/repository-governance.md) - CHANGELOG and metadata

**External Resources**:
- [Cargo Book - Publishing](https://doc.rust-lang.org/cargo/reference/publishing.html) - crates.io publishing
- [npm Docs - Publishing](https://docs.npmjs.com/packages-and-modules/contributing-packages-to-the-registry) - npm registry
- [PyPI Packaging Tutorial](https://packaging.python.org/en/latest/tutorials/packaging-projects/) - Python packaging
- [Maven Central Guide](https://central.sonatype.org/publish/publish-guide/) - Maven Central publishing

**Last Updated**: 2025-12-22  
**Version**: 1.0  
**Next Review**: 2026-03-22
