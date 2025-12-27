# Library/Framework Documentation Framework

A layered, audience-focused documentation structure for libraries and frameworks.

**Based on**: Rustboot implementation using SEA (Stratified Encapsulation Architecture)  
**Format**: WHAT-WHY-HOW structure

## Overview

This framework provides a hierarchical documentation structure that:
- Separates concerns by audience (architects, developers, users)
- Uses hub documents for navigation
- Maintains consistency with templates
- Scales from small libraries to large frameworks

## Directory Structure

```
project/
├── README.md                           # Lean entry point
├── docs/
│   ├── overview.md                     # Main documentation hub
│   ├── 0-ideation/                     # (Optional) Research & ideas
│   ├── 1-requirements/                 # (Optional) Requirements docs
│   ├── 2-architecture/                 # (Optional) High-level arch
│   ├── 3-design/
│   │   ├── architecture.md             # Design hub document
│   │   ├── [feature]-overview.md       # Feature architecture docs
│   │   ├── [feature]-audit.md          # Audit/compliance docs
│   │   └── adr/                        # Architecture Decision Records
│   │       ├── README.md               # ADR index
│   │       └── NNN-[decision].md       # Individual ADRs
│   ├── 4-development/
│   │   ├── developer-guide.md          # Development hub document
│   │   └── guide/
│   │       ├── [topic]-guide.md        # Development guides
│   │       └── [technology]-[topic].md # Technology-specific guides
│   ├── 5-testing/                      # (Optional) Test strategy
│   ├── 6-deployment/                   # (Optional) Deployment docs
│   ├── backlog.md                      # Feature backlog index
│   ├── framework-backlog.md            # Cross-cutting backlog
│   └── templates/
│       ├── README.md                   # Template usage guide
│       ├── crate-overview-template.md  # Module/component template
│       └── framework-doc-template.md   # Framework doc template
└── modules/                            # or crates/, packages/, lib/
    └── [module-name]/
        ├── Cargo.toml / package.json / setup.py
        ├── src/
        ├── doc/
        │   └── overview.md             # Module overview (WHAT-WHY-HOW)
        └── backlog.md                  # Module-specific backlog
```

## File Naming Conventions

> **📝 Important**: Follow consistent naming conventions for all documentation files.

### UPPERCASE (Git Standard Files)

These files use UPPERCASE as they are recognized by GitHub/GitLab and follow community conventions:

```
README.md
LICENSE
CONTRIBUTING.md
CODE_OF_CONDUCT.md
SECURITY.md
SUPPORT.md
CHANGELOG.md
INTERNAL_USAGE.md
.github/ISSUE_TEMPLATE/
.github/PULL_REQUEST_TEMPLATE.md
```

### lowercase-with-hyphens (Project Documentation)

All other documentation files use lowercase with hyphens:

```
docs/
├── overview.md
├── 3-design/
│   ├── architecture.md
│   ├── authentication.md
│   ├── rate-limiting.md
│   └── security-audit-report.md
├── 4-development/
│   ├── developer-guide.md
│   └── guide/
│       ├── cli-usage.md
│       ├── quick-start.md
│       └── getting-started.md
└── backlog.md
```

### Naming Rules

| Category | Convention | Examples |
|----------|------------|----------|
| Git standard files | UPPERCASE | `README.md`, `LICENSE`, `CONTRIBUTING.md` |
| GitHub templates | UPPERCASE | `ISSUE_TEMPLATE/`, `PULL_REQUEST_TEMPLATE.md` |
| Project docs | lowercase-hyphen | `architecture.md`, `quick-start.md` |
| Directories | lowercase-hyphen | `3-design/`, `4-development/` |

## Git Repository Files (Required)

> **🔒 Critical**: Every library/framework MUST include appropriate Git repository files based on its open-source status.

**See**: [Repository Governance Best Practices](../4-development/guide/repository-governance.md) for comprehensive guide.

### Quick Reference

