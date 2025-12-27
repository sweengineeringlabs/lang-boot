# Documentation Templates

This directory contains language-agnostic templates for comprehensive software documentation.

## Quick Reference

### Module/Component Documentation
- **[crate-overview-template.md](crate-overview-template.md)** - Module and component documentation

### Framework Documentation
- **[framework-doc-template.md](framework-doc-template.md)** - Framework-wide documentation (architecture, security, patterns)
- **[FRAMEWORK.md](FRAMEWORK.md)** - Complete documentation framework guide

### Deployment Documentation
- **[release-versioning-template.md](release-versioning-template.md)** - Version management and SemVer
- **[deployment-workflow-template.md](deployment-workflow-template.md)** - Deployment strategies and processes
- **[ci-cd-template.md](ci-cd-template.md)** - CI/CD pipeline configuration
- **[publishing-template.md](publishing-template.md)** - Package registry publishing

### Repository Files
- **[git-files/](git-files/)** - Git repository governance files (coming soon)

## How to Use

1. **Choose a template** based on what you're documenting
2. **Copy it** to your project's documentation directory
3. **Replace placeholders** marked with `[BRACKETS]`
4. **Follow the customization guide** at the end of each template
5. **Remove meta instructions** before publishing

## Template Structure

Each template follows this structure:

```
# Title
**Audience**: [Who should read this]

## WHAT: [Description]
## WHY: [Motivation and benefits]
## HOW: [Implementation]
  - Workflows/Diagrams
  - Instructions
  - Examples
  - Checklists
  - Best practices

## Summary
## Template Customization Guide
```

## Placeholder Convention

Templates use this placeholder format:
- `[PROJECT_NAME]` - Your project name
- `[Language]` - Programming language (Rust, Python, etc.)
- `[Description]` - Your description
- `YYYY-MM-DD` - Dates
- `[command]` - Commands specific to your tools
- `[X.Y.Z]` - Version numbers

## Support

For issues or questions about templates:
- See individual template customization guides
- Check [FRAMEWORK.md](FRAMEWORK.md) for overall guidance
- Open an issue on GitHub

---

**Happy Documenting! 📚**
