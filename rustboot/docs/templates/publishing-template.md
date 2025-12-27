# Publishing Guide

**Audience**: Library Maintainers, Release Managers, Package Publishers

## WHAT: Package Registry Publishing

This guide covers publishing [PROJECT_NAME] to package registries including preparation, execution, and post-publication tasks.

**Scope**:
- Package preparation and validation
- Registry-specific publishing procedures
- Metadata and documentation publishing
- Post-publication verification
- Package maintenance and yanking
- Multi-registry publishing strategies

**Out of Scope**:
- Versioning strategy (see Release Versioning Guide)
- CI/CD automation (see CI/CD Guide)
- Application deployment (see Deployment Workflow)

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

### Publishing Workflow Overview

#### Complete Publishing Flow

```
┌─────────────────┐
│  Code Ready     │
│  for Release    │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Pre-Publication│
│    Checklist    │ ──→ Metadata Validation
└────────┬────────┘ ──→ Content Verification
         │          ──→ Documentation Check
         │
         ├─── FAIL → Fix Issues
         │
         ▼ PASS
┌─────────────────┐
│   DRY RUN       │
│   Publishing    │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Review Package │
│    Contents     │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│    Publish      │
│  to Registry    │
└────────┬────────┘
         │
         ├──────────────────┬──────────────────┐
         │                  │                  │
         ▼                  ▼                  ▼
   [Package on        [Documentation]    [GitHub
    Registry]          Updates]           Release]
         │                  │                  │
         └──────────────────┴──────────────────┘
                            │
                            ▼
                   ┌─────────────────┐
                   │  Post-Publish   │
                   │  Verification   │
                   └────────┬────────┘
                            │
              ┌─────────────┴─────────────┐
              │                           │
              ▼ SUCCESS                   ▼ FAILURE
      ┌──────────────┐            ┌──────────────┐
      │   Monitor    │            │  Yank/Fix    │
      │  & Announce  │            │   Package    │
      └──────────────┘            └──────────────┘
```

#### Multi-Registry Publishing Flow

```
┌────────────────┐
│  Version Tag   │
│   Created      │
└───────┬────────┘
        │
        ▼
┌────────────────┐
│  Build All     │
│  Packages      │
└───────┬────────┘
        │
        ├──────────────┬──────────────┬──────────────┐
        │              │              │              │
        ▼              ▼              ▼              ▼
  [Language 1]   [Language 2]   [Language 3]   [Platform]
  Package        Package        Package        Specific
        │              │              │              │
        ▼              ▼              ▼              ▼
  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐
  │crates.io │  │   npm    │  │   PyPI   │  │  Maven   │
  │ Publish  │  │ Publish  │  │ Publish  │  │ Central  │
  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘
       │             │             │             │
       │   Wait &    │   Wait &    │   Wait &    │
       │  Verify     │  Verify     │  Verify     │
       │             │             │             │
       └─────────────┴─────────────┴─────────────┘
                             │
                             ▼
                    ┌──────────────┐
                    │  All Publish │
                    │   Complete   │
                    └──────┬───────┘
                           │
                           ▼
                  ┌─────────────────┐
                  │  Cross-Verify   │
                  │  All Registries │
                  └─────────────────┘
```

#### Package Preparation Dataflow

```
┌─────────────┐
│Source Files │
└──────┬──────┘
       │
       ▼
┌──────────────────┐
│  Filter Files    │──→ Include List
│  (Include/       │    ├─ src/**
│   Exclude)       │    ├─ README
│                  │    └─ LICENSE
└────────┬─────────┘
         │           Exclude List
         │            ├─ tests/**
         │            ├─ .github/**
         │            └─ target/**
         ▼
┌──────────────────┐
│  Add Metadata    │
│                  │
│ ├─ Name          │
│ ├─ Version       │
│ ├─ License       │
│ ├─ Authors       │
│ ├─ Description   │
│ └─ Dependencies  │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│  Build Package   │──→ Package Artifact
│                  │    ├─ Tarball/Archive
└────────┬─────────┘    ├─ Checksums
         │              └─ Signatures
         │
         ▼
┌──────────────────┐
│  Validate        │──→ Validation Report
│  Package         │    ├─ Size check
│                  │    ├─ File list
└────────┬─────────┘    ├─ Metadata valid
         │              └─ No secrets
         │
         ▼
┌──────────────────┐
│  Ready for       │
│  Publishing      │
└──────────────────┘
```

#### Publishing Decision Tree

