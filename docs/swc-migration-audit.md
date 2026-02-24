# SWC Migration Audit

**Date:** 2026-02-23
**Baseline test count:** 424 passed, 0 failed, 1 ignored (across all test binaries)

---

## Summary

SWC (`swc_common`, `swc_ecma_parser`, `swc_ecma_ast`) is used in **6 source files**.
Its roles fall into three categories:

| Role | SWC API used | Replacement target |
|---|---|---|
| Load file into SourceMap | `cm.load_file(path)` | `std::fs::read_to_string` + custom span |
| Create in-memory SourceFile | `cm.new_source_file(FileName::Custom, src)` | `std::fs::read_to_string` + custom span |
| Parse JS/TS to AST | Lexer, Parser, StringInput, Syntax, TsConfig, EsConfig | tree-sitter (already used for multi-lang) |
| Walk AST nodes | swc_ecma_ast::ModuleItem, ModuleDecl, Stmt, Decl, ClassMember, Method, Fn, PropName | tree-sitter walk |
| Resolve byte offsets to line numbers | `cm.lookup_char_pos(span.lo/hi).line` | Manual str::lines().count() or tree-sitter node row |
| Convert SWC span to miette SourceSpan | `span.lo.0 - fm.start_pos.0` arithmetic | SourceSpan::new(offset, len) from manual search |

---

## Files Using SWC

### `src/main.rs`

**Imports:**
```rust
use swc_common::SourceMap;
```

**Usages (6 call sites):**
- `Arc::new(SourceMap::default())` constructed at lines 105, 262, 365, 434, 777, 1104.
  Each call site creates a fresh SourceMap and passes it (as Arc<SourceMap>) into
  analyze_file, validate_method_length, build_graph, update_file, and count_functions.

**What it does:**
`SourceMap` acts as a file-content registry; reads a file off disk and tracks byte
offsets so SWC can resolve span positions to source text. In `main.rs` the object
is created and immediately handed down -- `main.rs` itself never calls any method on it.

**Replace with:** Once downstream functions no longer need a SourceMap parameter, remove
all six `Arc::new(SourceMap::default())` lines and the `use swc_common::SourceMap` import.

---

### `src/analyzer/swc_parser.rs`

**Imports:**
```rust
use swc_common::SourceMap;
use swc_ecma_parser::{lexer::Lexer, EsConfig, Parser, StringInput, Syntax, TsConfig};
```

**Usages:**

| Line | API | Purpose |
|---|---|---|
| `cm.load_file(path)` (x3) | `SourceMap::load_file` | Reads the file from disk into a SourceFile, returning a Lrc<SourceFile> |
| `swc_ecma_ast::ModuleItem::*` (many) | AST node pattern matching | Walks parsed module to find Import, Class, Method, Fn declarations |
| `fm: &swc_common::SourceFile` params | SourceFile | Passed to error-creation helpers so they can compute byte offsets |
| `span: swc_common::Span` params | Span | Carries .lo / .hi byte positions within the SourceFile |
| `span.lo.0 - fm.start_pos.0` arithmetic | Span to offset | Converts an SWC span into a miette::SourceSpan offset+length |
| `_cm: &SourceMap` (x2, prefixed with _) | unused parameter | Historical pass-through; no methods called on it |

**Public functions that take `cm: &SourceMap`:**
- `analyze_file(cm, path, ctx)` -- loads file, delegates to multi-lang parser or SWC fallback
- `validate_method_length(cm, path, ctx)` -- parses file and checks per-method line counts
- `collect_violations_from_file(...)` -- collects all violations for caching layer

**Replace with:**
- Replace `cm.load_file(path)` with `fs::read_to_string(path)`.
- Replace SourceFile-based span arithmetic with a helper in new `src/source_span.rs`.
- Replace swc_ecma_ast pattern matching with tree-sitter (already in use via typescript_pure.rs).

---

### `src/analyzer/metrics.rs`

**Imports:**
```rust
use swc_common::SourceMap;
use swc_ecma_parser::{lexer::Lexer, EsConfig, Parser, StringInput, Syntax, TsConfig};
```

**Usages:**

