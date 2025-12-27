# Release Versioning Guide

**Audience**: Developers, Maintainers, Release Managers

## WHAT: Release Versioning and Management

This guide defines versioning standards, release processes, and version management best practices for [PROJECT_NAME].

**Scope**:
- Semantic Versioning (SemVer) principles and application
- Version bump decision making
- Release workflow and checklist
- Git tagging and branch strategies
- CHANGELOG management
- Breaking change policies
- Pre-release version management

**Out of Scope**:
- Deployment and distribution (see deployment docs)
- CI/CD pipeline configuration (see CI/CD guides)
- Marketing and release announcements

## WHY: Problems and Motivation

### Problems Addressed

1. **Unpredictable Version Numbers**
   - Current impact: Users don't know if upgrade will break their code
   - Consequence: Fear of upgrading, stuck on old versions, security vulnerabilities

2. **Inconsistent Versioning Across Modules**
   - Current impact: Different modules use different versioning schemes
   - Consequence: Dependency hell, confusion, compatibility issues

3. **Unclear Breaking Changes**
   - Current impact: Breaking changes not communicated clearly in version numbers
   - Consequence: Production incidents, user frustration, lost trust

4. **Poor Release Documentation**
   - Current impact: Users don't know what changed or why to upgrade
   - Consequence: Low adoption of new versions, duplicated bug reports

### Benefits

- **Predictability**: Users know what to expect from version numbers
- **Trust**: Clear communication builds confidence in upgrades
- **Compatibility**: Easier dependency management and resolution
- **Clarity**: CHANGELOG and version numbers tell the complete story
- **Automation**: Consistent versioning enables automated tooling

## HOW: Implementation Guide

### Semantic Versioning (SemVer)

**Format**: `MAJOR.MINOR.PATCH` (e.g., `2.4.1`)

**Official Spec**: https://semver.org/

#### Version Components

Given a version number `MAJOR.MINOR.PATCH`, increment:

1. **MAJOR** (e.g., 1.0.0 → 2.0.0)
   - Breaking changes to public API
   - Incompatible API changes
   - Removal of deprecated features
   - Major architectural changes

2. **MINOR** (e.g., 1.4.0 → 1.5.0)
   - New features (backwards-compatible)
   - New functionality added
   - Deprecation warnings (but not removals)
   - Performance improvements

3. **PATCH** (e.g., 1.4.1 → 1.4.2)
   - Bug fixes (backwards-compatible)
   - Security patches
   - Documentation updates
   - Internal refactoring (no API changes)

#### Pre-1.0.0 Versions

**Before 1.0.0**: Initial development phase

- `0.y.z` - Anything may change at any time
- Public API should not be considered stable
- `0.1.0` - Initial development release
- `0.y.0` - Breaking changes acceptable in MINOR
- `0.y.z` - Bug fixes and minor features in PATCH

**When to go 1.0.0**:
- Public API is stable
- Production users depend on it
- Ready to commit to SemVer promises

### Version Bump Decision Matrix

Use this matrix to determine which version component to increment:

| Change Type | Examples | Version Bump |
|-------------|----------|--------------|
| **Breaking Changes** | Remove public API, change function signature, rename module | **MAJOR** |
| **API Changes** | Remove parameter, change return type, change behavior | **MAJOR** |
| **New Features** | Add new function, new module, new optional parameter | **MINOR** |
| **Enhancements** | Performance improvements, new config options | **MINOR** |
| **Deprecations** | Mark old API as deprecated (but still works) | **MINOR** |
| **Bug Fixes** | Fix incorrect behavior, patch security issue | **PATCH** |
| **Documentation** | Update docs, fix typos, add examples | **PATCH** |
| **Internal Changes** | Refactor code, update dependencies (no API impact) | **PATCH** |

### Breaking Changes Policy

#### What Constitutes a Breaking Change?

**For [Language] Libraries/Frameworks**:
- ✅ [Language-specific breaking change example 1]
- ✅ [Language-specific breaking change example 2]
- ✅ Changing function/method signatures
- ✅ Renaming public items
- ✅ Removing or renaming modules/packages
- ✅ Changing dependency requirements (major version bumps)
- ❌ Adding new public items (minor)
- ❌ Fixing bugs (even if code depended on buggy behavior - patch)
- ❌ Internal implementation changes (patch)

