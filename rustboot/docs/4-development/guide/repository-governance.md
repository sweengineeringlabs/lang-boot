# Repository Governance Best Practices

**Audience**: Project Maintainers, Technical Leaders, Open Source Contributors

## WHAT: Repository Governance Guidelines

This document defines best practices for repository governance, including required files, templates, and policies for both open-source and internal/proprietary projects.

**Scope**:
- Repository file requirements (CODE_OF_CONDUCT, SECURITY, etc.)
- Open-source vs internal project distinctions
- Community management files
- Issue and PR templates
- License and compliance requirements

**Out of Scope**:
- Code style guides (see language-specific guides)
- CI/CD configuration (see deployment docs)
- Branch protection rules (repository-specific)

## WHY: Problems and Motivation

### Problems Addressed

1. **Inconsistent Repository Structure**
   - Current impact: Different projects have different governance files, making it hard for contributors to understand processes
   - Consequence: Reduced contributions, confusion about how to report issues or contribute

2. **Missing Critical Files**
   - Current impact: Projects lack SECURITY.md, CODE_OF_CONDUCT.md, or proper issue templates
   - Consequence: Security vulnerabilities unreported, community issues unmanaged, poor contributor experience

3. **Unclear Project Type**
   - Current impact: Ambiguity about whether project is open-source or internal-only
   - Consequence: Licensing issues, compliance problems, inappropriate sharing

4. **Poor Contributor Experience**
   - Current impact: No clear guidance on how to contribute, report bugs, or get support
   - Consequence: Lost contributions, duplicate issues, support overload

### Benefits

- **Standardization**: Consistent structure across all repositories
- **Compliance**: Meet legal and organizational requirements
- **Community Health**: Clear governance fosters healthy communities
- **Risk Mitigation**: Proper security and support channels reduce risk
- **Efficiency**: Templates reduce repetitive work

## HOW: Implementation Guide

### Project Type Classification

**First Decision**: Determine if your project is open-source or internal/proprietary.

#### Open-Source Projects

**Characteristics**:
- Public repository (GitHub, GitLab, Bitbucket public)
- Accepting external contributions from community
- Using OSI-approved license (MIT, Apache 2.0, GPL, etc.)
- Building community around the project
- Public issue tracker and discussions

**Examples**:
- Framework libraries (React, Vue, Rustboot)
- Tools and utilities (ripgrep, exa)
- Educational projects
- Research prototypes intended for publication

#### Internal/Proprietary Projects

**Characteristics**:
- Private/internal repository only
- Limited to organization members or specific teams
- Contains proprietary code or trade secrets
- Subject to NDA or company IP policy
- Restricted distribution

**Examples**:
- Internal company frameworks
- Proprietary business logic libraries
- Client-specific implementations
- Pre-release commercial software

### Required Files by Project Type

#### For Open-Source Projects

##### 1. CODE_OF_CONDUCT.md (Required)

**Purpose**: Define acceptable behavior and community standards

**When to use**: All open-source projects accepting contributions

**Template location**: `docs/templates/git-files/open-source/CODE_OF_CONDUCT.md`

**Key sections**:
- Our Pledge
- Our Standards (expected behavior)
- Enforcement Responsibilities
- Scope
- Enforcement Guidelines
- Attribution

**Best practices**:
- Use Contributor Covenant 2.1 as standard
- Clearly state enforcement process
- Provide contact information for reporting
- Be specific about unacceptable behavior

##### 2. SECURITY.md (Required)

**Purpose**: Security policy and vulnerability reporting process

**When to use**: All projects (open-source and internal)

**Template location**: `docs/templates/git-files/open-source/SECURITY.md`

**Key sections**:
- Supported Versions
- Reporting a Vulnerability
- Security Update Process
- Security Best Practices

**Best practices**:
- Provide private reporting channel (security@project.org)
- Set response time expectations
- Define disclosure timeline
- List supported/vulnerable versions

##### 3. SUPPORT.md (Required)

**Purpose**: How users get help and support

**Template location**: `docs/templates/git-files/open-source/SUPPORT.md`

**Key sections**:
- Getting Help
- Resources (docs, examples, guides)
- Community Channels (Discord, Slack, Forums)
- Issue Tracker Guidelines
- Commercial Support (if available)

**Best practices**:
- Distinguish questions from bug reports
- Point to documentation and examples first
- List community channels (Discord, Discussions, Stack Overflow)
- Set expectations for response times

##### 4. CONTRIBUTING.md (Required)

**Purpose**: Contribution guidelines and process

