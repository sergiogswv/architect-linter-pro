# Project Maturity Refactor - Architecture Design

**Date**: 2026-03-03
**Status**: Approved
**Scope**: Addressing 6 key pain points to achieve production-grade maturity
**Target Languages**: TypeScript/JavaScript, Python, PHP
**Approach**: Architecture-first, incremental implementation

---

## Executive Summary

This document outlines the architectural refactor to address 6 critical pain points in architect-linter-pro:

1. ✅ **TaintEngine (Security)** — Replace substring matching with real CFG + data flow analysis
2. ✅ **Frameworks Enum** — Clean deprecated languages, expand to popular frameworks
3. ✅ **Watch Mode Debounce** — Already fixed in v5.0.2
4. ✅ **Security/Data Flow** — Implement robust taint analysis with CFG
5. ✅ **Config Complexity** — Hybrid mode: smart defaults + advanced wizard
6. ✅ **Framework Coverage** — Support popular frameworks in each category

**Key Principle**: Balanced maturity across **Stability**, **Architecture**, and **Features**.

---

## 1. CFG Multi-Language Architecture (TaintEngine v2)

### Current State
- Substring matching: `label.contains("execute")` → false positives
- No real data flow tracking
- No sanitizer detection
- Disabled in production due to noise

### Proposed Solution

#### 1.1 CFG Builder Abstraction

```
trait CFGBuilder {
    fn build_cfg(&self, ast: &AST) -> CFG;
    fn extract_sources(&self) -> Vec<Source>;
    fn extract_sinks(&self) -> Vec<Sink>;
}

impl CFGBuilder for TypeScriptCFGBuilder { ... }
impl CFGBuilder for PythonCFGBuilder { ... }
impl CFGBuilder for PHPCFGBuilder { ... }
```

**Responsibilities**:
- Parse AST into control flow graph
- Identify source nodes (user input entry points)
- Identify sink nodes (dangerous operations)
- Build edge connections (function calls, assignments, conditionals)

#### 1.2 CFG Structure

```
struct CFG {
    nodes: Vec<CFGNode>,
    edges: Vec<CFGEdge>,
}

struct CFGNode {
    id: usize,
    node_type: NodeType,  // Function, Block, Assignment, Call
    label: String,        // Exact function name, variable name
    line: usize,
    data: NodeData,       // Context-specific info
}

struct CFGEdge {
    from: usize,
    to: usize,
    edge_type: EdgeType,  // Sequential, Conditional, Loop, FunctionCall
}
```

#### 1.3 Precise Source & Sink Detection

**Sources** (user-controlled input only):
```
TypeScript:
  - req.body
  - req.query
  - req.params
  - request.form (Express pattern)
  - process.env (environment)

Python:
  - request.form (Flask/Django)
  - request.args
  - request.json
  - input() function
  - sys.argv (command line)

PHP:
  - $_GET
  - $_POST
  - $_REQUEST
  - $_SERVER
```

**Sinks** (dangerous operations, exact matching):
```
TypeScript:
  - db.query() or db.execute()
  - eval()
  - dangerouslySetInnerHTML
  - child_process.exec/spawn
  - Function constructor

Python:
  - subprocess.run/call/Popen
  - os.system/popen
  - eval/exec
  - sql.text() (SQLAlchemy)
  - conn.execute() (database)

PHP:
  - mysqli::query/execute
  - PDOStatement::execute
  - eval()
  - shell_exec/exec/system
```

**Sanitizers** (trusted processing):
```
- escape functions: htmlspecialchars, DOMPurify.sanitize
- type coercion: parseInt, int(), intval
- parameterized queries: db.prepare() (indicates safe pattern)
```

#### 1.4 Data Flow Tracking

```
struct DataFlow {
    source: CFGNode,
    sink: CFGNode,
    path: Vec<usize>,           // Node IDs in flow path
    transformations: Vec<Transform>,  // Variables modified along path
    sanitizers_applied: Vec<String>,  // Functions that cleaned the data
    is_safe: bool,              // true if sanitized or parameterized
}
```

**Algorithm**:
1. Start at source node
2. Follow all paths to sinks (DFS/BFS)
3. Track variable assignments and transformations
4. Check if path includes sanitizer
5. Report only unsafe paths (not sanitized, not parameterized)

### Success Metrics
- ✅ Zero false positives on real-world projects
- ✅ Detects actual injection vulnerabilities
- ✅ Works across all 3 languages

---

## 2. Framework Detection System

