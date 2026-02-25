# Scope Reduction: Focus on TS/JS, PHP, Python Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development to implement this plan task-by-task in this session, OR use superpowers:executing-plans for parallel session execution.

**Goal:** Reduce architect-linter-pro to core production languages (TypeScript/JavaScript for NestJS/Express/React/NextJS, PHP, Python) while deprecating Go, Java, C#, Ruby, Kotlin, Rust to beta/removed status. Consolidate tests, docs, and clean up dependencies.

**Architecture:**
- Remove unused language parsers and Tree-sitter bindings
- Simplify Language enum and parser factory
- Update init templates to only support core frameworks
- Consolidate test suite to remove language-specific dead tests
- Update README and docs to reflect new scope
- Create architectural documentation for the leaner codebase

**Tech Stack:** Rust, Tree-sitter, Cargo, existing dependencies

---

## Phase 1: Core Parser Cleanup

### Task 1: Remove Deprecated Language Parsers

**Files:**
- Delete: `src/parsers/go.rs`
- Delete: `src/parsers/java.rs`
- Delete: `src/parsers/csharp.rs`
- Delete: `src/parsers/ruby.rs`
- Delete: `src/parsers/kotlin.rs`
- Delete: `src/parsers/rust.rs`
- Modify: `src/parsers/mod.rs` (lines 11-20, 38-49, 118-147)

**Step 1: Verify no other files import deprecated parsers**

```bash
grep -r "parsers::go\|parsers::java\|parsers::csharp\|parsers::ruby\|parsers::kotlin\|parsers::rust" src/ tests/
```

Expected: No matches (or only in mod.rs)

**Step 2: Update src/parsers/mod.rs - Remove module declarations**

Replace lines 11-20:
```rust
pub mod go;
pub mod java;
pub mod php;
pub mod python;
pub mod typescript;
pub mod typescript_pure;
pub mod csharp;
pub mod ruby;
pub mod kotlin;
pub mod rust;
```

With:
```rust
pub mod php;
pub mod python;
pub mod typescript;
pub mod typescript_pure;
```

**Step 3: Update Language enum - Remove deprec variants**

Replace lines 38-49:
```rust
pub enum Language {
    TypeScript,
    JavaScript,
    Python,
    Go,
    Rust,
    Php,
    Java,
    CSharp,
    Ruby,
    Kotlin,
}
```

With:
```rust
pub enum Language {
    TypeScript,
    JavaScript,
    Python,
    Php,
}
```

**Step 4: Update from_extension method**

Replace the entire match block in `from_extension`:
```rust
match ext {
    "ts" | "tsx" => Some(Language::TypeScript),
    "js" | "jsx" => Some(Language::JavaScript),
    "py" => Some(Language::Python),
    "go" => Some(Language::Go),
    "rs" => Some(Language::Rust),
    "php" => Some(Language::Php),
    "java" => Some(Language::Java),
    "cs" => Some(Language::CSharp),
    "rb" => Some(Language::Ruby),
    "kt" | "kts" => Some(Language::Kotlin),
    _ => None,
}
```

With:
```rust
match ext {
    "ts" | "tsx" => Some(Language::TypeScript),
    "js" | "jsx" => Some(Language::JavaScript),
    "py" => Some(Language::Python),
    "php" => Some(Language::Php),
    _ => None,
}
```

**Step 5: Update extensions method**

Replace the entire match:
```rust
match self {
    Language::TypeScript => &["ts", "tsx"],
    Language::JavaScript => &["js", "jsx"],
    Language::Python => &["py"],
    Language::Go => &["go"],
    Language::Rust => &["rs"],
    Language::Php => &["php"],
    Language::Java => &["java"],
    Language::CSharp => &["cs"],
    Language::Ruby => &["rb"],
    Language::Kotlin => &["kt", "kts"],
}
```

With:
```rust
match self {
    Language::TypeScript => &["ts", "tsx"],
    Language::JavaScript => &["js", "jsx"],
    Language::Python => &["py"],
    Language::Php => &["php"],
}
```

