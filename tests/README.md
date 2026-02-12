# Test Suite Documentation

This directory contains the comprehensive test suite for Architect Linter Pro v4.1.0+

## Structure

```
tests/
├── common/           # Shared test utilities and helpers
│   └── mod.rs       # TestProject, config helpers, etc.
├── unit/            # Unit tests (will contain scoring, metrics, etc.)
├── integration/     # Integration tests (will contain parser tests, etc.)
├── e2e/             # End-to-end CLI tests
├── test_common.rs   # Tests for test utilities themselves
└── README.md        # This file
```

## Running Tests

```bash
# Run all tests
cargo test

# Run specific test file
cargo test --test test_common

# Run tests with output
cargo test -- --nocapture

# Run tests in parallel (default)
cargo test

# Run tests serially (when needed)
cargo test -- --test-threads=1
```

## Test Utilities

### TestProject

Helper for creating temporary test projects:

```rust
use common::TestProject;

let project = TestProject::new();

// Create files
project.create_file("src/user.ts", "export class User {}");

// Create config
project.create_minimal_config();

// Get path
let path = project.path();
```

### Config Helpers

```rust
use common::{forbidden_rule, join_rules};

// Single rule
let rule = forbidden_rule("/domain/", "/infrastructure/");

// Multiple rules
let rules = vec![
    forbidden_rule("/controllers/", "/repositories/"),
    forbidden_rule("/domain/", "/infrastructure/"),
];
let rules_str = join_rules(&rules);
```

## Test Coverage Goals

- **Unit Tests**: 90% coverage for core modules
  - `src/scoring.rs` - Health score calculation
  - `src/metrics.rs` - Metrics aggregation
  - `src/analysis_result.rs` - Result structures

- **Integration Tests**: All parsers
  - TypeScript/JavaScript
  - Python
  - Go
  - PHP
  - Java

- **E2E Tests**: CLI functionality
  - Basic analysis
  - Report generation (JSON/Markdown)
  - Watch mode
  - Git integration (--staged)

## Writing New Tests

### Unit Test Example

```rust
// tests/unit/scoring_tests.rs
#[path = "../common/mod.rs"]
mod common;

#[test]
fn test_health_score_calculation() {
    // Your test here
}
```

### Integration Test Example

```rust
// tests/integration/parser_tests.rs
#[path = "../common/mod.rs"]
mod common;

use common::TestProject;

#[test]
fn test_typescript_parser() {
    let project = TestProject::new();
    project.create_file("src/test.ts", "import X from 'y';");
    // Test parser
}
```

### E2E Test Example

```rust
// tests/e2e/cli_tests.rs
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_version() {
    Command::cargo_bin("architect-linter-pro")
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("4.0.0"));
}
```

## Snapshot Testing

We use `insta` for snapshot testing:

```rust
use insta::assert_json_snapshot;

#[test]
fn test_json_output() {
    let result = generate_report();
    assert_json_snapshot!(result);
}
```

First run creates the snapshot, subsequent runs compare against it.

## Test Dependencies

- `tempfile` - Temporary directories and files
- `assert_cmd` - CLI testing utilities
- `predicates` - Assertions for assert_cmd
- `insta` - Snapshot testing
- `pretty_assertions` - Better assertion output
- `serial_test` - Run tests serially when needed

## Current Status

✅ **Infrastructure Setup Complete** (v4.1.0 - Task #20)
- [x] Test utilities (TestProject, config helpers)
- [x] Directory structure
- [x] Dev dependencies added
- [x] 11 tests for utilities (all passing)

⏳ **Next Steps**
- [ ] Unit tests for scoring engine
- [ ] Integration tests for parsers
- [ ] Convert fixtures to automated tests
- [ ] E2E CLI tests
- [ ] CI/CD integration

## Running with Coverage

```bash
# Install cargo-tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage

# View coverage
open coverage/index.html
```

## Best Practices

1. **Use TestProject** for all file-based tests
2. **Keep tests isolated** - each test should be independent
3. **Use descriptive names** - `test_health_score_with_no_violations` not `test_1`
4. **Test edge cases** - empty input, maximum values, invalid data
5. **Use snapshots** for complex output validation
6. **Document non-obvious tests** with comments

---

**Created:** 2026-02-12
**Version:** v4.1.0
**Maintainer:** Core team