**Template location**: `docs/templates/git-files/open-source/CONTRIBUTING.md`

**Key sections**:
- How to Contribute
- Code of Conduct reference
- Development Setup
- Pull Request Process
- Coding Standards
- Testing Requirements
- Commit Message Format

**Best practices**:
- Link to developer guides
- Specify PR title/description format
- Require tests for new features
- Define review process and timeline
- Thank contributors!

##### 5. LICENSE (Required)

**Purpose**: Legal license for code usage and distribution

**Common licenses**:
- **MIT**: Permissive, simple, allows commercial use
- **Apache 2.0**: Permissive, includes patent grant
- **GPL v3**: Copyleft, requires derivative works to be open-source
- **BSD 3-Clause**: Permissive, includes attribution requirement

**Best practices**:
- Choose license early, don't change later
- Include license in every source file header (optional but recommended)
- Use SPDX identifiers: `// SPDX-License-Identifier: MIT`
- Ensure all dependencies are compatible

##### 6. Issue Templates (Required)

**Location**: `.github/ISSUE_TEMPLATE/`

**Required templates**:
- `bug_report.md` - Bug reports with reproduction steps
- `feature_request.md` - Feature proposals
- `question.md` - Questions and support requests (optional)

**Template location**: `docs/templates/git-files/open-source/`

**Best practices**:
- Use GitHub issue forms (YAML) for structured input
- Require environment details for bugs
- Include "Have you searched existing issues?" checkbox
- Provide clear examples in template

##### 7. Pull Request Template (Required)

**Location**: `.github/PULL_REQUEST_TEMPLATE.md`

