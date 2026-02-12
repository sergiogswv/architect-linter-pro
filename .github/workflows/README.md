# GitHub Actions Workflows

This directory contains the CI/CD workflows for Architect Linter Pro.

## Workflows

### 🔄 CI Pipeline (`ci.yml`)

**Triggers:** Push and PR to `master`, `main`, `develop` branches

**Jobs:**
- **Test Suite** - Runs all 134+ tests on Linux, macOS, and Windows
- **Clippy** - Rust linting with `cargo clippy`
- **Formatting** - Code style checks with `rustfmt`
- **Build** - Compilation verification on all platforms
- **Coverage** - Code coverage report with `tarpaulin`
- **Security Audit** - Dependency vulnerability scanning with `cargo-audit`
- **CI Success** - Final check that all jobs passed

**Duration:** ~5-10 minutes

**Status Badge:**
```markdown
[![CI](https://github.com/YOUR_USERNAME/architect-linter-pro/workflows/CI/badge.svg)](https://github.com/YOUR_USERNAME/architect-linter-pro/actions)
```

---

### 🚀 Release Pipeline (`release.yml`)

**Triggers:** Push tags matching `v*.*.*` (e.g., `v4.1.0`)

**Jobs:**
- **Create Release** - Creates GitHub release
- **Build Release** - Builds binaries for all platforms:
  - Linux (x86_64, musl)
  - macOS (x86_64, aarch64/M1)
  - Windows (x86_64)
- **Publish Crate** - (Optional) Publishes to crates.io

**To create a release:**
```bash
git tag -a v4.1.0 -m "Release v4.1.0"
git push origin v4.1.0
```

---

### ✅ PR Check (`pr-check.yml`)

**Triggers:** Pull requests (opened, synchronized, reopened)

**Jobs:**
- **PR Info** - Shows PR statistics (files changed, lines added/removed)
- **Quick Test** - Fast test run on Linux
- **Quality Check** - Formatting + Clippy
- **Build Check** - Ensures compilation succeeds
- **Dependency Check** - Checks for outdated dependencies
- **PR Check Success** - Final verification

**Features:**
- Fast feedback (~2-3 minutes)
- Detailed summary in PR comments
- Prevents merging broken code

---

## Configuration Files

### `rustfmt.toml`
Rust formatting configuration:
- Edition: 2021
- Max width: 100 characters
- Tab spaces: 4
- Unix newlines
- Import organization

### `.clippy.toml`
Clippy linting configuration:
- Cognitive complexity threshold: 30
- Max function parameters: 3 bools
- Max trait bounds: 5

---

## Local Development

### Run tests locally:
```bash
cargo test --verbose
```

### Check formatting:
```bash
cargo fmt --all -- --check
```

### Run clippy:
```bash
cargo clippy --all-targets --all-features -- -D warnings
```

### Generate coverage report:
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --verbose --all-features --workspace --out html
```

### Security audit:
```bash
cargo install cargo-audit
cargo audit
```

---

## CI/CD Best Practices

✅ **Always run tests locally before pushing**
```bash
./scripts/pre-push.sh  # If available
```

✅ **Keep workflows fast**
- Use caching for dependencies
- Run quick tests first, slow tests later
- Parallelize independent jobs

✅ **Monitor CI performance**
- Check workflow run times
- Optimize slow tests
- Update cache keys when dependencies change

✅ **Security**
- Never commit secrets to workflows
- Use GitHub Secrets for sensitive data
- Audit dependencies regularly

---

## Troubleshooting

### ❌ Tests failing in CI but passing locally?
- Check Rust version (CI uses `stable`)
- Verify all tests run on your OS
- Check for race conditions in tests

### ❌ Build failing on specific platform?
- Use platform-specific conditions
- Test with Docker locally
- Check platform-specific dependencies

### ❌ Cache not working?
- Update cache keys
- Clear cache manually in GitHub Actions settings
- Verify cache paths are correct

---

## Metrics

Current test coverage:
- **134 tests** (98 passing, 1 ignored)
- Unit tests: 12
- Integration tests: 36 (parsers)
- E2E tests: 33 (CLI)
- Fixture tests: 11
- Scoring tests: 37
- Common tests: 11

CI platforms:
- ✅ Linux (Ubuntu latest)
- ✅ macOS (latest)
- ✅ Windows (latest)

---

## Future Improvements

- [ ] Add benchmarking workflow
- [ ] Set up automated changelog generation
- [ ] Add performance regression tests
- [ ] Implement canary deployments
- [ ] Add Docker image build pipeline
- [ ] Set up automatic dependency updates (Dependabot)