| Line | API | Purpose |
|---|---|---|
| `cm: &SourceMap` parameter | SourceMap | Passed into count_functions and count_methods_in_file |
| `cm.lookup_char_pos(m.span.lo).line` (x4) | SourceMap::lookup_char_pos | Converts byte offset to 1-based line number to measure method body length |
| `swc_ecma_ast::ModuleItem::*` (many) | AST node pattern matching | Counts exported functions, class methods, standalone functions |

**Public functions that take `cm: &SourceMap`:**
- `count_functions(cm, path)` -- counts top-level functions/methods in a JS/TS file
- `count_methods_in_file(cm, path)` (internal) -- iterates AST, measures method line spans

**Replace with:**
- Replace `cm.load_file` with `fs::read_to_string`.
- Replace `cm.lookup_char_pos(span.lo/hi).line` with a pure-Rust newline-count helper (O(n)).
- Replace AST traversal with tree-sitter queries on the parsed TypeScript tree.

---

### `src/analyzer/collector.rs`

**Imports:**
```rust
use swc_common::sync::Lrc;
use swc_common::SourceMap;
```

**Usages:**

| Line | API | Purpose |
|---|---|---|
| `_cm: &SourceMap` parameter (line 34) | SourceMap | Accepted but never used (prefixed with _) |
| `let cm = Lrc::new(SourceMap::default())` (line 119) | SourceMap + Lrc | Creates thread-local SourceMap for cache-miss analysis path |

**What it does:**
`collector.rs` is the caching layer. On a cache miss it creates its own SourceMap and
passes it to collect_violations_from_file. The _cm parameter on the public function
is unused -- kept for API compatibility.

**Replace with:**
- Drop the `_cm: &SourceMap` parameter from the public function signature.
- Replace `Lrc::new(SourceMap::default())` with nothing -- no SourceMap needed once
  downstream callers use `fs::read_to_string` directly.

---

### `src/circular.rs`

**Imports:**
```rust
use swc_common::SourceMap;
use swc_ecma_parser::{lexer::Lexer, EsConfig, Parser, StringInput, Syntax, TsConfig};
```

**Usages:**

| Line | API | Purpose |
|---|---|---|
| `cm: &SourceMap` on build_graph, extract_imports, update_file, free fn (line 447) | SourceMap | Passed through to cm.load_file |
| `cm.load_file(file_path)` (line 159) | SourceMap::load_file | Reads file into SourceFile so SWC parser can tokenise it |
| `swc_ecma_ast::ModuleItem::ModuleDecl::Import` (line 169) | AST node | Extracts import source strings to build the dependency graph |

**What it does:**
`circular.rs` builds and incrementally updates a directed import-dependency graph. SWC
is used solely to parse the file and extract import path strings -- no span or error
information from SWC is used downstream.

**Replace with:**
- Replace `cm.load_file` + SWC parse with `fs::read_to_string` + tree-sitter parse (or
  the existing `src/parsers/typescript_pure::extract_imports_from_tree`).
- Drop `cm: &SourceMap` parameters from all public methods and the free function.

---

### `src/autofix.rs`

**Imports:**
```rust
use swc_common::SourceMap;
use swc_ecma_parser::{lexer::Lexer, EsConfig, Parser as SwcParser, StringInput, Syntax, TsConfig};
```

**Usages:**

| Line | API | Purpose |
|---|---|---|
| `Arc::new(SourceMap::default())` (line 392) | SourceMap | Creates a local SourceMap for parsing the in-memory source string |
| `cm.new_source_file(FileName::Custom(...), src)` (lines 407-408) | SourceMap::new_source_file | Registers an in-memory string as a SourceFile so SWC can parse it |
| `swc_common::FileName::Custom(...)` (line 408) | FileName | Names the virtual source file for error messages |
| Lexer, Parser, StringInput, Syntax, TsConfig, EsConfig | swc_ecma_parser | Full SWC parse pipeline for syntax validation before applying a fix |

**What it does:**
`autofix.rs` uses SWC to parse a proposed file content (after applying a fix) to verify
it is syntactically valid before writing it to disk. If the parse fails, the fix is rejected.

