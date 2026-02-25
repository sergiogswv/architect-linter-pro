---
title: Testing Guide
sidebar_position: 2
---
# Testing Guide for Contributors

This guide provides comprehensive information on running, writing, and understanding tests for the architect-linter-pro project.

## Table of Contents

- [Running Tests Locally](#running-tests-locally)
- [Coverage](#coverage)
- [Benchmarks](#benchmarks)
- [Platform-Specific Tests](#platform-specific-tests)
- [Writing Tests](#writing-tests)
- [Test Organization](#test-organization)
- [CI/CD Integration](#cicd-integration)
- [Troubleshooting](#troubleshooting)

## Running Tests Locally

### Run all tests:
```bash
cargo test
```

### Run specific test file:
```bash
cargo test --test test_scoring
cargo test --test test_parsers
cargo test --test test_cli
```

### Run unit tests only:
```bash
cargo test --lib
```

### Run integration tests:
```bash
# Multi-file analysis integration test
cargo test --test test_multi_file_analysis

# Scoring integration test
cargo test --test test_scoring_integration

# Analyzer integration test
cargo test --test test_analyzer
```

### Run E2E tests:
```bash
cargo test --test test_e2e_github_action
```

### Run specific test:
```bash
cargo test test_calculate_health_score_with_no_violations
cargo test test_ts_parser_simple_function
```

### Run tests with output:
```bash
# Show test output
cargo test -- --nocapture

# Show test output with colors
cargo test -- --nocapture --color=always
```

### Run tests in single thread (useful for debugging):
```bash
cargo test -- --test-threads=1
```

## Coverage

### Generate coverage report:
```bash
# Unix/Linux/macOS
./scripts/coverage.sh

# Windows
scripts\coverage.bat

# Or directly
cargo tarpaulin --verbose --all-features --timeout 300 --out xml
```

This generates a `cobertura.xml` file in the root directory.

### View coverage by module:
```bash
cargo tarpaulin --exclude-files -- --test
```

### View HTML report:
```bash
# Generate HTML coverage report
cargo tarpaulin --out Html --output-dir target/coverage

# View the report (platform-specific)
open target/coverage/index.html              # macOS
xdg-open target/coverage/index.html          # Linux
start target/coverage/index.html             # Windows
```

### Coverage with specific filters:
```bash
# Coverage for specific module
cargo tarpaulin --lib -- --test scoring

# Exclude test files from coverage
cargo tarpaulin --exclude-files "tests/*"
```

## Benchmarks

The project includes benchmarking tests to track performance over time.

### Run all benchmarks:
```bash
cargo bench
```

### Run specific benchmark:
```bash
cargo bench --bench parsing_bench
cargo bench --bench performance_bench
```

### Save baseline:
```bash
cargo bench -- --save-baseline main
```

### Compare against baseline:
```bash
cargo bench -- --baseline main
```

### Available Benchmarks

- **`parsing_bench`**: Benchmarks TypeScript and Python parsing performance
  - Small files (~10 lines)
  - Medium files (~100 lines)
  - Large files (~500 lines)

- **`performance_bench`**: Benchmarks scoring engine performance
  - Single violation scoring
  - Multiple violations scoring
  - Health score calculation

## Platform-Specific Tests

Some tests only run on specific platforms due to filesystem differences or other platform-specific behavior.

### Platform Guards

- `#[cfg(unix)]` - Linux and macOS only
- `#[cfg(not(windows))]` - Linux and macOS only (Windows excluded)
- `#[cfg(windows)]` - Windows only
- `#[cfg(target_os = "macos")]` - macOS only

### Platform-Specific Examples

**Test that skips on Windows:**
```rust
#[test]
#[cfg(not(windows))]
fn test_ast_dropped_after_analysis() {
    // Filesystem-specific test that fails on Windows
}
```

**Test that only runs on Windows:**
```rust
#[test]
#[cfg(windows)]
fn test_windows_path_handling() {
    // Windows-specific path handling
}
```

### Known Platform-Specific Tests

- `test_ast_dropped_after_analysis` - Skipped on Windows due to filesystem differences in file locking
- E2E tests work on all platforms (no platform guards)

## Writing Tests

### Unit Tests

**Location:** `tests/unit/` directory (for lib-level unit tests) or inline in source files

**Guidelines:**
- NO filesystem access (use mocks or pure functions)
- Fast, isolated, parallel execution
- Test business logic in isolation
- Use descriptive test names: `test_<feature>_<scenario>_<expected_outcome>`

**Example:**
```rust
#[test]
fn test_calculate_health_score_with_no_violations() {
    let violations = vec![];
    let score = calculate_health_score(&violations, 100.0);
    assert_eq!(score, 100.0);
}

#[test]
fn test_calculate_health_score_with_critical_violation() {
    let violations = vec![Violation {
        severity: Severity::Critical,
        line: 10,
        message: "Test violation".to_string(),
    }];
    let score = calculate_health_score(&violations, 100.0);
    assert!(score &lt; 100.0);
}
```

**Edge Cases to Test:**
- Empty inputs
- Single vs multiple items
- Boundary conditions (max values, min values)
- Error conditions
- Invalid inputs

### Integration Tests

**Location:** `tests/` directory with names like `test_*_integration.rs` or `tests/integration/`

**Guidelines:**
- Can use tempfile for test data
- May need platform guards for filesystem issues
- Test multiple components working together
- Use common test utilities from `tests/common/mod.rs`
- Test real-world scenarios

**Example:**
```rust
use tempfile::TempDir;
use crate::common::TestProject;

#[test]
fn test_multi_file_scoring_integration() {
    let temp_dir = TestProject::new();

    // Create test files
    temp_dir.create_file("src/user.ts", "export class User {}");
    temp_dir.create_file("src/user.service.ts", "export class UserService {}");
    temp_dir.create_minimal_config();

    // Run analysis
    let result = analyze_directory(temp_dir.path()).unwrap();

    // Verify integration
    assert_eq!(result.files_analyzed, 2);
    assert!(result.health_score > 0.0);
}
```

### E2E Tests

**Location:** `tests/e2e/` or `tests/test_e2e_*.rs`

**Guidelines:**
- Test complete workflows by running the actual binary
- Use real projects from `tests/fixtures/` if available
- Platform guards for flaky tests
- Test CLI behavior, exit codes, and output format
- Verify end-to-end user journeys

**Example:**
```rust
use std::process::Command;

#[test]
fn test_github_action_workflow_happy_path() {
    let output = Command::new("cargo")
        .args(["run", "--release", "--", "."])
        .output()
        .expect("Failed to execute");

    // Check exit code
    assert!(output.status.success());

    // Check output contains expected content
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Analysis complete"));
}
```

### Using Test Utilities

The project provides common test utilities in `tests/common/mod.rs`:

```rust
use crate::common::{TestProject, forbidden_rule, join_rules};

// Create a temporary test project
let project = TestProject::new();

// Create files
project.create_file("src/domain/user.ts", "export class User {}");

// Create config
project.create_config("MVC", 100, &forbidden_rule("/domain/", "/infrastructure/"));

// Access project path
let project_path = project.path();
```

## Test Organization

```
tests/
├── common/               # Shared test utilities
│   └── mod.rs           # TestProject, helpers, fixtures
├── fixtures/            # Test data
│   ├── projects/        # Sample projects for testing
│   ├── samples/         # Code snippets (TS, Py, etc.)
│   ├── configs/         # Architecture config examples
│   ├── circular_deps/   # Projects with circular dependencies
│   ├── forbidden_imports/ # Projects testing forbidden import rules
│   ├── long_functions/  # Projects with long functions
│   └── perfect_* / mixed_* # Various test scenarios
├── integration/         # Integration test modules
│   └── mod.rs
├── unit/               # Unit test modules
│   └── mod.rs
├── e2e/                # End-to-end tests
│   └── github_action/
├── snapshots/          # Insta snapshot files
├── test_*.rs           # Unit and integration tests
└── README.md           # Test documentation
```

### Test File Naming Conventions

- `test_<module>.rs` - General tests for a module (e.g., `test_scoring.rs`)
- `test_<module>_integration.rs` - Integration tests (e.g., `test_scoring_integration.rs`)
- `test_unit_<feature>.rs` - Specific unit tests (e.g., `test_unit_scoring_edge_cases.rs`)
- `test_e2e_<workflow>.rs` - End-to-end tests (e.g., `test_e2e_github_action.rs`)

### Test Fixtures

Available test fixtures in `tests/fixtures/`:

1. **`perfect_project/`** - Clean codebase with no violations
2. **`mixed_issues/`** - Various types of violations
3. **`forbidden_imports/`** - Tests for forbidden import rules
4. **`long_functions/`** - Tests for function length violations
5. **`circular_deps/`** - Tests for circular dependency detection
6. **`failing_hexagonal/`** - Tests hexagonal architecture violations
7. **`mixed_clean_arch/`** - Tests clean architecture violations
8. **`perfect_mvc_project/`** - Clean MVC architecture example

## CI/CD Integration

### CI automatically runs:

1. **Test Suite** - All tests on Linux, macOS, Windows
   - Unit tests
   - Integration tests
   - E2E tests

2. **Linting** - Clippy checks
   - `cargo clippy --all-targets --all-features -- -W clippy::all`

3. **Formatting** - rustfmt checks
   - `cargo fmt --all -- --check`

4. **Build** - Verify compilation on all platforms
   - `cargo build --release`

5. **Coverage** - Code coverage report (Linux only)
   - Generates `cobertura.xml`
   - Uploads to Codecov

6. **Security Audit** - Dependency vulnerability checks
   - `cargo audit`

### Coverage Goals

Current baseline: 39.22% overall (as of v4.1.0)

Target coverage:
- **Scoring module**: >90% coverage
- **Parser modules**: >80% coverage
- **Overall**: >75% coverage

### Viewing Coverage in CI

Coverage reports are available:
1. In GitHub Actions summary (CI logs)
2. Via Codecov (if configured)
3. Locally by generating reports

## Troubleshooting

### Test fails intermittently

**Symptoms:** Test passes sometimes, fails other times

**Solutions:**
- Check if it needs platform guard (`#[cfg(not(windows))]`)
- Use `cargo test -- --test-threads=1` to run serially
- Check for filesystem race conditions
- Look for timing-dependent tests
- Add proper cleanup or isolation

**Example fix:**
```rust
#[test]
#[cfg(not(windows))]  // Add platform guard
fn test_filesystem_behavior() {
    // Test code
}
```

### Slow tests

**Symptoms:** Tests take too long to run

**Solutions:**
- Use `cargo test -- --nocapture` to see progress
- Profile with `cargo bench` for benchmarks
- Check for unnecessary I/O operations
- Consider using mocks instead of real filesystem
- Add `#[ignore]` to very slow tests and run manually

**Example:**
```rust
#[test]
#[ignore]  // Run with: cargo test -- --ignored
fn test_very_slow_operation() {
    // Slow test code
}
```

### Coverage not updating

**Symptoms:** Coverage report doesn't show new changes

**Solutions:**
1. Delete `Cargo.lock` and regenerate: `rm Cargo.lock && cargo generate-lockfile`
2. Clear build cache: `cargo clean`
3. Check tarpaulin version compatibility: `cargo tarpaulin --version`
4. Reinstall tarpaulin: `cargo install cargo-tarpaulin --force`
5. Ensure you're running with `--all-features`

### Tests fail in CI but pass locally

**Symptoms:** Tests work on your machine but fail in CI

**Solutions:**
- Check platform differences (Linux vs macOS vs Windows)
- Ensure all dependencies are in `Cargo.lock`
- Check for environment-specific assumptions
- Look for hardcoded paths
- Ensure test fixtures are committed
- Check for timezone or locale dependencies

### "No such file or directory" errors

**Symptoms:** Tests can't find files or directories

**Solutions:**
- Use absolute paths or proper relative paths
- Ensure fixtures exist in repository
- Use `TestProject` from `tests/common/mod.rs`
- Check that temporary directories are properly created
- Verify working directory assumptions

### Clippy warnings in tests

**Symptoms:** Clippy fails on test code

**Solutions:**
- Fix the warning if it's legitimate
- Add `#[allow(clippy::warning_name)]` if necessary for test
- Use `#[expect(clippy::warning_name)]` (Rust 2024+)
- Ensure test code follows best practices

**Example:**
```rust
#[test]
#[allow(clippy::too_many_arguments)]
fn test_function_with_many_params() {
    // Test code
}
```

## Best Practices

### DO:

- Write descriptive test names that explain what is being tested
- Use `TestProject` for filesystem-based tests
- Test edge cases and error conditions
- Keep tests fast and isolated
- Use `tempfile` for temporary directories
- Add platform guards when needed
- Follow naming conventions
- Document complex test scenarios

### DON'T:

- Use hardcoded absolute paths
- Assume specific working directory
- Write slow integration tests for what should be unit tests
- Ignore flaky tests - fix them or add guards
- Commit test artifacts (snapshots除外)
- Test implementation details instead of behavior
- Overuse `unwrap()` in tests - prefer proper error assertions

## Additional Resources

- [Cargo Book - Testing](https://doc.rust-lang.org/cargo/guide/tests.html)
- [Rust Book - Writing Automated Tests](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Insta - Snapshot Testing](https://insta.rs/)
- [Tarpaulin - Code Coverage](https://github.com/xd009642/tarpaulin)
- [Criterion - Benchmarking](https://bheisler.github.io/criterion.rs/book/)

## Quick Reference

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run single-threaded
cargo test -- --test-threads=1

# Generate coverage
./scripts/coverage.sh  # Unix
scripts\coverage.bat  # Windows

# Run benchmarks
cargo bench

# Format code
cargo fmt

# Check linting
cargo clippy --all-targets --all-features

# Clean build
cargo clean
```