**For Open-Source Projects** - Required:
- `CODE_OF_CONDUCT.md` - Community guidelines (UPPERCASE)
- `SECURITY.md` - Vulnerability reporting (UPPERCASE)
- `SUPPORT.md` - Getting help (UPPERCASE)
- `CONTRIBUTING.md` - Contribution process (UPPERCASE)
- `LICENSE` - Open-source license (UPPERCASE)
- `.github/ISSUE_TEMPLATE/` - Bug/feature templates (UPPERCASE)
- `.github/PULL_REQUEST_TEMPLATE.md` - PR template (UPPERCASE)

**For Internal/Proprietary Projects** - Required:
- `SECURITY.md` - Internal security policy (UPPERCASE)
- `SUPPORT.md` - Internal support channels (UPPERCASE)
- `CONTRIBUTING.md` - Internal contribution process (UPPERCASE)
- `INTERNAL_USAGE.md` - Approved use cases (UPPERCASE)
- `.github/ISSUE_TEMPLATE/` - Internal issue templates (UPPERCASE)

### Template Locations

Templates available in: `docs/templates/git-files/{open-source|internal}/`

**Full details**: See [Repository Governance Best Practices](../4-development/guide/repository-governance.md)

## Navigation Flow

```
README.md (Quick Start)
    ↓
docs/overview.md (Main Hub)
    ├→ 3-design/architecture.md (Design Hub)
    │   ├→ Feature architecture docs
    │   ├→ Security/compliance docs
    │   └→ ADRs
    │
    ├→ 4-development/developer-guide.md (Dev Hub)
    │   └→ Development guides
    │
    └→ Module overviews (modules/*/doc/overview.md)
```

## Document Types & Templates

### 1. README.md (Entry Point)

**Purpose**: Quick Start & navigation  
**Audience**: Everyone  
**Content**:
- Project tagline (1 sentence)
- Key features (bullet points)
- Quick Start code example
- Link to `docs/overview.md`
- Installation instructions
- License

**Example**:
```markdown
# ProjectName

**Tagline** - Brief description

## Features
- Feature 1
- Feature 2

## Quick Start
\`\`\`language
// Code example
\`\`\`

## Documentation
See [docs/overview.md](docs/overview.md) for complete documentation.

## License
MIT
```

### 2. docs/overview.md (Main Hub)

**Purpose**: Central navigation to all documentation  
**Audience**: All  
**Content**:
- Quick navigation section
- Links to design hub
- Links to developer hub
- Complete module/crate list (organized by priority)
- Backlog links
- Template links

**Example**: See `docs/overview.md` in Rustboot

### 3. docs/3-design/architecture.md (Design Hub)

**Audience**: Architects, Technical Leadership, Security Teams  
**Format**: Audience + WHAT-WHY-HOW  
**Content**:
- Architecture overview
- Security documentation
- ADRs index
- Design guides
- Link to module overviews
- Link to developer guide

**Example**: See `docs/3-design/architecture.md` in Rustboot

### 4. docs/4-development/developer-guide.md (Development Hub)

**Audience**: Developers, Contributors  
**Format**: Audience + WHAT-WHY-HOW  
**Content**:
- Development guides organized by topic
- Testing guides
- Build/tooling guides
- Contributing guidelines
- Link to module overviews
- Link to architecture docs

**Example**: See `docs/4-development/developer-guide.md` in Rustboot

### 5. Module Overview (modules/*/doc/overview.md)

**Audience**: Developers (implicit)  
**Format**: WHAT-WHY-HOW (NO Audience section)  
**Content**:
- WHAT: Clear description of the module
- WHY: Problems solved, when to use
- HOW: Usage examples, API guide
- Relationship to other modules
- Status & backlog link

**Example**: See `crates/rustboot-security/doc/overview.md`

### 6. Framework Documentation (docs/*/\*.md)

**Audience**: Various (MUST specify)  
**Format**: Audience + WHAT-WHY-HOW  
**Content**:
- Audience declaration (required!)
- WHAT: What is covered
- WHY: Problems/motivation
- HOW: Implementation/application
- Best practices
- Related docs

**Example**: See `docs/4-development/guide/rust-test-organization.md`

## Examples and Tests (Critical!)