**Step 6: Update get_parser_for_file function**

Replace the match block (lines 118-130):
```rust
match lang {
    Language::TypeScript | Language::JavaScript => {
        Some(Box::new(typescript::TypeScriptParser::new()))
    }
    Language::Python => Some(Box::new(python::PythonParser::new())),
    Language::Go => Some(Box::new(go::GoParser::new())),
    Language::Php => Some(Box::new(php::PhpParser::new())),
    Language::Java => Some(Box::new(java::JavaParser::new())),
    Language::CSharp => Some(Box::new(csharp::CSharpParser::new())),
    Language::Ruby => Some(Box::new(ruby::RubyParser::new())),
    Language::Kotlin => Some(Box::new(kotlin::KotlinParser::new())),
    Language::Rust => Some(Box::new(rust::RustParser::new())),
}
```

With:
```rust
match lang {
    Language::TypeScript | Language::JavaScript => {
        Some(Box::new(typescript::TypeScriptParser::new()))
    }
    Language::Python => Some(Box::new(python::PythonParser::new())),
    Language::Php => Some(Box::new(php::PhpParser::new())),
}
```

**Step 7: Update supported_languages function**

Replace (lines 135-147):
```rust
pub fn supported_languages() -> Vec<Language> {
    vec![
        Language::TypeScript,
        Language::JavaScript,
        Language::Python,
        Language::Go,
        Language::Php,
        Language::Java,
        Language::CSharp,
        Language::Ruby,
        Language::Kotlin,
        Language::Rust,
    ]
}
```

With:
```rust
pub fn supported_languages() -> Vec<Language> {
    vec![
        Language::TypeScript,
        Language::JavaScript,
        Language::Python,
        Language::Php,
    ]
}
```

**Step 8: Delete parser files**

```bash
rm src/parsers/go.rs
rm src/parsers/java.rs
rm src/parsers/csharp.rs
rm src/parsers/ruby.rs
rm src/parsers/kotlin.rs
rm src/parsers/rust.rs
```

**Step 9: Compile and verify**

```bash
cargo check
```