```
              Ready to Publish?
                     │
                     ▼
        ┌────────────────────────┐
        │  Package Built?        │
        └─────┬─────────────┬────┘
              │ No          │ Yes
              ▼             │
        Build Package       │
              │             │
              └─────────────┘
                     │
                     ▼
        ┌────────────────────────┐
        │  Metadata Complete?    │
        └─────┬─────────────┬────┘
              │ No          │ Yes
              ▼             │
      Fill Metadata         │
                           ▼
        ┌────────────────────────┐
        │  Documentation OK?     │
        └─────┬─────────────┬────┘
              │ No          │ Yes
              ▼             │
     Update Documentation   │
                           ▼
        ┌────────────────────────┐
        │  Tests Pass?           │
        └─────┬─────────────┬────┘
              │ No          │ Yes
              ▼             │
          Fix Tests         │
                           ▼
        ┌────────────────────────┐
        │  Security Clean?       │
        └─────┬─────────────┬────┘
              │ No          │ Yes
              ▼             │
       Patch Security       │
                           ▼
              ┌────────────────────────┐
              │  Publish to Registry   │
              └──────────┬─────────────┘
                        │
                        ▼
              ┌────────────────────────┐
              │  Publishing Success?   │
              └─────┬─────────────┬────┘
                    │ No          │ Yes
                    ▼             │
              Troubleshoot        │
              & Retry             │
                                  ▼
                            ┌────────────┐
                            │  Verify    │
                            │ & Monitor  │
                            └────────────┘
```

### Pre-Publication Checklist

#### Package Metadata

**[Package Manifest File]**:
```[config-language]
[Package name] = "[package-name]"
[Version field] = "[X.Y.Z]"
[Authors field] = ["Your Name <email@example.com>"]
[License field] = "[LICENSE-ID]"
[Description] = "Package description"
[Documentation URL] = "https://docs.[project].dev"
[Homepage] = "https://[project].dev"
[Repository] = "https://github.com/org/repo"
[README] = "README.md"
[Keywords] = ["keyword1", "keyword2", ...]
[Categories] = ["category1", "category2"]

# What files to include
[Include field] = [
    "src/**/*",
    "[manifest-file]",
    "README.md",
    "LICENSE",
    "CHANGELOG.md",
]

# What to exclude
[Exclude field] = [
    "target/",
    "tests/fixtures/",
    ".github/",
]
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
# [Your language/tool]
[package list command]
[package creation command]
```

#### Documentation Validation

- [ ] **API docs build successfully**
  ```bash
  [documentation build command]
  ```

- [ ] **Examples compile and run**
  ```bash
  [example build command]
  [example run command]
  ```

- [ ] **Broken links check**
  - Use tools like `markdown-link-check`
  - Verify external URLs accessible

### Publishing to [Primary Registry]

#### Initial Setup

**One-time setup**:
```bash
# 1. Create account at [registry-url]

# 2. Generate API token
# [Steps to generate token]

# 3. Login with token
[login command]

# Token stored in [credential-location]
```

#### Publishing Steps

**Step 1: Dry Run**
```bash
# Test package creation without publishing
[dry-run command]

# Check warnings
[warning check command]
```

**Step 2: Publish**
```bash
# Publish to [registry-name]
[publish command]

# [Additional steps if needed]
```

**Step 3: Verify**
```bash
# Check package appears
[search command]

# Try installing
[install command]

# Visit registry page
# [registry-url]/packages/[package-name]
```

#### [Special Case: Workspace/Monorepo Publishing]

**For multi-package projects**:

```bash
#!/bin/bash
# publish-all.sh

set -e

# Publish in dependency order
PACKAGES=(
    "[package-1]"
    "[package-2]"
    "[package-3]"
)

for package in "${PACKAGES[@]}"; do
    echo "Publishing $package..."
    [publish command for package]
    echo "Waiting [N]s for registry to update..."
    sleep [N]
done

echo "All packages published!"
```

### Publishing to [Secondary Registry] (If Applicable)

#### Initial Setup

```bash
# 1. Create account at [registry-url]

# 2. [Authentication steps]

# 3. [Configuration steps]
```

#### Publishing Steps

**Step 1: Build**
```bash
# Build production bundle
[build command]

# Run tests
[test command]

# Check package contents
[package check command]
```

**Step 2: Publish**
```bash
# Publish to [registry-name]
[publish command]

# [Additional parameters if needed]
```

**Step 3: Verify**
```bash
# Check on [registry-name]
[verification command]

# Test installation
[install command]
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
  [create environment command]
  
  # Install your package
  [install command]
  
  # Verify it works
  [verification command]
  ```

- [ ] **Documentation check**
  - [Documentation site] built successfully
  - Package page renders correctly
  - Links work correctly

#### Extended Verification (Within 1 hour)