> **🎯 Every module MUST have examples and tests**

### Why Examples and Tests Matter

1. **Examples** (`examples/` directory):
   - Show users HOW to use your code
   - Provide copy-paste starting points
   - Demonstrate best practices
   - Compile-checked documentation

2. **Integration Tests** (`tests/` directory):
   - Show users HOW to test their code
   - Verify public API works
   - Serve as additional usage examples
   - Catch breaking changes

3. **Navigation**: Documentation must link to code
   - Module overviews → examples + tests
   - Examples show usage patterns
   - Tests show testing patterns

### Requirements Checklist

Every module/component must have:

- [ ] **At minimum**: `examples/basic.rs` - Simple usage
- [ ] **At minimum**: `tests/integration.rs` - Public API tests
- [ ] **In doc/overview.md**: "Examples and Tests" section with links
- [ ] **Links to**: Testing guides (Rust Test Organization, etc.)

### Example Structure

```
module/
├── examples/
│   ├── basic.rs           # Always required
│   ├── advanced.rs        # For complex features
│   └── [feature].rs       # One per major feature
├── tests/
│   ├── integration.rs     # Always required
│   └── [feature]_test.rs  # Additional test files as needed
└── doc/
    └── overview.md        # Must link to above
```

## Documentation Rules

| Location | Format | Audience | Examples |
|----------|--------|----------|----------|
| `README.md` | Quick Start | Everyone | Project entry |
| `docs/overview.md` | Hub + Links | All | Main index |
| `docs/3-design/architecture.md` | Hub + Links | Architects | Design index |
| `docs/4-development/developer-guide.md` | Hub + Links | Developers | Dev index |
| `docs/3-design/*.md` | Audience + WHAT-WHY-HOW | Specified | Architecture docs |
| `docs/4-development/guide/*.md` | Audience + WHAT-WHY-HOW | Specified | Dev guides |
| `modules/*/doc/overview.md` | WHAT-WHY-HOW only | Developers (implicit) | Module docs |

### Key Principles

1. **Audience Required for Framework Docs** - Multiple audiences need clarity
2. **No Audience for Module Docs** - Technical docs, audience is obvious
3. **WHAT-WHY-HOW Structure** - Consistent across all docs
4. **Hub Documents** - Navigate to specialized docs
5. **No TLDR/TOC** - Only if doc is very long (200+ lines)

## Implementation Flow

The documentation framework follows a **six-phase sequential implementation**:

```
Phase 0: Git Repository Files
    ↓ (MUST complete before Phase 1)
Phase 1: Foundation (README, docs/overview.md, templates)
    ↓
Phase 2: Design Documentation (architecture.md, ADRs)
    ↓
Phase 3: Development Documentation (developer-guide.md, guides)
    ↓
Phase 4: Module Documentation (module overviews, examples, tests)
    ↓
Phase 5: Backlog & Planning (backlog files)
    ↓
Phase 6: Validation (check all phases complete)
```

**Critical Path**:
- **Phase 0 is mandatory first** - Repository governance files must exist before other documentation
- **Phase 1 creates structure** - Foundation for all other documentation
- **Phases 2-5 can overlap** - Design and development docs can be created in parallel
- **Phase 6 validates everything** - Final check before project release

**Why this order?**:
1. **Phase 0 first**: Establishes project governance, licensing, and contribution process
2. **Foundation next**: Creates navigation structure for all documentation
3. **Design & Development**: Fill in the structure with content
4. **Modules**: Document individual components
5. **Backlog**: Plan future work
6. **Validation**: Ensure quality and completeness

## Implementation Checklist

### Phase 0: Git Repository Files (MUST DO FIRST)
- [ ] **Determine project type**: Open-source or internal/non-open-source
- [ ] **For Open-Source projects**:
  - [ ] Create `CODE_OF_CONDUCT.md` (required)
  - [ ] Create `SECURITY.md` (required)
  - [ ] Create `SUPPORT.md` (required)
  - [ ] Create `CONTRIBUTING.md` (required)
  - [ ] Add/verify `LICENSE` file (required)
  - [ ] Create `.github/ISSUE_TEMPLATE/bug_report.md` (required)
  - [ ] Create `.github/ISSUE_TEMPLATE/feature_request.md` (required)
  - [ ] Create `.github/PULL_REQUEST_TEMPLATE.md` (required)
  - [ ] Create `CHANGELOG.md` (recommended)