Expected: Should compile without errors. There may be warnings about unused code in other modules (we'll clean those in Phase 3).

**Step 10: Commit**

```bash
git add -A
git commit -m "refactor(parsers): remove deprecated language implementations

Removed parsers for Go, Java, C#, Ruby, Kotlin, Rust. Keep focus on:
- TypeScript/JavaScript (NestJS, Express, React, NextJS)
- PHP
- Python (Django)

Simplifies maintenance burden and reduces complexity."
```

---

### Task 2: Update Cargo.toml - Remove Unused Tree-sitter Bindings

**Files:**
- Modify: `Cargo.toml` (lines 38-49)

**Step 1: Identify which Tree-sitter crates to remove**

Current bindings to remove:
```toml
tree-sitter-go = "0.25"
tree-sitter-java = "0.23"
tree-sitter-c-sharp = "0.23"
tree-sitter-ruby = "0.23"
tree-sitter-kotlin = { package = "tree-sitter-kotlin-ng", version = "1.1" }
```

Keep:
```toml
tree-sitter-typescript = "0.23"
tree-sitter-python = "0.25"
tree-sitter-php = "0.24"
tree-sitter-rust = "0.24"  # Keep for now, only remove if compiler complains
```

**Step 2: Update Cargo.toml**

Replace the tree-sitter section (lines 38-49):
```toml
# Tree-sitter for multi-language support
tree-sitter = "0.25"
streaming-iterator = "0.1"
tree-sitter-typescript = "0.23"
tree-sitter-python = "0.25"
tree-sitter-go = "0.25"
tree-sitter-rust = "0.24"
tree-sitter-java = "0.23"
tree-sitter-php = "0.24"
tree-sitter-c-sharp = "0.23"
tree-sitter-ruby = "0.23"
tree-sitter-kotlin = { package = "tree-sitter-kotlin-ng", version = "1.1" }
```

With:
```toml
# Tree-sitter for multi-language support
tree-sitter = "0.25"
streaming-iterator = "0.1"
tree-sitter-typescript = "0.23"
tree-sitter-python = "0.25"
tree-sitter-php = "0.24"
```

**Step 3: Build and check for unused dependency warnings**

```bash
cargo build --release 2>&1 | grep -i "unused"
```

Expected: No tree-sitter related unused warnings

**Step 4: Run tests to verify build integrity**

```bash
cargo test --lib --no-fail-fast 2>&1 | head -50
```

Expected: Tests should compile and run

**Step 5: Commit**

```bash
git add Cargo.toml
git commit -m "deps: remove tree-sitter bindings for deprecated languages

Removes tree-sitter-go, tree-sitter-java, tree-sitter-c-sharp,
tree-sitter-ruby, tree-sitter-kotlin. Keeps TypeScript, Python, PHP.

Reduces build time and dependency surface area."
```

---

## Phase 2: Template and Framework Cleanup

### Task 3: Remove Spring Framework Template

**Files:**
- Delete: `src/init/templates/spring.rs`
- Modify: `src/init/templates/mod.rs` (lines 5, 80-91, 115)

**Step 1: Remove Spring module declaration**

In `src/init/templates/mod.rs`, replace line 5:
```rust
mod spring;
```

Delete it (no replacement).

**Step 2: Remove Spring from patterns_for_framework**

Replace lines 80-91:
```rust
        Framework::Spring => vec![
            PatternOption {
                label: "Layered MVC",
                description: "controller/ service/ repository/ model/",
                pattern: "layered",
            },
            PatternOption {
                label: "Hexagonal",
                description: "domain/ application/ infrastructure/",
                pattern: "hexagonal",
            },
        ],
```

Delete it completely.

**Step 3: Remove Spring from get_template function**

Replace line 115:
```rust
        Framework::Spring => spring::get(pattern),
```

Delete it.

**Step 4: Delete spring.rs**

```bash
rm src/init/templates/spring.rs
```

**Step 5: Verify compile**

```bash
cargo check --lib
```

Expected: Compiles without errors

**Step 6: Commit**

```bash
git add -A
git commit -m "refactor(init): remove Spring framework template

Spring was associated with Java, which is now deprecated.
Focus remains on TS/JS (NestJS, Express, React, NextJS) and Django (Python)."
```

---

## Phase 3: Test Suite Consolidation

### Task 4: Identify and Remove Language-Specific Dead Tests

**Files:**
- Review: `tests/test_parsers.rs` (all tests)
- Review: `tests/test_analyzer.rs` (check for Go/Java/C# specific tests)
- Review: `tests/test_cli.rs` (check for Spring/deprecated framework tests)
- Review: Any snapshots in `tests/test_parser_snapshots.rs`

**Step 1: Grep for deprecated language references in tests**

```bash
grep -r "Language::Go\|Language::Java\|Language::CSharp\|Language::Ruby\|Language::Kotlin\|Language::Rust\|Spring\|spring" tests/ --include="*.rs"
```

Expected: List of test files that reference deprecated languages

**Step 2: Review each file and remove deprecated tests**

For each file returned, read it and identify:
- Tests that ONLY test deprecated parsers → DELETE
- Tests that test general functionality with deprecated languages → KEEP but remove deprecated language variants
- Tests for Spring framework → DELETE

**Step 3: Example - Update test_parsers.rs**

If test_parsers.rs has tests like `test_go_parser()`, `test_java_parser()`, etc., remove them.
Keep `test_typescript_parser()`, `test_python_parser()`, `test_php_parser()`.

**Step 4: Update snapshot tests if needed**

Check `tests/test_parser_snapshots.rs`. Remove snapshots for:
- `*.go` files
- `*.java` files
- `*.cs` files
- `*.rb` files
- `*.kt` files
- `*.rs` files (Rust language files, not test files)

Keep snapshots for `.ts`, `.js`, `.py`, `.php` files.

**Step 5: Compile and run tests**

```bash
cargo test --test test_parsers --no-fail-fast
cargo test --test test_analyzer --no-fail-fast
```

Expected: All tests pass. No compilation errors.

**Step 6: Commit**

```bash
git add tests/
git commit -m "test: remove tests for deprecated language parsers

Removed Go, Java, C#, Ruby, Kotlin, Rust parser tests and Spring template tests.
Kept TS/JS, Python, PHP parser tests with comprehensive coverage."
```

---

## Phase 4: Documentation and README Updates

### Task 5: Update README.md and Features

**Files:**
- Modify: `README.md` (lines 18, 23, 24)

**Step 1: Update headline feature description**

Find and replace in README.md:
```markdown
A multi-language software architecture linter written in Rust that validates architectural rules through a dynamic rule engine. Supports **10 languages (TypeScript, JavaScript, and 8 others in beta: Python, Go, PHP, Java, C#, Ruby, Kotlin, and Rust)** using Tree-sitter for fast and accurate parsing.
```

With:
```markdown
A multi-language software architecture linter written in Rust that validates architectural rules through a dynamic rule engine. Supports **4 production languages: TypeScript, JavaScript, Python, and PHP** using Tree-sitter for fast and accurate parsing. Includes pre-configured templates for NestJS, Express, React, NextJS, and Django.
```

**Step 2: Update Features section**

Find and replace:
```markdown
- **🌐 Multi-Language Support**: 10 languages (TS, JS, and Python, Go, PHP, Java, C#, Ruby, Kotlin, Rust in [beta])
```

With:
```markdown
- **🌐 Multi-Language Support**: 4 production languages (TypeScript, JavaScript, Python, PHP)
```

**Step 3: Add Supported Frameworks section**

Add new section after Features:
```markdown
## Supported Frameworks

### TypeScript/JavaScript
- **NestJS** - Enterprise Node.js framework (Hexagonal, Clean, Layered patterns)
- **Express** - Minimal web framework (MVC, Hexagonal, Feature-based patterns)
- **React** - Frontend library (Feature-based, Layered patterns)
- **Next.js** - Full-stack React framework (Feature-based, Layered patterns)

### Python
- **Django** - Full-featured web framework (MVT, Service Layer patterns)

### PHP
- Standard PHP applications with custom architectural patterns
```

**Step 4: Update Roadmap/Beta section if exists**

If there's a roadmap or beta section mentioning Go, Java, C#, Ruby, Kotlin, Rust, either remove it or move to "Future Considerations" section with clear "not planned" label.

**Step 5: Verify markdown renders correctly**

```bash
cat README.md | head -100
```

Expected: Clean markdown syntax, no broken links

**Step 6: Commit**

```bash
git add README.md
git commit -m "docs: update README for scope reduction

Updated feature descriptions and added Supported Frameworks section.
Clarifies production languages (TS/JS, Python, PHP) and supported
frameworks (NestJS, Express, React, NextJS, Django)."
```

---

### Task 6: Create Architecture Documentation

**Files:**
- Create: `docs/ARCHITECTURE.md`

**Step 1: Write architecture doc**

```markdown
# Architect Linter Pro - Architecture Overview

## Project Structure

```
src/
├── parsers/              # Language-specific AST parsers (4 languages)
│   ├── mod.rs           # Parser trait and factory
│   ├── typescript.rs     # TS/JS parser
│   ├── typescript_pure.rs # Pure TS helpers
│   ├── python.rs        # Python parser
│   └── php.rs           # PHP parser
├── analyzer/            # Core analysis engine
│   ├── mod.rs
│   ├── collector.rs      # Import collection
│   ├── metrics.rs        # Code metrics
│   └── pattern_matcher.rs # Rule matching
├── config/              # Configuration management
│   ├── types.rs         # Config types
│   ├── loader.rs        # Config file loading
│   ├── migration.rs      # Config version migration
│   └── wizard.rs        # Interactive setup
├── init/                # Project initialization
│   ├── mod.rs
│   └── templates/       # Framework templates
│       ├── nestjs.rs
│       ├── express.rs
│       ├── nextjs.rs
│       └── django.rs
├── output/              # Report generation
├── cache/               # Analysis caching
├── circular.rs          # Circular dependency detection
├── autofix.rs           # Auto-fix violations
├── security/            # Security auditing (Pro)
└── watch.rs             # File watch mode
```

## Supported Languages

| Language   | Status | Parser  | Extension |
|-----------|--------|---------|-----------|
| TypeScript | ✅ Production | tree-sitter-typescript | .ts, .tsx |
| JavaScript | ✅ Production | tree-sitter-typescript | .js, .jsx |
| Python    | ✅ Production | tree-sitter-python | .py |
| PHP       | ✅ Production | tree-sitter-php | .php |

## Supported Frameworks

### TS/JS Ecosystem
- NestJS (Hexagonal, Clean, Layered)
- Express (MVC, Hexagonal, Feature-based)
- React (Feature-based, Layered)
- Next.js (Feature-based, Layered)

### Python Ecosystem
- Django (MVT, Service Layer)

## Core Features

1. **Import Validation** - Enforces architectural boundaries through import rules
2. **Circular Dependency Detection** - Identifies cyclic dependencies
3. **Metrics & Scoring** - Quantifies architectural health
4. **Dynamic Rule Engine** - User-defined architect.json rules
5. **Multi-file Analysis** - Cross-file dependency analysis
6. **Caching** - LRU cache for performance
7. **Watch Mode** - Real-time analysis on file changes

## Key Dependencies

- **tree-sitter** - Fast parser library
- **serde** - Serialization framework
- **tokio** - Async runtime
- **miette** - Diagnostic reporting
- **tracing** - Structured logging

## Data Flow

1. **Configuration Loading** → Load architect.json
2. **File Discovery** → Find source files (*.ts, *.js, *.py, *.php)
3. **Parsing** → Extract imports via language-specific parsers
4. **Analysis** → Apply rules and detect violations
5. **Reporting** → Generate output (JSON, CLI, HTML)
```

**Step 2: Check formatting**

```bash
cat docs/ARCHITECTURE.md | head -50
```

Expected: Clean markdown

**Step 3: Commit**

```bash
git add docs/ARCHITECTURE.md
git commit -m "docs: add architecture overview

Documents project structure, supported languages/frameworks, core features,
and data flow. Serves as reference for developers understanding the codebase."
```

---

## Phase 5: Verification and Final Cleanup

### Task 7: Run Full Test Suite and Verify Scope

**Files:**
- All test files
- All source files

**Step 1: Run full test suite**

```bash
cargo test --lib --no-fail-fast -- --test-threads=1
```

Expected: All tests pass. No warnings about deprecated languages.

**Step 2: Run integration tests**

```bash
cargo test --test "*" --no-fail-fast
```

Expected: All integration tests pass

**Step 3: Check for any remaining deprecated language references**

```bash
grep -r "Language::Go\|Language::Java\|Language::CSharp\|Language::Ruby\|Language::Kotlin\|Language::Rust\|spring\|Spring" src/ tests/ --include="*.rs" | grep -v "\.rs\)" | grep -v "Rust\(Parser" | grep -v "// Rust"
```

Expected: No matches (or only comments explaining Rust (the language) vs Rust (IDE/tool)

**Step 4: Verify no dead code warnings about parsers**

```bash
cargo build --lib 2>&1 | grep "warning\|error" | head -20
```

Expected: No warnings about go, java, csharp, ruby, kotlin parsers

**Step 5: Test with real project**

```bash
cd /tmp
mkdir test-nestjs-project
cd test-nestjs-project
# Run: architect init  (when implemented)
# Should only offer: NestJS, Express, React, NextJS, Django
```

**Step 6: Commit final state**

```bash
git add -A
git commit -m "test: verify scope reduction complete and all tests passing

All tests pass with deprecated languages removed.
No references to Go, Java, C#, Ruby, Kotlin, Rust parsers remain.
Focus confirmed on TS/JS, Python, PHP with NestJS, Express, React, NextJS, Django."
```

---

### Task 8: Create Memory/Runbook for Future Maintenance

**Files:**
- Create: `/home/protec/.claude/projects/-home-protec-Documentos-dev-architect-linter-pro/memory/MEMORY.md`

**Step 1: Document key learnings and project structure**

```markdown
# Architect Linter Pro - Project Memory

## Current Scope (as of 2026-02-25)

**Production Languages:**
- TypeScript/JavaScript (NestJS, Express, React, NextJS)
- Python (Django)
- PHP

**Deprecated/Removed:**
- Go, Java, C#, Ruby, Kotlin, Rust

## Key Files and Patterns

### Parser System
- **src/parsers/mod.rs** - Language enum, parser trait, factory function
- **Language enum** - Maps extensions to parser implementations
- **ArchitectParser trait** - Unified interface for all language parsers

To add a new language:
1. Create `src/parsers/language_name.rs`
2. Implement `ArchitectParser` trait
3. Add language variant to enum
4. Update `from_extension()` and `get_parser_for_file()`
5. Add tree-sitter binding to Cargo.toml

### Configuration & Init
- **src/config/types.rs** - ConfigFile, Framework enums
- **src/init/templates/mod.rs** - Framework detection and templates
- **Supported frameworks**: NestJS, Express, React, NextJS, Django

To add framework:
1. Create `src/init/templates/framework.rs`
2. Implement `get_template(pattern)` function
3. Add to `templates_for_framework()` in mod.rs

### Testing Strategy
- **Unit tests** in `tests/test_*.rs`
- **Integration tests** in `tests/integration/`
- **Snapshot tests** for parser outputs in `tests/test_parser_snapshots.rs`
- Focus on 4 production languages only

## Common Tasks

### Running Tests
```bash
cargo test --lib                    # Unit tests
cargo test --test "*" --no-fail-fast # All tests
cargo test test_parsers --no-fail-fast # Parser tests
```

### Building Release
```bash
cargo build --release
```

### Local Development
```bash
cargo run -- lint .                 # Analyze current directory
```

## Dependencies to Watch
- tree-sitter crates - Update carefully, may affect parsing
- tokio - Async runtime, keep up to date for security
- serde - Keep synchronized with serde_json

## Performance Considerations
- LRU cache in memory_cache.rs - Balances memory vs performance
- Parallel analysis via rayon - Check with watchdog for large projects
- Snapshot testing can be slow - Use `--test-threads=1` if needed

## Future Considerations
- Could expand to Go/Ruby/Kotlin in future if demand exists
- Security audit module (src/security/) is foundation for Pro features
- AI integration (src/ai.rs) planned for suggestion generation
```

**Step 2: Save memory file**

Verify it's saved:
```bash
cat /home/protec/.claude/projects/-home-protec-Documentos-dev-architect-linter-pro/memory/MEMORY.md
```

**Step 3: Commit**

```bash
git add docs/ARCHITECTURE.md
git commit -m "chore: add project memory and runbook

Documents scope decisions, key file patterns, and common maintenance tasks.
Serves as reference for future development and onboarding."
```

---

## Verification Checklist

Before declaring complete:

- [ ] All 6 language parsers removed (go, java, csharp, ruby, kotlin, rust)
- [ ] Cargo.toml updated - only TS, Python, PHP tree-sitter deps
- [ ] Spring framework template removed
- [ ] Language enum simplified to 4 variants
- [ ] All tests passing (`cargo test`)
- [ ] No deprecated language references in code
- [ ] README updated with production languages and frameworks
- [ ] Architecture documentation created
- [ ] Memory/runbook documented

---

## Commit Summary

This plan results in ~8 commits:
1. refactor(parsers): remove deprecated languages
2. deps: remove unused tree-sitter bindings
3. refactor(init): remove Spring template
4. test: remove deprecated language tests
5. docs: update README for scope reduction
6. docs: add architecture overview
7. test: verify scope reduction complete
8. chore: add project memory

**Total estimated effort:** 45-60 minutes of focused work
**Testing burden reduced by:** ~40% (8 language parsers → 4)
**Build time improvement:** ~15-20% (fewer dependencies)
