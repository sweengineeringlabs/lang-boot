# Documentation Templates

Templates for consistent documentation across Pyboot.

## Available Templates

### [Module Overview Template](module-overview-template.md)
For documenting individual modules (WHAT-WHY-HOW structure).

### [Framework Doc Template](framework-doc-template.md)
For framework-level documentation (Audience + WHAT-WHY-HOW).

## Documentation Standards

### Module Documentation (No Audience)
- Use WHAT-WHY-HOW structure
- Audience is implicit (developers)
- Link to examples and tests

### Framework Documentation (With Audience)
- Specify Audience first
- Use WHAT-WHY-HOW structure
- Link to related docs

## WHAT-WHY-HOW Structure

Every document should answer:

1. **WHAT** - What does this provide?
2. **WHY** - Why use this? What problems does it solve?
3. **HOW** - How to use it? Examples and API.

## Quick Reference

| Location | Format | Audience |
|----------|--------|----------|
| `README.md` | Quick Start | Everyone |
| `docs/overview.md` | Hub | All |
| `docs/3-design/*.md` | Audience + WHAT-WHY-HOW | Specified |
| `docs/4-development/*.md` | Audience + WHAT-WHY-HOW | Specified |
| Module `__init__.py` docstrings | WHAT-WHY-HOW | Developers |

---

**Based on**: Rustboot documentation framework