- [ ] **For Internal/Non-Open-Source projects**:
  - [ ] Create `SECURITY.md` for internal security policy (required)
  - [ ] Create `SUPPORT.md` for internal support channels (required)
  - [ ] Create `CONTRIBUTING.md` for internal contribution process (required)
  - [ ] Create `INTERNAL_USAGE.md` for approved use cases (required)
  - [ ] Create `.github/ISSUE_TEMPLATE/internal_issue.md` (required)
  - [ ] Create `OWNERS.md` for code ownership (recommended)
  - [ ] Create `COMPLIANCE.md` if applicable (recommended)

### Phase 1: Foundation
- [ ] Create lean README.md with Quick Start
- [ ] Create docs/overview.md as main hub
- [ ] Create docs/templates/ with both templates
- [ ] Set up directory structure (0-6 folders)

### Phase 2: Design Documentation
- [ ] Create docs/3-design/architecture.md hub
- [ ] Add architecture/design documents
- [ ] Create docs/3-design/adr/ for decisions
- [ ] Add security/compliance docs (if applicable)

### Phase 3: Development Documentation
- [ ] Create docs/4-development/developer-guide.md hub
- [ ] Add development guides in docs/4-development/guide/
- [ ] Add testing guides
- [ ] Add technology-specific guides

### Phase 4: Module Documentation
- [ ] Create doc/overview.md for each module
- [ ] Follow WHAT-WHY-HOW structure
- [ ] Add relationship tables
- [ ] Link from docs/overview.md
- [ ] **Create examples/basic.rs for each module**
- [ ] **Create tests/integration.rs for each module**
- [ ] **Add "Examples and Tests" section to each overview**
- [ ] **Link to testing guides**

### Phase 5: Backlog & Planning
- [ ] Create module-level backlog.md files
- [ ] Create docs/backlog.md index
- [ ] Create docs/framework-backlog.md for cross-cutting work

### Phase 6: Validation
- [ ] **Verify Phase 0 complete**: All Git repository files present
- [ ] **Check project type alignment**: Files match open-source or internal status
- [ ] **Validate required files**: SECURITY.md, SUPPORT.md, CONTRIBUTING.md, issue templates
- [ ] **For open-source**: Verify CODE_OF_CONDUCT.md and LICENSE exist
- [ ] **For internal**: Verify INTERNAL_USAGE.md exists
- [ ] **Verify file naming conventions**:
  - [ ] Git standard files are UPPERCASE (README.md, LICENSE, CONTRIBUTING.md, etc.)
  - [ ] Project docs are lowercase-with-hyphens (architecture.md, quick-start.md)
  - [ ] Directories are lowercase-with-hyphens (3-design/, 4-development/)
- [ ] Verify no broken links
- [ ] Check all Audience declarations in framework docs
- [ ] Verify no Audience in module docs
- [ ] Ensure WHAT-WHY-HOW in all docs
- [ ] Remove unnecessary TLDR/TOC sections
- [ ] **Verify all modules have examples/basic.rs**
- [ ] **Verify all modules have tests/integration.rs**
- [ ] **Check all overviews link to examples + tests**

## Technology-Specific Adaptations

### Rust Projects
- Use `crates/` instead of `modules/`
- Reference Cargo.toml
- Use Rust code examples
- Include testing guides (cargo test)

### JavaScript/TypeScript Projects
- Use `packages/` instead of `modules/`
- Reference package.json
- Use JS/TS code examples
- Include npm/yarn guides

### Python Projects
- Use `packages/` instead of `modules/`
- Reference setup.py / pyproject.toml
- Use Python code examples
- Include pip/poetry guides

### Java Projects
- Use `modules/` as-is
- Reference pom.xml / build.gradle
- Use Java code examples
- Include Maven/Gradle guides