**Key sections**:
- Description of changes
- Related issues (Fixes #123)
- Type of change (bugfix, feature, breaking change)
- Checklist (tests added, docs updated, etc.)

**Best practices**:
- Require issue linking
- Checklist for tests and documentation
- Request before/after behavior description
- Include breaking change warning section

##### 8. CHANGELOG.md (Recommended)

**Purpose**: Version history and release notes

**Format**: Keep a Changelog (https://keepachangelog.com/)

**Structure**:
```markdown
## [Unreleased]
### Added
### Changed
### Deprecated
### Removed
### Fixed
### Security

## [1.0.0] - 2024-01-15
...
```

**Best practices**:
- Update with every PR
- Keep Unreleased section at top
- Use semantic versioning
- Group by change type (Added, Fixed, etc.)

##### 9. AUTHORS.md / CONTRIBUTORS.md (Recommended)

**Purpose**: Recognize contributors

**Best practices**:
- Auto-generate from git history
- Include all types of contributions (code, docs, design)
- Use all-contributors specification
- Update with releases

#### For Internal/Proprietary Projects

##### 1. SECURITY.md (Required)

**Purpose**: Internal security policy and incident reporting

**Template location**: `docs/templates/git-files/internal/SECURITY.md`

**Differences from open-source**:
- Reference internal security team/process
- Include incident response procedures
- List internal security tools/scanners
- Reference company security policy

**Key sections**:
- Internal Security Contact
- Incident Reporting Process
- Security Review Requirements
- Compliance Requirements
- Restricted Access Information

##### 2. SUPPORT.md (Required)

**Purpose**: Internal support channels and escalation

**Template location**: `docs/templates/git-files/internal/SUPPORT.md`

**Key sections**:
- Internal Support Channels (Slack, Teams, Email)
- Escalation Path
- SLA Expectations
- On-Call Rotation (if applicable)
- Documentation Resources

**Best practices**:
- Link to internal wiki/Confluence
- Provide team contact information
- Define severity levels
- Include escalation matrix

##### 3. CONTRIBUTING.md (Required)

**Purpose**: Internal contribution process and code review

**Template location**: `docs/templates/git-files/internal/CONTRIBUTING.md`

**Key sections**:
- Internal Development Workflow
- Code Review Process
- Required Approvals
- Branch Naming Conventions
- CI/CD Requirements
- Release Process

**Best practices**:
- Reference internal coding standards
- Link to internal developer portal
- Specify required reviewers (CODEOWNERS)
- Define merge requirements

##### 4. INTERNAL_USAGE.md (Required)

**Purpose**: Approved use cases, restrictions, and compliance

**Template location**: `docs/templates/git-files/internal/INTERNAL_USAGE.md`

**Key sections**:
- Approved Use Cases
- Restricted/Prohibited Uses
- Data Classification
- Compliance Requirements (GDPR, SOC2, etc.)
- IP and Licensing Restrictions
- Distribution Rules

**Best practices**:
- Be explicit about what's NOT allowed
- Reference company policies
- Include data handling requirements
- List geographical restrictions if any

##### 5. Issue Template (Required)

**Location**: `.github/ISSUE_TEMPLATE/internal_issue.md`

**Template location**: `docs/templates/git-files/internal/internal_issue.md`

**Differences from open-source**:
- Include internal project/team references
- Add priority/severity fields
- Reference internal ticketing system
- Include affected systems/customers

##### 6. OWNERS.md / CODEOWNERS (Recommended)

**Purpose**: Code ownership and review requirements

**Location**: `OWNERS.md` (documentation) or `.github/CODEOWNERS` (GitHub enforcement)

**Format** (CODEOWNERS):
```
# Default owners
* @team/core-maintainers

# Module-specific owners
/crates/security/ @team/security-team
/crates/database/ @team/data-team
```

**Best practices**:
- Define clear ownership boundaries
- Require approvals from owners
- Keep owner list current
- Document owner responsibilities

##### 7. COMPLIANCE.md (Recommended)

**Purpose**: Compliance requirements and audit trails

**Key sections**:
- Applicable Regulations (SOC2, HIPAA, GDPR, etc.)
- Audit Requirements
- Compliance Checklist
- Required Documentation
- Review Schedule

### Template Directory Structure

Store all repository governance templates in:

```
docs/
└── templates/
    └── git-files/
        ├── open-source/
        │   ├── CODE_OF_CONDUCT.md
        │   ├── SECURITY.md
        │   ├── SUPPORT.md
        │   ├── CONTRIBUTING.md
        │   ├── CHANGELOG.md
        │   ├── bug_report.md
        │   ├── feature_request.md
        │   ├── question.md
        │   └── PULL_REQUEST_TEMPLATE.md
        └── internal/
            ├── SECURITY.md
            ├── SUPPORT.md
            ├── CONTRIBUTING.md
            ├── INTERNAL_USAGE.md
            ├── COMPLIANCE.md
            ├── OWNERS.md
            └── internal_issue.md
```

### Implementation Workflow

#### Step 1: Classify Your Project

**Decision Matrix**:

| Criteria | Open-Source | Internal |
|----------|-------------|----------|
| Repository access | Public | Private |
| Contributions | External community | Team members only |
| License | OSI-approved | Proprietary |
| Distribution | Unrestricted | Restricted |
| Community | Building community | Internal team |

#### Step 2: Copy Required Templates

**For open-source**:
```bash
# From your project root
cp docs/templates/git-files/open-source/CODE_OF_CONDUCT.md .
cp docs/templates/git-files/open-source/SECURITY.md .
cp docs/templates/git-files/open-source/SUPPORT.md .
cp docs/templates/git-files/open-source/CONTRIBUTING.md .
mkdir -p .github/ISSUE_TEMPLATE
cp docs/templates/git-files/open-source/bug_report.md .github/ISSUE_TEMPLATE/
cp docs/templates/git-files/open-source/feature_request.md .github/ISSUE_TEMPLATE/
cp docs/templates/git-files/open-source/PULL_REQUEST_TEMPLATE.md .github/
```

**For internal**:
```bash
# From your project root
cp docs/templates/git-files/internal/SECURITY.md .
cp docs/templates/git-files/internal/SUPPORT.md .
cp docs/templates/git-files/internal/CONTRIBUTING.md .
cp docs/templates/git-files/internal/INTERNAL_USAGE.md .
mkdir -p .github/ISSUE_TEMPLATE
cp docs/templates/git-files/internal/internal_issue.md .github/ISSUE_TEMPLATE/
```

#### Step 3: Customize Templates

Replace placeholders:
- `[PROJECT_NAME]` → Your project name
- `[CONTACT_EMAIL]` → Appropriate contact
- `[TEAM_NAME]` → Your team/organization
- `[LICENSE]` → Your license type
- `[VERSION]` → Current version

#### Step 4: Validate Completeness

**Checklist**:

**Open-source projects**:
- [ ] CODE_OF_CONDUCT.md exists and is customized
- [ ] SECURITY.md exists with reporting process
- [ ] SUPPORT.md exists with help resources
- [ ] CONTRIBUTING.md exists with contribution process
- [ ] LICENSE file exists and is correct
- [ ] .github/ISSUE_TEMPLATE/bug_report.md exists
- [ ] .github/ISSUE_TEMPLATE/feature_request.md exists
- [ ] .github/PULL_REQUEST_TEMPLATE.md exists

**Internal projects**:
- [ ] SECURITY.md exists with internal contacts
- [ ] SUPPORT.md exists with internal channels
- [ ] CONTRIBUTING.md exists with internal process
- [ ] INTERNAL_USAGE.md exists with restrictions
- [ ] .github/ISSUE_TEMPLATE/internal_issue.md exists
- [ ] OWNERS.md or .github/CODEOWNERS exists (recommended)

### Best Practices

#### ✅ DO:

- **Create governance files FIRST** - Before writing any code or documentation
- **Choose project type early** - Open-source vs internal affects many decisions
- **Customize templates thoroughly** - Don't leave placeholder text
- **Keep files updated** - Review and update quarterly
- **Link files together** - CONTRIBUTING.md links to CODE_OF_CONDUCT.md, etc.
- **Be explicit about processes** - Clear > clever
- **Provide examples** - Show, don't just tell
- **Set expectations** - Response times, review timelines, etc.

#### ❌ DON'T:

- **Skip required files** - They exist for good reasons
- **Copy without customization** - Generic templates look unprofessional
- **Mix open-source and internal** - Choose one, be consistent
- **Use wrong license** - Get legal review if unsure
- **Ignore template updates** - When templates improve, update your files
- **Duplicate information** - Link to single source of truth
- **Forget about enforcement** - CODE_OF_CONDUCT without enforcement is useless

### Maintenance

#### Regular Reviews

- **Monthly**: Check for new issues/PRs using templates correctly
- **Quarterly**: Review and update contact information
- **Per Release**: Update CHANGELOG.md, verify version support in SECURITY.md
- **Annually**: Full governance file review and update

#### Template Updates

When templates in `docs/templates/git-files/` are updated:

1. Review changelog for template changes
2. Identify affected files in your project
3. Apply updates while preserving customizations
4. Commit with message: `chore: update governance files to match latest templates`

### Integration with Documentation Framework

Repository governance is **Phase 0** of the documentation framework (see [FRAMEWORK.md](../../templates/FRAMEWORK.md)):

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

**Why Phase 0?**:
1. **Sets project tone and expectations** - Contributors know what to expect immediately
2. **Legal/compliance requirements** - Licensing and security policies in place from start
3. **Affects contribution workflow** - Defines how people can contribute from day one
4. **Community health foundation** - Behavior standards and support channels established early
5. **Risk mitigation** - Security reporting and support processes prevent issues


### Troubleshooting

#### "Should my research prototype be open-source or internal?"

**Consider**:
- Will you publish results? → Open-source
- Contains proprietary algorithms? → Internal
- Building on public datasets? → Open-source
- NDA with partners? → Internal

**Recommendation**: Default to internal until publication, then open-source the cleaned-up version.

#### "Can I change from internal to open-source later?"

**Yes, but**:
- Requires legal review for IP clearance
- Must audit for sensitive information (keys, credentials, internal references)
- Need to choose and apply license
- Must create all open-source governance files
- Can't include proprietary dependencies

**Process**:
1. Legal approval
2. Code audit and cleanup
3. Add open-source governance files
4. Choose license
5. Make repository public

#### "We have custom requirements beyond the templates"

**Best practice**:
- Keep template structure
- Add additional sections clearly marked as "Organization-Specific"
- Document deviations in project README
- Consider contributing improvements back to templates

## Summary

Repository governance files are critical for project health, compliance, and community management. Choose your project type (open-source or internal) early, implement required files using templates, and maintain them throughout the project lifecycle.

**Key Takeaways**:
1. **Governance is Phase 0** - Do it before anything else
2. **Project type matters** - Open-source and internal have different requirements
3. **Use templates** - Don't reinvent, customize existing best practices
4. **Maintain actively** - Review and update regularly
5. **Enforcement matters** - Files without enforcement are just documentation

---

**Related Documentation**:
- [FRAMEWORK.md](../../templates/FRAMEWORK.md) - Overall documentation framework
- [Developer Guide](../developer-guide.md) - Development workflows
- [Templates Directory](../../templates/) - All documentation templates

**External Resources**:
- [Contributor Covenant](https://www.contributor-covenant.org/) - CODE_OF_CONDUCT standard
- [Keep a Changelog](https://keepachangelog.com/) - CHANGELOG format
- [Choose a License](https://choosealicense.com/) - License selection guide
- [SPDX License List](https://spdx.org/licenses/) - License identifiers

**Last Updated**: 2025-12-22  
**Version**: 1.0  
**Next Review**: 2026-03-22
