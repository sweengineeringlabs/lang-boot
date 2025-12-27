# Contributing to SEA Documentation Template Engine

Thank you for your interest in contributing! This project aims to provide high-quality, language-agnostic documentation templates for software projects.

## How to Contribute

### 1. Reporting Issues

If you find problems with templates or have suggestions:

- Check if the issue already exists
- Provide a clear description
- Include examples if applicable
- Tag appropriately (bug, enhancement, documentation)

### 2. Improving Existing Templates

To improve a template:

1. Fork the repository
2. Create a branch (`feature/improve-ci-cd-template`)
3. Make your changes
4. Test with a real project
5. Submit a pull request

**What we look for:**
- Clearer instructions
- Better examples
- Additional best practices
- Fixed errors or ambiguities
- Improved workflow diagrams

### 3. Adding New Templates

To add a new template:

1. Check if it fits the project scope
2. Follow the standard template structure:
   ```
   # Title
   **Audience**: [...]
   ## WHAT
   ## WHY
   ## HOW
   ## Summary
   ## Template Customization Guide
   ```
3. Include workflow diagrams where applicable
4. Add comprehensive placeholders
5. Include best practices and troubleshooting
6. Update the main README.md with your template
7. Add entry to templates/README.md

**Template requirements:**
- Language-agnostic (no language-specific examples in main content)
- Comprehensive customization guide
- Clear placeholders in `[BRACKETS]`
- At least one workflow or dataflow diagram
- Best practices (DO/DON'T)
- Troubleshooting section

### 4. Documentation Improvements

Help improve:
- README clarity
- Examples
- Quick start guides
- Template usage instructions

## Development Guidelines

### Writing Style

- **Clear and concise** - Avoid jargon
- **Actionable** - Provide specific instructions
- **Inclusive** - Use "you" not "we"
- **Professional** - Maintain formal tone

### Template Formatting

- Use Markdown consistently
- Use ASCII diagrams for workflows
- Include code blocks with language tags
- Use checklists for steps
- Bold key terms

### Placeholder Format

Always use:
- `[PLACEHOLDER]` - For values users should replace
- `YYYY-MM-DD` - For dates
- `X.Y.Z` - For versions
- `[command]` - For commands

### Diagram Standards

Use ASCII art for diagrams:
- Box characters: `┌─┐│└┘├┤┬┴┼`
- Arrows: `→ ← ↓ ↑ ↔ ↕`
- Clear flow direction (top-to-bottom or left-to-right)
- Consistent spacing

## Pull Request Process

1. **Create an issue** first for significant changes
2. **Fork and branch** from `main`
3. **Make your changes** following guidelines
4. **Test thoroughly** with a real project
5. **Update documentation** (README, templates/README)
6. **Submit PR** with clear description
7. **Respond to feedback** promptly

### PR Description Should Include

- What problem does this solve?
- What changes were made?
- How was it tested?
- Screenshots/examples (if visual changes)

## Code of Conduct

### Our Standards

- Be respectful and inclusive
- Welcome newcomers
- Focus on constructive feedback
- Assume good intentions

### Unacceptable Behavior

- Harassment or discrimination
- Trolling or insulting comments
- Personal attacks
- Spam or off-topic content

## Review Process

Maintainers will review PRs for:

- **Quality**: Does it improve the project?
- **Consistency**: Does it match our style?
- **Completeness**: Is it fully documented?
- **Testing**: Has it been validated?

## Getting Help

- 💬 [GitHub Discussions](https://github.com/[org]/template-engine/discussions)
- 🐛 [GitHub Issues](https://github.com/[org]/template-engine/issues)
- 📧 Email maintainers

## Recognition

Contributors will be:
- Listed in README acknowledgments
- Credited in release notes
- Mentioned in relevant template headers (opt-in)

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

**Thank you for making documentation better for everyone! 🙏**
