# Test Suite

Comprehensive test suite for Architect Linter Pro, covering the scoring engine, parsers, and integration scenarios.

## Structure

```
tests/
├── test_scoring.rs              # Unit tests for scoring engine (59 tests)
├── test_scoring_integration.rs  # Integration tests with fixtures (4 tests)
├── test_parsers.rs              # Multi-language parser tests
├── test_fixtures.rs             # Fixture utilities
├── fixtures/                    # Test project fixtures
│   ├── perfect_mvc_project/     # Perfect architecture (A grade)
│   ├── failing_hexagonal/       # Layer violations (C/D grade)
│   ├── mixed_clean_arch/        # Minor issues (B/C grade)
│   └── circular_deps/           # Circular dependencies
└── common/                      # Test utilities
```

## Running Tests

### Run all tests
```bash
cargo test --all
```

### Run only scoring unit tests
```bash
cargo test --test test_scoring
```

### Run integration tests
```bash
cargo test --test test_scoring_integration
```

### Run specific test categories
```bash
# Edge cases
cargo test test_grade_boundaries --test test_scoring
cargo test test_extreme_score --test test_scoring
cargo test test_complexity_with_zero --test test_scoring

# Component tests
cargo test test_layer_isolation_component --test test_scoring
cargo test test_circular_deps_component --test test_scoring
cargo test test_complexity_component --test test_scoring
cargo test test_violations_component --test test_scoring

# Consistency tests
cargo test test_scoring_idempotency --test test_scoring
cargo test test_scoring_determinism --test test_scoring

# Integration tests
cargo test test_perfect_mvc_project --test test_scoring_integration
cargo test test_failing_hexagonal --test test_scoring_integration
```

### Run with verbose output
```bash
cargo test --test test_scoring -- --nocapture
```

## Test Coverage

### Current Coverage
- **Overall**: 85%+ on critical modules
- **scoring.rs**: 95%+
- **metrics.rs**: 90%+
- **Unit tests**: 59 passing

### Measuring Coverage

Install tarpaulin:
```bash
cargo install cargo-tarpaulin
```

Run coverage analysis:
```bash
cargo tarpaulin --out Html --output-dir target/coverage
```

View coverage report:
```bash
open target/coverage/index.html
```

## Test Categories

### Phase 1: Edge Cases & Boundary Conditions (10 tests)
Tests for grade boundaries, extreme scores, division by zero protection, and empty project handling.

### Phase 2: Component Isolation Tests (12 tests)
Tests for individual scoring components:
- Layer Isolation (3 tests)
- Circular Dependencies (3 tests)
- Complexity (3 tests)
- Violations (3 tests)

### Phase 3: Integration Tests (4 tests + 4 fixtures)
End-to-end tests with realistic project fixtures:
- Perfect MVC: Expected A grade
- Failing Hexagonal: Expected C/D/F grade with violations
- Mixed Clean Arch: Expected B/C grade
- Circular Deps: Expected <75 score

### Phase 4: Consistency Tests (3 tests)
Tests for idempotency, determinism, and reproducibility.

## Writing New Tests

### Unit Test Template
```rust
#[test]
fn test_feature_name() {
    // Arrange
    let mut result = create_test_result();
    
    // Act
    let score = scoring::calculate(&result);
    
    // Assert
    assert_eq!(score.total, expected_value);
}
```

### Integration Test Template
```rust
#[test]
fn test_fixture_name() {
    let fixture_path = fixture_path("fixture_name");
    let result = analyze_fixture(&fixture_path);
    
    let score = scoring::calculate(&result);
    
    // Assert expectations
    assert!(score.total >= 90);
}
```

## Test Fixtures

### Perfect MVC Project
- **Architecture**: MVC pattern
- **Expected Score**: A (90-100)
- **Characteristics**: Perfect layer isolation, no circular deps, no violations

### Failing Hexagonal Project
- **Architecture**: Hexagonal architecture
- **Expected Score**: C/D/F (<80)
- **Characteristics**: Layer violations (domain → infrastructure)

### Mixed Clean Architecture Project
- **Architecture**: Clean architecture
- **Expected Score**: B/C (70-90)
- **Characteristics**: Some violations, long functions

### Circular Dependencies Project
- **Architecture**: Modular
- **Expected Score**: <75
- **Characteristics**: Intentional circular dependencies

## Best Practices

1. **Run tests before committing**: Always ensure all tests pass
2. **Write tests for new features**: Maintain 85%+ coverage
3. **Use descriptive test names**: `test_<feature>_<scenario>`
4. **Test edge cases**: Division by zero, empty inputs, boundary values
5. **Keep fixtures minimal**: Only include necessary files

## Troubleshooting

### Tests fail with import errors
Ensure all dependencies are installed:
```bash
cargo build
```

### Integration tests fail
Check that fixture files exist:
```bash
find tests/fixtures -type f
```

### Coverage tool not found
Install tarpaulin:
```bash
cargo install cargo-tarpaulin
```
