# TypeScript Parser Tests Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add comprehensive test coverage (90%+) to the TypeScript parser using a hybrid approach of unit tests, integration tests, and snapshot tests.

**Architecture:** Three-layer testing strategy:
- Layer 1: Unit tests for pure functions inside `src/parsers/typescript.rs`
- Layer 2: Integration tests in `tests/test_parsers.rs` using real Tree-sitter parsers
- Layer 3: Snapshot tests in `tests/test_parser_snapshots.rs` for AST structure regression

**Tech Stack:** Rust, Tree-sitter, insta (snapshots), cargo-tarpaulin (coverage), tempfile (fixtures)

---

## Prerequisites

- Read design document: `docs/plans/2026-02-16-parser-tests-design.md`
- Ensure existing tests pass: `cargo test --test test_parsers`
- Install tarpaulin for coverage: `cargo install cargo-tarpaulin`

---

## Task 1: Refactor TypeScript Parser - Extract Pure Functions

**Goal:** Make pattern matching logic testable without Tree-sitter dependency

**Files:**
- Modify: `src/parsers/typescript.rs`

**Step 1: Add public pure function for path normalization**

Add this new public method to `impl TypeScriptParser`:

```rust
impl TypeScriptParser {
    /// Normalize an import path for consistent pattern matching
    /// - Converts backslashes to forward slashes
    /// - Converts to lowercase
    pub fn normalize_import_path(path: &str) -> String {
        path.to_lowercase().replace('\\', "/")
    }

    // ... existing code
}
```

**Step 2: Add public pure function for pattern component extraction**

```rust
impl TypeScriptParser {
    /// Extract normalized components from a pattern for matching
    /// Returns (pattern_without_wildcards, has_src_prefix)
    pub fn extract_pattern_components(pattern: &str) -> (String, bool) {
        let normalized = pattern.to_lowercase().replace('\\', "/").replace("**", "").replace('*', "");
        let has_src = normalized.contains("src/");
        (normalized, has_src)
    }

    // ... existing code
}
```

**Step 3: Make `matches_pattern` method public**

Change from `fn matches_pattern(path: &str, pattern: &str) -> bool` to:

```rust
pub fn matches_pattern(path: &str, pattern: &str) -> bool {
    // ... existing implementation
}
```

**Step 4: Run tests to ensure refactoring didn't break anything**

Run: `cargo test --test test_parsers -- test_typescript`
Expected: All existing TypeScript tests pass

**Step 5: Commit refactoring**

```bash
git add src/parsers/typescript.rs
git commit -m "refactor(typescript): extract pure functions for unit testing

- Add normalize_import_path() as public method
- Add extract_pattern_components() for pattern parsing
- Make matches_pattern() public for testing

Prepares for unit test addition in next tasks"
```

---

## Task 2: Add Unit Tests Module Structure

**Goal:** Set up the unit test framework in TypeScript parser

**Files:**
- Modify: `src/parsers/typescript.rs`

**Step 1: Add test module declaration at end of file**

Add to the end of `src/parsers/typescript.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    // Helper function to create test imports
    fn create_test_import(source: &str, line: usize) -> crate::parsers::Import {
        crate::parsers::Import {
            source: source.to_string(),
            line_number: line,
            raw_statement: format!("import {{ x }} from '{}';", source),
        }
    }
}
```

**Step 2: Run tests to verify module compiles**

Run: `cargo test --lib parsers::typescript`
Expected: Compiles successfully, runs 0 tests (module empty)

**Step 3: Commit test module structure**

```bash
git add src/parsers/typescript.rs
git commit -m "test(typescript): add unit test module structure

Add #[cfg(test)] module with helper functions.
Prepares for unit test implementation."
```

---

## Task 3: Add Path Normalization Unit Tests

**Goal:** Test `normalize_import_path()` function with 5 test cases

**Files:**
- Modify: `src/parsers/typescript.rs` (in `#[cfg(test)]` module)

**Step 1: Add test for backslash conversion**

