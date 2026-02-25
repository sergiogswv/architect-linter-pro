# Parser Tests Design - Comprehensive Test Suite for Multi-Language Parsers

**Date:** 2026-02-16
**Author:** Claude (AI Assistant)
**Status:** Approved
**Related:** ROADMAP v4.1.0 - Priority: Core Critical

## Overview

This document outlines the comprehensive testing strategy for the parser modules to achieve 90%+ code coverage while ensuring functional correctness and regression prevention.

**Goal:** Add rigorous tests for all 5 parsers (TypeScript, Python, Go, Java, PHP) with a hybrid approach combining unit, integration, and snapshot tests.

---

## Architecture: Three-Layer Testing Strategy

### Layer 1: Unit Tests (Inside Each Parser)

**Location:** `src/parsers/{parser}.rs` with `#[cfg(test)]` modules
**Purpose:** Test pure functions, business logic, no external dependencies
**Speed:** Very fast (<1ms per test)
**Coverage Target:** 80-90% of parser logic

**What to Test:**
- Pattern matching logic
- Path normalization
- String manipulation functions
- Validation logic
- Edge cases in isolation

**Example Structure:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_path_removes_dots() {
        assert_eq!(TypeScriptParser::normalize_path("../foo/bar"), "foo/bar");
    }

    #[test]
    fn test_matches_pattern_with_wildcard() {
        assert!(TypeScriptParser::matches_pattern("src/foo/bar", "src/**"));
    }
}
```

### Layer 2: Integration Tests (Consolidated)

**Location:** `tests/test_parsers.rs` (amplify existing 36 tests)
**Purpose:** Test end-to-end flows with real Tree-sitter parsers
**Speed:** Moderate (10-50ms per test)
**Coverage Target:** Real-world scenarios

**What to Test:**
- `extract_imports()` with real source code
- `find_violations()` with complex scenarios
- Language-specific syntax features
- Multi-file scenarios
- Parser error handling

**Example Structure:**
```rust
#[test]
fn test_typescript_extract_imports_from_decorated_class() {
    let parser = TypeScriptParser::new();
    let source = r#"
        @Injectable()
        import { Service } from './service';
    "#;
    let imports = parser.extract_imports(source, Path::new("test.ts")).unwrap();
    assert_eq!(imports.len(), 1);
}
```

### Layer 3: Snapshot Tests

**Location:** `tests/test_parser_snapshots.rs` (amplify existing)
**Purpose:** Detect unintended AST changes, verify parser output structure
**Tool:** `insta` crate
**Speed:** Fast (after initial snapshot creation)

**What to Test:**
- Function extraction patterns
- Class structure detection
- Import/export AST nodes
- Decorator handling

**Example Structure:**
```rust
#[test]
fn test_extract_function_names_snapshot() {
    let parser = TypeScriptParser::new();
    let source = r#"
        export function foo() {}
        export class Bar { constructor() {} }
    "#;
    let result = parser.extract_functions(source);
    insta::assert_debug_snapshot!(result);
}
```

---

## Implementation Plan: TypeScript Parser (Pilot)

### Phase 1: Extract Pure Functions (Refactoring)

**Goal:** Make parser logic testable without Tree-sitter

**Actions:**
1. Extract `matches_pattern()` to public method (already exists)
2. Create `normalize_import_path()` as public pure function
3. Create `extract_pattern_components()` for pattern parsing
4. Add helper functions for path manipulation

**Files:**
- `src/parsers/typescript.rs` (refactor)

**Deliverable:**
- 3-4 new public pure functions
- Existing logic preserved, just exposed

### Phase 2: Unit Tests for Pure Functions

**Goal:** 15-20 unit tests in TypeScript parser

**Categories:**

#### Path Normalization (5 tests)
```rust
- test_normalize_path_backslashes
- test_normalize_path_relative_dots
- test_normalize_path_mixed_separators
- test_normalize_path_case_insensitive
- test_normalize_path_empty_components
```

#### Pattern Matching (8 tests)
```rust
- test_matches_pattern_exact_match
- test_matches_pattern_with_wildcard
- test_matches_pattern_with_double_wildcard
- test_matches_pattern_case_insensitive
- test_matches_pattern_src_folder
- test_matches_pattern_at_alias
- test_matches_pattern_relative_import
- test_matches_pattern_no_match
```

#### Edge Cases (2-3 tests)
```rust
- test_matches_pattern_empty_string
- test_matches_pattern_special_characters
- test_matches_pattern_very_long_pattern
```

**Files:**
- `src/parsers/typescript.rs` (add `#[cfg(test)]` module)

**Deliverable:**
- 15-20 unit tests
- 80-90% coverage of pure functions

