# Design: Scoring Engine Test Suite Enhancement

**Date:** 2026-02-13
**Author:** Sergio Guadarrama + Claude
**Status:** Approved
**Related:** v4.1.0 - Stabilization release

---

## Overview

Comprehensive test suite enhancement for the scoring engine to achieve 85%+ coverage on critical modules, focusing on edge cases, component isolation, integration testing, and consistency validation.

## Goals

- Increase test coverage from ~60-70% to 85%+ on critical modules
- Add 28-35 new tests covering edge cases, components, and integration
- Create 3-4 real project fixtures for integration testing
- Ensure scoring engine reliability and predictability

---

## Architecture

### File Structure

```
tests/
├── test_scoring.rs              # Extended with edge cases & components
│   ├── Edge Cases (new)
│   │   ├── test_score_boundaries()
│   │   ├── test_zero_score()
│   │   ├── test_perfect_score()
│   │   ├── test_division_by_zero_protection()
│   │   └── test_negative_values_handling()
│   ├── Component Tests (new)
│   │   ├── test_layer_isolation_component()
│   │   ├── test_circular_deps_component()
│   │   ├── test_complexity_component()
│   │   └── test_violations_component()
│   └── Consistency Tests (new)
│       ├── test_scoring_idempotency()
│       └── test_scoring_determinism()
├── fixtures/                     # New directory
│   ├── perfect_mvc_project/
│   │   ├── architect.json
│   │   └── src/
│   │       ├── models/user.model.ts
│   │       ├── views/user.view.ts
│   │       └── controllers/user.controller.ts
│   ├── failing_hexagonal/
│   │   ├── architect.json
│   │   └── src/
│   │       ├── domain/user.entity.ts
│   │       ├── infrastructure/user.repo.ts
│   │       └── application/user.service.ts
│   ├── mixed_clean_arch/
│   │   ├── architect.json
│   │   └── src/
│   │       ├── entities/
│   │       ├── usecases/
│   │       └── adapters/
│   └── circular_deps/
│       ├── architect.json
│       └── src/
│           ├── module_a.ts
│           └── module_b.ts
└── test_scoring_integration.rs   # New file
    ├── test_perfect_mvc_project()
    ├── test_failing_hexagonal_with_violations()
    ├── test_mixed_clean_arch_partial_score()
    └── test_scoring_with_circular_dependencies()
```

---

## Implementation Phases

### Phase 1: Edge Cases & Boundary Conditions (30 min)

**Priority:** High
**Tests:** 8-10 new tests

Focus on testing grade boundaries and extreme values:

- Grade boundary tests (A/B at 90, B/C at 80, C/D at 70, D/F at 60)
- Extreme scores (0, 100, negative values, >100 values)
- Division by zero protection (0 functions, 0 imports)
- Empty project handling

**Key Tests:**
```rust
test_grade_boundaries()           // Test all grade transition points
test_extreme_scores()             // Test 0, 100, -1, 101
test_complexity_with_zero_functions()  // Division by zero protection
test_layer_isolation_no_imports() // Edge case: no imports at all
```

---

### Phase 2: Individual Component Testing (1 hour)

**Priority:** High
**Tests:** 12-15 new tests

Test each health score component in isolation:

#### Layer Isolation Component
- Perfect score: 0 violations out of 100 imports
- Warning level: 10 violations (90% clean)
- Fail level: 30+ violations

#### Circular Dependencies Component
- Clean: No cycles → 100 score
- Detected: Any cycle → 0 score (binary)

#### Complexity Component
- Low complexity: 5% long functions → 95+ score
- Medium complexity: 20% long functions → 70-80 score
- High complexity: 40% long functions → <70 score

#### Violations Component
- No violations: 100 score
- Multiple violations: Decreased score based on count

**Key Tests:**
```rust
test_layer_isolation_perfect()
test_layer_isolation_with_violations()
test_circular_deps_clean()
test_circular_deps_detected()
test_complexity_low()
test_complexity_high()
test_violations_none()
test_violations_multiple_types()
```

---

### Phase 3: Integration Tests with Real Projects (1.5 hours)

**Priority:** Medium
**Tests:** 5-6 new tests
**Deliverable:** 4 fixture projects

Create realistic project fixtures and test full scoring pipeline:

#### Fixture 1: perfect_mvc_project
- **Architecture:** MVC pattern
- **Expected Score:** 100 (A)
- **Characteristics:**
  - Perfect layer isolation
  - No circular dependencies
  - No long functions
  - Zero violations

#### Fixture 2: failing_hexagonal
- **Architecture:** Hexagonal architecture
- **Expected Score:** <80 (C or lower)
- **Characteristics:**
  - Layer violations (domain → infrastructure)
  - Should detect specific violations

#### Fixture 3: mixed_clean_arch
- **Architecture:** Clean architecture
- **Expected Score:** 70-90 (B or C)
- **Characteristics:**
  - Some violations
  - Maybe 1-2 long functions
  - Partial score