**Replace with:**
- Replace `cm.new_source_file` + SWC parse with tree-sitter parse of the string.
- Check `tree_sitter::Tree::root_node().has_error()` as the validity gate.
- Drop `Arc<SourceMap>`, FileName, and all swc_ecma_parser imports from this file.

---

## Cargo.toml Dependencies to Remove

After migration, the following entries in `[dependencies]` should be removed:

```toml
swc_common = "..."
swc_ecma_parser = "..."
swc_ecma_ast = "..."
```

(Exact version strings visible in Cargo.toml.)

---

## Migration Order

Ordered from least coupled to most coupled, minimising broken intermediate states:

1. **`src/source_span.rs`** _(new file)_ -- Create a pure-Rust helper module:
   - `fn offset_of_line(src: &str, line: usize) -> usize` -- byte offset of 1-based line
   - `fn line_of_offset(src: &str, offset: usize) -> usize` -- inverse
   - `fn span_for_line(src: &str, line: usize) -> SourceSpan` -- full-line SourceSpan
   No SWC dependencies.

2. **`src/circular.rs`** -- Replace `cm.load_file` + SWC parse with `fs::read_to_string`
   + `typescript_pure::extract_imports_from_tree`. Drop `cm: &SourceMap` parameters.

3. **`src/autofix.rs`** -- Replace SWC syntax-check with tree-sitter `.has_error()`.
   Drop SourceMap, FileName::Custom, and all swc_ecma_parser imports.

4. **`src/analyzer/metrics.rs`** -- Replace `cm.lookup_char_pos` with `line_of_offset`
   from the new source_span module. Replace AST walk with tree-sitter queries. Drop cm parameter.

5. **`src/analyzer/swc_parser.rs`** -- Replace `cm.load_file` with `fs::read_to_string`.
   Replace SourceFile-based span arithmetic with source_span helpers.
   Replace SWC AST walk with tree-sitter. Drop cm parameters. May merge into typescript_pure.

6. **`src/analyzer/collector.rs`** -- Drop `_cm: &SourceMap` parameter and Lrc::new(SourceMap::default()).

7. **`src/main.rs`** -- Remove all six `Arc::new(SourceMap::default())` lines and the import.

8. **`Cargo.toml`** -- Remove swc_common, swc_ecma_parser, swc_ecma_ast dependencies.
   Run `cargo build` to confirm clean compile.

---

## Baseline Test Results (pre-migration)

```
test result: ok. 39 passed;  0 failed; 0 ignored   (main binary, run 1)
test result: ok. 39 passed;  0 failed; 0 ignored   (main binary, run 2)
test result: ok.  4 passed;  0 failed; 0 ignored
test result: ok.  2 passed;  0 failed; 0 ignored
test result: ok.  1 passed;  0 failed; 0 ignored
test result: ok. 22 passed;  0 failed; 0 ignored
test result: ok. 33 passed;  0 failed; 0 ignored
test result: ok. 22 passed;  0 failed; 0 ignored
test result: ok. 11 passed;  0 failed; 0 ignored
test result: ok. 24 passed;  0 failed; 0 ignored
test result: ok. 19 passed;  0 failed; 0 ignored
test result: ok. 11 passed;  0 failed; 1 ignored
test result: ok.  1 passed;  0 failed; 0 ignored
test result: ok. 25 passed;  0 failed; 0 ignored
test result: ok.  2 passed;  0 failed; 0 ignored
test result: ok. 19 passed;  0 failed; 0 ignored
test result: ok.  1 passed;  0 failed; 0 ignored
test result: ok.  9 passed;  0 failed; 0 ignored
test result: ok. 49 passed;  0 failed; 0 ignored
test result: ok. 59 passed;  0 failed; 0 ignored
test result: ok.  4 passed;  0 failed; 0 ignored
test result: ok.  7 passed;  0 failed; 0 ignored
test result: ok. 12 passed;  0 failed; 0 ignored
test result: ok.  9 passed;  0 failed; 0 ignored   (doc-tests)
---------------------------------------------------------
TOTAL: 424 passed, 0 failed, 1 ignored
```

Every subsequent migration step must pass `cargo test` with >= 424 tests passing and 0
failing before proceeding to the next step.