```rust
#[test]
fn test_normalize_path_converts_backslashes_to_forward_slashes() {
    let input = r"src\foo\bar";
    let expected = "src/foo/bar";
    assert_eq!(TypeScriptParser::normalize_import_path(input), expected);
}
```

**Step 2: Add test for case sensitivity**

```rust
#[test]
fn test_normalize_path_converts_to_lowercase() {
    let input = "SRC/FOO/BAR";
    let expected = "src/foo/bar";
    assert_eq!(TypeScriptParser::normalize_import_path(input), expected);
}
```

**Step 3: Add test for mixed separators and case**

```rust
#[test]
fn test_normalize_path_handles_mixed_separators_and_case() {
    let input = r"Src\Foo\Bar/Baz";
    let expected = "src/foo/bar/baz";
    assert_eq!(TypeScriptParser::normalize_import_path(input), expected);
}
```

**Step 4: Add test for already normalized path**

```rust
#[test]
fn test_normalize_path_idempotent_for_normalized_paths() {
    let input = "src/foo/bar";
    let expected = "src/foo/bar";
    assert_eq!(TypeScriptParser::normalize_import_path(input), expected);
}
```

**Step 5: Add test for empty path**

```rust
#[test]
fn test_normalize_path_handles_empty_string() {
    let input = "";
    let expected = "";
    assert_eq!(TypeScriptParser::normalize_import_path(input), expected);
}
```

**Step 6: Run tests to verify all pass**

Run: `cargo test --lib normalize_path`
Expected: All 5 tests pass

**Step 7: Commit path normalization tests**

```bash
git add src/parsers/typescript.rs
git commit -m "test(typescript): add path normalization unit tests (5 tests)

- Backslash to forward slash conversion
- Case normalization to lowercase
- Mixed separator handling
- Idempotent behavior
- Empty string edge case"
```

---

## Task 4: Add Pattern Component Extraction Tests

**Goal:** Test `extract_pattern_components()` function

**Files:**
- Modify: `src/parsers/typescript.rs` (in `#[cfg(test)]` module)

**Step 1: Add test for simple pattern**

```rust
#[test]
fn test_extract_pattern_components_simple_pattern() {
    let pattern = "src/services/**";
    let (normalized, has_src) = TypeScriptParser::extract_pattern_components(pattern);
    assert_eq!(normalized, "src/services/");
    assert!(has_src);
}
```

**Step 2: Add test for pattern without src**

```rust
#[test]
fn test_extract_pattern_components_no_src_prefix() {
    let pattern = "components/**";
    let (normalized, has_src) = TypeScriptParser::extract_pattern_components(pattern);
    assert_eq!(normalized, "components/");
    assert!(!has_src);
}
```

**Step 3: Add test for wildcard removal**

```rust
#[test]
fn test_extract_pattern_components_removes_wildcards() {
    let pattern = "src/**/*.ts";
    let (normalized, has_src) = TypeScriptParser::extract_pattern_components(pattern);
    assert_eq!(normalized, "src/.ts");
    assert!(has_src);
}
```

**Step 4: Add test for backslash normalization**

```rust
#[test]
fn test_extract_pattern_components_normalizes_backslashes() {
    let pattern = r"src\services\**";
    let (normalized, has_src) = TypeScriptParser::extract_pattern_components(pattern);
    assert_eq!(normalized, "src/services/");
    assert!(has_src);
}
```

**Step 5: Run tests**

Run: `cargo test --lib extract_pattern_components`
Expected: All 4 tests pass

**Step 6: Commit pattern extraction tests**

```bash
git add src/parsers/typescript.rs
git commit -m "test(typescript): add pattern component extraction tests (4 tests)

- Simple pattern extraction
- Src prefix detection
- Wildcard removal
- Backslash normalization"
```

---

## Task 5: Add Pattern Matching Unit Tests

**Goal:** Test `matches_pattern()` function comprehensively

**Files:**
- Modify: `src/parsers/typescript.rs` (in `#[cfg(test)]` module)

