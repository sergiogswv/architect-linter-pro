# Init Command & Framework Templates Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add `architect-linter-pro init` command that auto-detects the framework, asks which architectural pattern the user wants, shows a preview of the generated `architect.json`, and writes it — reducing time-to-value to under 2 minutes.

**Architecture:** New `src/init/` module with three responsibilities: template registry (11 pre-built configs), user prompts (using the existing `dialoguer` crate), and orchestration. Reuses existing `src/detector.rs` for framework detection. Adds `init` as a subcommand in `src/cli.rs` with `--force` and `--path` flags.

**Tech Stack:** Rust, `dialoguer` (already in Cargo.toml), `serde_json` (already in Cargo.toml), `std::fs`.

---

## Context: How the Codebase Works

Before touching code, understand these key facts:

1. **`src/cli.rs`**: `process_args()` parses args manually into `CliArgs`. No external CLI library (no clap). To add `init`, detect the word `"init"` as the first positional arg and set `init_mode = true`.

2. **`src/main.rs`**: Routes modes based on CliArgs fields. Add `if cli_args.init_mode { run_init(...) }` before the existing mode checks.

3. **`src/detector.rs`**: Already has `detect_framework(path: &Path) -> Framework` that reads `package.json`, `requirements.txt`, `pom.xml`, etc. **Reuse this directly** — do not create a new detector.

4. **`src/config/types.rs`**: `ForbiddenRule { from, to, severity }` — no `reason` field yet. Task 1 adds it.

5. **`src/config/loader.rs`**: `ConfigFile` is the struct that gets serialized to `architect.json`. Templates must produce this struct.

6. **`dialoguer`** crate is already in `Cargo.toml`. Use `dialoguer::Select` for numbered menus and `dialoguer::Confirm` for Y/n.

7. **Test baseline**: 443 tests passing. Must not drop.

---

## Task 1: Add `reason` Field to `ForbiddenRule`

Templates need a `reason` field for human-readable explanations. The JSON schema already has it (added in v4.4.0), but the Rust struct doesn't.

**Files:**
- Modify: `src/config/types.rs`

**Step 1: Add `reason` to `ForbiddenRule`**

In `src/config/types.rs`, find:
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ForbiddenRule {
    pub from: String,
    pub to: String,
    pub severity: Option<Severity>,
}
```

Replace with:
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ForbiddenRule {
    pub from: String,
    pub to: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<Severity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
```

**Step 2: Run tests to verify nothing broke**

```bash
cargo test 2>&1 | tail -5
```

Expected: 443 passed, 0 failed.

**Step 3: Commit**

```bash
git add src/config/types.rs
git commit -m "feat: add optional reason field to ForbiddenRule"
```

---

## Task 2: Add `init` Subcommand to CLI

**Files:**
- Modify: `src/cli.rs`

**Step 1: Add fields to `CliArgs`**

In `src/cli.rs`, inside the `CliArgs` struct, add after `check_mode`:
```rust
/// Run init wizard to generate architect.json
pub init_mode: bool,
/// Overwrite existing architect.json (used with init)
pub init_force: bool,
/// Target directory for init (default: current dir)
pub init_path: Option<String>,
```

In the `Default` impl, add:
```rust
init_mode: false,
init_force: false,
init_path: None,
```

In the final `Some(CliArgs { ... })` return, add:
```rust
init_mode,
init_force,
init_path,
```

**Step 2: Add parsing in `process_args()`**

Add local variables before the loop:
```rust
let mut init_mode = false;
let mut init_force = false;
let mut init_path: Option<String> = None;
```

Inside the `while` loop, add these match arms (before the `_` catch-all):
```rust
"init" => {
    init_mode = true;
}
"--force" => {
    init_force = true;
}
"--path" => {
    if i + 1 < args.len() {
        i += 1;
        init_path = Some(args[i].clone());
    } else {
        eprintln!("Error: --path requiere una ruta de directorio");
        return None;
    }
}
```

**Step 3: Add `init` to the help text**

In `print_help()`, after the `--check` line, add:
```rust
println!("  init                 Genera architect.json para tu proyecto");
println!("    --force            Sobreescribe architect.json si ya existe");
println!("    --path <DIR>       Directorio destino (default: directorio actual)");
```

