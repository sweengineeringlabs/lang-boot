# CI/CD Files Index

Complete index of all CI/CD related files in the rustboot framework.

## Directory Structure

```
.github/
├── workflows/
│   ├── ci.yml                    # Main CI pipeline
│   ├── release.yml               # Release automation
│   ├── security.yml              # Security scanning
│   ├── benchmark.yml             # Performance benchmarks
│   ├── docs.yml                  # Documentation deployment
│   ├── nightly.yml               # Nightly testing
│   └── README.md                 # Workflow documentation
├── ISSUE_TEMPLATE/
│   ├── bug_report.md             # Bug report template
│   ├── feature_request.md        # Feature request template
│   ├── documentation.md          # Documentation issue template
│   └── config.yml                # Template configuration
├── CODEOWNERS                    # Code ownership
├── dependabot.yml                # Dependency updates
├── deny.toml                     # cargo-deny configuration
├── pull_request_template.md     # PR template
├── CICD_SETUP.md                # Complete setup guide
├── QUICK_REFERENCE.md           # Quick reference for developers
└── INDEX.md                      # This file

.typos.toml                       # Spell checker config (root)
```

## File Descriptions

### Workflows

| File | Purpose | Trigger | Duration |
|------|---------|---------|----------|
| `ci.yml` | Main CI pipeline with tests, linting, coverage | Push/PR to main | ~15-20 min |
| `release.yml` | Automated releases to crates.io | Version tags (v*.*.*) | ~20-30 min |
| `security.yml` | Security audits and scanning | Daily + Cargo changes | ~10-15 min |
| `benchmark.yml` | Performance benchmarks | Push/PR + manual | ~10 min |
| `docs.yml` | Documentation deployment | Push to main + manual | ~5 min |
| `nightly.yml` | Nightly Rust testing | Daily 02:00 UTC + manual | ~15 min |

### Configuration Files

| File | Purpose |
|------|---------|
| `CODEOWNERS` | Define code ownership and auto-assign reviewers |
| `dependabot.yml` | Configure automated dependency updates |
| `deny.toml` | Security, license, and dependency policy |
| `pull_request_template.md` | Template for pull requests |
| `.typos.toml` | Spell checker configuration |

### Documentation

| File | Purpose | Audience |
|------|---------|----------|
| `CICD_SETUP.md` | Complete setup guide | Repository admins |
| `QUICK_REFERENCE.md` | Quick commands and tips | Developers |
| `workflows/README.md` | Detailed workflow docs | All users |
| `INDEX.md` | This file - file index | All users |

### Issue Templates

| File | Purpose |
|------|---------|
| `bug_report.md` | Report bugs |
| `feature_request.md` | Request new features |
| `documentation.md` | Report documentation issues |
| `config.yml` | Configure issue templates |

## Quick Navigation

### For Developers
- **Getting Started**: Start with `QUICK_REFERENCE.md`
- **Local Testing**: Check `QUICK_REFERENCE.md` → Local Pre-Commit Checks
- **Release Process**: See `QUICK_REFERENCE.md` → Release Workflow

### For Maintainers
- **Initial Setup**: Read `CICD_SETUP.md`
- **Workflow Details**: See `workflows/README.md`
- **Configuration**: Review individual config files

### For Contributors
- **PR Template**: `.github/pull_request_template.md`
- **Issue Templates**: `.github/ISSUE_TEMPLATE/`
- **Code Ownership**: `.github/CODEOWNERS`

## File Dependencies

### Required for CI to Work
- `workflows/ci.yml` - Main CI pipeline
- `.github/deny.toml` - For cargo-deny checks
- `.typos.toml` - For spell checking

### Required for Releases
- `workflows/release.yml` - Release automation
- Secret: `CARGO_REGISTRY_TOKEN`

### Optional but Recommended
- `workflows/security.yml` - Security scanning
- `workflows/docs.yml` - Documentation
- `dependabot.yml` - Dependency updates
- `CODEOWNERS` - Auto-assign reviewers

## Modification Guidelines

### When to Update

| File | Update When |
|------|-------------|
| `ci.yml` | Adding new checks, changing MSRV, updating matrix |
| `release.yml` | Changing release process, crate publishing order |
| `security.yml` | Adding security tools, changing scan frequency |
| `deny.toml` | Updating license policy, ignoring advisories |
| `dependabot.yml` | Changing update frequency, grouping strategy |
| `CODEOWNERS` | Team changes, module ownership changes |

### How to Test Changes

**Workflow Changes:**
```bash
# Use act for local testing (if possible)
act -W .github/workflows/ci.yml

# Or create a test branch and PR
git checkout -b test/workflow-changes
# Make changes, push, create PR
```

**Configuration Changes:**
- `deny.toml`: Run `cargo deny check`
- `dependabot.yml`: Changes take effect on next scheduled run
- `.typos.toml`: Run `typos` locally

## Common Tasks

### Add New Workflow
1. Create `.github/workflows/new-workflow.yml`
2. Add documentation to `workflows/README.md`
3. Update this index
4. Test with PR

### Update MSRV
1. Update `ci.yml` → `msrv` job toolchain version
2. Update `README.md` if documented there
3. Test locally with updated version

### Change License Policy
1. Edit `.github/deny.toml` → `[licenses]` section
2. Run `cargo deny check licenses`
3. Update documentation if needed

### Add Code Owner
1. Edit `.github/CODEOWNERS`
2. Add entry for path and owner(s)
3. Test with a PR to that path

## Badges

Add workflow badges to README:

```markdown
[![CI](https://github.com/phdsystems/rustboot/actions/workflows/ci.yml/badge.svg)](https://github.com/phdsystems/rustboot/actions/workflows/ci.yml)
[![Security](https://github.com/phdsystems/rustboot/actions/workflows/security.yml/badge.svg)](https://github.com/phdsystems/rustboot/actions/workflows/security.yml)
[![Docs](https://github.com/phdsystems/rustboot/actions/workflows/docs.yml/badge.svg)](https://github.com/phdsystems/rustboot/actions/workflows/docs.yml)
```

## Resources

### GitHub Actions
- [Official Documentation](https://docs.github.com/en/actions)
- [Marketplace](https://github.com/marketplace?type=actions)

### Rust Tooling
- [cargo-deny](https://embarkstudios.github.io/cargo-deny/)
- [cargo-audit](https://github.com/rustsec/rustsec/tree/main/cargo-audit)
- [cargo-nextest](https://nexte.st/)
- [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov)

### Security
- [RustSec Advisory Database](https://rustsec.org/)
- [Trivy](https://aquasecurity.github.io/trivy/)
- [Semgrep](https://semgrep.dev/)

### Documentation
- [GitHub Pages](https://docs.github.com/en/pages)
- [Codecov](https://docs.codecov.com/)

---

**Maintained by**: @elvischidera  
**Last Updated**: 2024-12-24  
**Version**: 1.0