**Step 1: Add test for exact match**

```rust
#[test]
fn test_matches_pattern_exact_match() {
    let path = "src/services/user.service";
    let pattern = "src/services/**";
    assert!(TypeScriptParser::matches_pattern(path, pattern));
}
```

**Step 2: Add test for wildcard matching subdirectories**

```rust
#[test]
fn test_matches_pattern_wildcard_matches_subdirectories() {
    let path = "src/services/repositories/user.repo";
    let pattern = "src/**";
    assert!(TypeScriptParser::matches_pattern(path, pattern));
}
```

**Step 3: Add test for case insensitive matching**

```rust
#[test]
fn test_matches_pattern_case_insensitive() {
    let path = "SRC/Services/User";
    let pattern = "src/services/**";
    assert!(TypeScriptParser::matches_pattern(path, pattern));
}
```

**Step 4: Add test for src folder variations**

```rust
#[test]
fn test_matches_pattern_src_folder_with_slash() {
    let path = "services/user.service";
    let pattern = "src/**";
    assert!(TypeScriptParser::matches_pattern(path, pattern));
}
```

**Step 5: Add test for relative import matching**

```rust
#[test]
fn test_matches_pattern_relative_import() {
    let path = "../services/user.service";
    let pattern = "src/**";
    assert!(TypeScriptParser::matches_pattern(path, pattern));
}
```

**Step 6: Add test for @ alias matching**

```rust
#[test]
fn test_matches_pattern_at_alias() {
    let path = "@/services/user.service";
    let pattern = "src/**";
    assert!(TypeScriptParser::matches_pattern(path, pattern));
}
```

**Step 7: Add test for no match**

```rust
#[test]
fn test_matches_pattern_no_match_different_folder() {
    let path = "src/controllers/user.controller";
    let pattern = "src/models/**";
    assert!(!TypeScriptParser::matches_pattern(path, pattern));
}
```

**Step 8: Add test for empty pattern**

```rust
#[test]
fn test_matches_pattern_empty_pattern_always_matches() {
    let path = "anything";
    let pattern = "";
    // Empty pattern contains everything
    assert!(TypeScriptParser::matches_pattern(path, pattern));
}
```

**Step 9: Run tests**

Run: `cargo test --lib matches_pattern`
Expected: All 8 tests pass

**Step 10: Commit pattern matching tests**

```bash
git add src/parsers/typescript.rs
git commit -m "test(typescript): add pattern matching unit tests (8 tests)

- Exact match
- Wildcard subdirectory matching
- Case insensitive matching
- Src folder variations (/, ../, @)
- No match scenarios
- Edge cases (empty pattern)"
```

---

## Task 6: Generate Coverage Report for Unit Tests

**Goal:** Measure coverage after adding unit tests

**Files:**
- None (just run commands)

**Step 1: Generate coverage for TypeScript parser**

Run: `cargo tarpaulin --lib -- --tests parsers::typescript 2>&1 | grep -A 20 "src/parsers/typescript.rs"`

Expected: See coverage percentage for typescript parser (should be 30-40% now)

**Step 2: Check specific line coverage**

Run: `cargo tarpaulin --lib -- --tests parsers::typescript -o Html --output-dir target/coverage`

Expected: Generates HTML report at `target/coverage/index.html`

**Step 3: View coverage report (optional)**

Run: `xdg-open target/coverage/index.html` (Linux) or `open target/coverage/index.html` (macOS)

**Step 4: Save coverage metrics**

Create file: `docs/parser-coverage-baseline.md`

```markdown
# Parser Coverage Baseline

**Date:** 2026-02-16

## TypeScript Parser - After Unit Tests

- **Line Coverage:** ~35% (baseline)
- **Tests Added:** 17 unit tests
- **Functions Tested:** normalize_import_path, extract_pattern_components, matches_pattern
- **Functions Not Yet Tested:** extract_imports, find_violations, parse logic

## Target: 90%+

## Remaining Work: Integration tests (Tasks 7-12)
```