Also add to EJEMPLOS:
```rust
println!("  architect-linter-pro init                  # Wizard en directorio actual");
println!("  architect-linter-pro init --force          # Sobreescribir config existente");
println!("  architect-linter-pro init --path ./backend # Init en subdirectorio");
```

**Step 4: Run tests**

```bash
cargo test 2>&1 | tail -5
```

Expected: 443 passed, 0 failed.

**Step 5: Commit**

```bash
git add src/cli.rs
git commit -m "feat: add init subcommand to CLI (--force, --path flags)"
```

---

## Task 3: Template Registry

**Files:**
- Create: `src/init/templates/mod.rs`
- Create: `src/init/templates/nestjs.rs`
- Create: `src/init/templates/nextjs.rs`
- Create: `src/init/templates/express.rs`
- Create: `src/init/templates/django.rs`
- Create: `src/init/templates/spring.rs`

### Step 1: Create `src/init/templates/mod.rs`

```rust
//! Pre-built architect.json templates for popular frameworks and patterns.

use crate::config::{ArchPattern, ForbiddenRule, Severity};
use crate::config::loader::ConfigFile;
use crate::config::types::Framework;

mod django;
mod express;
mod nestjs;
mod nextjs;
mod spring;

/// Human-readable pattern name shown in the selection menu.
pub struct PatternOption {
    pub label: &'static str,
    pub description: &'static str,
    pub pattern: &'static str, // internal key passed to get_template
}

/// Returns the pattern options available for a given framework.
pub fn patterns_for_framework(framework: &Framework) -> Vec<PatternOption> {
    match framework {
        Framework::NestJS => vec![
            PatternOption { label: "Hexagonal", description: "domain/ application/ infrastructure/", pattern: "hexagonal" },
            PatternOption { label: "Clean Architecture", description: "entities/ use-cases/ adapters/ frameworks/", pattern: "clean" },
            PatternOption { label: "Layered", description: "controllers/ services/ repositories/", pattern: "layered" },
        ],
        Framework::React | Framework::Unknown => vec![
            PatternOption { label: "Feature-based", description: "features/ con colocación por dominio", pattern: "feature-based" },
            PatternOption { label: "Layered", description: "components/ lib/ utils/", pattern: "layered" },
        ],
        Framework::Express => vec![
            PatternOption { label: "MVC", description: "routes/ controllers/ models/ middleware/", pattern: "mvc" },
            PatternOption { label: "Hexagonal", description: "domain/ application/ infrastructure/", pattern: "hexagonal" },
            PatternOption { label: "Feature-based", description: "features/ con colocación", pattern: "feature-based" },
        ],
        Framework::Django => vec![
            PatternOption { label: "MVT (Django standard)", description: "models/ views/ templates/ por app", pattern: "mvt" },
            PatternOption { label: "Service Layer", description: "models/ views/ services/ repositories/", pattern: "service-layer" },
        ],
        Framework::Spring => vec![
            PatternOption { label: "Layered MVC", description: "controller/ service/ repository/ model/", pattern: "layered" },
            PatternOption { label: "Hexagonal", description: "domain/ application/ infrastructure/", pattern: "hexagonal" },
        ],
        _ => vec![
            PatternOption { label: "MVC", description: "controllers/ services/ models/", pattern: "mvc" },
            PatternOption { label: "Hexagonal", description: "domain/ application/ infrastructure/", pattern: "hexagonal" },
        ],
    }
}

/// Retrieve the ConfigFile for a given framework + pattern combination.
/// Returns None if the combination is not supported.
pub fn get_template(framework: &Framework, pattern: &str) -> Option<ConfigFile> {
    match framework {
        Framework::NestJS => nestjs::get(pattern),
        Framework::React | Framework::Unknown => nextjs::get(pattern),
        Framework::Express => express::get(pattern),
        Framework::Django => django::get(pattern),
        Framework::Spring => spring::get(pattern),
        _ => None,
    }
}

/// Helper to build a ForbiddenRule with a reason.
pub(super) fn rule(from: &str, to: &str, severity: Severity, reason: &str) -> ForbiddenRule {
    ForbiddenRule {
        from: from.to_string(),
        to: to.to_string(),
        severity: Some(severity),
        reason: Some(reason.to_string()),
    }
}

/// Default ConfigFile shell that all templates start from.
pub(super) fn base_config(pattern: ArchPattern, rules: Vec<ForbiddenRule>) -> ConfigFile {
    ConfigFile {
        max_lines_per_function: 40,
        architecture_pattern: pattern,
        forbidden_imports: rules,
        ignored_paths: crate::config::default_ignored_paths(),
        build_command: None,
        ai_fix_retries: 3,
    }
}
```

