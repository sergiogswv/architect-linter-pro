# Init Command & Framework Templates — Design Document

**Date:** 2026-02-23
**Status:** Approved

---

## Goal

Add `architect-linter-pro init` to reduce time-to-value from "just installed" to "linter running on my project" to under 2 minutes. The command auto-detects the framework, asks the user which architectural pattern they use, shows a preview, and writes `architect.json`.

---

## User Flow

```
$ architect-linter-pro init

Detected: NestJS (package.json)

¿Qué patrón arquitectónico usas?
  1. Hexagonal  (domain/ application/ infrastructure/)
  2. Clean Architecture  (entities/ use-cases/ adapters/ frameworks/)
  3. Layered  (controllers/ services/ repositories/)

Patrón [1]: 2

Vista previa de architect.json:
{
  "pattern": "custom",
  "forbidden_imports": [ ... ]
}

¿Crear architect.json? [Y/n]: Y

✅ architect.json creado. Ejecuta `architect-linter-pro .` para analizar tu proyecto.
```

If `architect.json` already exists:
```
❌ Ya existe architect.json. Usa --force para sobreescribir.
```

**Flags:**
- `--force` — overwrite existing architect.json
- `--path <dir>` — target directory (default: current directory)

---

## Framework Detection

| Framework | Detection signals |
|-----------|-------------------|
| Next.js | `next.config.js` / `next.config.ts` present, OR `next` in package.json dependencies |
| NestJS | `@nestjs/core` in package.json dependencies |
| Express | `express` in package.json (without Next or Nest signals) |
| Django | `manage.py` present, OR `django` in requirements.txt / pyproject.toml |
| Spring Boot | `pom.xml` with `spring-boot`, OR `build.gradle` with `spring-boot` |

If no framework is detected → ask the user directly with all 5 options listed.

---

## Patterns per Framework

| Framework | Option 1 | Option 2 | Option 3 |
|-----------|----------|----------|----------|
| Next.js | Feature-based | Layered | — |
| NestJS | Hexagonal | Clean Architecture | Layered |
| Express | MVC | Hexagonal | Feature-based |
| Django | MVT (standard) | Service Layer | — |
| Spring Boot | Layered MVC | Hexagonal | — |

Always ask the user — even when there's an obvious default. This avoids generating wrong configs silently.

---

## Templates (11 total)

### Next.js — Feature-based
```json
{
  "pattern": "custom",
  "forbidden_imports": [
    { "from": "/features/", "to": "/app/", "severity": "error",
      "reason": "Features must not import from app layer" },
    { "from": "/features/*/", "to": "/features/*/", "severity": "warning",
      "reason": "Features should be independent from each other" },
    { "from": "/components/", "to": "/features/", "severity": "error",
      "reason": "Shared components must not depend on specific features" }
  ]
}
```

### Next.js — Layered
```json
{
  "pattern": "custom",
  "forbidden_imports": [
    { "from": "/components/", "to": "/lib/server/", "severity": "error",
      "reason": "Client components must not import server-only lib" },
    { "from": "/pages/", "to": "/components/ui/", "severity": "warning",
      "reason": "Pages should use feature components, not raw UI primitives" }
  ]
}
```

### NestJS — Hexagonal
```json
{
  "pattern": "custom",
  "forbidden_imports": [
    { "from": "/domain/", "to": "/application/", "severity": "error",
      "reason": "Domain must not depend on application layer" },
    { "from": "/domain/", "to": "/infrastructure/", "severity": "error",
      "reason": "Domain must not depend on infrastructure" },
    { "from": "/application/", "to": "/infrastructure/", "severity": "error",
      "reason": "Application layer must not depend on infrastructure directly" }
  ]
}
```

### NestJS — Clean Architecture
```json
{
  "pattern": "custom",
  "forbidden_imports": [
    { "from": "/entities/", "to": "/use-cases/", "severity": "error",
      "reason": "Entities must not depend on use cases" },
    { "from": "/entities/", "to": "/adapters/", "severity": "error",
      "reason": "Entities must not depend on adapters" },
    { "from": "/use-cases/", "to": "/adapters/", "severity": "error",
      "reason": "Use cases must not depend on adapters" },
    { "from": "/use-cases/", "to": "/frameworks/", "severity": "error",
      "reason": "Use cases must not depend on frameworks" }
  ]
}
```

### NestJS — Layered
```json
{
  "pattern": "custom",
  "forbidden_imports": [
    { "from": "/controllers/", "to": "/repositories/", "severity": "error",
      "reason": "Controllers must go through services, not access repositories directly" },
    { "from": "/repositories/", "to": "/controllers/", "severity": "error",
      "reason": "Repositories must not depend on controllers" },
    { "from": "/repositories/", "to": "/services/", "severity": "error",
      "reason": "Repositories must not depend on services" }
  ]
}
```