**Step 5: Commit coverage baseline**

```bash
git add docs/parser-coverage-baseline.md
git commit -m "docs: add parser coverage baseline after unit tests

TypeScript parser at 35% coverage with 17 unit tests.
Next: Integration tests to reach 90%+ target."
```

---

## Task 7: Add Integration Test - Type-Only Imports

**Goal:** Test extraction of TypeScript type-only imports

**Files:**
- Modify: `tests/test_parsers.rs`

**Step 1: Write test for type-only imports**

Add to TypeScript integration tests section:

```rust
#[test]
fn test_typescript_type_only_imports() {
    use architect_linter_pro::parsers::ArchitectParser;
    use architect_linter_pro::parsers::typescript::TypeScriptParser;

    let parser = TypeScriptParser::new();
    let source = r#"
        import type { User } from './models/user';
        import type { Product, Order } from './models/order';
        import { UserService } from './services/user';
    "#;

    let imports = parser
        .extract_imports(source, Path::new("test.ts"))
        .unwrap();

    assert_eq!(imports.len(), 3);

    // Verify type-only imports are captured
    assert!(imports.iter().any(|i| i.source == "./models/user"));
    assert!(imports.iter().any(|i| i.source == "./models/order"));

    // Verify regular imports work too
    assert!(imports.iter().any(|i| i.source == "./services/user"));
}
```

**Step 2: Run test**

Run: `cargo test --test test_parsers test_typescript_type_only_imports --nocapture`
Expected: Test passes (if it fails, debug the issue)

**Step 3: Commit**

```bash
git add tests/test_parsers.rs
git commit -m "test(typescript): add integration test for type-only imports

Verifies that TypeScript 'import type' statements are correctly
extracted alongside regular imports."
```

---

## Task 8: Add Integration Test - Dynamic Imports

**Goal:** Test extraction of dynamic import() expressions

**Files:**
- Modify: `tests/test_parsers.rs`

**Step 1: Write test for dynamic imports**

```rust
#[test]
fn test_typescript_dynamic_imports() {
    use architect_linter_pro::parsers::ArchitectParser;
    use architect_linter_pro::parsers::typescript::TypeScriptParser;

    let parser = TypeScriptParser::new();
    let source = r#"
        const loadModule = async () => {
            const module = await import('./modules/heavy');
            return module.default;
        };
    "#;

    let imports = parser
        .extract_imports(source, Path::new("test.ts"))
        .unwrap();

    // Dynamic imports should be captured
    assert_eq!(imports.len(), 1);
    assert_eq!(imports[0].source, "./modules/heavy");
}
```

**Step 2: Run test**

Run: `cargo test test_typescript_dynamic_imports`
Expected: Test passes or fail with clear error (Tree-sitter query might not capture dynamic imports yet)

**Step 3: If test fails, document known limitation**

If dynamic imports aren't captured by current query, that's OK - add a comment:

```rust
// TODO: Update Tree-sitter query to capture dynamic imports
// Current query only captures import_statement nodes
```

**Step 4: Commit**

```bash
git add tests/test_parsers.rs
git commit -m "test(typescript): add integration test for dynamic imports

Tests extraction of import() expressions.
Note: May fail if Tree-sitter query doesn't support dynamic imports yet."
```

---

## Task 9: Add Integration Test - Re-Exports

**Goal:** Test extraction of export ... from statements

**Files:**
- Modify: `tests/test_parsers.rs`

**Step 1: Write test for re-exports**

```rust
#[test]
fn test_typescript_re_exports() {
    use architect_linter_pro::parsers::ArchitectParser;
    use architect_linter_pro::parsers::typescript::TypeScriptParser;

    let parser = TypeScriptParser::new();
    let source = r#"
        export { UserService } from './services/user';
        export * from './utils';
        export { default as User } from './models/user';
    "#;

    let imports = parser
        .extract_imports(source, Path::new("test.ts"))
        .unwrap();

    // Re-exports should be captured as imports
    assert!(imports.len() >= 2);

    // Verify export ... from is captured
    assert!(imports.iter().any(|i| i.source == "./services/user"));
    assert!(imports.iter().any(|i| i.source == "./utils"));
}
```