### Phase 3: Integration Tests Expansion

**Goal:** Expand from 6 to 15 TypeScript tests in `tests/test_parsers.rs`

**New Test Scenarios:**

#### Import Statement Variations (5 tests)
```rust
- test_typescript_type_only_imports
- test_typescript_dynamic_imports
- test_typescript_re_exports
- test_typescript_side_effect_imports
- test_typescript_import_with_type_assertions
```

#### Syntax Features (4 tests)
```rust
- test_typescript_tsx_imports
- test_typescript_decorated_classes
- test_typescript_generic_imports
- test_typescript_namespace_imports
```

#### Complex Scenarios (3 tests)
```rust
- test_typescript_multiple_violations_same_file
- test_typescript_import_from_node_modules_ignored
- test_typescript_very_long_import_list
```

**Files:**
- `tests/test_parsers.rs` (amplify TypeScript section)

**Deliverable:**
- 9 new integration tests
- Coverage of real-world TS/JS patterns

### Phase 4: Snapshot Tests Expansion

**Goal:** 3-5 new snapshots for TypeScript

**Snapshots:**
```rust
- test_extract_class_methods_snapshot (new)
- test_extract_decorators_snapshot (new)
- test_extract_generic_types_snapshot (new)
```

**Files:**
- `tests/test_parser_snapshots.rs` (amplify)
- `tests/snapshots/*.snap` (new snapshot files)

**Deliverable:**
- 3-5 new snapshots
- Regression protection for AST structure

---

## Success Criteria

### Coverage Metrics (TypeScript Parser)

| Metric Type | Target | Measurement |
|-------------|--------|-------------|
| **Line Coverage** | 90%+ | `cargo tarpaulin --lib -- -p parsers::typescript` |
| **Function Coverage** | 100% | All public functions tested |
| **Branch Coverage** | 85%+ | Conditionals tested |

### Functional Checklist

**Must Have:**
- ✅ All public methods have tests
- ✅ Pattern matching edge cases covered
- ✅ Path normalization variants tested
- ✅ Real import syntax variations tested
- ✅ Error handling scenarios tested
- ✅ Snapshot tests for AST changes

**Should Have:**
- ⚠️ Performance benchmarks (parse time)
- ⚠️ Memory usage tests (large files)
- ⚠️ Concurrent access tests (Mutex handling)

### Regression Prevention

**Tracked Scenarios:**
- Pattern matching changes don't break existing rules
- Path normalization handles all OS separators
- Import extraction handles new TS/JS syntax
- Tree-sitter version changes detected via snapshots

---

## Timeline Estimates

### TypeScript Parser (Pilot)
- **Phase 1** (Refactoring): 2-3 hours
- **Phase 2** (Unit Tests): 3-4 hours
- **Phase 3** (Integration Tests): 4-5 hours
- **Phase 4** (Snapshot Tests): 2-3 hours

**Total:** 11-15 hours for TypeScript parser

### Remaining Parsers (After TypeScript Template)

Based on TypeScript experience:
- **Python Parser:** 8-10 hours (simpler than TS)
- **Go Parser:** 6-8 hours (straightforward syntax)
- **Java Parser:** 8-10 hours (moderate complexity)
- **PHP Parser:** 6-8 hours (similar to Python)

**Total All Parsers:** 40-50 hours

---

## Rollout Strategy

### Step 1: TypeScript Pilot
1. Implement Phase 1-4 for TypeScript
2. Run full test suite and measure coverage
3. Adjust approach if needed based on learnings
4. Document patterns and anti-patterns discovered

### Step 2: Template Extraction
1. Extract TypeScript approach into reusable patterns
2. Create `tests/parser_test_helpers.rs` with shared utilities
3. Document test-writing guidelines in `docs/testing-guide.md`

### Step 3: Batch Implementation
1. Python → Go → Java → PHP (in that order, complexity-based)
2. Each parser uses patterns from TypeScript pilot
3. Continuous integration: ensure each parser passes before starting next

### Step 4: Validation
1. Full test suite run with coverage
2. Cross-parser integration tests
3. Performance benchmarks
4. Documentation updates

---

## Testing Guidelines

### Naming Conventions

**Unit Tests:**
```rust
test_{function}_{scenario}_{expected}
// Examples:
test_normalize_path_backslash_converted_to_forward_slash
test_matches_pattern_wildcard_matches_subdirectories
```

**Integration Tests:**
```rust
test_{language}_{feature}_scenario
// Examples:
test_typescript_type_only_imports_extracted
test_python_relative_imports_from_subdirectory
```

