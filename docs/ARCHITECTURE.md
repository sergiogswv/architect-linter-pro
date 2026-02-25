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