**For Applications**:
- Configuration file format changes
- CLI argument changes
- Database schema changes
- [Add application-specific breaking changes]

#### Handling Breaking Changes

**Option 1: Deprecation Path** (Recommended)
```
Version 2.3.0: Add new API, deprecate old API (MINOR)
Version 2.4.0: Continue supporting both (MINOR)
Version 3.0.0: Remove old API (MAJOR)
```

**Option 2: Direct Removal**
```
Version 3.0.0: Remove old API immediately (MAJOR)
```

**Best Practice**:
- Provide at least one MINOR version with deprecation warnings
- Document migration path in CHANGELOG
- Provide automated migration tools when possible

### Pre-Release Versions

**Format**: `MAJOR.MINOR.PATCH-PRERELEASE` (e.g., `2.0.0-alpha.1`)

#### Pre-Release Identifiers

Standard progression:

```
1.0.0-alpha.1    → Early testing, unstable
1.0.0-alpha.2    → More alpha releases
1.0.0-beta.1     → Feature complete, stabilizing
1.0.0-beta.2     → More beta releases
1.0.0-rc.1       → Release candidate, final testing
1.0.0-rc.2       → More release candidates
1.0.0            → Stable release
```

**When to use**:
- **alpha**: Unstable, missing features, API may change
- **beta**: Feature complete, API mostly stable, testing needed
- **rc** (release candidate): No new features, only critical fixes

**[Language]-Specific**:
```[language]
[Add language-specific version syntax]
// Example for package manifest
version = "2.0.0-beta.1"
```

### Git Tagging Strategy

#### Tag Format

**Standard tags**:
```
v1.0.0
v1.2.3
v2.0.0-beta.1
```

**Prefix with 'v'**: Common convention, makes tags clearly version tags

#### Creating Tags

**Annotated tags** (recommended):
```bash
git tag -a v1.4.0 -m "Release version 1.4.0

- [Summary of changes]
- [Major features]
- [Important fixes]"

git push origin v1.4.0
```

**Why annotated?**
- Contains tagger information and date
- Can include release notes
- More complete metadata

### CHANGELOG Management