### Express — MVC
```json
{
  "pattern": "custom",
  "forbidden_imports": [
    { "from": "/routes/", "to": "/models/", "severity": "error",
      "reason": "Routes must go through controllers, not access models directly" },
    { "from": "/models/", "to": "/controllers/", "severity": "error",
      "reason": "Models must not depend on controllers" },
    { "from": "/middleware/", "to": "/controllers/", "severity": "warning",
      "reason": "Middleware should not depend on specific controllers" }
  ]
}
```

### Express — Hexagonal
```json
{
  "pattern": "custom",
  "forbidden_imports": [
    { "from": "/domain/", "to": "/infrastructure/", "severity": "error",
      "reason": "Domain must not depend on infrastructure" },
    { "from": "/domain/", "to": "/adapters/", "severity": "error",
      "reason": "Domain must not depend on adapters" },
    { "from": "/application/", "to": "/infrastructure/", "severity": "error",
      "reason": "Application must not depend on infrastructure directly" }
  ]
}
```

### Express — Feature-based
```json
{
  "pattern": "custom",
  "forbidden_imports": [
    { "from": "/features/*/", "to": "/features/*/", "severity": "warning",
      "reason": "Features should be independent from each other" },
    { "from": "/shared/", "to": "/features/", "severity": "error",
      "reason": "Shared utilities must not depend on specific features" }
  ]
}
```

### Django — MVT
```json
{
  "pattern": "custom",
  "forbidden_imports": [
    { "from": "/templates/", "to": "/models/", "severity": "error",
      "reason": "Templates must not import models directly" },
    { "from": "/views/", "to": "/urls/", "severity": "warning",
      "reason": "Views should not import URL configuration" }
  ]
}
```

### Django — Service Layer
```json
{
  "pattern": "custom",
  "forbidden_imports": [
    { "from": "/views/", "to": "/models/", "severity": "warning",
      "reason": "Views should go through services, not access models directly" },
    { "from": "/services/", "to": "/views/", "severity": "error",
      "reason": "Services must not depend on views" },
    { "from": "/repositories/", "to": "/services/", "severity": "error",
      "reason": "Repositories must not depend on services" }
  ]
}
```

### Spring Boot — Layered MVC
```json
{
  "pattern": "custom",
  "forbidden_imports": [
    { "from": "/controller/", "to": "/repository/", "severity": "error",
      "reason": "Controllers must go through services, not access repositories directly" },
    { "from": "/repository/", "to": "/controller/", "severity": "error",
      "reason": "Repositories must not depend on controllers" },
    { "from": "/repository/", "to": "/service/", "severity": "error",
      "reason": "Repositories must not depend on services" }
  ]
}
```

### Spring Boot — Hexagonal
```json
{
  "pattern": "custom",
  "forbidden_imports": [
    { "from": "/domain/", "to": "/infrastructure/", "severity": "error",
      "reason": "Domain must not depend on infrastructure" },
    { "from": "/domain/", "to": "/application/", "severity": "error",
      "reason": "Domain must not depend on application layer" },
    { "from": "/application/", "to": "/infrastructure/", "severity": "error",
      "reason": "Application must not depend on infrastructure directly" }
  ]
}
```

---

## Code Architecture

```
src/
├── init/
│   ├── mod.rs          — orchestration: detect → ask pattern → preview → confirm → write
│   ├── detector.rs     — framework detection by file system signals
│   ├── prompts.rs      — terminal interaction (numbered menus, Y/n confirm)
│   └── templates/
│       ├── mod.rs      — Template struct, registry, lookup by (framework, pattern)
│       ├── nextjs.rs   — 2 templates
│       ├── nestjs.rs   — 3 templates
│       ├── express.rs  — 3 templates
│       ├── django.rs   — 2 templates
│       └── spring.rs   — 2 templates
├── main.rs             — add init subcommand routing
└── cli.rs              — add InitArgs { force: bool, path: Option<String> }
```

**No new dependencies.** Uses only `std::io`, `std::fs`, `std::path`. Templates stored as Rust string constants (no `include_str!` needed since they're small).

---

## Error Handling

| Situation | Behavior |
|-----------|----------|
| `architect.json` exists | Error message + hint to use `--force` |
| `--force` flag | Overwrites without backup |
| No framework detected | Falls through to manual selection menu |
| Write permission denied | Clear error with path |
| User cancels at confirmation | Exit 0, nothing written |

---

## Out of Scope (this version)

- Monorepo support (multiple architect.json files)
- Remote template fetching
- Custom pattern input (free-form)
- Template validation against real project structure