**Snapshot Tests:**
```rust
test_{language}_extract_{structure}_snapshot
// Examples:
test_typescript_extract_function_names_snapshot
test_python_extract_class_definitions_snapshot
```

### Test Data Management

**Small Code Snippets:** Inline in test
```rust
let source = r#"import { foo } from './bar'"#;
```

**Medium Snippets:** Constants at top of test module
```rust
const COMPLEX_TYPESCRIPT_CLASS: &str = r#"
    @Injectable()
    export class UserService { ... }
"#;
```

**Large/Complex:** Fixtures in `tests/fixtures/parsers/{lang}/`
```
tests/fixtures/parsers/typescript/
  ├── decorated-class.ts
  ├── generic-class.ts
  └── react-component.tsx
```

---

## Anti-Patterns to Avoid

### ❌ Don't Test Tree-sitter Internals
```rust
// Bad: Tests implementation details of Tree-sitter
#[test]
fn test_query_cursor_position() {
    let cursor = QueryCursor::new();
    // Tests Tree-sitter, not our code
}
```

### ✅ Do Test Our Logic
```rust
// Good: Tests our pattern matching logic
#[test]
fn test_matches_pattern_with_ts_wildcard() {
    assert!(TypeScriptParser::matches_pattern("src/foo", "src/**"));
}
```

### ❌ Don't Test Invalid Syntax Exhaustively
```rust
// Bad: Too many edge cases for invalid syntax
#[test]
fn test_parser_handles_unclosed_bracket() { }
#[test]
fn test_parser_handles_missing_comma() { }
// ... 20 more invalid syntax tests
```

### ✅ Do Focus on Our Error Messages
```rust
// Good: Test our error handling
#[test]
fn test_extract_imports_returns_error_on_syntax_failure() {
    let result = parser.extract_imports(broken_code, path);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Failed to parse"));
}
```

---

## Dependencies

### Required (Already Present)
- ✅ `tree_sitter` - Parser engine
- ✅ `tree_sitter_typescript` - TS/JS grammar
- ✅ `tree_sitter_python` - Python grammar
- ✅ `tree_sitter_go` - Go grammar
- ✅ `tree_sitter_java` - Java grammar
- ✅ `tree_sitter_php` - PHP grammar
- ✅ `insta` - Snapshot testing
- ✅ `cargo-tarpaulin` - Coverage reporting

### Optional (Future Enhancement)
- ⚠️ `criterion` - Benchmarking (if performance tests needed)
- ⚠️ `quickcheck` - Property-based testing (if beneficial)

---

## Documentation Updates

### Files to Update

1. **`docs/testing-guide.md`** - Add parser testing section
2. **`ROADMAP_ES.md`** - Mark parser tests as in-progress
3. **`tests/README.md`** - Document parser test organization

### New Documentation

1. **`docs/parser-testing-patterns.md`** - Patterns learned from TypeScript pilot
2. **`docs/parser-coverage-report.md`** - Coverage metrics per parser

---

## Risk Mitigation

### Risk 1: Tree-sitter Version Incompatibility
**Mitigation:** Snapshot tests will catch AST changes
**Action:** Pin Tree-sitter versions in Cargo.lock

### Risk 2: Tests Too Slow
**Mitigation:** Separate unit tests (fast) from integration (slower)
**Action:** Use `cargo test --lib` for fast feedback during development

### Risk 3: Test Maintenance Burden
**Mitigation:** Focus on pure functions and snapshots
**Action:** Document test data organization clearly

### Risk 4: Incomplete Coverage
**Mitigation:** Continuous coverage monitoring in CI
**Action:** Fail CI if coverage drops below 85%

---

## Success Metrics

### Quantitative
- [ ] TypeScript parser: 90%+ coverage
- [ ] All parsers: 85%+ average coverage
- [ ] 150+ new tests across all parsers
- [ ] All tests pass in <30 seconds

### Qualitative
- [ ] Confidence to refactor parser code
- [ ] Caught at least 1 real bug during testing
- [ ] Easy to add new parser following patterns
- [ ] Team understands test organization

---

## Next Steps

1. **Immediate:** Create implementation plan using `writing-plans` skill
2. **This Week:** Implement TypeScript parser pilot (Phases 1-4)
3. **Next Sprint:** Roll out to Python and Go parsers
4. **Following:** Complete Java and PHP parsers
5. **Final:** Full test suite with coverage reports

---

**Appendix: References**

- [Tree-sitter Documentation](https://tree-sitter.github.io/tree-sitter/)
- [Insta Snapshot Testing](https://insta.rs/)
- [Rust Testing Guidelines](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Existing Parser Tests](../../tests/test_parsers.rs)
- [Testing Guide](../../docs/testing-guide.md)