**NOTE:** `ConfigFile` needs to be `pub` in `src/config/loader.rs`. Check if it's already public. If not, change `pub(crate) struct ConfigFile` → `pub struct ConfigFile` and make sure `default_ignored_paths` is also accessible. You may need to re-export `ConfigFile` from `src/config/mod.rs`:

```rust
pub use loader::ConfigFile;
```

### Step 2: Create `src/init/templates/nestjs.rs`

```rust
use super::{base_config, rule};
use crate::config::{ArchPattern, loader::ConfigFile, Severity::Error};

pub fn get(pattern: &str) -> Option<ConfigFile> {
    match pattern {
        "hexagonal" => Some(base_config(
            ArchPattern::Hexagonal,
            vec![
                rule("/domain/", "/application/", Error, "Domain must not depend on application layer"),
                rule("/domain/", "/infrastructure/", Error, "Domain must not depend on infrastructure"),
                rule("/application/", "/infrastructure/", Error, "Application layer must not depend on infrastructure directly"),
            ],
        )),
        "clean" => Some(base_config(
            ArchPattern::Clean,
            vec![
                rule("/entities/", "/use-cases/", Error, "Entities must not depend on use cases"),
                rule("/entities/", "/adapters/", Error, "Entities must not depend on adapters"),
                rule("/use-cases/", "/adapters/", Error, "Use cases must not depend on adapters"),
                rule("/use-cases/", "/frameworks/", Error, "Use cases must not depend on frameworks"),
            ],
        )),
        "layered" => Some(base_config(
            ArchPattern::MVC,
            vec![
                rule("/controllers/", "/repositories/", Error, "Controllers must go through services, not access repositories directly"),
                rule("/repositories/", "/controllers/", Error, "Repositories must not depend on controllers"),
                rule("/repositories/", "/services/", Error, "Repositories must not depend on services"),
            ],
        )),
        _ => None,
    }
}
```

### Step 3: Create `src/init/templates/nextjs.rs`

```rust
use super::{base_config, rule};
use crate::config::{ArchPattern, loader::ConfigFile, Severity::{Error, Warning}};

pub fn get(pattern: &str) -> Option<ConfigFile> {
    match pattern {
        "feature-based" => Some(base_config(
            ArchPattern::Custom("feature-based".to_string()),
            vec![
                rule("/features/", "/app/", Error, "Features must not import from the app layer"),
                rule("/features/*/", "/features/*/", Warning, "Features should be independent from each other"),
                rule("/components/", "/features/", Error, "Shared components must not depend on specific features"),
            ],
        )),
        "layered" => Some(base_config(
            ArchPattern::MVC,
            vec![
                rule("/components/", "/lib/server/", Error, "Client components must not import server-only lib"),
                rule("/pages/", "/components/ui/", Warning, "Pages should use feature components, not raw UI primitives"),
            ],
        )),
        _ => None,
    }
}
```

### Step 4: Create `src/init/templates/express.rs`