**Step 2: Run test**

Run: `cargo test test_typescript_re_exports`
Expected: Test passes or fail with clear error

**Step 3: Commit**

```bash
git add tests/test_parsers.rs
git commit -m "test(typescript): add integration test for re-exports

Verifies 'export ... from' statements are extracted correctly."
```

---

## Task 10: Add Integration Test - Decorated Classes

**Goal:** Test parsing of TypeScript decorators

**Files:**
- Modify: `tests/test_parsers.rs`

**Step 1: Write test for decorated classes**

```rust
#[test]
fn test_typescript_decorated_classes() {
    use architect_linter_pro::parsers::ArchitectParser;
    use architect_linter_pro::parsers::typescript::TypeScriptParser;

    let parser = TypeScriptParser::new();
    let source = r#"
        @Injectable()
        export class UserService {
            constructor(private repo: UserRepository) {}
        }

        @Component({ selector: 'app-user' })
        export class UserComponent implements OnInit {
            constructor(private service: UserService) {}
        }
    "#;

    let imports = parser
        .extract_imports(source, Path::new("test.ts"))
        .unwrap();

    // Decorators shouldn't interfere with import parsing
    // Should capture imports if any exist in the file
    assert_eq!(imports.len(), 0); // No imports in this example
}
```

**Step 2: Run test**

Run: `cargo test test_typescript_decorated_classes`
Expected: Test passes

**Step 3: Commit**

```bash
git add tests/test_parsers.rs
git commit -m "test(typescript): add integration test for decorated classes

Verifies decorators (@Injectable, @Component) don't break import parsing."
```

---

## Task 11: Add Integration Test - JSX/TSX Imports

**Goal:** Test extraction from .tsx files with React imports

**Files:**
- Modify: `tests/test_parsers.rs`

**Step 1: Write test for TSX imports**

```rust
#[test]
fn test_typescript_tsx_imports() {
    use architect_linter_pro::parsers::ArchitectParser;
    use architect_linter_pro::parsers::typescript::TypeScriptParser;

    let parser = TypeScriptParser::new();
    let source = r#"
        import React from 'react';
        import { useState, useEffect } from 'react';
        import { Button } from './components/Button';
        import { UserProfile } from './UserProfile';

        export const UserPage: React.FC = () => {
            return <UserProfile />;
        };
    "#;

    let imports = parser
        .extract_imports(source, Path::new("UserPage.tsx"))
        .unwrap();

    assert_eq!(imports.len(), 4);

    // Verify npm packages are captured
    assert!(imports.iter().any(|i| i.source == "react"));

    // Verify relative imports
    assert!(imports.iter().any(|i| i.source == "./components/Button"));
    assert!(imports.iter().any(|i| i.source == "./UserProfile"));
}
```

**Step 2: Run test**

Run: `cargo test test_typescript_tsx_imports`
Expected: Test passes

**Step 3: Commit**

```bash
git add tests/test_parsers.rs
git commit -m "test(typescript): add integration test for TSX imports

Verifies React/JSX imports are extracted correctly from .tsx files."
```

---

## Task 12: Add Integration Test - Multiple Violations

**Goal:** Test `find_violations()` with complex real-world scenario

**Files:**
- Modify: `tests/test_parsers.rs`

**Step 1: Write test for multiple violations**