**Format**: Keep a Changelog (https://keepachangelog.com/)

#### CHANGELOG Structure

```markdown
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- [New feature description]

### Changed
- [Change description]

### Deprecated
- [Deprecated feature] - use [new feature] instead

### Removed
(none)

### Fixed
- [Bug fix description] (#[issue-number])

### Security
(none)

## [1.4.0] - YYYY-MM-DD

### Added
- [Feature 1]
- [Feature 2]

### Fixed
- [Bug fix] (#[issue-number])

## [1.3.1] - YYYY-MM-DD

### Fixed
- [Critical security fix] (CVE-YYYY-XXXXX)

[Unreleased]: https://github.com/[org]/[repo]/compare/v1.4.0...HEAD
[1.4.0]: https://github.com/[org]/[repo]/compare/v1.3.1...v1.4.0
[1.3.1]: https://github.com/[org]/[repo]/compare/v1.3.0...v1.3.1
```

#### CHANGELOG Categories

Use these standard categories:

- **Added**: New features
- **Changed**: Changes in existing functionality
- **Deprecated**: Soon-to-be removed features
- **Removed**: Removed features
- **Fixed**: Bug fixes
- **Security**: Security vulnerability fixes

#### CHANGELOG Best Practices

✅ **DO**:
- **Update with every PR** - Keep Unreleased section current
- **Group by category** - Added, Changed, Fixed, etc.
- **Write for users** - Not internal jargon
- **Link issues** - Reference issue/PR numbers
- **Date releases** - Use ISO format (YYYY-MM-DD)
- **Link versions** - GitHub compare links

❌ **DON'T**:
- Duplicate commit messages
- Include internal changes users don't care about
- Use vague descriptions ("Various improvements")
- Forget breaking changes
- Skip security fixes

### Release Workflow

#### Complete Release Checklist

**Pre-Release** (1-2 weeks before):
- [ ] Review and prioritize outstanding issues
- [ ] Ensure all planned features are merged
- [ ] Create release branch (e.g., `release/1.4.0`) [if using release branches]
- [ ] Update version numbers in all relevant files
- [ ] Update CHANGELOG from Unreleased to version
- [ ] Review and update documentation
- [ ] Run full test suite
- [ ] Perform security audit if major release
- [ ] Test upgrade path from previous version
- [ ] [Add project-specific pre-release tasks]

**Release Day**:
- [ ] Final code review
- [ ] Merge release branch to main [if using release branches]
- [ ] Create annotated Git tag
- [ ] Build release artifacts
- [ ] Publish to package registry ([registry-name])
- [ ] Push Git tag
- [ ] Create GitHub/GitLab release with notes
- [ ] Update documentation site
- [ ] Announce release (blog, social media, mailing list)
- [ ] [Add project-specific release tasks]

**Post-Release**:
- [ ] Monitor for issues
- [ ] Respond to user feedback
- [ ] Plan next release
- [ ] Create new Unreleased section in CHANGELOG
- [ ] Close release milestone
- [ ] [Add project-specific post-release tasks]

### [Language]-Specific Versioning

[Customize this section for your language/ecosystem]

#### Version Management

**Option 1: Synchronized Versions** (Recommended for frameworks)

All modules/packages share the same version:

```[language]
[Add language-specific configuration]
```

**Option 2: Independent Versions**

Each module/package has its own version:

```[language]
[Add language-specific configuration]
```

**When to use which**:
- **Synchronized**: Framework modules released together, tight coupling
- **Independent**: Loosely coupled utilities, different maturity levels

#### Publishing to [Registry]

```bash
# Verification
[Add verification commands]

# Publishing
[Add publishing commands]
```

### Branch Strategy for Releases

#### Approach 1: Trunk-Based Development (Recommended)

```
main (trunk)
  ├─ feature-x (PR → main)
  ├─ feature-y (PR → main)
  └─ v1.4.0 tag on main
```

**Flow**:
1. All development on `main` branch
2. Use feature flags for incomplete features
3. Tag releases directly on `main`
4. Hotfixes on `main` or short-lived branches

**Benefits**: Simple, fast, encourages small changes

#### Approach 2: Release Branches

```
main (develop)
  ├─ release/1.4.0 → v1.4.0 tag
  ├─ release/1.5.0 → v1.5.0 tag
  └─ hotfix/1.4.1 → v1.4.1 tag
```

**Flow**:
1. Development on `main`
2. Create `release/X.Y.0` branch when ready
3. Only bug fixes on release branch
4. Tag release on release branch
5. Merge release branch back to main

**Benefits**: Stabilization period, supports multiple versions

**[PROJECT_NAME] uses**: [Specify which approach your project uses]

### Version Automation Tools

[Customize this section with tools relevant to your language/ecosystem]

#### Recommended Tools

**[Tool 1]**:
```bash
[Installation command]
[Usage examples]
```

**[Tool 2]**:
```bash
[Installation command]
[Usage examples]
```

### Common Versioning Patterns

#### Pattern 1: Regular Releases

```
v1.0.0 → v1.1.0 → v1.2.0 → v2.0.0
  ↓        ↓        ↓        ↓
Every    Every    Every    When ready
[X] wks  [X] wks  [X] wks  for breaking
```

**[PROJECT_NAME] release cadence**: [Specify your release schedule]

#### Pattern 2: Security Patches

```
v1.4.0 (current stable)
  │
  ├─ v1.4.1 (security patch)
  └─ v1.4.2 (another security patch)

v1.5.0 (next minor)
```

#### Pattern 3: LTS (Long-Term Support) [If applicable]

```
v2.4.0 (LTS - supported until YYYY-MM-DD)
  ├─ v2.4.1
  ├─ v2.4.2
  └─ v2.4.3

v3.0.0 (Current)
v3.1.0 (Current)

v4.0.0 (Next LTS)
```

**[PROJECT_NAME] LTS policy**: [Specify if you have LTS versions and support timeline]

### Best Practices

#### ✅ DO:

- **Be consistent** - Same versioning scheme everywhere
- **Communicate early** - Announce breaking changes in advance
- **Test thoroughly** - Especially before MAJOR bumps
- **Document everything** - CHANGELOG is user-facing documentation
- **Automate when possible** - Reduce human error
- **Follow SemVer strictly** - Users depend on it
- **Provide migration guides** - For MAJOR versions
- **Keep CHANGELOG current** - Update with every PR

#### ❌ DON'T:

- **Skip versions** - Don't go from 1.2.0 to 1.4.0
- **Reuse version numbers** - Never unpublish and republish same version
- **Hide breaking changes** - In MINOR or PATCH versions
- **Use marketing versions** - 2.0 because it "sounds better"
- **Forget pre-releases** - Test major changes as beta first
- **Change released versions** - Published versions are immutable
- **Ignore security versions** - Always patch security issues

### Troubleshooting

#### "When should I bump to 2.0.0?"

**Bump to MAJOR (2.0.0) when**:
- Removing deprecated APIs that existed in 1.x
- Changing core behavior users depend on
- Updating dependencies with breaking changes
- Architectural shifts requiring code changes

**Don't bump to MAJOR just because**:
- It's been a long time
- You want to signal "maturity"
- You have lots of new features (use MINOR)

#### "I accidentally published wrong version"

**If published to registry**:
- **Cannot unpublish** (most registries)
- Immediately publish corrected version
- Yank broken version if registry supports it
- Document in CHANGELOG

**If only tagged in Git**:
```bash
# Delete tag locally
git tag -d v1.4.0

# Delete tag remotely
git push origin :refs/tags/v1.4.0

# Create correct tag
git tag -a v1.4.1 -m "Correct version"
git push origin v1.4.1
```

#### "Breaking change in PATCH version?"

**If you accidentally shipped breaking change in PATCH**:
1. Acknowledge the mistake
2. Immediately publish new MAJOR version with proper number
3. Yank the incorrect PATCH version if possible
4. Document in CHANGELOG what happened
5. Apologize to users and provide migration guide

## Summary

Semantic Versioning provides predictability and trust through consistent version numbering. Use MAJOR for breaking changes, MINOR for features, and PATCH for fixes. Maintain a detailed CHANGELOG, create annotated Git tags, and follow a consistent release workflow.

**Key Takeaways**:
1. **Follow SemVer strictly** - MAJOR.MINOR.PATCH with clear rules
2. **CHANGELOG is essential** - Keep it current, write for users
3. **Automate when possible** - Reduce errors, increase consistency
4. **Communicate breaking changes** - Deprecation warnings, migration guides
5. **Test before releasing** - Especially MAJOR versions

---

**Related Documentation**:
- [Repository Governance Best Practices](../4-development/guide/repository-governance.md) - CHANGELOG.md requirements
- [Developer Guide](../4-development/developer-guide.md) - Development workflows
- [Link to contributing guide] - Contribution process

**External Resources**:
- [Semantic Versioning 2.0.0](https://semver.org/) - Official SemVer specification
- [Keep a Changelog](https://keepachangelog.com/) - CHANGELOG format standard
- [Add language-specific resources]

**Last Updated**: YYYY-MM-DD  
**Version**: 1.0  
**Next Review**: YYYY-MM-DD

---

## Template Customization Guide

When using this template for your project:

1. **Replace all placeholders**:
   - `[PROJECT_NAME]` → Your project name
   - `[Language]` → Your programming language (Rust, Python, Java, etc.)
   - `[Registry]` → Package registry (crates.io, npm, PyPI, Maven Central, etc.)
   - `[org]`/`[repo]` → Your GitHub organization and repository
   - `YYYY-MM-DD` → Actual dates
   - `[X]` → Specific numbers/timeframes

2. **Customize language-specific sections**:
   - Breaking changes specific to your language
   - Version syntax for your package manager
   - Publishing commands for your registry
   - Automation tools for your ecosystem

3. **Define your policies**:
   - Release cadence (weekly, monthly, etc.)
   - LTS support if applicable
   - Branch strategy (trunk-based or release branches)
   - Review and approval requirements

4. **Add project-specific items**:
   - Additional checklist items
   - Custom automation scripts
   - Internal processes
   - Team-specific requirements

5. **Update related documentation links**:
   - Ensure all links point to your actual documentation files
   - Add links to CI/CD, deployment, and other relevant guides

6. **Review and validate**:
   - Test the workflow with a practice release
   - Get team buy-in on the process
   - Update as your project evolves
