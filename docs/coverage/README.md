# Test Coverage Reports

This directory contains baseline coverage reports and documentation for the Architect Linter Pro test suite.

## Directory Structure

```
coverage/
├── README.md                                    # This file
├── typescript-parser-baseline.md                # Baseline documentation for TypeScript parser
├── typescript-parser-cobertura.xml              # XML coverage report (machine-readable)
└── typescript-parser-report.html                # HTML coverage report (human-readable)
```

## Coverage Reports

### TypeScript Parser Unit Tests

**Report Date:** 2026-02-16
**Coverage Tool:** cargo-tarpaulin v0.35.1
**Test Suite:** `tests/test_parsers.rs`

**Quick Stats:**
- Overall Coverage: **74.14%** (81/116 lines)
- typescript.rs: **100%** (23/23 lines) ✅
- typescript_pure.rs: **62.37%** (58/93 lines) ⚠️
- Tests Passed: **36/36** ✅
- Execution Time: **0.17s** ⚡

**Files:**
- 📊 [Baseline Documentation](./typescript-parser-baseline.md) - Comprehensive analysis and recommendations
- 📈 [HTML Report](./typescript-parser-report.html) - Visual coverage report (open in browser)
- 🔧 [Cobertura XML](./typescript-parser-cobertura.xml) - Machine-readable coverage data for CI/CD

## How to Generate Coverage Reports

### Using cargo-tarpaulin

```bash
# Install tarpaulin (one-time)
cargo install cargo-tarpaulin

# Generate coverage for parser tests
cargo tarpaulin --verbose --all-features --timeout 300 \
    --test test_parsers \
    --out xml --out html \
    --output-dir docs/coverage

# Generate coverage for all tests
cargo tarpaulin --verbose --all-features --timeout 300 \
    --out xml --out html \
    --output-dir docs/coverage
```

### Using the coverage script

```bash
# Run the project's coverage script
./scripts/coverage.sh

# Output will be in cobertura.xml at project root
```

## Coverage Targets

| Module | Current | Target | Status |
|--------|---------|--------|--------|
| TypeScript Parser (Main) | 100% | 100% | ✅ Met |
| TypeScript Parser (Pure) | 62.37% | 70% | 🟡 Near |
| **Overall Parser** | **74.14%** | **80%** | 🟡 **Near** |

## Viewing Reports

### HTML Report (Recommended)

Open the HTML report in your browser for an interactive view:

```bash
# Linux
xdg-open docs/coverage/typescript-parser-report.html

# macOS
open docs/coverage/typescript-parser-report.html

# Windows
start docs/coverage/typescript-parser-report.html
```

The HTML report provides:
- Line-by-line coverage visualization
- Color-coded coverage indicators
- Function-level coverage breakdown
- Interactive navigation

### XML Report (CI/CD)

The Cobertura XML format is used for:
- CI/CD integration (GitHub Actions, GitLab CI)
- Coverage tracking over time
- Automated quality gates
- Coverage badges

Example with Codecov:
```bash
curl -Os https://codecov.io/bash -s -R -f docs/coverage/typescript-parser-cobertura.xml
```

## Understanding Coverage Metrics

### Line Coverage
Percentage of executable lines that were run during tests.

```
Covered Lines / Total Lines = Coverage %
81 / 116 = 74.14%
```

### Function Coverage
Percentage of functions that were called during tests.

- **Fully Covered:** Function executed in all paths
- **Partially Covered:** Function executed but some branches missed
- **Uncovered:** Function never called

### Branch Coverage
Not tracked by tarpaulin (shows as 0%).

## Coverage Analysis Tools

### Viewing Uncovered Lines

```bash
# Filter for uncovered lines in specific file
grep "typescript_pure" docs/coverage/typescript-parser-cobertura.xml | \
    grep 'hits="0"' | \
    sed 's/.*number="\([0-9]*\)".*/\1/' | \
    sort -n
```

### Coverage by Module

```bash
# Summary of all modules
cargo tarpaulin --test test_parsers 2>&1 | grep "src/"
```

## Baseline Documentation

The baseline documentation provides:
- ✅ Executive summary with key metrics
- ✅ Detailed module analysis
- ✅ Function-by-function coverage breakdown
- ✅ Test suite breakdown
- ✅ Uncovered code analysis with priorities
- ✅ Improvement recommendations
- ✅ Coverage metrics and targets

See [typescript-parser-baseline.md](./typescript-parser-baseline.md) for complete details.

## Next Steps

Based on the baseline analysis, the following improvements are recommended:

### Short-term (Next Sprint)
1. Add unit tests for helper functions (extract_folder_from_pattern, generate_import_patterns)
2. Add tests for alias imports (@/services/user)
3. Add tests for multi-level relative imports (../../../utils)

### Medium-term (Future Releases)
1. Add snapshot tests for complex TypeScript files
2. Add property-based tests for pattern matching
3. Add performance benchmarks

### Long-term (Technical Debt)
1. Review and document/remove unused utility functions
2. Enhance error handling tests
3. Add tests for dynamic imports and re-exports

## CI/CD Integration

### GitHub Actions

The project includes automated coverage reporting:

```yaml
# .github/workflows/coverage.yml
- name: Generate coverage report
  run: |
    cargo tarpaulin --verbose --all-features --timeout 300 \
      --out xml --output-dir coverage

- name: Upload to Codecov
  uses: codecov/codecov-action@v3
  with:
    files: coverage/cobertura.xml
```

See `.github/workflows/` for complete CI configuration.

## Historical Coverage

| Date | Version | Coverage | Change | Notes |
|------|---------|----------|--------|-------|
| 2026-02-16 | Baseline | 74.14% | - | Initial baseline after pure function refactoring |

## Contributing

When adding new tests:

1. Run coverage before and after changes
2. Ensure coverage doesn't decrease
3. Update baseline documentation if coverage significantly changes
4. Add new sections to this README for new test categories

### Coverage Guidelines

- **Minimum acceptable:** 70% for new code
- **Target:** 80% overall coverage
- **Critical paths:** 100% coverage required
- **Helper functions:** 60% minimum acceptable

## Resources

- [cargo-tarpaulin Documentation](https://github.com/xd009642/tarpaulin)
- [Cobertura Format Spec](https://cobertura.github.io/cobertura/)
- [Testing Guide](../TESTING_GUIDE.md)
- [Project README](../../README.md)

## Contact

For questions about test coverage:
- Review [typescript-parser-baseline.md](./typescript-parser-baseline.md)
- Check [TESTING_GUIDE.md](../TESTING_GUIDE.md)
- Open an issue on GitHub

---

**Last Updated:** 2026-02-16
**Coverage Tool Version:** cargo-tarpaulin v0.35.1
**Next Review:** After implementing short-term improvements