```rust
use super::{base_config, rule};
use crate::config::{ArchPattern, loader::ConfigFile, Severity::{Error, Warning}};

pub fn get(pattern: &str) -> Option<ConfigFile> {
    match pattern {
        "mvc" => Some(base_config(
            ArchPattern::MVC,
            vec![
                rule("/routes/", "/models/", Error, "Routes must go through controllers, not access models directly"),
                rule("/models/", "/controllers/", Error, "Models must not depend on controllers"),
                rule("/middleware/", "/controllers/", Warning, "Middleware should not depend on specific controllers"),
            ],
        )),
        "hexagonal" => Some(base_config(
            ArchPattern::Hexagonal,
            vec![
                rule("/domain/", "/infrastructure/", Error, "Domain must not depend on infrastructure"),
                rule("/domain/", "/adapters/", Error, "Domain must not depend on adapters"),
                rule("/application/", "/infrastructure/", Error, "Application must not depend on infrastructure directly"),
            ],
        )),
        "feature-based" => Some(base_config(
            ArchPattern::Custom("feature-based".to_string()),
            vec![
                rule("/features/*/", "/features/*/", Warning, "Features should be independent from each other"),
                rule("/shared/", "/features/", Error, "Shared utilities must not depend on specific features"),
            ],
        )),
        _ => None,
    }
}
```

### Step 5: Create `src/init/templates/django.rs`

```rust
use super::{base_config, rule};
use crate::config::{ArchPattern, loader::ConfigFile, Severity::{Error, Warning}};

pub fn get(pattern: &str) -> Option<ConfigFile> {
    match pattern {
        "mvt" => Some(base_config(
            ArchPattern::MVC,
            vec![
                rule("/templates/", "/models/", Error, "Templates must not import models directly"),
                rule("/views/", "/urls/", Warning, "Views should not import URL configuration"),
            ],
        )),
        "service-layer" => Some(base_config(
            ArchPattern::Custom("service-layer".to_string()),
            vec![
                rule("/views/", "/models/", Warning, "Views should go through services, not access models directly"),
                rule("/services/", "/views/", Error, "Services must not depend on views"),
                rule("/repositories/", "/services/", Error, "Repositories must not depend on services"),
            ],
        )),
        _ => None,
    }
}
```

### Step 6: Create `src/init/templates/spring.rs`

```rust
use super::{base_config, rule};
use crate::config::{ArchPattern, loader::ConfigFile, Severity::Error};

pub fn get(pattern: &str) -> Option<ConfigFile> {
    match pattern {
        "layered" => Some(base_config(
            ArchPattern::MVC,
            vec![
                rule("/controller/", "/repository/", Error, "Controllers must go through services, not access repositories directly"),
                rule("/repository/", "/controller/", Error, "Repositories must not depend on controllers"),
                rule("/repository/", "/service/", Error, "Repositories must not depend on services"),
            ],
        )),
        "hexagonal" => Some(base_config(
            ArchPattern::Hexagonal,
            vec![
                rule("/domain/", "/infrastructure/", Error, "Domain must not depend on infrastructure"),
                rule("/domain/", "/application/", Error, "Domain must not depend on application layer"),
                rule("/application/", "/infrastructure/", Error, "Application must not depend on infrastructure directly"),
            ],
        )),
        _ => None,
    }
}
```

### Step 7: Write failing test for template registry

Create `tests/test_init_templates.rs`:
```rust
use architect_linter_pro::config::types::Framework;

#[test]
fn test_nestjs_hexagonal_template_has_rules() {
    let tmpl = architect_linter_pro::init::templates::get_template(&Framework::NestJS, "hexagonal");
    assert!(tmpl.is_some(), "NestJS hexagonal template must exist");
    let config = tmpl.unwrap();
    assert!(!config.forbidden_imports.is_empty(), "Must have at least one rule");
    assert!(config.forbidden_imports.iter().any(|r| r.from.contains("/domain/")));
}

#[test]
fn test_unknown_pattern_returns_none() {
    let tmpl = architect_linter_pro::init::templates::get_template(&Framework::NestJS, "nonexistent");
    assert!(tmpl.is_none());
}

#[test]
fn test_all_frameworks_have_templates() {
    let cases = vec![
        (Framework::NestJS, "hexagonal"),
        (Framework::NestJS, "clean"),
        (Framework::NestJS, "layered"),
        (Framework::React, "feature-based"),
        (Framework::React, "layered"),
        (Framework::Express, "mvc"),
        (Framework::Express, "hexagonal"),
        (Framework::Express, "feature-based"),
        (Framework::Django, "mvt"),
        (Framework::Django, "service-layer"),
        (Framework::Spring, "layered"),
        (Framework::Spring, "hexagonal"),
    ];
    for (fw, pattern) in cases {
        let result = architect_linter_pro::init::templates::get_template(&fw, pattern);
        assert!(result.is_some(), "Missing template: {:?} / {}", fw, pattern);
    }
}
```