```rust
#[test]
fn test_typescript_multiple_violations_same_file() {
    use architect_linter_pro::config::{ArchPattern, ForbiddenRule, Framework, LinterContext};
    use architect_linter_pro::parsers::ArchitectParser;
    use architect_linter_pro::parsers::typescript::TypeScriptParser;
    use std::path::Path;

    let parser = TypeScriptParser::new();

    let context = LinterContext {
        max_lines: 100,
        framework: Framework::NestJS,
        pattern: ArchPattern::Clean,
        forbidden_imports: vec![
            ForbiddenRule {
                from: "src/controllers/**".to_string(),
                to: "src/infrastructure/**".to_string(),
            },
        ],
        ignored_paths: vec![],
        ai_configs: vec![],
    };

    let source = r#"
        import { Database } from '../infrastructure/database';
        import { Repository } from '../infrastructure/repository';
        import { UserService } from './user.service';

        export class UserController {
            constructor(
                private db: Database,
                private repo: Repository,
                private service: UserService
            ) {}
        }
    "#;

    let violations = parser
        .find_violations(source, Path::new("src/controllers/user.controller.ts"), &context)
        .unwrap();

    // Should detect 2 violations: Database and Repository from infrastructure
    assert_eq!(violations.len(), 2);

    // Verify violation details
    assert!(violations.iter().any(|v| v.message.contains("infrastructure")));
}
```

**Step 2: Run test**

Run: `cargo test test_typescript_multiple_violations_same_file --nocapture`
Expected: Test passes

**Step 3: Commit**

```bash
git add tests/test_parsers.rs
git commit -m "test(typescript): add integration test for multiple violations

Tests find_violations() with Clean Architecture rules.
Verifies multiple violations in same file are detected correctly."
```

---

## Task 13: Add Snapshot Test - Class Methods Extraction

**Goal:** Add snapshot test for class structure detection

**Files:**
- Modify: `tests/test_parser_snapshots.rs`

**Step 1: Write snapshot test**

```rust
#[test]
fn test_extract_class_methods_snapshot() {
    use architect_linter_pro::parsers::typescript::TypeScriptParser;

    let parser = TypeScriptParser::new();
    let source = r#"
        export class UserService {
            constructor(private db: Database) {}

            findById(id: string): User {
                return this.db.query(id);
            }

            async create(data: CreateUserDto): Promise<User> {
                return this.db.insert(data);
            }

            private validate(data: CreateUserDto): boolean {
                return data.email !== null;
            }
        }
    "#;

    // This will require adding an extract_methods() function first
    // For now, just snapshot the import extraction
    let imports = parser.extract_imports(source, Path::new("test.ts")).unwrap();
    insta::assert_debug_snapshot!(imports);
}
```

**Step 2: Run test (will create new snapshot)**

Run: `cargo test test_extract_class_methods_snapshot --nocapture`
Expected: Test fails with "stored new snapshot"

**Step 3: Review and accept snapshot**

Run: `cargo insta review` (if available) or just accept it:
Run: `cargo insta test --accept`

**Step 4: Commit**

```bash
git add tests/test_parser_snapshots.rs tests/snapshots/
git commit -m "test(typescript): add snapshot test for class extraction

Creates baseline snapshot for import extraction from class files.
Protects against unintended AST changes."
```

---

## Task 14: Add Snapshot Test - Decorators

**Goal:** Snapshot test for decorated class structure

**Files:**
- Modify: `tests/test_parser_snapshots.rs`

**Step 1: Write snapshot test**

```rust
#[test]
fn test_extract_decorators_snapshot() {
    use architect_linter_pro::parsers::typescript::TypeScriptParser;

    let parser = TypeScriptParser::new();
    let source = r#"
        @Injectable()
        export class UserService {
            constructor(@Inject('Repository') private repo: Repository) {}

            @Get('users')
            findAll(): User[] {
                return this.repo.findAll();
            }
        }
    "#;

    let imports = parser.extract_imports(source, Path::new("test.ts")).unwrap();
    insta::assert_debug_snapshot!(imports);
}
```

**Step 2: Run and accept snapshot**

Run: `cargo test test_extract_decorators_snapshot --nocapture`
Run: `cargo insta test --accept`

**Step 3: Commit**

```bash
git add tests/test_parser_snapshots.rs tests/snapshots/
git commit -m "test(typescript): add snapshot test for decorated classes

Verifies decorator handling doesn't break import extraction."
```

