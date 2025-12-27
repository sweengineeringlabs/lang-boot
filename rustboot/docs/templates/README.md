# SEA Documentation Templates

Language-agnostic templates for documentation in Stratified Encapsulation Architecture (SEA) projects.

## Quick Links

- **[FRAMEWORK.md](FRAMEWORK.md)** - Complete documentation framework guide (start here!)
- [crate-overview-template.md](crate-overview-template.md) - Module/component template
- [framework-doc-template.md](framework-doc-template.md) - Framework documentation template
- [release-versioning-template.md](release-versioning-template.md) - Release versioning guide template
- [deployment-workflow-template.md](deployment-workflow-template.md) - Deployment workflow template
- [ci-cd-template.md](ci-cd-template.md) - CI/CD pipeline template
- [publishing-template.md](publishing-template.md) - Publishing guide template
- **[git-files/](git-files/)** - Git repository file templates (open-source and internal)

## Overview

These templates provide a consistent WHAT-WHY-HOW structure for documenting SEA-based systems, regardless of implementation language (Rust, Java, C++, Python, etc.).

## Git Repository Files (Required for All Projects)

> **🔒 Critical**: Before creating any other documentation, set up Git repository files.

**Complete Guide**: [Repository Governance Best Practices](../4-development/guide/repository-governance.md)

### Quick Start

1. **Determine project type**: Open-source or internal/proprietary
2. **Copy templates**: From `git-files/{open-source|internal}/`
3. **Customize**: Replace placeholders with your project details
4. **Validate**: Use Phase 0 checklist from FRAMEWORK.md

### Required Files by Type

**Open-Source**: CODE_OF_CONDUCT.md, SECURITY.md, SUPPORT.md, CONTRIBUTING.md, LICENSE, issue/PR templates

**Internal**: SECURITY.md, SUPPORT.md, CONTRIBUTING.md, INTERNAL_USAGE.md, issue templates

**See**: [Repository Governance Best Practices](../4-development/guide/repository-governance.md) for:
- Detailed file descriptions and templates
- Best practices for each file type
- Decision criteria (open-source vs internal)
- Implementation workflow
- Examples and troubleshooting



## Available Templates

### 1. [crate-overview-template.md](crate-overview-template.md)

**Purpose**: Module/component-level documentation

**Use for**:
- Individual modules in SEA layers (Common, SPI, API, Core, Facade)
- Library packages
- Service components
- Standalone utilities

**Format**:
- WHAT-WHY-HOW structure
- No Audience section (implicit: developers)
- Code examples/pseudocode
- Inter-module relationships

**Naming conventions**:
- Rust: `crates/[name]/docs/overview.md`
- Java: `modules/[name]/docs/overview.md`
- Python: `packages/[name]/docs/overview.md`
- Other: `[module]/docs/overview.md`

### 2. [framework-doc-template.md](framework-doc-template.md)

**Purpose**: Framework-wide, cross-cutting documentation

**Use for**:
- Architecture Decision Records (ADRs)
- Security guidelines
- Design patterns
- Development guides
- Audit documentation

**Format**:
- **Audience declaration** (required!)
- WHAT-WHY-HOW structure
- Best practices and anti-patterns
- Decision matrices
- Related documentation links

**Common audiences**:
- Developers
- Architects
- Security Auditors
- Compliance Officers
- Leadership/Management
- Operations Teams

### 3. [release-versioning-template.md](release-versioning-template.md)

**Purpose**: Release versioning and version management guide

**Use for**:
- Defining semantic versioning policies
- Release workflow documentation
- Version bump decision rules
- CHANGELOG management standards
- Git tagging strategies

**Format**:
- **Audience declaration** (required!)
- WHAT-WHY-HOW structure
- Language-agnostic with customization placeholders
- Includes customization guide at end

**Key features**:
- SemVer decision matrix
- Breaking change policies
- Release checklist templates
- Pre-release version handling
- CHANGELOG format standards

**Target location**: `docs/6-deployment/guide/release-versioning.md` (or equivalent)

###4. [deployment-workflow-template.md](deployment-workflow-template.md)

**Purpose**: Deployment process and strategies guide

**Use for**:
- Defining deployment environments
- Deployment strategy documentation
- Pre-deployment validation checklists
- Rollback procedures
- Environment-specific configuration

**Format**:
- **Audience declaration** (required!)
- WHAT-WHY-HOW structure
- Language-agnostic with customization placeholders
- Includes customization guide at end

**Key features**:
- Deployment strategy options
- Environment management
- Pre/post-deployment checklists
- Rollback procedures
- Monitoring and alerting

**Target location**: `docs/6-deployment/guide/deployment-workflow.md` (or equivalent)

### 5. [ci-cd-template.md](ci-cd-template.md)

**Purpose**: CI/CD pipeline implementation guide

**Use for**:
- Continuous integration setup
- Automated testing configuration
- Quality gates definition
- Continuous deployment automation
- Multi-platform testing strategy

**Format**:
- **Audience declaration** (required!)
- WHAT-WHY-HOW structure
- Platform-agnostic (GitHub Actions, GitLab CI, Jenkins, etc.)
- Includes customization guide at end

