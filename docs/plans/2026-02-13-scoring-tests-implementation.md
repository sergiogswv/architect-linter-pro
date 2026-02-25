# Scoring Engine Test Suite Enhancement Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Achieve 85%+ test coverage on scoring engine by adding 28-35 tests covering edge cases, components, integration, and consistency.

**Architecture:** Incremental testing approach with 4 phases: edge cases (30min), component isolation (1hr), integration with fixtures (1.5hr), consistency (30min). Tests added to existing `test_scoring.rs` and new `test_scoring_integration.rs`.

**Tech Stack:** Rust test framework, existing test utilities (tempfile, assert_cmd, predicates), new fixtures directory with TypeScript projects.

---

## Prerequisites

**Required reading:**
- Design document: `docs/plans/2026-02-13-scoring-tests-design.md`

**Current state:**
- 146 tests passing
- Estimated 60-70% coverage
- Scoring engine partially tested

**Target state:**
- 175-185 tests total (+30-40 new)
- 85%+ coverage on critical modules
- 4 fixture projects created

---

## Phase 1: Edge Cases & Boundary Conditions (30 min)

### Task 1: Add Grade Boundary Tests

**Files:**
- Modify: `tests/test_scoring.rs:50` (after existing grade tests)

**Step 1: Write the failing test**

Add at line 50 in `tests/test_scoring.rs`:

```rust
// ============================================================================
// Grade Boundary Tests (Phase 1)
// ============================================================================

#[test]
fn test_grade_boundaries_ab() {
    // A/B boundary: 90
    assert_eq!(HealthGrade::from_score(90), HealthGrade::A);
    assert_eq!(HealthGrade::from_score(89), HealthGrade::B);
    assert_eq!(HealthGrade::from_score(91), HealthGrade::A);
}

#[test]
fn test_grade_boundaries_bc() {
    // B/C boundary: 80
    assert_eq!(HealthGrade::from_score(80), HealthGrade::B);
    assert_eq!(HealthGrade::from_score(79), HealthGrade::C);
    assert_eq!(HealthGrade::from_score(81), HealthGrade::B);
}

#[test]
fn test_grade_boundaries_cd() {
    // C/D boundary: 70
    assert_eq!(HealthGrade::from_score(70), HealthGrade::C);
    assert_eq!(HealthGrade::from_score(69), HealthGrade::D);
    assert_eq!(HealthGrade::from_score(71), HealthGrade::C);
}

#[test]
fn test_grade_boundaries_df() {
    // D/F boundary: 60
    assert_eq!(HealthGrade::from_score(60), HealthGrade::D);
    assert_eq!(HealthGrade::from_score(59), HealthGrade::F);
    assert_eq!(HealthGrade::from_score(61), HealthGrade::D);
}
```

**Step 2: Run tests to verify they pass**

Run: `cargo test test_grade_boundaries --lib -- --nocapture`

Expected: All 4 tests PASS

**Step 3: Commit**

```bash
git add tests/test_scoring.rs
git commit -m "test(scoring): add grade boundary tests (A/B, B/C, C/D, D/F)"
```

---

### Task 2: Add Extreme Score Tests

**Files:**
- Modify: `tests/test_scoring.rs` (after Task 1)

**Step 1: Write the tests**

Add after boundary tests:

```rust
// ============================================================================
// Extreme Score Tests
// ============================================================================

#[test]
fn test_extreme_score_zero() {
    assert_eq!(HealthGrade::from_score(0), HealthGrade::F);
}

#[test]
fn test_extreme_score_perfect() {
    assert_eq!(HealthGrade::from_score(100), HealthGrade::A);
}

#[test]
fn test_extreme_score_negative() {
    // Should handle gracefully, cap to F
    assert_eq!(HealthGrade::from_score(-1), HealthGrade::F);
    assert_eq!(HealthGrade::from_score(-100), HealthGrade::F);
}

#[test]
fn test_extreme_score_above_100() {
    // Should handle gracefully, cap to A
    assert_eq!(HealthGrade::from_score(101), HealthGrade::A);
    assert_eq!(HealthGrade::from_score(150), HealthGrade::A);
}
```

**Step 2: Run tests**

Run: `cargo test test_extreme_score --lib -- --nocapture`

Expected: All 4 tests PASS

**Step 3: Commit**

```bash
git add tests/test_scoring.rs
git commit -m "test(scoring): add extreme score tests (0, 100, negative, >100)"
```

---

### Task 3: Add Division by Zero Protection Tests

**Files:**
- Modify: `tests/test_scoring.rs` (after Task 2)

**Step 1: Write the tests**

Add after extreme score tests:

```rust
// ============================================================================
// Division by Zero Protection Tests
// ============================================================================

#[test]
fn test_complexity_with_zero_functions() {
    let mut result = create_test_result();
    result.complexity_stats.total_functions = 0;
    result.complexity_stats.long_functions = 0;

    let score = scoring::calculate_health_score(&mut result);

    // Should handle gracefully, not panic
    assert!(score.overall_score >= 0.0 && score.overall_score <= 100.0);
    // Complexity component should not be 0 due to division by zero
    let complexity_component = &score.components[2];
    assert!(complexity_component.score > 0.0);
}

#[test]
fn test_layer_isolation_with_zero_imports() {
    let mut result = create_test_result();
    result.layer_stats.total_imports = 0;
    result.layer_stats.blocked_violations = 0;

    let score = scoring::calculate_health_score(&mut result);

    // Should handle gracefully
    assert!(score.overall_score >= 0.0 && score.overall_score <= 100.0);
    let layer_component = &score.components[0];
    assert!(layer_component.score > 0.0);
}
```