Run: `cargo test test_init_templates 2>&1 | tail -10`
Expected: FAIL (module doesn't exist yet).

### Step 8: Create `src/init/mod.rs` (minimal — just re-exports for now)

```rust
pub mod templates;
```

### Step 9: Add `pub mod init;` to `src/lib.rs`

In `src/lib.rs`, add after the last `pub mod` line:
```rust
pub mod init;
```

Also add `mod init;` to `src/main.rs` in the mod declarations block.

### Step 10: Run tests to verify templates compile and pass

```bash
cargo test test_init_templates 2>&1 | tail -10
```

Expected: 3 passed, 0 failed.

### Step 11: Commit

```bash
git add src/init/ src/lib.rs src/main.rs src/config/mod.rs src/config/loader.rs tests/test_init_templates.rs
git commit -m "feat(init): add template registry for 5 frameworks and 11 patterns"
```

---

## Task 4: Prompts Module

**Files:**
- Create: `src/init/prompts.rs`

### Step 1: Create `src/init/prompts.rs`

```rust
//! User interaction for the init command.
//! Uses `dialoguer` for interactive terminal prompts.

use crate::config::types::Framework;
use crate::init::templates::PatternOption;

/// Show a numbered menu and return the selected pattern key.
/// Returns None if the user cancels (Ctrl+C).
pub fn ask_pattern(framework: &Framework, options: &[PatternOption]) -> Option<String> {
    use dialoguer::{theme::ColorfulTheme, Select};

    let items: Vec<String> = options
        .iter()
        .map(|o| format!("{} ({})", o.label, o.description))
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("¿Qué patrón arquitectónico usas?")
        .items(&items)
        .default(0)
        .interact_opt()
        .ok()??;

    Some(options[selection].pattern.to_string())
}

/// Show a numbered menu to select the framework (used when auto-detection fails).
pub fn ask_framework() -> Option<Framework> {
    use dialoguer::{theme::ColorfulTheme, Select};

    let options = vec![
        ("Next.js", Framework::React),
        ("NestJS", Framework::NestJS),
        ("Express", Framework::Express),
        ("Django", Framework::Django),
        ("Spring Boot", Framework::Spring),
    ];

    let items: Vec<&str> = options.iter().map(|(label, _)| *label).collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("¿Qué framework usas?")
        .items(&items)
        .default(0)
        .interact_opt()
        .ok()??;

    Some(options[selection].1.clone())
}

/// Print the JSON preview to the terminal.
pub fn show_preview(json: &str) {
    println!();
    println!("Vista previa de architect.json:");
    println!("─────────────────────────────────");
    println!("{}", json);
    println!("─────────────────────────────────");
    println!();
}

/// Ask yes/no confirmation. Returns true if confirmed.
pub fn confirm_write() -> bool {
    use dialoguer::{theme::ColorfulTheme, Confirm};

    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("¿Crear architect.json?")
        .default(true)
        .interact()
        .unwrap_or(false)
}
```

**NOTE:** `Framework` needs to derive `Clone` if it doesn't already. Check `src/config/types.rs` — if `Framework` doesn't have `#[derive(Clone)]`, add it.

### Step 2: Run `cargo check`

```bash
cargo check 2>&1 | grep "error" | head -10
```

Expected: No errors. Fix any import errors.

### Step 3: Commit

```bash
git add src/init/prompts.rs
git commit -m "feat(init): add terminal prompts using dialoguer"
```

---

## Task 5: Init Orchestrator

**Files:**
- Modify: `src/init/mod.rs`

### Step 1: Write a failing integration test

In `tests/test_init.rs`:
```rust
use std::fs;
use tempfile::TempDir;

fn make_nestjs_project(dir: &TempDir) {
    let pkg = r#"{"dependencies": {"@nestjs/core": "^10.0.0"}}"#;
    fs::write(dir.path().join("package.json"), pkg).unwrap();
}

// NOTE: run_init is hard to test end-to-end because it requires interactive input.
// We test the non-interactive parts: file existence check, JSON writing.

#[test]
fn test_init_fails_if_config_exists_without_force() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("architect.json"), "{}").unwrap();

    let result = architect_linter_pro::init::check_no_existing_config(dir.path(), false);
    assert!(result.is_err(), "Should fail if architect.json exists and force=false");
}

#[test]
fn test_init_succeeds_if_config_exists_with_force() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("architect.json"), "{}").unwrap();

    let result = architect_linter_pro::init::check_no_existing_config(dir.path(), true);
    assert!(result.is_ok(), "Should succeed with --force even if file exists");
}

#[test]
fn test_write_config_creates_valid_json() {
    let dir = TempDir::new().unwrap();
    use architect_linter_pro::config::types::Framework;

    let tmpl = architect_linter_pro::init::templates::get_template(&Framework::NestJS, "hexagonal").unwrap();
    architect_linter_pro::init::write_config(dir.path(), &tmpl).unwrap();

    let content = fs::read_to_string(dir.path().join("architect.json")).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert!(parsed["forbidden_imports"].is_array());
    assert!(!parsed["forbidden_imports"].as_array().unwrap().is_empty());
}
```

Run: `cargo test test_init 2>&1 | tail -10`
Expected: FAIL (functions not defined yet).

### Step 2: Implement `src/init/mod.rs`

Replace the minimal `mod.rs` with the full implementation:

```rust
//! The `init` subcommand — generates architect.json from a framework template.

pub mod prompts;
pub mod templates;

use crate::config::types::Framework;
use crate::config::loader::ConfigFile;
use miette::{IntoDiagnostic, Result};
use std::fs;
use std::path::Path;

/// Check that architect.json does not exist at `root`, unless `force` is true.
/// Returns Ok(()) if it's safe to proceed, Err if the user should abort.
pub fn check_no_existing_config(root: &Path, force: bool) -> Result<()> {
    let config_path = root.join("architect.json");
    if config_path.exists() && !force {
        return Err(miette::miette!(
            "❌ Ya existe architect.json en {}.\n   Usa --force para sobreescribir.",
            root.display()
        ));
    }
    Ok(())
}

/// Serialize a ConfigFile and write it to `root/architect.json`.
pub fn write_config(root: &Path, config: &ConfigFile) -> Result<()> {
    let json = serde_json::to_string_pretty(config).into_diagnostic()?;
    let config_path = root.join("architect.json");
    fs::write(&config_path, json).into_diagnostic()?;
    Ok(())
}

/// Main entry point for the `init` subcommand.
///
/// Flow:
/// 1. Check if architect.json exists (respect --force)
/// 2. Detect framework from project files
/// 3. If not detected, ask the user
/// 4. Show pattern options and ask which to use
/// 5. Get template for (framework, pattern)
/// 6. Show preview and ask for confirmation
/// 7. Write architect.json
pub fn run_init(root: &Path, force: bool) -> Result<()> {
    // Step 1: Check for existing config
    check_no_existing_config(root, force)?;

    // Step 2: Detect framework
    let detected = crate::detector::detect_framework(root);
    let framework = if detected == Framework::Unknown {
        println!("No se detectó ningún framework conocido.");
        match prompts::ask_framework() {
            Some(fw) => {
                println!("Framework seleccionado: {}", fw.as_str());
                fw
            }
            None => {
                println!("Operación cancelada.");
                return Ok(());
            }
        }
    } else {
        println!("Detectado: {} ", detected.as_str());
        detected
    };

    // Step 3: Get pattern options and ask
    let options = templates::patterns_for_framework(&framework);
    let pattern_key = match prompts::ask_pattern(&framework, &options) {
        Some(p) => p,
        None => {
            println!("Operación cancelada.");
            return Ok(());
        }
    };

    // Step 4: Get the template
    let config = match templates::get_template(&framework, &pattern_key) {
        Some(c) => c,
        None => {
            return Err(miette::miette!(
                "No hay template para {} / {}. Por favor reporta esto como un bug.",
                framework.as_str(),
                pattern_key
            ));
        }
    };

    // Step 5: Show preview
    let json = serde_json::to_string_pretty(&config).into_diagnostic()?;
    prompts::show_preview(&json);

    // Step 6: Confirm
    if !prompts::confirm_write() {
        println!("Operación cancelada. No se creó architect.json.");
        return Ok(());
    }

    // Step 7: Write
    write_config(root, &config)?;

    println!();
    println!("✅ architect.json creado en {}", root.display());
    println!("   Ejecuta `architect-linter-pro .` para analizar tu proyecto.");

    Ok(())
}
```

### Step 3: Run tests

```bash
cargo test test_init 2>&1 | tail -10
```

Expected: 3 passed, 0 failed.

```bash
cargo test 2>&1 | tail -5
```

Expected: ≥ 443 passed, 0 failed.

### Step 4: Commit

```bash
git add src/init/mod.rs tests/test_init.rs
git commit -m "feat(init): implement init orchestrator with detect → ask → preview → write flow"
```

---

## Task 6: Wire `init` into `main.rs`

**Files:**
- Modify: `src/main.rs`

### Step 1: Check that `mod init;` is already in `src/main.rs`

(Added in Task 3 Step 9.) If not, add it now to the mod declarations block.

### Step 2: Find where modes are dispatched in `main()`

Look for the section that checks `cli_args.check_mode`, `cli_args.fix_mode`, etc. Add `init` routing **before** all other modes:

```rust
// Handle init mode before loading config (init creates the config)
if cli_args.init_mode {
    let root = if let Some(ref p) = cli_args.init_path {
        std::path::PathBuf::from(p)
    } else if let Some(ref p) = cli_args.project_path {
        std::path::PathBuf::from(p)
    } else {
        std::env::current_dir().into_diagnostic()?
    };
    return init::run_init(&root, cli_args.init_force);
}
```

**IMPORTANT**: This must come BEFORE `config::setup_or_load_config()` is called, because `init` creates the config file that the rest of the app needs. The existing wizard in `setup_or_load_config` runs when no `architect.json` exists, which conflicts with `init`. Place the `if cli_args.init_mode` block as the very first thing after `cli_args` is obtained.

### Step 3: Run all tests

```bash
cargo test 2>&1 | tail -5
```

Expected: ≥ 443 passed, 0 failed.

### Step 4: Manual smoke test

```bash
cargo build 2>&1 | tail -3
./target/debug/architect-linter-pro.exe init --help
```

Expected: help text includes `init` subcommand info.

### Step 5: Commit

```bash
git add src/main.rs
git commit -m "feat(init): wire init subcommand into main.rs dispatch"
```

---

## Task 7: Update README and push

**Files:**
- Modify: `README.md`

### Step 1: Add `init` to Quick Start section

Find the Quick Start / Installation section in `README.md`. After the install instructions, add:

```markdown
### Inicialización (recomendado)

```bash
architect-linter-pro init
```

El comando detecta tu framework automáticamente, te pregunta el patrón arquitectónico y genera `architect.json` listo para usar.

```bash
# Opciones
architect-linter-pro init --force          # Sobreescribir config existente
architect-linter-pro init --path ./backend # Inicializar subdirectorio
```
```

### Step 2: Run full test suite one last time

```bash
cargo test 2>&1 | tail -5
```

Expected: ≥ 443 passed, 0 failed.

### Step 3: Commit and push

```bash
git add README.md
git commit -m "docs: document init command in README"
git push origin master
```

---

## Troubleshooting

**`ConfigFile` not public:** If `loader.rs` has `pub(crate) struct ConfigFile`, change to `pub struct ConfigFile` and add `pub use loader::ConfigFile;` in `src/config/mod.rs`.

**`Framework` doesn't implement `Clone`:** Add `Clone` to the derive macro in `src/config/types.rs`.

**`detect_framework` signature mismatch:** Check `src/detector.rs` — the function may take `&Path` or `&PathBuf`. Adjust the call in `src/init/mod.rs` accordingly.

**`dialoguer::Select::interact_opt` not available:** If using an older dialoguer version, use `.interact()` instead and handle Ctrl+C with `unwrap_or(0)`.

**Tests fail due to missing `tempfile` crate:** Add to `[dev-dependencies]` in `Cargo.toml`:
```toml
tempfile = "3"
```
Check if it's already there with `grep tempfile Cargo.toml`.
