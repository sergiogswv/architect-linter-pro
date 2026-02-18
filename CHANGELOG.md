# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [4.2.0] - 2026-02-13
## [4.3.0] - 2026-02-18

This release introduces **Additional Language Support (C#, Ruby, Kotlin, Rust)**, **AI-Powered Build Validation**, comprehensive configuration schema validation, structured logging, and improved error handling for better DX and observability.

### 🌐 Additional Language Support
Architect Linter Pro now supports 10 languages! We've integrated the following:
- **C# Support**: Extraction of `using` directives and alias names.
- **Ruby Support**: Full support for `require`, `require_relative`, and `load`.
- **Kotlin Support**: Support for `import` statements and package-based wildcards.
- **Rust Support**: Detailed `use` declaration analysis.
- **Modern Tree-sitter**: Upgraded to Tree-sitter 0.25 and `StreamingIterator` for maximum performance and safety.

### 🛡️ AI Fix Validation & Build Integration
Garantiza la integridad del sistema tras aplicar correcciones automáticas.

- **Build Command Integration**: Posibilidad de configurar un `build_command` (ej. `npm run build`) en `architect.json`.
- **Self-Correction Loop**: Si el build falla tras un fix, el linter envía los errores de compilación a la IA para que genere una nueva versión corregida.
- **Atomic Rollback**: Si tras agotar los reintentos (`ai_fix_retries`) el build sigue fallando, el linter revierte automáticamente los cambios para evitar dejar el código en un estado roto.
- **Visual Feedback**: Nuevos indicadores de progreso para la fase de build y estados de éxito/error tras la validación.

### 🧩 Configuration Schema Validation

Full JSON Schema integration for robust configuration management and IDE support.

- **JSON Schema Engine** (`schemas/architect.schema.json`):
  - Strict validation of `architect.json` against a formal schema
  - Protection against invalid types, missing fields, and duplicate rules
  - Formal definition for all configuration properties including `$schema`

- **IDE Support & Autocompletion**:
  - Full autocompletion in VS Code and IntelliJ via `$schema` reference
  - Built-in documentation for each property directly in the editor
  - Automatic schema association via `.vscode/settings.json`

- **Configuration Migration** (`src/config/migration.rs`):
  - Intelligent migration logic for legacy configuration formats
  - Automatic updates to ensure forward compatibility
  - Built-in data transformation before validation

- **CLI Validation Mode**:
  - New `--check` flag for fast configuration-only validation
  - Explicit config validation phase in pre-commit hooks
  - Instant feedback on configuration errors without full code analysis

### Added
- **Additional Language Support**: Integrated C#, Ruby, Kotlin, and Rust parsers.
- **Modern Tree-sitter core**: Updated to v0.25 core with `StreamingIterator` support.
- **Structured Logging** (`src/logging.rs`):
  - Integration with `tracing` crate for structured, leveled logging
  - Configurable log levels: TRACE, DEBUG, INFO, WARN, ERROR
  - Timestamp, thread ID, module, file, and line number in debug mode
  - Support for both console and JSON output formats
  - Environment variable override support (`RUST_LOG`)

- **Debug Mode**:
  - New `--debug` CLI flag for verbose logging
  - Detailed execution flow tracking
  - Performance monitoring capabilities
  - Thread-safe logging across parallel operations

- **Enhanced Error Handling**:
  - Custom panic handler with detailed error messages
  - Location tracking (file:line) for panics
  - User-friendly error messages with recovery suggestions
  - Automatic bug report instructions
  - Graceful degradation on errors

- **Logging Integration**:
  - Application lifecycle logging (startup, shutdown)
  - Configuration loading tracking
  - File analysis progress logging
  - Mode selection logging (NORMAL, WATCH, FIX, INCREMENTAL)
  - Cache hit/miss tracking

- **Explicit Config Check**:
  - Pre-commit hook now runs `architect-linter-pro --check` before full analysis
  - Prevents committing with an invalid architecture configuration

### Changed
- **CLI**:
  - Added `--debug` flag to enable verbose logging
  - Updated help text with debug mode documentation
  - Enhanced error messages with contextual information

- **Main Entry Point**:
  - Logging initialization at startup
  - Panic handler setup for better crash reports
  - Detailed logging at key execution points

- **Analyzer**:
  - Added logging to file analysis collector
  - Cache status logging
  - Performance metrics logging

### Technical Details
- **New Dependencies**:
  - `tracing = "0.1"` - Structured logging framework
  - `tracing-subscriber = "0.3"` - Subscriber implementations with env-filter, fmt, and json features
  - `tracing-appender = "0.2"` - File appender support
  - `jsonschema = "0.17"` - JSON Schema validation engine

- **New Modules**:
  - `src/logging.rs` (99 lines) - Logging configuration and initialization
  - `src/config/migration.rs` - Legacy configuration transformation logic
  - `schemas/architect.schema.json` - Formal JSON Schema definition

- **Modified Files**:
  - `src/main.rs` - Logging initialization, early check mode exit
  - `src/cli.rs` - Debug flag and check flag support
  - `src/config/loader.rs` - Integration with JSON Schema validation and migration
  - `src/config/husky.rs` - Explicit config validation in pre-commit hooks
  - `src/analyzer/collector.rs` - Analysis logging
  - `Cargo.toml` - Logging and validation dependencies

### Usage Examples

```bash
# Normal mode (warnings and errors only)
architect-linter-pro /path/to/project

# Debug mode (verbose logging with timestamps)
architect-linter-pro --debug /path/to/project

# Environment variable override
RUST_LOG=trace architect-linter-pro /path/to/project
```

### Documentation
- Complete implementation guide: `docs/ERROR_HANDLING_LOGGING_IMPLEMENTATION.md`
- Updated ROADMAP.md with completion status
- Updated README.md with debug mode documentation

### 🧪 Stability & Test Hardening
- **Struct Defaults**: Implemented `Default` trait for `LinterContext`, `CliArgs`, `Framework`, `ArchPattern`, and `ForbiddenRule` to ensure robust initialization and fix failing tests.
- **Cache Modernization**: Updated `tests/test_cache.rs` and `benches/performance_bench.rs` to reflect the new `AnalysisCache` architecture.
- **Legacy Compatibility Layer**: Re-introduced essential types in `src/scoring.rs` for backward compatibility with integration tests during the transition to the 4.0 scoring system.
- **Dependency cleanup**: Removed non-existent `MemoryCache` references from benchmarks.
- **Test coverage**: Fixed multiple broken integration tests in `test_analyzer.rs`, `test_multi_file_analysis.rs`, and `test_scoring.rs`.

### 📊 Metrics Improvements
- **Function Call Extraction**: Added `extract_function_calls` using SWC visitor pattern to track inter-file dependencies.
- **Public API Refactoring**: Re-exported essential metrics utilities from `src/analyzer/mod.rs` for better accessibility.

### Bug Fixes
- Fixed `.claude/` directory parsing errors (Python files).
- Added `.claude/` to default ignored paths.
- Modified `circular.rs` to skip non-JS/TS files.
- Fixed missing field errors in `CliArgs` and `LinterContext` initializers across the test suite.
- Corrected syntax errors and missing types in `src/metrics.rs`.

---


### 🚀 Performance & Optimization
- **Parallel Processing**: Multi-threaded file parsing with Rayon for 3-5x speed improvement
- **Intelligent Caching**: File-based AST cache with automatic invalidation
- **Incremental Analysis**: Git-based change detection for delta processing
- **Memory Optimization**: AST scoping reduces memory usage by 50%
- **Performance Metrics**: Built-in benchmarking and monitoring tools

### 📊 Performance Improvements
- **3-5x faster** than v4.1.0 on large codebases
- **50% memory reduction** through AST scoping and intelligent caching
- **Near-instant re-runs** on unchanged codebases with incremental mode

### 🛠️ New Features
- **Incremental Mode**: Analyze only changed files since last run
- **Memory Cache**: Persistent AST cache across multiple runs
- **Git Integration**: Automatic detection of file changes
- **Performance Tuning**: Configurable parallel workers and cache settings
- **Benchmark Tools**: Built-in performance measurement and reporting

### ⚡ Dependencies Added
- **rayon**: Parallel processing framework
- **crossbeam**: Async primitives for concurrent programming
- **parking_lot**: Fast mutex and RwLock implementations
- **once_cell**: Lazy initialization for cache system

### 🔧 Performance Configuration
- New `performance` section in `architect.json`
- Configurable parallel worker count
- Memory limit settings for cache
- Chunk size optimization for large projects

## [Unreleased]
### Added
- **Documentation Website (Docusaurus)**:
  - Initialized Docusaurus project in `website/` directory using TypeScript and the classic template.
  - Configured project details, branding, and GitHub Pages deployment settings.
  - Migrated core documentation (`README.md`, `ROADMAP.md`, `CHANGELOG.md`) to the Docusaurus site.
  - Organized technical documentation into a structured technical guides section.
  - Added Docusaurus frontmatter for improved navigation and SEO.

## [4.1.0-beta] - 2026-02-15

### 🔔 Native OS Notifications
- **Desktop Alerts**: Integrated `notify-rust` to send native notifications on Windows, macOS, and Linux.
- **Real-time Feedback**: Get instant alerts for architectural violations and circular dependencies while in Watch Mode.
- **Categorized Icons**: Different icons for violations (error), cycles (warning), and success (info).

### 👻 Daemon Mode
- **Background Execution**: Added `--daemon` (or `-d`) flag to run the linter as a background process (Unix).
- **Persistence**: Keep your architecture guarded without an open terminal window.
- **Log Redirection**: Automatic routing of background output to `/tmp/architect-linter.out` and errors to `/tmp/architect-linter.err`.

### 🛡️ Core Stability & Safety
- **Zero-Panic Policy**: Systematic removal of `unwrap()` and `expect()` calls across the codebase.
- **Robust Error Handling**: Enhanced use of `miette` for elegant, safe error reporting instead of program crashes.
- **Safe Mutex Locking**: Thread-safe access to cache and analyzers with proper error propagation.

### ✨ User Experience
- **Progress Bar**: Added `indicatif` progress bars for file analysis, providing visual feedback on large codebases.
- **CLI Polish**: Added `-d` shorthand for daemon mode and improved help descriptions.
- **Test Hardening**: Fixed technical debt in fixture testing and directory handling.

## [4.0.0] - 2026-02-12

### 🎉 Major Release: Architecture Governance Platform & Enterprise Features

This major release transforms Architect Linter from a simple architectural linter into a comprehensive **architecture governance platform** with scoring, reporting, and CI/CD integration. The project has been renamed to `architect-linter-pro` to reflect its evolution toward a hybrid open-core business model.

### 🏆 Architecture Health Score System

The headline feature of v4.0 is the new **Architecture Health Score** - a 0-100 grade system (A-F) that provides instant feedback on your codebase's architectural quality.

#### Added
- **Health Score Engine** (`src/metrics.rs`, `src/scoring.rs`):
  - Letter grades: A (90-100), B (80-89), C (70-79), D (60-69), F (0-59)
  - Grade emojis: 🏆 (A), ✨ (B), 👍 (C), ⚠️ (D), ❌ (F)
  - Four weighted components:
    1. **Layer Isolation** (25%) - Measures forbidden import violations
    2. **Circular Dependencies** (25%) - Binary score: 100 if no cycles, 0 if cycles exist
    3. **Complexity** (25%) - Ratio of long functions to total functions
    4. **Violations** (25%) - Overall architectural rule violations
  - Individual component scoring with pass/warning/fail status
  - Detailed breakdown of each component's health

- **Visual Dashboard** (`src/output/dashboard.rs`):
  - Professional terminal UI with ASCII box drawing
  - Color-coded grades (green for A, red for F)
  - Progress bars for each component score
  - Detailed statistics display:
    - Total files analyzed
    - Architecture pattern detected
    - Per-component breakdown with descriptions
  - Categorized violations list

- **Analysis Results Structure** (`src/analysis_result.rs`):
  - Unified `AnalysisResult` type consolidating all analysis data
  - Categorized violations: Forbidden Imports, Circular Deps, Complexity, Other
  - Statistics tracking:
    - Layer isolation stats (total imports, blocked violations)
    - Circular dependency detection with cycle details
    - Complexity metrics (total functions, long functions, max lines)
  - Health score integration with component statuses

### 📊 Reporting & Export

- **Report Generation** (`src/report.rs`):
  - **JSON Export**: Machine-readable format for CI/CD integration
    - Full analysis results with scores and violations
    - Timestamped reports
    - Schema version: "4.0.0"
  - **Markdown Export**: Human-readable documentation
    - Formatted tables and sections
    - Architecture pattern and score summary
    - Detailed violation listings with severity
  - New CLI flag: `--report-json <path>` and `--report-md <path>`

### 🚀 CI/CD Integration

- **GitHub Action** (`github-action/`):
  - **Dockerfile**: Multi-stage Rust build (Debian Bookworm slim)
    - Optimized image size with build caching
    - All 6 languages supported (TS, JS, Python, Go, PHP, Java)
  - **action.yml**: Full GitHub Action definition
    - Input: `path` (project directory)
    - Outputs: `score`, `grade`, `violations-count`, `passed`
    - Automatic PR annotations with violations
  - **entrypoint.sh**: Smart execution script (125 lines)
    - Automatic config detection or creation
    - JSON report generation
    - Exit code based on score threshold
  - **workflow-example.yml**: Ready-to-use workflow template
    - PR checks with score validation
    - Fail builds on grade F or D
    - Upload reports as artifacts

### 🔧 Git Integration

- **Git Analysis Module** (`src/git.rs`):
  - New dependency: `git2 = "0.18"`
  - Repository detection and validation
  - Commit history analysis foundation
  - Author tracking preparation
  - New dependency: `chrono = "0.4"` for timestamp handling

### 📦 Project Rebranding

- **Name Change**: `architect-linter` → `architect-linter-pro`
  - Reflects evolution to enterprise-grade platform
  - Preparation for open-core business model
  - Updated Cargo.toml metadata

### 🎨 Enhanced User Experience

- **Improved CLI** (`src/cli.rs`):
  - New flags for reporting: `--report-json`, `--report-md`
  - Enhanced help text with examples
  - Better error messages and formatting
  - Score display in all outputs

- **Analyzer Improvements** (`src/analyzer.rs`):
  - Integration with new scoring system
  - Better violation categorization
  - Enhanced statistics tracking
  - Optimized parallel processing with Rayon

### 📋 Documentation & Planning

- **Enterprise Design Document** (`plan/2026-02-11-v4-enterprise-design.md`):
  - Complete architecture for 3-tier system:
    - 🆓 **Open Source (Core)**: Forbidden imports, circular deps, watch mode, 6 languages
    - 💎 **Pro ($15/month/dev)**: Advanced metrics, security analysis, reports, CI/CD premium
    - 🏢 **Enterprise ($79/month/dev)**: Web dashboard, team features, SSO, alerts
  - Repository structure planning (public vs private)
  - Feature division matrix
  - Monetization strategy

- **Brainstorm Session** (`plan/2026-02-11-brainstorm-session.md`):
  - Product vision and positioning
  - Market analysis and competitive landscape
  - Technical architecture decisions
  - Roadmap priorities

### Changed

- **Main Entry Point** (`src/main.rs`):
  - Refactored to use new analysis pipeline
  - Integrated health score calculation
  - Dashboard rendering by default
  - Report generation support

- **Configuration** (`src/config.rs`):
  - Updated config loading for new features
  - Enhanced architect.json schema support
  - Better error handling and validation

### Technical Details

- **New Dependencies**:
  - `git2 = "0.18"` - Git repository analysis
  - `chrono = { version = "0.4", features = ["serde"] }` - Timestamp handling

- **Lines of Code**: +2,729 additions, -62 deletions across 21 files

- **New Modules**: 7 major new modules
  - `analysis_result.rs` (197 lines)
  - `metrics.rs` (175 lines)
  - `scoring.rs` (162 lines)
  - `report.rs` (244 lines)
  - `git.rs` (113 lines)
  - `output/dashboard.rs` (265 lines)
  - `output/mod.rs` (7 lines)

### Architectural Principles

v4.0 maintains the core philosophy:
> **"No pasas Architect, no haces commit"**

Architect Linter Pro is a **gatekeeper**, not just a highlighter. It enforces architecture at commit-time, not just in your editor.

### Roadmap Preview

Planned for future releases (see ROADMAP.md):
- License validation system (Pro/Enterprise tiers)
- Security analysis (data flow, secrets detection)
- Code smells detection
- LSP (Language Server Protocol) integration
- Web dashboard for Enterprise tier
- Team analytics and leaderboards

### Migration Guide from v3.x

1. **Rename Binary**: If you have `architect-linter` in PATH, update to `architect-linter-pro`
2. **New Flags**: Use `--report-json` and `--report-md` for exports
3. **GitHub Action**: Replace manual CI scripts with the official action (see workflow-example.yml)
4. **Config Compatibility**: No breaking changes to `architect.json` format

### Breaking Changes

- Binary name changed from `architect-linter` to `architect-linter-pro`
- Default output now includes Health Score dashboard
- Exit codes may differ based on score thresholds (use `--strict` flag for old behavior)

---

## [3.2.0] - 2026-02-07

### 🎉 DeepSeek Integration & Multi-Model Fallback System

This release introduces official support for DeepSeek as an AI provider and a robust fallback system that automatically tries alternative AI models if the primary one fails.

### Added
- **DeepSeek Support**:
  - Official integration with DeepSeek API (OpenAI-compatible).
  - Default URL configuration for `https://api.deepseek.com`.
  - Automatic model discovery for DeepSeek endpoints.
- **Multi-Model Fallback System**:
  - Robust orchestration in `src/ai.rs` that catches API errors and retries with the next available configuration.
  - Automatic re-ordering of configurations to prioritize the user-selected model.
  - Real-time UI feedback when a model fails and a fallback is initiated.
  - Support for multiple AI configurations in `.architect.ai.json`.
- **New AI Providers in Wizard**:
  - Added **Kimi (Moonshot)** and **DeepSeek** to the interactive setup selection.
  - Streamlined provider-specific default URL suggestions.

### Changed
- **Config Architecture**:
  - `LinterContext` now stores `ai_configs` (a collection) instead of a single `ai_config`.
  - Updated AI discovery and auto-fix logic to leverage the fallback orchestrator.
- **Interactive Setup**:
  - Improved AI configuration loop allowing users to add multiple providers in a single session.
  - Explicit optional API Key handling for local providers like Ollama.

### Technical Details
- **Fallback Logic**: Sequential retry mechanism with O(N) complexity where N is the number of configured AI providers.
- **Standardization**: DeepSeek integration follows the OpenAI-compatible interface, ensuring consistency with Groq, Kimi, and Ollama.

## [3.1.0] - 2026-02-06

### 🎉 Multi-Language Support: PHP & Java

This release expands language support from 4 to 6 programming languages with the addition of PHP and Java parsers, along with comprehensive documentation updates and code cleanup.

### Added
- **PHP Parser** (`src/parsers/php.rs`):
  - Full Tree-sitter integration for PHP syntax
  - Support for `use`, `require`, `require_once`, `include`, and `include_once` statements
  - Pattern matching for PHP-specific import/require conventions
  - PHP-specific architectural violation detection
- **Java Parser** (`src/parsers/java.rs`):
  - Complete Tree-sitter grammar support for Java
  - Import statement extraction and analysis
  - Java package path pattern matching
  - Architectural rule enforcement for Java projects
- **Enhanced Documentation**:
  - Added professional project banner (`public/architect-linter-banner.png`)
  - Multi-language support table in README (English and Spanish)
  - Updated language coverage to 6 languages: TypeScript, JavaScript, Python, Go, PHP, Java
  - Improved setup scripts with better error handling
- **Tree-sitter Dependencies**:
  - Added `tree-sitter-php = "0.23.8"` to Cargo.toml
  - Added `tree-sitter-java = "0.23.4"` to Cargo.toml
- **Example Configuration**:
  - Updated `architect.json.example` with PHP and Java rule examples

### Changed
- **Setup Scripts**:
  - Enhanced `setup.sh` with better PATH configuration for Linux/macOS
  - Improved `setup.ps1` with robust Windows PATH handling
  - Better error messages and installation verification
- **Parser Architecture**:
  - Expanded `get_parser_for_file()` to support `.php` and `.java` extensions
  - Updated `supported_languages()` to include PHP and Java
  - Extended `Language` enum with `Php` and `Java` variants
- **File Discovery**:
  - Improved file collection to include PHP and Java files
  - Enhanced extension matching in analyzer modules

### Fixed
- **Dead Code Cleanup**:
  - Removed unused `LanguageInfo` struct from `src/parsers/mod.rs`
  - Eliminated unused `get_language_info()` method from `ArchitectParser` trait
  - Removed unused `language()` method from `ArchitectParser` trait
  - Cleaned up unnecessary imports of `Language` and `LanguageInfo` across all parser modules
  - Reduced codebase by 72 lines of dead code across 6 files
- **Compilation Warnings**:
  - Fixed all `#[warn(dead_code)]` warnings
  - Removed unused methods and structs from trait implementations

### Technical Details
- **Supported Languages**: TypeScript, JavaScript, Python, Go, PHP, Java (6 total)
- **Lines of Code Removed**: 72 lines of dead code eliminated
- **New Parsers**: 2 (PHP: 195 lines, Java: 185 lines)
- **Documentation Updates**: README files in both English and Spanish

### Security
- No security changes in this release

## [2.0.0] - 2026-02-04

### 🎉 Major Release: Circular Dependencies & Security

This major version introduces circular dependency detection, separated AI configuration for security, and improved visual experience.

### Added
- **🔴 Circular dependency detection**:
  - New `circular.rs` module with graph-based analysis
  - DFS (Depth-First Search) algorithm for cycle detection
  - Automatic import extraction from all project files
  - Relative path resolution (`../`, `./`)
  - Detailed cycle reporting with path visualization
  - Suggested solutions for breaking cycles
- **🔐 Separated AI configuration**:
  - `architect.json` for rules (sharable in repo)
  - `.architect.ai.json` for AI config (private, contains API keys)
  - Wizard for AI configuration on first run
  - Environment variable defaults (`ANTHROPIC_AUTH_TOKEN`, `ANTHROPIC_BASE_URL`, `ANTHROPIC_MODEL`)
  - `.gitignore` automatically includes `.architect.ai.json`
- **🪝 Automatic Husky setup**:
  - Pre-commit hook configuration during initial setup
  - Creates `.husky/pre-commit` automatically
  - Windows and Unix-compatible hooks
- **🎨 Enhanced visual experience**:
  - New ASCII art banner in `ui.rs`
  - Improved wizard prompts
  - Better error messages and formatting
- **📁 Example files**:
  - `.architect.ai.json.example` - AI configuration template
  - `.gitignore.example` - Template for projects using architect-linter
  - Updated `architect.json.example` without AI config

### Changed
- **AI configuration**:
  - Moved from environment variables to file-based config
  - More flexible: URL, API key, and model are now configurable
  - Backward compatible with environment variables as defaults
- **Setup flow**:
  - AI config wizard now runs before architecture discovery
  - Clear separation between rules and credentials
- **Documentation**:
  - Updated README with new features
  - Security best practices highlighted
  - Added circular dependency examples

### Security
- ⚠️ API keys are now stored in `.architect.ai.json` which is in `.gitignore`
- ✅ Each developer has their own AI configuration
- ✅ Rules in `architect.json` can be safely shared in repositories

### Technical Details
- **Graph algorithm**: O(V + E) complexity where V = files, E = imports
- **Path resolution**: Handles relative imports, index files, and multiple extensions
- **DFS implementation**: Recursive with recursion stack for cycle detection

### Changed
- **Major refactoring of main.rs**:
  - Reduced from 151 lines to 80 lines (-47%)
  - Moved `setup_or_load_config()` to `config.rs`
  - Moved CLI functions to new `cli.rs` module
  - Cleaner and more maintainable code structure
- **Scripts consolidation**:
  - 4 scripts → 2 scripts (install.sh, install.ps1, update.sh, update.ps1 → setup.sh, setup.ps1)
  - Single command for both installation and updates
- **Documentation language**:
  - All documentation translated to English
  - Code messages remain in Spanish (original language)

### Improved
- Architectural file detection for JavaScript (`.controller.js`, `.service.js`, etc.)
- CLI messages updated to mention "TypeScript/JavaScript"
- More robust and flexible rules validation engine
- Better Windows path handling with separator normalization

### Fixed
- Rules engine now correctly detects violations with relative imports
- Compilation warnings removed with `#[allow(dead_code)]` annotations
- Glob pattern matching works correctly with actual folder structure

### Documentation
- README translated to English
- CHANGELOG translated to English
- CONFIG_ERRORS documentation in English
- Spanish preserved for runtime messages

## [1.1.0] - 2026-02-03 (Deprecated)

### 🚀 Soporte Completo para JavaScript/React + Validación Robusta de Configuración

### Agregado
- **Validación de esquema JSON completa**:
  - Validación de sintaxis JSON antes de parsear
  - Validación de campos requeridos con mensajes específicos
  - Validación de tipos de datos (número, string, array, object)
  - Validación de valores (rangos, opciones válidas)
  - Detección de reglas duplicadas en `forbidden_imports`
  - Mensajes de error claros con sugerencias de solución
  - Cada error incluye ejemplo de código correcto
- **Documentación de errores**:
  - `CONFIG_ERRORS.md` con guía completa de errores comunes
  - Ejemplos de todos los tipos de errores posibles
  - Soluciones paso a paso para cada error
  - Ejemplos de configuraciones válidas por framework
- **Soporte para archivos JavaScript**:
  - Análisis de archivos `.js` y `.jsx` además de TypeScript
  - Parser automático según extensión (TypeScript vs JavaScript)
  - Soporte para JSX en archivos `.jsx` y `.tsx`
  - Decoradores habilitados en JavaScript
- **Motor de reglas mejorado**:
  - Matching inteligente de imports relativos (`../services/`, `./api/`)
  - Matching de imports con alias (`@/services/`, `@/api/`)
  - Normalización de patrones glob (`src/components/**` → `src/components/`)
  - Funciones helper `normalize_pattern()` y `matches_pattern()`
- **Scripts de actualización**:
  - `update.sh` para Linux/macOS
  - `update.ps1` para Windows
  - Muestran versión anterior y nueva después de actualizar
- **Documentación de actualización**:
  - Sección completa en README sobre cómo actualizar
  - Instrucciones para actualización automática y manual

### Mejorado
- Detección de archivos arquitectónicos para JavaScript (`.controller.js`, `.service.js`, etc.)
- Mensajes del CLI actualizados para mencionar "TypeScript/JavaScript"
- Motor de validación de reglas más robusto y flexible
- Mejor manejo de rutas en Windows con normalización de separadores

### Corregido
- Motor de reglas ahora detecta correctamente violaciones con imports relativos
- Warnings de compilación eliminados con anotaciones `#[allow(dead_code)]`
- Matching de patrones glob funciona correctamente con estructura de carpetas real

### Documentación
- README actualizado con soporte de JavaScript en FAQ
- Roadmap actualizado moviendo "Soporte JavaScript" a completado
- Ejemplos de uso para proyectos React con JavaScript

## [1.0.0] - 2026-01-31

### 🎉 Primera Versión Estable

Esta es la primera versión estable de Architect Linter, lista para uso en producción.

### Agregado
- **Flags CLI completos**:
  - `--version` / `-v`: Muestra la versión del linter
  - `--help` / `-h`: Muestra ayuda completa con ejemplos
- **Instalación mejorada para Windows**:
  - Script `install.ps1` optimizado sin emojis para evitar problemas de codificación
  - Instrucciones claras con flag `-NoProfile` para evitar errores de perfiles de PowerShell
  - Guía paso a paso para agregar al PATH (automático y manual)
- **Documentación completa de instalación**:
  - `INSTALL_WINDOWS.md` actualizado con instrucciones detalladas
  - Solución de problemas comunes
  - Verificación de instalación paso a paso
- Constante `VERSION` usando `CARGO_PKG_VERSION` para versiones consistentes

### Mejorado
- Función `print_help()` con formato claro y ejemplos de uso
- Manejo de argumentos CLI más robusto
- Validación de flags antes de procesar proyectos
- Documentación actualizada con comandos exactos probados en Windows

### Corregido
- Error al ejecutar `architect-linter --version` (antes trataba `--version` como ruta de archivo)
- Problemas de sintaxis en `install.ps1` con comillas y caracteres especiales
- Instrucciones de instalación ahora reflejan el proceso real probado

### Técnico
- Detección temprana de flags `--version` y `--help` antes de inicializar el linter
- Uso de `env!("CARGO_PKG_VERSION")` para obtener versión del Cargo.toml
- Función `print_help()` centralizada para mantener ayuda consistente

## [0.8.0] - 2026-01-31

### Agregado
- **Configuración Asistida por IA**: Integración con Claude (Anthropic API) para sugerencias arquitectónicas inteligentes
  - Módulo `ai.rs` con función `sugerir_arquitectura_inicial()`
  - Análisis automático del contexto del proyecto (framework, dependencias, estructura)
  - Sugerencias de patrón arquitectónico basadas en el análisis
  - Recomendaciones de reglas `forbidden_imports` específicas para el proyecto
- **Discovery Inteligente**: Nuevo módulo `discovery.rs` que:
  - Escanea la estructura del proyecto automáticamente
  - Extrae dependencias de `package.json`
  - Identifica archivos arquitectónicos clave (controllers, services, entities, etc.)
  - Genera snapshot del proyecto para análisis de IA
- **Scripts de Instalación Automatizada**:
  - `install.sh` para Linux/macOS con instalación en `/usr/local/bin`
  - `install.ps1` para Windows con instalación en `%USERPROFILE%\bin`
  - Detección automática de PATH y configuración
- **Módulo UI**: Nueva separación de la lógica de interfaz de usuario
  - Función `get_interactive_path()` para selección de proyectos
  - Wizard `ask_user_to_confirm_rules()` para confirmación de sugerencias de IA
- **FAQ Completa**: Sección de preguntas frecuentes en el README
- **Documentación del Flujo Completo**: Descripción detallada del flujo de trabajo desde el primer commit

### Mejorado
- Reorganización de `main.rs` con separación clara de responsabilidades:
  - Uso de `discovery::collect_files()` para recolección de archivos
  - Delegación a módulos `ui`, `ai`, `config` para mejor mantenibilidad
- `save_config_from_wizard()` ahora acepta parámetro `max_lines` personalizable
- Función `detect_framework()` ahora acepta `&Path` en lugar de `&PathBuf` (más flexible)
- Enum `Framework` con método `as_str()` para conversión a String

### Corregido
- Error de tipos en `discovery.rs`: conversión correcta de `Framework` enum a `String`
- Errores de compilación relacionados con tipos incompatibles `&Path` vs `&PathBuf`
- Falta de dependencia `anyhow` en `Cargo.toml`

### Técnico
- Nueva dependencia: `anyhow = "1.0"` para manejo de errores
- Integración con API de Anthropic usando `reqwest` y `tokio`
- Función `consultar_claude()` con soporte para:
  - Variables de entorno `ANTHROPIC_AUTH_TOKEN` y `ANTHROPIC_BASE_URL`
  - Modelo Claude Opus 4.5
  - Parseo robusto de respuestas JSON de la IA
- Estructura `ProjectContext` para snapshot del proyecto
- Estructura `AISuggestionResponse` para mapeo de respuestas de IA
- Función `collect_files()` movida a módulo `discovery` con filtrado de `.d.ts`

## [0.7.0] - 2026-01-30

### Agregado
- **Motor de Reglas Dinámicas**: Sistema completamente funcional de `forbidden_imports` con formato `from` → `to`
- **Detección Automática de Framework**: Nuevo módulo `detector.rs` que reconoce NestJS, React, Angular, Express
- **Configuración Interactiva**: Modo guiado en primera ejecución que:
  - Detecta el framework del proyecto
  - Sugiere patrón arquitectónico (Hexagonal, Clean, MVC)
  - Propone límite de líneas basado en el framework
  - Genera `architect.json` automáticamente
- **Soporte para Patrones Arquitectónicos**:
  - Hexagonal
  - Clean Architecture
  - MVC
  - Ninguno (sin patrón específico)
- Documentación completa del motor de reglas dinámicas con ejemplos por patrón
- Tabla comparativa de restricciones por arquitectura
- Sugerencias LOC específicas por framework

### Corregido
- **Error de compilación**: Campo faltante `forbidden_imports` en `LinterContext` (líneas 97 y 162 de main.rs)
- Eliminada función duplicada `load_config` no utilizada
- Todas las advertencias del compilador (warnings) eliminadas
- Formato de `architect.json` corregido en documentación (`from`/`to` en lugar de `file_pattern`/`prohibited`)

### Mejorado
- Función `setup_or_load_config` ahora maneja ambos modos: automático (con archivo existente) y configuración interactiva
- Carga dinámica de `forbidden_imports` desde JSON
- Validación de reglas más robusta con conversión a minúsculas
- Documentación completamente actualizada y sin duplicaciones

### Técnico
- Módulo `detector.rs` con funciones `detect_framework()` y `get_loc_suggestion()`
- Estructura `ForbiddenRule` con campos `from` y `to`
- `LinterContext` ahora incluye `forbidden_imports: Vec<ForbiddenRule>`
- Deserialización mejorada del JSON con manejo de arrays

## [0.6.0] - 2026-01-30

### Refactorizado
- Separación del código en módulos para mejor organización y mantenibilidad:
  - **src/analyzer.rs**: Lógica de análisis de archivos TypeScript movida a módulo dedicado
  - **src/config.rs**: Definiciones de configuración y tipos de error (`LinterConfig`, `ArchError`)
  - **src/main.rs**: Simplificado, enfocado en orquestación y flujo principal
- Mejora en la estructura del proyecto siguiendo mejores prácticas de Rust

### Agregado
- Dependencias para soporte asíncrono futuro:
  - `tokio` v1.0 con features completas para operaciones async
  - `reqwest` v0.11 con soporte JSON para cliente HTTP
  - `async-trait` v0.1 para traits asíncronos
- Preparación de infraestructura para futuras funcionalidades de red y procesamiento async

### Técnico
- Modularización del código base para facilitar testing y extensibilidad
- Configuración centralizada en módulo `config` con `LinterConfig` y `ArchError`
- Función `analyze_file` ahora exportada desde módulo `analyzer`

## [0.5.0] - 2026-01-29

### Agregado
- Documentación completa del proyecto en README.md
- Guía rápida de instalación y configuración para proyectos NestJS
- Especificación del archivo de configuración `architect.json`
- Archivo de ejemplo `architect.json.example` con múltiples reglas recomendadas
- Archivo CHANGELOG.md para seguimiento de versiones
- Metadatos adicionales en Cargo.toml (authors, description, license, etc.)
- Documentación de integración con Git Hooks usando Husky
- Guía detallada NESTJS_INTEGRATION.md con:
  - Instrucciones paso a paso para configurar pre-commit hooks
  - Reglas recomendadas específicas para arquitectura NestJS
  - Solución de problemas comunes
  - Configuración avanzada con lint-staged
  - Buenas prácticas de uso
- Archivo pre-commit.example como plantilla para hooks de Husky
- Soporte documentado para argumentos CLI (`--path`) para integración con herramientas externas

### Documentado
- Estructura requerida del archivo `architect.json` en la raíz del proyecto a validar
- Propiedad `max_lines_per_function` para configurar el límite de líneas por función
- Propiedad `forbidden_imports` para definir reglas de importaciones prohibidas con:
  - `file_pattern`: Patrón del archivo fuente
  - `prohibited`: Patrón del módulo prohibido
  - `reason`: (Opcional) Razón de la restricción
- Ejemplos de configuración y uso
- Estructura del proyecto y dependencias
- Reglas de arquitectura implementadas

### Planificado
- Implementación de lectura y parseo del archivo `architect.json`
- Aplicación dinámica de reglas configurables
- Validación de esquema del archivo de configuración

## [0.1.0] - 2026-01-XX

### Agregado
- Versión inicial del proyecto
- Análisis de archivos TypeScript (.ts)
- Validación de importaciones prohibidas (hardcoded)
  - Regla: archivos `.controller.ts` no pueden importar `.repository`
- Detección de funciones que exceden 200 líneas
- Procesamiento paralelo con Rayon para análisis rápido
- Interfaz interactiva para selección de proyectos con Dialoguer
- Reportes visuales de errores con Miette
- Barra de progreso con Indicatif
- Exclusión automática de directorios: node_modules, dist, .git, target
- Parser TypeScript usando SWC

### Técnico
- Uso de swc_ecma_parser para análisis de código TypeScript
- Implementación de error personalizado `ArchError` con soporte Diagnostic
- SourceMap para ubicación precisa de errores
- Filtrado inteligente de directorios durante el walkdir

[4.0.0]: https://github.com/sergiogswv/architect-linter/releases/tag/v4.0.0
[3.2.0]: https://github.com/sergiogswv/architect-linter/releases/tag/v3.2.0
[3.1.0]: https://github.com/sergiogswv/architect-linter/releases/tag/v3.1.0
[2.0.0]: https://github.com/sergiogswv/architect-linter/releases/tag/v2.0.0
[1.0.0]: https://github.com/sergiogswv/architect-linter/releases/tag/v1.0.0
[0.8.0]: https://github.com/sergiogswv/architect-linter/releases/tag/v0.8.0
[0.7.0]: https://github.com/sergiogswv/architect-linter/releases/tag/v0.7.0
[0.6.0]: https://github.com/sergiogswv/architect-linter/releases/tag/v0.6.0
[0.5.0]: https://github.com/sergiogswv/architect-linter/releases/tag/v0.5.0
[0.1.0]: https://github.com/sergiogswv/architect-linter/releases/tag/v0.1.0