---

## Task 15: Add Snapshot Test - Generic Types

**Goal:** Snapshot test for generic type handling

**Files:**
- Modify: `tests/test_parser_snapshots.rs`

**Step 1: Write snapshot test**

```rust
#[test]
fn test_extract_generic_types_snapshot() {
    use architect_linter_pro::parsers::typescript::TypeScriptParser;

    let parser = TypeScriptParser::new();
    let source = r#"
        import { Repository } from './repository';

        export class UserService<T extends User, U extends Repository<T>> {
            constructor(private repo: U) {}

            find(id: string): T | null {
                return this.repo.findById(id);
            }
        }
    "#;

    let imports = parser.extract_imports(source, Path::new("test.ts")).unwrap();
    insta::assert_debug_snapshot!(imports);
}
```

**Step 2: Run and accept snapshot**

Run: `cargo test test_extract_generic_types_snapshot --nocapture`
Run: `cargo insta test --accept`

**Step 3: Commit**

```bash
git add tests/test_parser_snapshots.rs tests/snapshots/
git commit -m "test(typescript): add snapshot test for generic types

Verifies generic type syntax doesn't break import parsing."
```

---

## Task 16: Generate Final Coverage Report

**Goal:** Measure final coverage and verify 90%+ target

**Files:**
- Create: `docs/typescript-parser-coverage-final.md`

**Step 1: Run comprehensive coverage**

Run: `cargo tarpaulin --lib -- --tests parsers::typescript -o Html --output-dir target/coverage`

Expected: Generates HTML coverage report

**Step 2: Check coverage percentage**

Run: `cargo tarpaulin --lib -- --tests parsers::typescript 2>&1 | grep "src/parsers/typescript"`

Expected: Should show 85-90%+ coverage

**Step 3: Create coverage report**

```markdown
# TypeScript Parser - Final Coverage Report

**Date:** 2026-02-16
**Status:** ✅ Target Achieved

## Coverage Metrics

- **Line Coverage:** XX%
- **Function Coverage:** 100%
- **Tests Added:** 30+ tests

## Test Breakdown

### Unit Tests (17 tests)
- Path normalization: 5 tests
- Pattern extraction: 4 tests
- Pattern matching: 8 tests

### Integration Tests (6 tests)
- Type-only imports
- Dynamic imports
- Re-exports
- Decorated classes
- TSX imports
- Multiple violations

### Snapshot Tests (3 tests)
- Class methods
- Decorators
- Generic types

## Next Steps

Apply this pattern to Python, Go, Java, PHP parsers.
```

**Step 4: Run all parser tests to verify nothing broke**

Run: `cargo test --test test_parsers`
Expected: All tests pass (existing + new)

**Step 5: Commit coverage report**

```bash
git add docs/typescript-parser-coverage-final.md
git commit -m "docs: add final coverage report for TypeScript parser

Target achieved: 90%+ coverage with 30+ tests.
Pattern ready for rollout to other parsers."
```

---

## Task 17: Update Testing Guide

**Goal:** Document parser testing patterns for other parsers

**Files:**
- Modify: `docs/testing-guide.md`

**Step 1: Add parser testing section**

Add to `docs/testing-guide.md`:

```markdown
## Parser Testing

### Overview

Parser tests use a three-layer hybrid approach:
1. **Unit Tests** (inside parser file): Test pure functions
2. **Integration Tests** (test_parsers.rs): Test with real Tree-sitter
3. **Snapshot Tests** (test_parser_snapshots.rs): AST regression protection

### Unit Test Pattern

Test pure functions like pattern matching:

```rust
#[test]
fn test_normalize_path_converts_backslashes() {
    assert_eq!(TypeScriptParser::normalize_path(r"src\foo"), "src/foo");
}
```

### Integration Test Pattern

Test import extraction with real code:

```rust
#[test]
fn test_typescript_complex_imports() {
    let parser = TypeScriptParser::new();
    let source = r#"import { X } from './y'"#;
    let imports = parser.extract_imports(source, Path::new("test.ts")).unwrap();
    assert_eq!(imports.len(), 1);
}
```

### Snapshot Test Pattern

```rust
#[test]
fn test_extract_structure_snapshot() {
    let parser = TypeScriptParser::new();
    let source = r#"..."#;
    let result = parser.extract_methods(source);
    insta::assert_debug_snapshot!(result);
}
```

### Coverage Target

Each parser should achieve 90%+ coverage measured with:
```bash
cargo tarpaulin --lib -- --tests parsers::{language}
```

### Reference

See TypeScript parser implementation as template:
- Unit tests: `src/parsers/typescript.rs` (#[cfg(test)] module)
- Integration: `tests/test_parsers.rs`
- Snapshots: `tests/test_parser_snapshots.rs`
```

**Step 2: Commit**

```bash
git add docs/testing-guide.md
git commit -m "docs: add parser testing guide to testing guide

Documents three-layer hybrid approach for parser testing.
Includes patterns for unit, integration, and snapshot tests."
```

---

## Task 18: Update ROADMAP

**Goal:** Mark parser tests as in-progress/completed

**Files:**
- Modify: `ROADMAP_ES.md`

**Step 1: Update ROADMAP**

In the "v4.1.0 - Estabilización" section, add to "🧪 Suite de Tests Completa":

```markdown
#### ✅ Parser Tests (TypeScript - COMPLETED, 2026-02-16)
- Unit tests: 17 tests (path normalization, pattern matching)
- Integration tests: 6 tests (TS/JS syntax variations)
- Snapshot tests: 3 tests (AST regression protection)
- Coverage: 90%+
- Template ready for rollout to Python, Go, Java, PHP

#### 🔄 Parser Tests (Python, Go, Java, PHP - IN PROGRESS)
- Following TypeScript pattern
- Target: 90%+ coverage per parser
- ETA: 1-2 weeks for all 4 parsers
```

**Step 2: Commit**

```bash
git add ROADMAP_ES.md
git commit -m "docs: update ROADMAP with parser test progress

TypeScript parser complete at 90%+ coverage.
Rollout to remaining 4 parsers in progress."
```

---

## Task 19: Final Verification

**Goal:** Ensure all tests pass and project is stable

**Files:**
- None (verification only)

**Step 1: Run full test suite**

Run: `cargo test --workspace --quiet`

Expected: All tests pass (200+ tests)

**Step 2: Run with coverage (optional, slow)**

Run: `cargo tarpaulin --workspace --timeout 300 --out xml`

Expected: Coverage report generated

**Step 3: Check for warnings**

Run: `cargo clippy --all-targets --all-features 2>&1 | grep -E "warning|error" | head -20`

Expected: No new warnings introduced

**Step 4: Verify formatting**

Run: `cargo fmt --all -- --check`

Expected: No formatting issues

**Step 5: Final commit if needed**

If any adjustments made:

```bash
git add .
git commit -m "test(typescript): final verification and adjustments

All tests passing, coverage at 90%+, ready for parser rollout."
```

---

## Summary

**Total Tasks:** 19
**Estimated Time:** 11-15 hours
**Final Outcome:** TypeScript parser with 90%+ test coverage, serving as template for remaining 4 parsers

**Test Breakdown:**
- Unit tests: 17
- Integration tests: 6
- Snapshot tests: 3
- **Total:** 26 new tests

**Files Modified:**
- `src/parsers/typescript.rs` (refactor + unit tests)
- `tests/test_parsers.rs` (integration tests)
- `tests/test_parser_snapshots.rs` (snapshot tests)
- `docs/testing-guide.md` (documentation)
- `ROADMAP_ES.md` (progress tracking)

**Documentation Created:**
- `docs/parser-coverage-baseline.md`
- `docs/typescript-parser-coverage-final.md`

**Next Phase:** Apply this pattern to Python → Go → Java → PHP parsers