**Key features**:
- CI/CD platform configurations
- Quality gate definitions
- Security scanning setup
- Pipeline optimization strategies
- Branch protection rules

**Target location**: `docs/6-deployment/guide/ci-cd.md` (or equivalent)

### 6. [publishing-template.md](publishing-template.md)

**Purpose**: Package registry publishing guide

**Use for**:
- Package preparation workflows
- Registry-specific publishing procedures
- Metadata and documentation requirements
- Post-publication verification
- Multi-registry coordination

**Format**:
- **Audience declaration** (required!)
- WHAT-WHY-HOW structure
- Multi-registry support (crates.io, npm, PyPI, Maven, etc.)
- Includes customization guide at end

**Key features**:
- Package metadata templates
- Publishing checklists
- Registry-specific procedures
- Yanking/deprecation policies
- Multi-registry strategies

**Target location**: `docs/6-deployment/guide/publishing.md` (or equivalent)

## Documentation Structure (SEA Framework)

```
project/
├── [implementation-layer]/     # e.g., src/, lib/, modules/
│   └── [module-name]/
│       └── docs/
│           └── overview.md     # Use module template
└── docs/
    ├── 0-ideation/
    ├── 1-requirements/
    ├── 2-architecture/
    ├── 3-design/
    │   ├── adr/               # Architecture Decision Records
    │   └── *.md               # Use framework template
    ├── 4-development/
    │   └── guides/
    │       └── *.md           # Use framework template
    ├── 5-testing/
    ├── 6-deployment/
    └── templates/             # This directory
```

## WHAT-WHY-HOW Philosophy

### WHAT: Clear Description
- Define what is being documented
- Scope and boundaries
- Key capabilities or concepts

### WHY: Problem and Motivation
- Problems being solved
- Impact if not addressed  
- Benefits of the solution

### HOW: Implementation and Application
- Practical examples
- Usage patterns
- Best practices and anti-patterns

## Documentation Rules

| Location | Format | Audience Section | Language |
|----------|--------|------------------|----------|
| Module/Component docs | WHAT-WHY-HOW | ❌ Not included | Implementation-specific or pseudocode |
| Framework docs | Audience + WHAT-WHY-HOW | ✅ Required | Language-agnostic |

### Why Different?

- **Module documentation**: Technical, audience implicit (developers working in that module)
- **Framework documentation**: Multiple audiences with different needs (devs, architects, auditors, management)

## Quick Start

### Creating Module Documentation

1. Copy `crate-overview-template.md`
2. Save to `[module]/docs/overview.md`
3. Fill in WHAT-WHY-HOW sections
4. Use language-appropriate examples
5. **Do NOT add Audience** (not needed)

### Creating Framework Documentation

1. Copy `framework-doc-template.md`
2. Save to appropriate `docs/` subdirectory
3. **Define Audience** (required!)
4. Fill in WHAT-WHY-HOW sections
5. Use language-agnostic examples/pseudocode
6. Add related documentation links

## Examples Across Languages

### Module Documentation (Any Language)

#### ✅ Good (Rust)
```markdown
# Authentication Module Overview

## WHAT: User Identity Verification
[No Audience section]

```rust
let token = generate_jwt(user_id)?;
```
```

#### ✅ Good (Java)
```markdown
# Authentication Module Overview

## WHAT: User Identity Verification
[No Audience section]

```java
String token = AuthService.generateJWT(userId);
```
```

#### ✅ Good (Python)
```markdown
# Authentication Module Overview

## WHAT: User Identity Verification
[No Audience section]

```python
token = auth_service.generate_jwt(user_id)
```
```

### Framework Documentation (Language-Agnostic)

```markdown
# Security Best Practices

**Audience**: Application Developers, Security Auditors

## WHAT: Framework Security Guidelines

// Pseudocode or language-agnostic examples
authenticator.validate(credentials)
```

## SEA Layer Documentation

When documenting SEA layers, clarify the layer purpose:

- **Common**: Shared utilities, no dependencies on other layers
- **SPI**: Service Provider Interface, extension points
- **API**: Internal contracts, interfaces
- **Core**: Business logic implementations
- **Facade**: Public-facing API

## Contribution Guidelines

When contributing documentation to SEA research:

1. **Use these templates** for consistency
2. **Keep language-agnostic** where possible
3. **Follow WHAT-WHY-HOW** structure
4. **Specify Audience** for framework docs
5. **Include version and update dates**
6. **Link related documentation**

## Adaptation for Your Language

### Language-Specific Adjustments

- **Module naming**: Adjust paths (crates/ vs modules/ vs packages/)
- **Code examples**: Use your language's conventions
- **Build tools**: Reference appropriate tools (Cargo, Maven, pip, etc.)
- **Structure**: Keep WHAT-WHY-HOW, adapt examples

### Preserving SEA Principles

- Maintain layer separation in documentation
- Document dependencies clearly
- Explain architectural decisions (ADRs)
- Keep interface documentation in API layer
- Keep implementation details in Core layer

---

**Maintained by**: SEA Research Team  
**Contributions**: Welcome via pull requests  
**Questions**: Open an issue in the SEA framework repository

**License**: [Specify license - e.g., MIT, Apache 2.0]
