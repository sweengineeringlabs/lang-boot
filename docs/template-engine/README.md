# SEA Documentation Template Engine

**Language-agnostic documentation templates for software projects following the Stratified Encapsulation Architecture (SEA) and modern best practices.**

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Documentation](https://img.shields.io/badge/docs-templates-green.svg)](templates/)

## What is This?

A comprehensive collection of production-ready documentation templates that follow the **Audience + WHAT-WHY-HOW** structure. These templates are:

- 🌍 **Language-Agnostic** - Works for Rust, Python, Java, JavaScript, Go, C++, and more
- 📋 **Comprehensive** - Covers all aspects from code modules to production deployment
- 🎯 **Battle-Tested** - Used in production projects
- 🔧 **Customizable** - Easy-to-replace placeholders
- 📊 **Visual** - Includes workflow diagrams and dataflow charts
- ✅ **Complete** - Checklists, best practices, troubleshooting included

### 1. Setup Git Authentication (if needed)

If you haven't set up SSH authentication with GitHub yet:

📖 **See [GIT_SSH_SETUP.md](GIT_SSH_SETUP.md)** for complete SSH setup instructions (Linux, macOS, Windows, WSL).

### 2. Choose Your Templates

```bash
# Clone or download this repository
git clone git@github.com:[org]/template-engine.git
cd template-engine/templates
```

### 3. Copy Templates to Your Project

```bash
# For a new project
cp templates/crate-overview-template.md your-project/docs/overview.md
cp templates/framework-doc-template.md your-project/docs/architecture.md

# Replace placeholders
# [PROJECT_NAME] → your-project-name
# [Language] → Rust, Python, Java, etc.
# [Description] → your description
```

### 4. Customize for Your Needs

Each template includes:
- **Placeholders** in `[BRACKETS]` - Replace with your values
- **Customization Guide** at the end - Step-by-step instructions
- **Examples** - See how it's used in real projects

## Available Templates

### Core Documentation Templates

| Template | Purpose | Target Location | Best For |
|----------|---------|-----------------|----------|
| [crate-overview-template.md](templates/crate-overview-template.md) | Module/component docs | `docs/overview.md` | Libraries, modules, packages |
| [framework-doc-template.md](templates/framework-doc-template.md) | Framework-wide docs | `docs/guides/*.md` | Architecture, security, patterns |

### Deployment Templates

| Template | Purpose | Target Location | Best For |
|----------|---------|-----------------|----------|
| [release-versioning-template.md](templates/release-versioning-template.md) | Version management | `docs/deployment/versioning.md` | All projects |
| [deployment-workflow-template.md](templates/deployment-workflow-template.md) | Deployment process | `docs/deployment/workflow.md` | Libraries, applications |
| [ci-cd-template.md](templates/ci-cd-template.md) | CI/CD pipelines | `docs/deployment/ci-cd.md` | All projects |
| [publishing-template.md](templates/publishing-template.md) | Registry publishing | `docs/deployment/publishing.md` | Libraries, packages |

### Repository Files

| Template | Purpose | Location | Required |
|----------|---------|----------|----------|
| [git-files/](templates/git-files/) | Repository governance | Repository root | Yes (see guide) |

## Template Structure

All templates follow a consistent structure:

```markdown
# [Document Title]

**Audience**: [Who should read this]

## WHAT: [What is this]
- Clear description
- Scope definition
- Out of scope

## WHY: [Why it matters]
- Problems addressed
- Benefits

## HOW: [How to implement]
- Workflow diagrams
- Step-by-step instructions
- Code examples
- Checklists
- Best practices
- Troubleshooting

## Summary
- Key takeaways
- Related documentation
- External resources

---

## Template Customization Guide
[Detailed instructions for adapting the template]
```

## Documentation Framework

### Phase 0: Git Repository Files (REQUIRED FIRST)
Set up governance files before any other documentation:
- CODE_OF_CONDUCT.md (open-source)
- SECURITY.md
- SUPPORT.md
- CONTRIBUTING.md
- LICENSE
- Issue/PR templates

See [Repository Governance Guide](templates/FRAMEWORK.md#git-repository-files)

### Phase 1-6: Structured Documentation
Follow the phased approach defined in [FRAMEWORK.md](templates/FRAMEWORK.md)

## Features

### 🎯 Audience-First Approach
Every document declares its intended audience upfront

### 📊 Visual Workflows
Includes ASCII diagrams for:
- Process flows
- Decision trees
- Data flows
- Pipeline visualizations

### ✅ Comprehensive Checklists
Ready-to-use checklists for:
- Pre-deployment validation
- Quality gates
- Publishing readiness
- Post-deployment verification

### 🔧 Multi-Language Support
Templates work for:
- **Compiled**: Rust, Go, C++, Java, C#
- **Interpreted**: Python, Ruby, PHP, JavaScript
- **Package Managers**: cargo, npm, pip, maven, gradle, nuget

### 🚀 CI/CD Ready
Platform-agnostic CI/CD templates for:
- GitHub Actions
- GitLab CI
- Jenkins
- CircleCI
- Travis CI

## Examples

### Using the Module Template

```bash
# Copy template
cp templates/crate-overview-template.md myproject/src/auth/docs/overview.md

# Edit and replace:
[Module Name] → Authentication
[Language] → Rust  
[Description] → JWT-based authentication system
```

Result: Professional module documentation in minutes!

### Using the CI/CD Template

```bash
# Copy template
cp templates/ci-cd-template.md myproject/docs/ci-cd.md

# Customize for your platform:
[CI Platform] → GitHub Actions
[Language] → Python
[test command] → pytest
```

Result: Complete CI/CD documentation with workflows!

## Project Types Supported

- ✅ **Libraries** - Reusable packages published to registries
- ✅ **Frameworks** - Tools and abstractions for building applications
- ✅ **Applications** - End-user software and services
- ✅ **Microservices** - Distributed service architectures
- ✅ **CLI Tools** - Command-line utilities
- ✅ **APIs** - REST, GraphQL, gRPC services

## Language-Specific Guides

Templates include language-specific sections for:
- Version management (SemVer)
- Package manifest formats
- Build commands
- Test frameworks
- Publishing procedures
- Registry specifics

## Best Practices Included

Every template includes:
- ✅ **DO** recommendations
- ❌ **DON'T** anti-patterns
- 🔍 **Troubleshooting** common issues
- 📚 **External resources** for deep dives

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md)

Ways to contribute:
- 📝 Improve existing templates
- ➕ Add new templates
- 🐛 Fix issues
- 📖 Enhance documentation
- 💡 Share examples

## License

This project is licensed under  the MIT License - see the [LICENSE](LICENSE) file for details.

## Maintained By

SEA Documentation Template Engine Team

## Related Projects

- [Rustboot Framework](https://github.com/[org]/rustboot) - Example usage of these templates
- [SEA Architecture Guide](https://github.com/[org]/sea-architecture) - SEA methodology

## Support

- 📖 [Documentation](templates/)
- 💬 [Discussions](https://github.com/[org]/template-engine/discussions)
- 🐛 [Issues](https://github.com/[org]/template-engine/issues)
- 📧 Email: [contact email]

---

**Start documenting better, today! 🚀**