## Example Implementations

### Small Library (5-10 modules)
- Minimal structure: README, docs/overview.md, module docs
- Single architecture.md
- Single developer-guide.md
- Templates in docs/templates/

### Medium Framework (10-20 modules)
- Full structure with design + dev hubs
- Security documentation
- ADRs for major decisions
- Technology-specific guides

### Large Framework (20+ modules)
- Complete structure with all 0-6 folders
- Multiple design documents
- Comprehensive ADRs
- Extensive development guides
- Compliance documentation

## Tools & Automation

### Link Checking
```bash
# Find broken links (example with grep)
find docs -name "*.md" -exec grep -H "\[.*\](.*)" {} \;
```

### Template Validation
```bash
# Check for Audience in framework docs
grep -r "**Audience**" docs/3-design docs/4-development

# Verify no Audience in module docs
! grep -r "**Audience**" modules/*/doc/
```

### WHAT-WHY-HOW Validation
```bash
# Check structure in all docs
for file in $(find . -name "overview.md"); do
  echo "Checking $file"
  grep -q "## WHAT" $file && echo "  ✓ WHAT"
  grep -q "## WHY" $file && echo "  ✓ WHY"
  grep -q "## HOW" $file && echo "  ✓ HOW"
done
```

## Best Practices

### ✅ DO
- Keep README lean (< 100 lines)
- Use hub documents for navigation
- Specify Audience for framework docs
- Maintain WHAT-WHY-HOW structure
- Link related docs
- Update docs/overview.md when adding modules
- Use templates for consistency

### ❌ DON'T
- Put all docs in README
- Add Audience to module docs
- Create TLDR/TOC for short docs
- Have broken links
- Duplicate content across docs
- Skip WHY sections
- Ignore templates

## Maintenance

### Regular Reviews
- **Monthly**: Check for broken links
- **Per Release**: Update version numbers
- **Per Module**: Create overview.md
- **Per Decision**: Create ADR

### Documentation Debt
Track in `docs/framework-backlog.md`:
- Missing module docs
- Outdated examples
- Broken links
- Unclear sections

## Migration Guide

### From Unstructured Docs

1. **Audit current docs**: List all existing documentation
2. **Create structure**: Set up directory hierarchy
3. **Categorize**: Sort docs into design vs development
4. **Create hubs**: Write architecture.md and developer-guide.md
5. **Convert format**: Apply WHAT-WHY-HOW to each doc
6. **Add Audience**: Mark framework docs with target readers
7. **Link everything**: Update docs/overview.md
8. **Clean README**: Simplify to Quick Start only

### From Other Formats

**From README-heavy**:
- Extract architecture → docs/3-design/
- Extract guides → docs/4-development/
- Keep only Quick Start in README

**From Wiki-style**:
- Organize by audience
- Create hub documents
- Apply WHAT-WHY-HOW structure

## Success Metrics

- ✅ **Git repository files present** (Phase 0 complete)
- ✅ **Appropriate files for project type** (open-source or internal)
- ✅ **CODE_OF_CONDUCT.md exists** (open-source only)
- ✅ **SECURITY.md exists** (all projects)
- ✅ **SUPPORT.md exists** (all projects)
- ✅ **CONTRIBUTING.md exists** (all projects)
- ✅ **Issue templates exist** (all projects)
- ✅ **File naming conventions followed**:
  - Git standard files UPPERCASE
  - Project docs lowercase-with-hyphens
- ✅ All modules have doc/overview.md
- ✅ No broken links
- ✅ README < 100 lines
- ✅ Framework docs have Audience
- ✅ Module docs don't have Audience
- ✅ All docs use WHAT-WHY-HOW
- ✅ Hub documents exist at each level
- ✅ New contributors can navigate easily
- ✅ **All modules have examples/basic.rs**
- ✅ **All modules have tests/integration.rs**
- ✅ **All overviews link to examples and tests**
- ✅ **Documentation guides users to working code**

---

**Based on**: Rustboot framework implementation  
**License**: MIT (adapt freely)  
**Contributions**: Welcome improvements and adaptations