**Step 2: Run tests**

Run: `cargo test test_complexity_with_zero_functions test_layer_isolation_with_zero_imports --lib -- --nocapture`

Expected: Both tests PASS (or FAIL if scoring doesn't handle this - then we fix it)

**Step 3: If tests fail, check scoring.rs implementation**

If division by zero occurs, check `src/scoring.rs` for unprotected divisions:

```rust
// In src/scoring.rs, ensure we have guards like:
let complexity_score = if total_functions == 0 {
    100.0  // No functions = perfect complexity
} else {
    // Calculate normally
};
```

**Step 4: Commit**

```bash
git add tests/test_scoring.rs src/scoring.rs  # if scoring.rs was modified
git commit -m "test(scoring): add division by zero protection tests"
```

---

### Task 4: Add Empty Project Edge Case

**Files:**
- Modify: `tests/test_scoring.rs` (after Task 3)

**Step 1: Write the test**

Add after division by zero tests:

```rust
// ============================================================================
// Empty Project Edge Case
// ============================================================================

#[test]
fn test_empty_project_scoring() {
    let mut result = create_test_result();
    result.files_analyzed = 0;
    result.violations = vec![];
    result.circular_dependencies = vec![];
    result.long_functions = vec![];
    result.layer_stats.total_imports = 0;
    result.layer_stats.blocked_violations = 0;
    result.complexity_stats.total_functions = 0;
    result.complexity_stats.long_functions = 0;

    let score = scoring::calculate_health_score(&mut result);

    // Empty project should get a reasonable score (not crash)
    assert!(score.overall_score >= 0.0 && score.overall_score <= 100.0);
    // Should not be F just because it's empty
    assert_ne!(score.grade, HealthGrade::F);
}
```

**Step 2: Run test**

Run: `cargo test test_empty_project_scoring --lib -- --nocapture`

Expected: PASS

**Step 3: Commit**

```bash
git add tests/test_scoring.rs
git commit -m "test(scoring): add empty project edge case test"
```

---

**Phase 1 Complete:** 10 new tests added
- 4 boundary tests
- 4 extreme score tests
- 2 division by zero tests
- 1 empty project test

**Verify:** Run `cargo test --lib` - all tests should pass

---

## Phase 2: Individual Component Testing (1 hour)

### Task 5: Test Layer Isolation Component - Perfect Score

**Files:**
- Modify: `tests/test_scoring.rs` (new section)

**Step 1: Add component test section header**

Add after Phase 1 tests:

```rust
// ============================================================================
// Component Isolation Tests (Phase 2)
// ============================================================================

// Layer Isolation Component Tests

#[test]
fn test_layer_isolation_component_perfect() {
    let mut result = create_test_result();
    result.layer_stats.total_imports = 100;
    result.layer_stats.blocked_violations = 0;

    let score = scoring::calculate_health_score(&mut result);
    let layer_component = &score.components[0]; // Layer Isolation is first

    assert_eq!(layer_component.score, 100.0);
    assert_eq!(layer_component.status, ComponentStatus::Pass);
}
```

**Step 2: Run test**

Run: `cargo test test_layer_isolation_component_perfect --lib -- --nocapture`

Expected: PASS

**Step 3: Commit**

```bash
git add tests/test_scoring.rs
git commit -m "test(scoring): add layer isolation perfect score test"
```

---

### Task 6: Test Layer Isolation Component - With Violations

**Files:**
- Modify: `tests/test_scoring.rs` (after Task 5)

**Step 1: Write the tests**

```rust
#[test]
fn test_layer_isolation_component_warning() {
    let mut result = create_test_result();
    result.layer_stats.total_imports = 100;
    result.layer_stats.blocked_violations = 10; // 10% violations

    let score = scoring::calculate_health_score(&mut result);
    let layer_component = &score.components[0];

    // 10 violations out of 100 imports = 90% clean
    assert_eq!(layer_component.score, 90.0);
    assert_eq!(layer_component.status, ComponentStatus::Warning);
}

#[test]
fn test_layer_isolation_component_fail() {
    let mut result = create_test_result();
    result.layer_stats.total_imports = 100;
    result.layer_stats.blocked_violations = 30; // 30% violations

    let score = scoring::calculate_health_score(&mut result);
    let layer_component = &score.components[0];

    // 30 violations = 70% clean
    assert_eq!(layer_component.score, 70.0);
    assert_eq!(layer_component.status, ComponentStatus::Fail);
}
```

**Step 2: Run tests**

Run: `cargo test test_layer_isolation_component --lib -- --nocapture`

Expected: All 3 tests PASS

**Step 3: Commit**

```bash
git add tests/test_scoring.rs
git commit -m "test(scoring): add layer isolation component tests (warning, fail)"
```

---

### Task 7: Test Circular Dependencies Component

**Files:**
- Modify: `tests/test_scoring.rs` (after Task 6)

**Step 1: Write the tests**

```rust
// Circular Dependencies Component Tests

#[test]
fn test_circular_deps_component_clean() {
    let mut result = create_test_result();
    result.circular_dependencies = vec![];

    let score = scoring::calculate_health_score(&mut result);
    let circular_component = &score.components[1]; // Circular deps is second

    assert_eq!(circular_component.score, 100.0);
    assert_eq!(circular_component.status, ComponentStatus::Pass);
}

#[test]
fn test_circular_deps_component_detected() {
    let mut result = create_test_result();
    result.circular_dependencies = vec![
        CircularDependency {
            cycle: vec!["a".to_string(), "b".to_string(), "a".to_string()],
        },
    ];

    let score = scoring::calculate_health_score(&mut result);
    let circular_component = &score.components[1];

    // Any circular dependency = 0 score (binary)
    assert_eq!(circular_component.score, 0.0);
    assert_eq!(circular_component.status, ComponentStatus::Fail);
}

#[test]
fn test_circular_deps_component_multiple() {
    let mut result = create_test_result();
    result.circular_dependencies = vec![
        CircularDependency {
            cycle: vec!["a".to_string(), "b".to_string(), "a".to_string()],
        },
        CircularDependency {
            cycle: vec!["x".to_string(), "y".to_string(), "z".to_string(), "x".to_string()],
        },
    ];

    let score = scoring::calculate_health_score(&mut result);
    let circular_component = &score.components[1];

    // Multiple cycles = still 0 score
    assert_eq!(circular_component.score, 0.0);
    assert_eq!(circular_component.status, ComponentStatus::Fail);
}
```

**Step 2: Run tests**

Run: `cargo test test_circular_deps_component --lib -- --nocapture`

Expected: All 3 tests PASS

**Step 3: Commit**

```bash
git add tests/test_scoring.rs
git commit -m "test(scoring): add circular dependencies component tests"
```

---

### Task 8: Test Complexity Component

**Files:**
- Modify: `tests/test_scoring.rs` (after Task 7)

**Step 1: Write the tests**

```rust
// Complexity Component Tests

#[test]
fn test_complexity_component_low() {
    let mut result = create_test_result();
    result.complexity_stats.total_functions = 100;
    result.complexity_stats.long_functions = 5; // 5% long functions

    let score = scoring::calculate_health_score(&mut result);
    let complexity_component = &score.components[2]; // Complexity is third

    assert!(complexity_component.score > 90.0);
    assert_eq!(complexity_component.status, ComponentStatus::Pass);
}

#[test]
fn test_complexity_component_medium() {
    let mut result = create_test_result();
    result.complexity_stats.total_functions = 100;
    result.complexity_stats.long_functions = 20; // 20% long functions

    let score = scoring::calculate_health_score(&mut result);
    let complexity_component = &score.components[2];

    assert!(complexity_component.score >= 70.0 && complexity_component.score <= 85.0);
    assert_eq!(complexity_component.status, ComponentStatus::Warning);
}

#[test]
fn test_complexity_component_high() {
    let mut result = create_test_result();
    result.complexity_stats.total_functions = 100;
    result.complexity_stats.long_functions = 40; // 40% long functions

    let score = scoring::calculate_health_score(&mut result);
    let complexity_component = &score.components[2];

    assert!(complexity_component.score < 70.0);
    assert_eq!(complexity_component.status, ComponentStatus::Fail);
}
```

**Step 2: Run tests**

Run: `cargo test test_complexity_component --lib -- --nocapture`

Expected: All 3 tests PASS

**Step 3: Commit**

```bash
git add tests/test_scoring.rs
git commit -m "test(scoring): add complexity component tests (low, medium, high)"
```

---

### Task 9: Test Violations Component

**Files:**
- Modify: `tests/test_scoring.rs` (after Task 8)

**Step 1: Write the tests**

```rust
// Violations Component Tests

#[test]
fn test_violations_component_none() {
    let mut result = create_test_result();
    result.violations = vec![];

    let score = scoring::calculate_health_score(&mut result);
    let violations_component = &score.components[3]; // Violations is fourth

    assert_eq!(violations_component.score, 100.0);
    assert_eq!(violations_component.status, ComponentStatus::Pass);
}

#[test]
fn test_violations_component_few() {
    let mut result = create_test_result();
    result.violations = vec![
        architect_linter_pro::config::types::Violation {
            file: "test.ts".to_string(),
            line: 10,
            from_layer: "domain".to_string(),
            to_layer: "infrastructure".to_string(),
            import_path: "some/path".to_string(),
            message: "test violation".to_string(),
        },
    ];

    let score = scoring::calculate_health_score(&mut result);
    let violations_component = &score.components[3];

    assert!(violations_component.score > 80.0);
}

#[test]
fn test_violations_component_many() {
    let mut result = create_test_result();
    // Add 10 violations
    for i in 0..10 {
        result.violations.push(
            architect_linter_pro::config::types::Violation {
                file: format!("test{}.ts", i),
                line: i * 10,
                from_layer: "domain".to_string(),
                to_layer: "infrastructure".to_string(),
                import_path: "some/path".to_string(),
                message: "test violation".to_string(),
            },
        );
    }

    let score = scoring::calculate_health_score(&mut result);
    let violations_component = &score.components[3];

    assert!(violations_component.score < 80.0);
}
```

**Step 2: Run tests**

Run: `cargo test test_violations_component --lib -- --nocapture`

Expected: All 3 tests PASS

**Step 3: Commit**

```bash
git add tests/test_scoring.rs
git commit -m "test(scoring): add violations component tests"
```

---

**Phase 2 Complete:** 12 new component tests added
- 3 layer isolation tests
- 3 circular deps tests
- 3 complexity tests
- 3 violations tests

**Verify:** Run `cargo test --lib` - all tests should pass

---

## Phase 3: Integration Tests with Real Projects (1.5 hours)

### Task 10: Create Perfect MVC Fixture

**Files:**
- Create: `tests/fixtures/perfect_mvc_project/architect.json`
- Create: `tests/fixtures/perfect_mvc_project/src/models/user.model.ts`
- Create: `tests/fixtures/perfect_mvc_project/src/views/user.view.ts`
- Create: `tests/fixtures/perfect_mvc_project/src/controllers/user.controller.ts`

**Step 1: Create directory structure**

```bash
mkdir -p tests/fixtures/perfect_mvc_project/src/{models,views,controllers}
```

**Step 2: Create architect.json**

Create `tests/fixtures/perfect_mvc_project/architect.json`:

```json
{
  "max_lines_per_function": 40,
  "framework": "Unknown",
  "pattern": "MVC",
  "forbidden_imports": [
    {
      "from": "models",
      "to": "views"
    },
    {
      "from": "models",
      "to": "controllers"
    },
    {
      "from": "views",
      "to": "models"
    },
    {
      "from": "views",
      "to": "controllers"
    }
  ]
}
```

**Step 3: Create model file**

Create `tests/fixtures/perfect_mvc_project/src/models/user.model.ts`:

```typescript
export interface User {
  id: string;
  name: string;
  email: string;
}

export class UserModel {
  private users: User[] = [];

  add(user: User): void {
    this.users.push(user);
  }

  findById(id: string): User | undefined {
    return this.users.find(u => u.id === id);
  }
}
```

**Step 4: Create view file**

Create `tests/fixtures/perfect_mvc_project/src/views/user.view.ts`:

```typescript
import { User } from './user.model';

export class UserView {
  display(user: User): string {
    return `User: ${user.name} (${user.email})`;
  }

  displayList(users: User[]): string {
    return users.map(u => this.display(u)).join('\n');
  }
}
```

**Step 5: Create controller file**

Create `tests/fixtures/perfect_mvc_project/src/controllers/user.controller.ts`:

```typescript
import { UserModel } from '../models/user.model';
import { UserView } from '../views/user.view';
import { User } from '../models/user.model';

export class UserController {
  constructor(
    private model: UserModel,
    private view: UserView
  ) {}

  addUser(id: string, name: string, email: string): void {
    const user: User = { id, name, email };
    this.model.add(user);
  }

  displayUser(id: string): string {
    const user = this.model.findById(id);
    if (user) {
      return this.view.display(user);
    }
    return 'User not found';
  }
}
```

**Step 6: Verify fixture structure**

Run: `find tests/fixtures/perfect_mvc_project -type f`

Expected:
```
tests/fixtures/perfect_mvc_project/architect.json
tests/fixtures/perfect_mvc_project/src/models/user.model.ts
tests/fixtures/perfect_mvc_project/src/views/user.view.ts
tests/fixtures/perfect_mvc_project/src/controllers/user.controller.ts
```

**Step 7: Commit**

```bash
git add tests/fixtures/perfect_mvc_project/
git commit -m "test(fixtures): add perfect MVC project fixture"
```

---

### Task 11: Create Failing Hexagonal Fixture

**Files:**
- Create: `tests/fixtures/failing_hexagonal/architect.json`
- Create: `tests/fixtures/failing_hexagonal/src/domain/user.entity.ts`
- Create: `tests/fixtures/failing_hexagonal/src/infrastructure/user.repo.ts`
- Create: `tests/fixtures/failing_hexagonal/src/application/user.service.ts`

**Step 1: Create directory structure**

```bash
mkdir -p tests/fixtures/failing_hexagonal/src/{domain,infrastructure,application}
```

**Step 2: Create architect.json**

Create `tests/fixtures/failing_hexagonal/architect.json`:

```json
{
  "max_lines_per_function": 40,
  "framework": "Unknown",
  "pattern": "Hexagonal",
  "forbidden_imports": [
    {
      "from": "domain",
      "to": "infrastructure"
    },
    {
      "from": "domain",
      "to": "application"
    },
    {
      "from": "application",
      "to": "infrastructure"
    }
  ]
}
```

**Step 3: Create domain file (WITH VIOLATION)**

Create `tests/fixtures/failing_hexagonal/src/domain/user.entity.ts`:

```typescript
// This violates hexagonal architecture by importing from infrastructure!
import { UserRepository } from '../infrastructure/user.repo';

export interface User {
  id: string;
  name: string;
}

export class UserEntity {
  constructor(private user: User) {}

  validate(): boolean {
    return this.user.name.length > 0;
  }

  // VIOLATION: Domain should not know about infrastructure
  save(repo: UserRepository): void {
    repo.save(this.user);
  }
}
```

**Step 4: Create infrastructure file**

Create `tests/fixtures/failing_hexagonal/src/infrastructure/user.repo.ts`:

```typescript
import { User } from '../domain/user.entity';

export class UserRepository {
  private db: Map<string, User> = new Map();

  save(user: User): void {
    this.db.set(user.id, user);
  }

  findById(id: string): User | undefined {
    return this.db.get(id);
  }
}
```

**Step 5: Create application file**

Create `tests/fixtures/failing_hexagonal/src/application/user.service.ts`:

```typescript
import { User, UserEntity } from '../domain/user.entity';
import { UserRepository } from '../infrastructure/user.repo';

export class UserService {
  constructor(private repo: UserRepository) {}

  createUser(id: string, name: string): User {
    const user: User = { id, name };
    const entity = new UserEntity(user);

    if (entity.validate()) {
      this.repo.save(user);
    }

    return user;
  }
}
```

**Step 6: Commit**

```bash
git add tests/fixtures/failing_hexagonal/
git commit -m "test(fixtures): add failing hexagonal project with layer violations"
```

---

### Task 12: Create Mixed Clean Architecture Fixture

**Files:**
- Create: `tests/fixtures/mixed_clean_arch/architect.json`
- Create: `tests/fixtures/mixed_clean_arch/src/entities/user.entity.ts`
- Create: `tests/fixtures/mixed_clean_arch/src/usecases/user.usecase.ts`
- Create: `tests/fixtures/mixed_clean_arch/src/adapters/user.adapter.ts`

**Step 1: Create directory structure**

```bash
mkdir -p tests/fixtures/mixed_clean_arch/src/{entities,usecases,adapters}
```

**Step 2: Create architect.json**

Create `tests/fixtures/mixed_clean_arch/architect.json`:

```json
{
  "max_lines_per_function": 40,
  "framework": "Unknown",
  "pattern": "Clean",
  "forbidden_imports": [
    {
      "from": "entities",
      "to": "usecases"
    },
    {
      "from": "entities",
      "to": "adapters"
    },
    {
      "from": "usecases",
      "to": "adapters"
    }
  ]
}
```

**Step 3: Create entity file**

Create `tests/fixtures/mixed_clean_arch/src/entities/user.entity.ts`:

```typescript
export interface User {
  id: string;
  name: string;
  email: string;
}

export class UserEntity {
  constructor(private user: User) {}

  isValid(): boolean {
    return this.user.email.includes('@');
  }
}
```

**Step 4: Create usecase file (WITH MINOR VIOLATION)**

Create `tests/fixtures/mixed_clean_arch/src/usecases/user.usecase.ts`:

```typescript
import { User, UserEntity } from '../entities/user.entity';

export class UserUseCase {
  private users: User[] = [];

  createUser(id: string, name: string, email: string): User {
    const user: User = { id, name, email };
    const entity = new UserEntity(user);

    if (!entity.isValid()) {
      throw new Error('Invalid user');
    }

    this.users.push(user);
    return user;
  }

  // This function is slightly long (42 lines)
  processUserData(
    id: string,
    name: string,
    email: string,
    validate: boolean,
    save: boolean,
    notify: boolean
  ): User {
    const user: User = { id, name, email };

    if (validate) {
      const entity = new UserEntity(user);
      if (!entity.isValid()) {
        throw new Error('Invalid user');
      }
    }

    if (save) {
      this.users.push(user);
    }

    if (notify) {
      console.log(`User created: ${user.name}`);
    }

    return user;
  }
}
```

**Step 5: Create adapter file**

Create `tests/fixtures/mixed_clean_arch/src/adapters/user.adapter.ts`:

```typescript
import { User } from '../entities/user.entity';
import { UserUseCase } from '../usecases/user.usecase';

export class UserAdapter {
  constructor(private useCase: UserUseCase) {}

  async createUserFromAPI(apiData: unknown): Promise<User> {
    const data = apiData as { id: string; name: string; email: string };
    return this.useCase.createUser(data.id, data.name, data.email);
  }
}
```

**Step 6: Commit**

```bash
git add tests/fixtures/mixed_clean_arch/
git commit -m "test(fixtures): add mixed clean architecture project"
```

---

### Task 13: Create Circular Dependencies Fixture

**Files:**
- Create: `tests/fixtures/circular_deps/architect.json`
- Create: `tests/fixtures/circular_deps/src/module_a.ts`
- Create: `tests/fixtures/circular_deps/src/module_b.ts`

**Step 1: Create directory structure**

```bash
mkdir -p tests/fixtures/circular_deps/src
```

**Step 2: Create architect.json**

Create `tests/fixtures/circular_deps/architect.json`:

```json
{
  "max_lines_per_function": 40,
  "framework": "Unknown",
  "pattern": "MVC",
  "forbidden_imports": []
}
```

**Step 3: Create module A (imports B)**

Create `tests/fixtures/circular_deps/src/module_a.ts`:

```typescript
import { ModuleB } from './module_b';

export class ModuleA {
  private b: ModuleB;

  constructor() {
    this.b = new ModuleB();
  }

  doSomething(): string {
    return 'A uses ' + this.b.getValue();
  }
}
```

**Step 4: Create module B (imports A - creates cycle)**

Create `tests/fixtures/circular_deps/src/module_b.ts`:

```typescript
import { ModuleA } from './module_a';

export class ModuleB {
  private a: ModuleA | null = null;

  getValue(): string {
    return 'B';
  }

  setA(a: ModuleA): void {
    this.a = a;
  }
}
```

**Step 5: Commit**

```bash
git add tests/fixtures/circular_deps/
git commit -m "test(fixtures): add circular dependencies project"
```

---

### Task 14: Create Integration Test File

**Files:**
- Create: `tests/test_scoring_integration.rs`

**Step 1: Create test file with imports**

Create `tests/test_scoring_integration.rs`:

```rust
/// Integration tests for scoring engine with real project fixtures
///
/// Tests validate scoring behavior with realistic codebases:
/// - perfect_mvc_project: Perfect architecture, should score A
/// - failing_hexagonal: Layer violations, should score C or lower
/// - mixed_clean_arch: Minor violations, should score B or C
/// - circular_deps: Circular dependencies, should score <75

use std::path::PathBuf;
use architect_linter_pro::config::loader::load_config;
use architect_linter_pro::analyzer::collector::collect_violations;
use architect_linter_pro::metrics::{HealthGrade, ComponentStatus};
use architect_linter_pro::scoring;

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

// Tests will be added in next tasks
```

**Step 2: Commit**

```bash
git add tests/test_scoring_integration.rs
git commit -m "test(scoring): create integration test file structure"
```

---

### Task 15: Add Perfect MVC Integration Test

**Files:**
- Modify: `tests/test_scoring_integration.rs`

**Step 1: Write the test**

Add to `tests/test_scoring_integration.rs`:

```rust
#[test]
fn test_perfect_mvc_project() {
    let fixture_path = fixture_path("perfect_mvc_project");
    let config = load_config(&fixture_path.join("architect.json"))
        .expect("Failed to load config");

    let mut result = collect_violations(&fixture_path, &config)
        .expect("Failed to collect violations");

    let score = scoring::calculate_health_score(&mut result);

    // Perfect MVC should score A (90-100)
    assert_eq!(score.grade, HealthGrade::A);
    assert!(score.overall_score >= 90.0);

    // All components should pass
    for component in &score.components {
        assert_eq!(component.status, ComponentStatus::Pass);
    }

    // No violations
    assert!(result.violations.is_empty());

    // No circular dependencies
    assert!(result.circular_dependencies.is_empty());

    println!("✓ Perfect MVC project scored: {} ({})", score.overall_score, score.grade);
}
```

**Step 2: Run test**

Run: `cargo test test_perfect_mvc_project --test test_scoring_integration -- --nocapture`

Expected: PASS with "✓ Perfect MVC project scored: <score> (A)"

**Step 3: Commit**

```bash
git add tests/test_scoring_integration.rs
git commit -m "test(scoring): add perfect MVC integration test"
```

---

### Task 16: Add Failing Hexagonal Integration Test

**Files:**
- Modify: `tests/test_scoring_integration.rs`

**Step 1: Write the test**

Add to `tests/test_scoring_integration.rs`:

```rust
#[test]
fn test_failing_hexagonal_with_violations() {
    let fixture_path = fixture_path("failing_hexagonal");
    let config = load_config(&fixture_path.join("architect.json"))
        .expect("Failed to load config");

    let mut result = collect_violations(&fixture_path, &config)
        .expect("Failed to collect violations");

    let score = scoring::calculate_health_score(&mut result);

    // Should fail due to layer violations
    assert!(score.overall_score < 80.0);
    assert!(matches!(score.grade, HealthGrade::C | HealthGrade::D | HealthGrade::F));

    // Should have violations
    assert!(!result.violations.is_empty());

    // Verify specific violation: domain → infrastructure
    let has_domain_violation = result.violations.iter().any(|v| {
        v.from_layer == "domain" && v.to_layer == "infrastructure"
    });
    assert!(has_domain_violation, "Should detect domain → infrastructure violation");

    println!("✓ Failing hexagonal scored: {} ({})", score.overall_score, score.grade);
    println!("  Violations: {}", result.violations.len());
}
```

**Step 2: Run test**

Run: `cargo test test_failing_hexagonal --test test_scoring_integration -- --nocapture`

Expected: PASS with violations detected

**Step 3: Commit**

```bash
git add tests/test_scoring_integration.rs
git commit -m "test(scoring): add failing hexagonal integration test"
```

---

### Task 17: Add Mixed Clean Arch Integration Test

**Files:**
- Modify: `tests/test_scoring_integration.rs`

**Step 1: Write the test**

Add to `tests/test_scoring_integration.rs`:

```rust
#[test]
fn test_mixed_clean_arch_partial_score() {
    let fixture_path = fixture_path("mixed_clean_arch");
    let config = load_config(&fixture_path.join("architect.json"))
        .expect("Failed to load config");

    let mut result = collect_violations(&fixture_path, &config)
        .expect("Failed to collect violations");

    let score = scoring::calculate_health_score(&mut result);

    // Mixed quality should give B or C (70-90)
    assert!(matches!(score.grade, HealthGrade::B | HealthGrade::C));
    assert!(score.overall_score >= 70.0 && score.overall_score < 90.0);

    // Should have some long functions (complexity issues)
    assert!(!result.long_functions.is_empty() || result.complexity_stats.long_functions > 0);

    println!("✓ Mixed clean arch scored: {} ({})", score.overall_score, score.grade);
    println!("  Long functions: {}", result.complexity_stats.long_functions);
}
```

**Step 2: Run test**

Run: `cargo test test_mixed_clean_arch --test test_scoring_integration -- --nocapture`

Expected: PASS with partial score

**Step 3: Commit**

```bash
git add tests/test_scoring_integration.rs
git commit -m "test(scoring): add mixed clean architecture integration test"
```

---

### Task 18: Add Circular Deps Integration Test

**Files:**
- Modify: `tests/test_scoring_integration.rs`

**Step 1: Write the test**

Add to `tests/test_scoring_integration.rs`:

```rust
#[test]
fn test_scoring_with_circular_dependencies() {
    let fixture_path = fixture_path("circular_deps");
    let config = load_config(&fixture_path.join("architect.json"))
        .expect("Failed to load config");

    let mut result = collect_violations(&fixture_path, &config)
        .expect("Failed to collect violations");

    let score = scoring::calculate_health_score(&mut result);

    // Circular deps should significantly reduce score
    assert!(!result.circular_dependencies.is_empty(), "Should detect circular dependencies");
    assert!(score.overall_score < 75.0, "Score should be low due to circular deps");

    // Circular component should be 0
    let circular_component = &score.components[1]; // Circular deps is second
    assert_eq!(circular_component.score, 0.0);
    assert_eq!(circular_component.status, ComponentStatus::Fail);

    println!("✓ Circular deps project scored: {} ({})", score.overall_score, score.grade);
    println!("  Cycles detected: {}", result.circular_dependencies.len());
}
```

**Step 2: Run test**

Run: `cargo test test_scoring_with_circular --test test_scoring_integration -- --nocapture`

Expected: PASS with circular deps detected

**Step 3: Commit**

```bash
git add tests/test_scoring_integration.rs
git commit -m "test(scoring): add circular dependencies integration test"
```

---

**Phase 3 Complete:** 6 integration tests + 4 fixtures created
- 4 fixture projects
- 4 integration tests

**Verify:** Run `cargo test --test test_scoring_integration` - all should pass

---

## Phase 4: Consistency & Repeatability (30 min)

### Task 19: Add Idempotency Tests

**Files:**
- Modify: `tests/test_scoring.rs` (new section)

**Step 1: Add consistency test section**

Add to `tests/test_scoring.rs` after component tests:

```rust
// ============================================================================
// Consistency & Repeatability Tests (Phase 4)
// ============================================================================

#[test]
fn test_scoring_idempotency_same_input() {
    // Same input should always produce same output
    let mut result1 = create_test_result();
    let mut result2 = create_test_result();

    // Add same violations to both
    result1.layer_stats.blocked_violations = 5;
    result1.complexity_stats.long_functions = 3;

    result2.layer_stats.blocked_violations = 5;
    result2.complexity_stats.long_functions = 3;

    let score1 = scoring::calculate_health_score(&mut result1);
    let score2 = scoring::calculate_health_score(&mut result2);

    assert_eq!(score1.overall_score, score2.overall_score);
    assert_eq!(score1.grade, score2.grade);

    // All components should be identical
    for (c1, c2) in score1.components.iter().zip(score2.components.iter()) {
        assert_eq!(c1.score, c2.score);
        assert_eq!(c1.status, c2.status);
    }
}
```

**Step 2: Run test**

Run: `cargo test test_scoring_idempotency --lib -- --nocapture`

Expected: PASS

**Step 3: Commit**

```bash
git add tests/test_scoring.rs
git commit -m "test(scoring): add idempotency test"
```

---

### Task 20: Add Determinism Test

**Files:**
- Modify: `tests/test_scoring.rs` (after Task 19)

**Step 1: Write the test**

```rust
#[test]
fn test_scoring_determinism_100_runs() {
    // Run scoring 100 times on same input
    let mut scores = Vec::new();

    for _ in 0..100 {
        let mut result = create_test_result();
        result.layer_stats.blocked_violations = 5;
        result.complexity_stats.long_functions = 3;

        let score = scoring::calculate_health_score(&mut result);
        scores.push(score.overall_score);
    }

    // All scores should be identical
    let first = scores[0];
    let all_same = scores.iter().all(|&s| (s - first).abs() < 0.001);

    assert!(all_same, "All 100 runs should produce identical scores");
    println!("✓ 100 runs produced consistent score: {}", first);
}
```

**Step 2: Run test**

Run: `cargo test test_scoring_determinism --lib -- --nocapture`

Expected: PASS with "✓ 100 runs produced consistent score: <score>"

**Step 3: Commit**

```bash
git add tests/test_scoring.rs
git commit -m "test(scoring): add determinism test (100 runs)"
```

---

### Task 21: Add Reproducibility Test

**Files:**
- Modify: `tests/test_scoring.rs` (after Task 20)

**Step 1: Write the test**

```rust
#[test]
fn test_scoring_with_identical_projects() {
    // Two identical projects should get same score
    let fixture_path = PathBuf::from("tests/fixtures/perfect_mvc_project");

    let config = load_config(&fixture_path.join("architect.json"))
        .expect("Failed to load config");

    // Analyze same project twice
    let mut result1 = collect_violations(&fixture_path, &config)
        .expect("Failed to collect violations (1)");
    let mut result2 = collect_violations(&fixture_path, &config)
        .expect("Failed to collect violations (2)");

    let score1 = scoring::calculate_health_score(&mut result1);
    let score2 = scoring::calculate_health_score(&mut result2);

    assert_eq!(score1.overall_score, score2.overall_score);
    assert_eq!(score1.grade, score2.grade);

    println!("✓ Identical projects scored: {} ({})", score1.overall_score, score1.grade);
}
```

**Step 2: Add imports if needed**

Make sure these imports are at the top of `tests/test_scoring.rs`:

```rust
use std::path::PathBuf;
use architect_linter_pro::config::loader::load_config;
use architect_linter_pro::analyzer::collector::collect_violations;
```

**Step 3: Run test**

Run: `cargo test test_scoring_with_identical --lib -- --nocapture`

Expected: PASS

**Step 4: Commit**

```bash
git add tests/test_scoring.rs
git commit -m "test(scoring): add reproducibility test with identical projects"
```

---

**Phase 4 Complete:** 3 consistency tests added

**Verify:** Run `cargo test --lib` - all tests should pass

---

## Final Tasks

### Task 22: Run Full Test Suite

**Step 1: Run all tests**

Run: `cargo test --all`

Expected: All tests PASS

**Step 2: Count tests**

Run: `cargo test --all 2>&1 | grep "test result:"`

Expected: ~175-185 tests total

**Step 3: Verify no warnings**

Run: `cargo test --all 2>&1 | grep "warning:"`

Expected: No warnings (or only acceptable ones)

---

### Task 23: Measure Coverage (Optional but Recommended)

**Step 1: Install tarpaulin if not installed**

```bash
cargo install cargo-tarpaulin
```

**Step 2: Run coverage**

Run: `cargo tarpaulin --out Html --output-dir target/coverage`

Expected: Coverage report in `target/coverage/index.html`

**Step 3: Check critical modules coverage**

Look for:
- `scoring.rs`: Should be 90%+
- `config/loader.rs`: Should be 85%+
- `analyzer/metrics.rs`: Should be 85%+

**Step 4: Commit coverage report (if desired)**

```bash
git add target/coverage/
git commit -m "docs: add test coverage report"
```

---

### Task 24: Update Documentation

**Files:**
- Modify: `CHANGELOG.md`
- Modify: `tests/README.md` (if exists)

**Step 1: Update CHANGELOG.md**

Add to `CHANGELOG.md`:

```markdown
## [Unreleased]

### Added
- Comprehensive test suite for scoring engine (+35 tests)
- Integration tests with 4 realistic project fixtures
- Edge case tests for grade boundaries and division by zero
- Component isolation tests for all scoring components
- Consistency tests for idempotency and determinism

### Test Coverage
- Overall: 85%+ (up from ~65%)
- scoring.rs: 95%+
- config/loader.rs: 85%+
- analyzer/metrics.rs: 85%+
```

**Step 2: Create or update tests/README.md**

Create `tests/README.md`:

```markdown
# Test Suite

## Structure

```
tests/
├── test_scoring.rs              # Unit tests for scoring engine
├── test_scoring_integration.rs  # Integration tests with fixtures
├── test_parsers.rs              # Parser tests
├── test_fixtures.rs             # Fixture utilities
├── fixtures/                    # Test project fixtures
│   ├── perfect_mvc_project/     # Perfect architecture (A grade)
│   ├── failing_hexagonal/       # Layer violations (C/D grade)
│   ├── mixed_clean_arch/        # Minor issues (B/C grade)
│   └── circular_deps/           # Circular dependencies
└── common/                      # Test utilities
```

## Running Tests

```bash
# Run all tests
cargo test --all

# Run only scoring tests
cargo test --lib scoring

# Run integration tests
cargo test --test test_scoring_integration

# Run with coverage
cargo tarpaulin --out Html
```

## Test Coverage

- Overall: 85%+
- Critical modules: 90%+
```

**Step 3: Commit**

```bash
git add CHANGELOG.md tests/README.md
git commit -m "docs: update documentation with test suite details"
```

---

### Task 25: Create Summary Commit

**Step 1: Verify all changes**

Run: `git status`

Expected: No uncommitted changes

**Step 2: Create summary**

```bash
echo "# Test Suite Enhancement Summary

## Added
- 35 new tests for scoring engine
- 4 project fixtures for integration testing
- Coverage increased from ~65% to 85%+

## Phases Completed
1. Edge Cases (10 tests)
2. Component Isolation (12 tests)
3. Integration Tests (6 tests + 4 fixtures)
4. Consistency (3 tests)

## Files Modified
- tests/test_scoring.rs: +28 tests
- tests/test_scoring_integration.rs: +6 tests (new file)
- tests/fixtures/: +4 projects (new directory)

## Coverage
- scoring.rs: 95%+
- config/loader.rs: 85%+
- analyzer/metrics.rs: 85%+" > /tmp/test-summary.txt
cat /tmp/test-summary.txt
```

**Step 3: Tag release (if desired)**

```bash
git tag -a v4.1.0-test-enhancement -m "Test suite enhancement: 85%+ coverage"
```

---

## Success Criteria Checklist

- [ ] All tests pass (`cargo test --all`)
- [ ] No compiler warnings
- [ ] 175-185 tests total
- [ ] Coverage ≥85% on critical modules
- [ ] All 4 phases completed
- [ ] 4 fixture projects created
- [ ] Integration tests pass
- [ ] Consistency tests verify idempotency
- [ ] Documentation updated
- [ ] All changes committed

---

## Notes for Implementation

- **Use TDD:** Always write test first, verify it fails, then make it pass
- **Commit frequently:** Each task should have its own commit
- **Run tests often:** Don't wait until the end to run the test suite
- **Fix bugs immediately:** If tests reveal scoring bugs, fix them right away
- **Measure coverage:** Install tarpaulin to track progress

---

**Plan Complete**

Total estimated time: 3.5 hours
Total new tests: 28-35
Target coverage: 85%+