- [ ] **Release created**
  - Git tag matches published version
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
- Issues/bug reports  
- Community feedback
- Registry health

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
- Unpublish without good reason (most registries)

#### Yanking/Deprecating Versions

**[Registry Name]**:
```bash
# Yank broken version
[yank command]

# Unyank if mistake
[unyank command]

# Deprecate version
[deprecate command]
```

**Registry Policies**:
- [Describe yanking/unpublishing policies]
- [Describe deprecation options]
- [Describe emergency procedures]

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

VERSION="[X.Y.Z]"

# 1. Test all packages
[test command 1]
[test command 2]

# 2. Build all packages
[build command 1]
[build command 2]

# 3. Publish in order
echo "Publishing to [registry-1]..."
[publish command 1]

echo "Waiting for [registry-1] to update..."
sleep [N]

echo "Publishing to [registry-2]..."
[publish command 2]

echo "All registries published successfully!"
```

#### Version Consistency

**Keep versions synchronized**:
- Same version number across all registries
- Publish to all registries on same day
- Update all changelogs together
- Coordinate release announcements

### Registry-Specific Best Practices

#### [Registry 1]

**✅ DO**:
- [Best practice 1]
- [Best practice 2]
- [Best practice 3]

**❌ DON'T**:
- [Anti-pattern 1]
- [Anti-pattern 2]
- [Anti-pattern 3]

#### [Registry 2] (If Applicable)

**✅ DO**
- [Best practice 1]
- [Best practice 2]  
- [Best practice 3]

**❌ DON'T**:
- [Anti-pattern 1]
- [Anti-pattern 2]
- [Anti-pattern 3]

### Troubleshooting

#### Issue: "Package name already taken"

**Solutions**:
- Choose different name
- Request ownership transfer (if abandoned)
- Use scoped/namespaced name
- Add suffix or prefix

#### Issue: "Package too large"

**Registry limits**:
- [Registry]: [size limit]

**Solutions**:
```bash
# Find large files
[find large files command]

# Exclude unnecessary files
[exclusion configuration]
```

#### Issue: "Version conflict during publish"

**Symptoms**: "version [X.Y.Z] already exists"

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
- [Release Versioning Guide] - Version management and SemVer
- [Deployment Workflow] - Overall deployment process
- [CI/CD Guide] - Automated publishing pipelines
- [Repository Governance] - CHANGELOG and metadata

**External Resources**:
- [Add registry-specific documentation links]
- [Add packaging guides]
- [Add best practices resources]

**Last Updated**: YYYY-MM-DD  
**Version**: 1.0  
**Next Review**: YYYY-MM-DD

---

## Template Customization Guide

When using this template for your project:

1. **Replace all placeholders**:
   - `[PROJECT_NAME]` → Your project name
   - `[Package Manifest File]` → Cargo.toml, package.json, setup.py, pom.xml, etc.
   - `[config-language]` → TOML, JSON, Python, XML, etc.
   - `[package-name]` → Your package name
   - `[X.Y.Z]` → Version number format
   - `[LICENSE-ID]` → SPDX license identifier (MIT, Apache-2.0, etc.)
   - `[registry-url]` → Registry website URL
   - `YYYY-MM-DD` → Actual dates

2. **Identify target registries**:
   - crates.io (Rust)
   - npm (JavaScript/TypeScript)
   - PyPI (Python)
   - Maven Central (Java)
   - RubyGems (Ruby)
   - NuGet (.NET)
   - Document all registries you publish to

3. **Document manifest format**:
   - Package name and version fields
   - Author/maintainer information
   - License specification
   - Dependencies format
   - Include/exclude patterns

4. **Add publishing commands**:
   - Dry run command
   - Actual publish command
   - Verification commands
   - Registry search commands
   - Installation test commands

5. **Configure authentication**:
   - How to create registry account
   - How to generate API tokens
   - Where tokens are stored
   - How to configure credentials

6. **Define package contents**:
   - What files should be included
   - What should be excluded
   - How to verify package contents
   - Size limits to be aware of

7. **Add monorepo handling** (if applicable):
   - Publishing order for interdependent packages
   - Wait times between publishes
   - Verification between packages
   - Rollback strategy

8. **Document yanking/deprecation**:
   - Registry-specific policies
   - Commands to yank/unyank
   - When yanking is appropriate
   - Alternative approaches

9. **Add troubleshooting**:
   - Common errors your team encounters
   - Registry-specific issues
   - Network/timeout problems
   - Version conflicts

10. **Review and validate**:
    - Test publish process with practice package
    - Verify all commands work
    - Check registry documentation for updates
    - Keep current with registry changes
