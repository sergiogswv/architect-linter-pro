# Test Fixtures for Architect Linter Pro v4.0

This directory contains test fixtures (sample projects) for validating the Health Score system and other features.

## Fixtures Overview

### 1. `perfect_project/` - Expected Score: **A (100)**

**Description:** A clean, well-architected project with no violations.

**Characteristics:**
- ✅ No forbidden imports
- ✅ No circular dependencies
- ✅ All functions under 50 lines
- ✅ Clean MVC architecture

**Files:**
- `user.controller.ts`, `user.service.ts`
- `product.controller.ts`, `product.service.ts`

**Expected Results:**
```
Layer Isolation:  100 ✅
Circular Deps:    100 ✅
Complexity:       100 ✅
Violations:       100 ✅
-----------------------
Total Score: A (100)
```

---

### 2. `circular_deps/` - Expected Score: **F (<50)**

**Description:** Project with circular dependencies.

**Characteristics:**
- ❌ Circular dependency: A → B → A
- ❌ Circular dependency: C → D → E → C
- ✅ No forbidden imports
- ✅ Functions are short

**Files:**
- `a.ts` imports `b.ts`
- `b.ts` imports `a.ts` ← Cycle!
- `c.ts` → `d.ts` → `e.ts` → `c.ts` ← Another cycle!

**Expected Results:**
```
Layer Isolation:  100 ✅
Circular Deps:    0   ❌ (2 cycles detected)
Complexity:       100 ✅
Violations:       varies
-----------------------
Total Score: F (25-40)
```

---

### 3. `long_functions/` - Expected Score: **D (60-69)**

**Description:** Project with extremely long functions (>200 lines).

**Characteristics:**
- ✅ No forbidden imports
- ✅ No circular dependencies
- ❌ Multiple functions exceed 200 lines
- High complexity score penalty

**Files:**
- `huge.ts` - Contains 3 functions >200 lines each

**Expected Results:**
```
Layer Isolation:  100 ✅
Circular Deps:    100 ✅
Complexity:       0-20 ❌ (many long functions)
Violations:       varies
-----------------------
Total Score: D (60-69)
```

---

### 4. `forbidden_imports/` - Expected Score: **C (70-79)**

**Description:** Project with layer architecture violations.

**Characteristics:**
- ❌ Controllers import repositories directly (forbidden)
- ✅ No circular dependencies
- ✅ Functions are short

**Forbidden Import Rules:**
- `controller` → `repository` ❌ (should use service)
- `service` → `controller` ❌
- `repository` → `controller` ❌

**Files:**
- `user.controller.ts` - imports `user.repository.ts` ❌
- `product.controller.ts` - imports `product.repository.ts` ❌
- `user.service.ts` - correctly imports `user.repository.ts` ✅

**Expected Results:**
```
Layer Isolation:  30-50 ❌ (multiple violations)
Circular Deps:    100 ✅
Complexity:       100 ✅
Violations:       varies
-----------------------
Total Score: C (70-79)
```

---

### 5. `mixed_issues/` - Expected Score: **F (<30)**

**Description:** Project with ALL types of violations combined.

**Characteristics:**
- ❌ Forbidden imports (controller → repository)
- ❌ Circular dependencies (A → B → C → A)
- ❌ Extremely long functions (>200 lines)
- Multiple violations of all types

**Files:**
- `bad.controller.ts` - Forbidden import + huge function (150+ lines)
- `bad.repository.ts`
- `circular-a.ts` → `circular-b.ts` → `circular-c.ts` → `circular-a.ts` (cycle)

**Expected Results:**
```
Layer Isolation:  0-30  ❌ (forbidden imports)
Circular Deps:    0     ❌ (cycle detected)
Complexity:       0-20  ❌ (long functions)
Violations:       0-30  ❌ (many violations)
-----------------------
Total Score: F (0-30)
```

---

## How to Test Fixtures

### Manual Testing

Test each fixture individually:

```bash
# Build the linter
cargo build --release

# Test perfect project (should get A)
./target/release/architect-linter-pro --path tests/fixtures/perfect_project

# Test circular deps (should detect cycles)
./target/release/architect-linter-pro --path tests/fixtures/circular_deps

# Test long functions (should detect complexity)
./target/release/architect-linter-pro --path tests/fixtures/long_functions

# Test forbidden imports (should detect violations)
./target/release/architect-linter-pro --path tests/fixtures/forbidden_imports

# Test mixed issues (should fail everything)
./target/release/architect-linter-pro --path tests/fixtures/mixed_issues
```

### Automated Testing

Run integration tests (once implemented):

```bash
cargo test --test integration_test
```

---

## Using Fixtures in Tests

Example test code:

```rust
#[test]
fn test_perfect_project_gets_grade_a() {
    let result = run_linter("tests/fixtures/perfect_project");

    assert_eq!(result.health_score.unwrap().grade, HealthGrade::A);
    assert_eq!(result.health_score.unwrap().total, 100);
}

#[test]
fn test_circular_deps_detected() {
    let result = run_linter("tests/fixtures/circular_deps");

    assert!(!result.circular_dependencies.is_empty());
    assert_eq!(result.health_score.unwrap().components.circular_deps, 0);
}
```

---

## Adding New Fixtures

To add a new fixture:

1. Create directory: `tests/fixtures/your_fixture_name/`
2. Add `architect.json` with rules
3. Add `src/*.ts` files with code
4. Document expected behavior in this README
5. Add test case in `tests/integration_test.rs`

---

## Fixture Validation Checklist

Before committing new fixtures:

- [ ] Each fixture has `architect.json`
- [ ] Each fixture has at least one `.ts` file in `src/`
- [ ] Expected score is documented
- [ ] Expected violations are documented
- [ ] Fixture is tested manually
- [ ] Integration test added (if applicable)

---

**Last Updated:** 2026-02-12