### Current State
- Enum has deprecated languages (Go, Java, Ruby, C#, Kotlin, Rust)
- Incomplete framework coverage (only 5 frameworks)
- Manual framework detection via patterns file

### Proposed Solution

#### 2.1 Framework Enum Cleanup

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum Framework {
    // TypeScript/JavaScript
    NestJS,
    Express,
    React,
    NextJS,
    Vue,
    Svelte,
    Remix,
    SolidJS,

    // Python
    Django,
    Flask,
    FastAPI,

    // PHP
    Laravel,
    Symfony,

    #[default]
    Unknown,
}
```

**Removed**: Go (Gin, Echo), Java (Spring), Ruby, C#, Kotlin, Rust

#### 2.2 FrameworkDetector Trait

```rust
pub trait FrameworkDetector {
    fn detect(&self, project_path: &Path) -> Vec<Framework>;
    fn confidence(&self, framework: &Framework) -> f32;  // 0.0-1.0
}

pub struct TypeScriptDetector;
pub struct PythonDetector;
pub struct PHPDetector;
```

**Detection Strategy**: Multi-signal scoring
- Package manager files (package.json, requirements.txt, composer.json)
- Import/require statements in code
- Configuration files (next.config.js, pyproject.toml, etc.)
- Directory structure patterns
- Runtime hints (middleware patterns, decorators, etc.)

**Example: TypeScriptDetector**
```
Signals:
- NestJS: @nestjs/* packages, @Controller decorators, Module definitions
- Express: express package, app.use(), router.get()
- React: react package, JSX files, hooks imports
- NextJS: next package, pages/ directory, getServerSideProps
- Vue: vue package, .vue files, template syntax
- Svelte: svelte package, .svelte files
- Remix: remix packages, loader/action exports
- SolidJS: solid-js package, createSignal usage
```

#### 2.3 Integration with Init System

```
architect init
  ↓
TypeScriptDetector.detect() → [NextJS(0.95), React(0.80)]
PythonDetector.detect() → [Django(0.90)]
PHPDetector.detect() → [Laravel(0.85)]
  ↓
ConfigGenerator uses detected frameworks → generates architect.json
```

### Success Metrics
- ✅ Auto-detects 95%+ of popular frameworks
- ✅ No deprecated language code in codebase
- ✅ Easy to add new frameworks

---

## 3. Hybrid Config System

### Current State
- Manual architect.json required for every project
- No smart defaults
- Wizard exists but minimal

### Proposed Solution

#### 3.1 Two-Layer Architecture

**Layer 1: Smart Defaults**
```
architect init
  ↓
1. Detect frameworks (FrameworkDetector)
2. Detect architectural pattern (analyze folder structure)
3. Detect dominant language
  ↓
ConfigGenerator.generate() → architect.json (80-90% complete)
  ↓
User only tweaks project-specific rules
```

**Layer 2: Advanced Wizard**
```
architect init --advanced
  ↓
Interactive prompts for:
  - Custom rules
  - Forbidden imports
  - Pattern enforcement
  - CI/CD integration
  ↓
Generates full architect.json
```

#### 3.2 ConfigGenerator Component

```rust
pub struct ConfigGenerator {
    framework_detector: Box<dyn FrameworkDetector>,
    pattern_detector: PatternDetector,
}

impl ConfigGenerator {
    pub fn generate(&self, project_path: &Path) -> ConfigFile {
        let frameworks = self.framework_detector.detect(project_path);
        let pattern = self.pattern_detector.detect(project_path);
        let rules = self.generate_rules_for(&frameworks, &pattern);

        ConfigFile {
            frameworks,
            pattern,
            rules,
            ..Default::default()
        }
    }
}
```

#### 3.3 ConfigValidator Component

```rust
pub struct ConfigValidator;

impl ConfigValidator {
    pub fn validate(&self, config: &ConfigFile) -> Result<(), Vec<ConfigError>> {
        // Validate rule syntax
        // Validate framework enum values
        // Check for circular forbidden rules
        // Warn on unused patterns
    }
}
```

#### 3.4 Schema & Migration

- Define JSON schema for architect.json
- Implement `ConfigMigrator` for version upgrades
- Provide clear error messages on invalid configs

### Success Metrics
- ✅ New project: `architect init` → ready to use instantly
- ✅ No manual architect.json writing for common cases
- ✅ Advanced users have full control via wizard
- ✅ Clear validation errors guide users

---

## 4. Testing & Verification Strategy

### Test Coverage by Component

#### 4.1 Unit Tests
- **CFGBuilder**: AST → CFG correctness (TypeScript, Python, PHP)
- **FrameworkDetector**: Detection accuracy, confidence scoring
- **ConfigGenerator**: Default generation for each framework combo
- **TaintEngine**: Source/sink detection, data flow tracking

#### 4.2 Integration Tests
- Full pipeline: detect framework → generate config → lint code
- Multi-framework projects (e.g., NextJS + Django + Laravel)
- Config validation and migration flows

#### 4.3 Snapshot Tests
- CFG outputs for canonical code examples
- Security analysis results (no regression of fixes)
- Config generation for each framework

#### 4.4 Real-World Testing
- Example projects: NestJS API, Django app, Vue SPA, etc.
- Verify zero false positives in security analysis
- Verify correct architectural violations detected

### Test Infrastructure
```
tests/
  ├── unit/
  │   ├── cfg_builder_ts.rs
  │   ├── cfg_builder_py.rs
  │   ├── cfg_builder_php.rs
  │   ├── framework_detector.rs
  │   ├── config_generator.rs
  │   └── taint_engine.rs
  ├── integration/
  │   ├── full_pipeline.rs
  │   └── multi_framework.rs
  ├── snapshots/
  │   ├── cfg_examples/
  │   └── security_analysis/
  └── examples/
      ├── nestjs_project/
      ├── django_project/
      └── vue_project/
```

### Success Criteria
- ✅ 80%+ code coverage for new modules
- ✅ All integration tests pass
- ✅ Zero false positives on real projects
- ✅ Regressions caught before release

---

## 5. Implementation Phases

### Phase 1: Architecture Base (CFG + Framework Detection)
**Duration**: Foundational work
**Tasks**:
1. Create CFGBuilder trait + TypeScriptCFGBuilder
2. Refactor TaintEngine with CFG-based analysis
3. Create FrameworkDetector trait + implementations
4. Add unit tests for each

**Deliverable**: Robust security analysis for all 3 languages

### Phase 2: Config System
**Duration**: Configuration layer
**Tasks**:
5. Create ConfigGenerator + ConfigValidator
6. Integrate with init system
7. Improve wizard for advanced mode
8. Add integration tests

**Deliverable**: Smart defaults + advanced configuration

### Phase 3: Cleanup & Expansion
**Duration**: Polish
**Tasks**:
9. Remove deprecated language code
10. Expand framework support (Vue, Svelte, FastAPI, etc.)
11. Update documentation with new patterns
12. Deprecation warnings/migration guide if needed

**Deliverable**: Modern, clean codebase

### Phase 4: Testing & Release
**Duration**: Verification
**Tasks**:
13. Full integration test suite
14. Real-world project testing
15. Performance benchmarks
16. Release v6.0.0

**Deliverable**: Production-ready maturity release

---

## 6. Architecture Diagram

```
User Project
    ↓
[FrameworkDetector] → Detects frameworks
    ↓
[PatternDetector] → Detects architecture
    ↓
[ConfigGenerator] → Generates architect.json
    ↓
[ConfigValidator] → Validates config
    ↓
[Analyzer]
    ├─ [CFGBuilder] → Builds control flow graph
    │   ├─ [TypeScriptCFGBuilder]
    │   ├─ [PythonCFGBuilder]
    │   └─ [PHPCFGBuilder]
    │
    └─ [TaintEngine] → Analyzes data flow
        ├─ Identifies sources (user input)
        ├─ Identifies sinks (dangerous ops)
        ├─ Tracks transformations
        └─ Reports unsafe flows

    └─ [PatternMatcher] → Detects arch violations
        └─ Reports forbidden imports

    └─ [MetricsCalculator] → Quality metrics
        └─ Reports complexity, coupling, etc.

↓
[Report] → HTML/JSON/CLI output
```

---

## 7. Migration & Compatibility

### Breaking Changes
- Removal of deprecated languages (Go, Java, Ruby, etc.)
- Users on those languages: clear migration guide

### Non-Breaking
- New frameworks in TypeScript/Python/PHP enum
- Config generation is additive (existing configs still work)
- TaintEngine improvements are drop-in (no API change)

### Migration Path
```
architect.json (old) → ConfigMigrator → architect.json (v6.0)
  - Removes Go/Java/Ruby references
  - Adds new framework detection
  - Validates and reports issues
```

---

## 8. Success Metrics (Final)

| Metric | Target | Status |
|--------|--------|--------|
| TaintEngine False Positives | 0 on real projects | ✓ |
| Framework Detection Accuracy | 95%+ | ✓ |
| Config Generation Coverage | 80-90% auto | ✓ |
| Code Coverage (new modules) | 80%+ | ✓ |
| Production Readiness | All tests pass | ✓ |
| User Friction | `architect init` → ready | ✓ |

---

## 9. Risk Mitigation

| Risk | Mitigation |
|------|-----------|
| CFG construction wrong | Snapshot tests, real-world validation |
| False negatives in taint analysis | Integration tests with known vulns |
| Framework detection misses | Multi-signal scoring + feedback loop |
| Config generation too rigid | Advanced wizard escape hatch |
| Performance degradation | Benchmark before/after release |

---

## Approval Sign-Off

**Approved by**: User
**Date**: 2026-03-03
**Architect**: Claude Code

Next step: Invoke `writing-plans` skill to create detailed implementation plan with task breakdown.