#### Fixture 4: circular_deps
- **Architecture:** Modular
- **Expected Score:** <75
- **Characteristics:**
  - Intentional circular dependencies
  - Circular component should score 0

**Key Tests:**
```rust
test_perfect_mvc_project()
test_failing_hexagonal_with_violations()
test_mixed_clean_arch_partial_score()
test_scoring_with_circular_dependencies()
```

---

### Phase 4: Consistency & Repeatability (30 min)

**Priority:** Medium
**Tests:** 3-4 new tests

Ensure scoring is predictable and deterministic:

#### Idempotency
- Same input → same output (always)
- Run 100 times, all results identical

#### Determinism
- Order-independent scoring
- No random elements in scoring algorithm

#### Reproducibility
- Two identical projects → same score
- Copy of project → same score

**Key Tests:**
```rust
test_scoring_idempotency()         // Same input = same output
test_scoring_determinism()         // 100 runs, all identical
test_scoring_with_identical_projects()  // Copy = same score
```

---

## Success Metrics

### Coverage Targets

| Module | Current | Target | Priority |
|--------|---------|--------|----------|
| scoring.rs | ~70% | 95%+ | High |
| config/loader.rs | ~60% | 85%+ | Medium |
| analyzer/metrics.rs | ~65% | 85%+ | Medium |
| **Overall** | ~65% | 85%+ | - |

### Test Count

| Category | Current | New | Total |
|----------|---------|-----|-------|
| Edge Cases | 5 | 8-10 | 13-15 |
| Component Tests | 15 | 12-15 | 27-30 |
| Integration Tests | 3 | 5-6 | 8-9 |
| Consistency Tests | 0 | 3-4 | 3-4 |
| **Total** | ~146 | 28-35 | 175-185 |

### Quality Gates

- [ ] All tests pass (`cargo test`)
- [ ] No compiler warnings
- [ ] Coverage ≥85% on critical modules
- [ ] Each component has ≥2 test cases
- [ ] All grade boundaries tested
- [ ] Integration tests use realistic fixtures
- [ ] Consistency tests verify idempotency

---

## Timeline

| Phase | Duration | Start | End |
|-------|----------|-------|-----|
| Phase 1: Edge Cases | 30 min | T+0 | T+0.5h |
| Phase 2: Components | 1 hour | T+0.5h | T+1.5h |
| Phase 3: Integration | 1.5 hours | T+1.5h | T+3h |
| Phase 4: Consistency | 30 min | T+3h | T+3.5h |
| **Total** | **3.5 hours** | - | - |

---

## Risks & Mitigation

### Risk 1: Fixture Creation Time
- **Risk:** Creating realistic fixtures takes longer than expected
- **Mitigation:** Start with minimal fixtures, enhance later

### Risk 2: Edge Cases Reveal Bugs
- **Risk:** Tests may expose scoring bugs
- **Mitigation:** Fix bugs as we find them, document edge cases

### Risk 3: Coverage Measurement
- **Risk:** No coverage tool installed (tarpaulin missing)
- **Mitigation:** Install `cargo-tarpaulin` or use `cargo-llvm-cov`

---

## Dependencies

### New Dev Dependencies Needed
- None (all testing tools already in Cargo.toml)

### Tools to Install
```bash
# For coverage measurement (optional but recommended)
cargo install cargo-tarpaulin
# OR
cargo install cargo-llvm-cov
```

---

## Next Steps

1. Invoke writing-plans skill to create detailed implementation plan
2. Set up coverage measurement tool
3. Begin Phase 1: Edge Cases implementation
4. Iterate through all phases
5. Measure final coverage
6. Commit and create PR

---

## Appendix: Test Examples

### Example: Grade Boundary Test
```rust
#[test]
fn test_grade_boundaries() {
    // A/B boundary: 90
    assert_eq!(HealthGrade::from_score(90), HealthGrade::A);
    assert_eq!(HealthGrade::from_score(89), HealthGrade::B);

    // B/C boundary: 80
    assert_eq!(HealthGrade::from_score(80), HealthGrade::B);
    assert_eq!(HealthGrade::from_score(79), HealthGrade::C);

    // C/D boundary: 70
    assert_eq!(HealthGrade::from_score(70), HealthGrade::C);
    assert_eq!(HealthGrade::from_score(69), HealthGrade::D);

    // D/F boundary: 60
    assert_eq!(HealthGrade::from_score(60), HealthGrade::D);
    assert_eq!(HealthGrade::from_score(59), HealthGrade::F);
}
```

### Example: Component Isolation Test
```rust
#[test]
fn test_layer_isolation_with_violations() {
    let mut result = create_test_result();
    result.layer_stats.total_imports = 100;
    result.layer_stats.blocked_violations = 10;

    let score = scoring::calculate_health_score(&mut result);
    let layer_component = &score.components[0];

    // 10 violations out of 100 imports = 90% clean
    assert_eq!(layer_component.score, 90.0);
    assert_eq!(layer_component.status, ComponentStatus::Warning);
}
```

---

**Document Status:** ✅ Approved
**Next Action:** Invoke writing-plans skill
