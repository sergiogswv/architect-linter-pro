# Architect Linter v4.0 - Enterprise Design

**Fecha:** 2026-02-11
**Estado:** Diseño aprobado
**Autor:** Sergio Guadarrama + Claude

---

## Resumen Ejecutivo

Architect Linter evoluciona de linter arquitectónico a plataforma completa de governance de arquitectura, manteniendo su esencia de "guardián de arquitectura" (no solo otro linter de editor).

### Modelo de Negocio: Híbrido (Open Core)

- **Open Source (Core):** Linting básico, reglas dinámicas, circular deps, watch mode
- **Pro ($15/mes/dev):** Métricas avanzadas, security, reports, CI/CD premium
- **Enterprise ($79/mes/dev):** Dashboard web, team features, SSO, alertas

### Principio Fundamental

> "No pasas Architect, no haces commit"

Architect es un **gatekeeper**, no un highlighter. Su poder está en que no se puede ignorar.

---

## 1. Arquitectura de Tiers

```
┌─────────────────────────────────────────────────────────────────┐
│                    ARCHITECT LINTER v4.0                        │
├─────────────────────────────────────────────────────────────────┤
│  🆓 OPEN SOURCE (Core)                                          │
│  ├── Forbidden imports engine                                   │
│  ├── Circular dependency detection                              │
│  ├── Basic complexity (max lines)                               │
│  ├── Watch mode                                                 │
│  ├── AI auto-fix (user provides API key)                        │
│  └── 6 languages: TS, JS, Python, Go, PHP, Java                 │
├─────────────────────────────────────────────────────────────────┤
│  💎 PRO ($15/mes/dev)                                           │
│  ├── Advanced metrics (cyclomatic, coupling, cohesion)          │
│  ├── Code smells detection                                      │
│  ├── Security analysis (data flow, secrets)                     │
│  ├── HTML/JSON/Markdown reports                                 │
│  └── CI/CD annotations (GitHub/GitLab)                          │
├─────────────────────────────────────────────────────────────────┤
│  🏢 ENTERPRISE ($79/mes/dev, mínimo 10 seats)                   │
│  ├── Todo lo de Pro                                             │
│  ├── Web dashboard multi-repo                                   │
│  ├── Historical analytics & trends                              │
│  ├── Team leaderboard & gamification                            │
│  ├── Slack/Email alerts                                         │
│  ├── SSO (SAML, OAuth)                                          │
│  ├── Audit logs                                                 │
│  └── Priority support                                           │
└─────────────────────────────────────────────────────────────────┘
```

---

## 2. Estructura de Repositorios

### Repo Público (Open Source)
```
github.com/sergiogswv/architect-linter
License: MIT

src/
├── core/                    # Engine base
│   ├── parser.rs            # Tree-sitter parsing
│   ├── rules.rs             # Forbidden imports engine
│   ├── circular.rs          # Cycle detection
│   └── config.rs            # Config loading
├── parsers/                 # 6 language parsers
├── cli.rs                   # CLI básico
└── lib.rs                   # Public API para extensions
```

### Repo Privado (Commercial)
```
github.com/sergiogswv/architect-linter-pro (PRIVATE)
License: Commercial

src/
├── metrics/                 # Complejidad, acoplamiento
├── security/                # Análisis de seguridad
├── smells/                  # Code smells detection
├── reports/                 # HTML/JSON exporters
├── lsp/                     # Language Server (opcional)
└── license.rs               # Validación de licencias

binaries/                    # Distribución compilada
└── dashboard/               # Web app (Enterprise)
```

### División de Features

| Feature | Público | Privado |
|---------|:-------:|:-------:|
| Forbidden imports engine | ✅ | |
| Circular dependency detection | ✅ | |
| Watch mode | ✅ | |
| AI auto-fix (user's API key) | ✅ | |
| 6 language parsers | ✅ | |
| Cyclomatic complexity | | ✅ |
| Coupling/Cohesion metrics | | ✅ |
| Code smells detection | | ✅ |
| Security analysis | | ✅ |
| HTML/JSON reports | | ✅ |
| Web dashboard | | ✅ |

---

## 3. Motor de Métricas Avanzadas (Pro)

### Arquitectura del Módulo

```rust
// src/metrics/mod.rs

pub struct MetricsEngine {
    pub cyclomatic: CyclomaticAnalyzer,
    pub coupling: CouplingAnalyzer,
    pub cohesion: CohesionAnalyzer,
    pub security: SecurityAnalyzer,
    pub code_smells: CodeSmellDetector,
}

pub struct FileMetrics {
    pub path: PathBuf,
    pub cyclomatic_complexity: u32,
    pub cognitive_complexity: u32,
    pub coupling_score: f32,        // 0.0 - 1.0
    pub cohesion_score: f32,        // 0.0 - 1.0
    pub lines_of_code: u32,
    pub maintainability_index: f32,
    pub technical_debt_minutes: u32,
    pub security_issues: Vec<SecurityIssue>,
    pub code_smells: Vec<CodeSmell>,
}
```

### Métricas Implementadas

| Métrica | Threshold | Descripción |
|---------|-----------|-------------|
| `max_cyclomatic_complexity` | 10 | Complejidad ciclomática por función |
| `max_cognitive_complexity` | 15 | Complejidad cognitiva (lectura) |
| `max_coupling_score` | 0.7 | Acoplamiento máximo permitido |
| `min_cohesion_score` | 0.5 | Cohesión mínima requerida |
| `max_technical_debt` | 60min | Deuda técnica máxima por archivo |

### Config Extended (architect.json Pro)

```json
{
  "max_lines_per_function": 40,
  "architecture_pattern": "Hexagonal",
  "forbidden_imports": [...],

  "metrics": {
    "max_cyclomatic_complexity": 10,
    "max_cognitive_complexity": 15,
    "max_coupling_score": 0.7,
    "min_cohesion_score": 0.5,
    "max_technical_debt_minutes": 60
  },
  "security": {
    "detect_secrets": true,
    "check_sensitive_data_flow": true,
    "dependency_vulnerabilities": true
  },
  "code_smells": {
    "detect_long_methods": { "max_lines": 40 },
    "detect_large_classes": { "max_lines": 300 },
    "detect_dead_code": true,
    "detect_duplicate_code": { "min_tokens": 50 }
  }
}
```

---

## 4. DX: CLI-First Strategy (Sin Extensión)

### Principio

> El poder de Architect está en que **no se puede ignorar**.

**NO hacer extensión de editor** porque:
- Se vuelve "solo otro linter"
- Los developers lo ignoran
- Pierde su esencia de guardián

### CLI Output Mejorado

```
╔═══════════════════════════════════════════════════════════════╗
║                    🏗️  ARCHITECT LINTER                       ║
╠═══════════════════════════════════════════════════════════════╣
║  Project: my-api                                              ║
║  Pattern: Hexagonal Architecture                              ║
║  Files:   142 analyzed                                        ║
╠═══════════════════════════════════════════════════════════════╣
║                                                               ║
║  📊 ARCHITECTURE HEALTH: 78/100  ████████████░░░░  🟡        ║
║                                                               ║
║  ├── ✅ Layer isolation: 100%                                ║
║  ├── ✅ No circular deps: Pass                               ║
║  ├── ⚠️  Complexity:     3 functions > 10 (warning)          ║
║  └── ❌ Violations:      2 layer violations (blocked)        ║
║                                                               ║
╠═══════════════════════════════════════════════════════════════╣
║  🚫 VIOLATIONS (must fix to commit)                           ║
╠═══════════════════════════════════════════════════════════════╣
║                                                               ║
║  1. src/domain/user.entity.ts:12                              ║
║     └─ domain → infrastructure                                ║
║     └─ import { UserRepository } from '../infrastructure/...' ║
║                                                               ║
║  2. src/application/services/auth.service.ts:45              ║
║     └─ application → controllers                              ║
║     └─ import { AuthController } from '../controllers/...'   ║
║                                                               ║
╠═══════════════════════════════════════════════════════════════╣
║  💡 Run 'architect-linter --fix' for AI-powered suggestions  ║
╚═══════════════════════════════════════════════════════════════╝
```

### Git Hook Robusto

```bash
#!/bin/sh
. "$(dirname "$0")/_/husky.sh"

echo "🏗️  Architect Linter - Guardián de Arquitectura"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

architect-linter --staged

if [ $? -ne 0 ]; then
    echo ""
    echo "🚫 COMMIT BLOQUEADO"
    echo "   El código no cumple con la arquitectura definida."
    echo ""
    echo "💡 Opciones:"
    echo "   • architect-linter --fix     → Auto-fix con IA"
    echo "   • git commit --no-verify     → Saltar (solo emergencias)"
    exit 1
fi

echo "✅ Arquitectura validada. Commit permitido."
```

### GitHub Action (Pro)

```yaml
# .github/workflows/architect.yml
name: Architect Linter

on: [pull_request]

jobs:
  architect:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: architect-linter/action@v1
        with:
          license-key: ${{ secrets.ARCHITECT_LICENSE }}
```

---

## 5. Dashboard Enterprise (Web App)

### Stack Tecnológico

| Componente | Tecnología |
|------------|------------|
| Frontend | Next.js 14 + TypeScript + Tailwind + Recharts |
| Backend | Rust (Axum) o Node.js (Fastify) |
| Database | PostgreSQL + TimescaleDB (time-series) |
| Auth | NextAuth.js (GitHub OAuth, Google, SAML) |
| Deploy | Vercel (frontend) + Railway/Fly.io (backend) |

### Pantallas Principales

#### Overview (Dashboard Home)
- Score de organización (salud general)
- Conteo de repos, developers, archivos, issues
- Tendencia 30 días
- Top repos por score
- Recent violations

#### Vista de Repositorio
- Score del repo
- Metrics breakdown (layer isolation, complexity, coupling)
- Layer map (dominio, infra, controllers)
- History (90 días)
- Hotspots (archivos problemáticos)

#### Team View (Gamificación)
- Leaderboard de architects
- Stats del equipo
- Violations por developer
- Tiempo promedio de fix

### Integración CLI → Dashboard

```json
// .architect.ai.json (Enterprise)
{
  "dashboard_url": "https://dashboard.architect-linter.com",
  "org_id": "acme-corp",
  "api_key": "arch_live_xxx",
  "upload_on_ci": true
}
```

### Alertas Slack

```
🏗️ Architect Linter

⚠️  Architecture degradation detected in user-service

Score dropped: 95 → 78 (-17 points)

• 2 new layer violations
• 1 circular dependency introduced

Caused by: PR #234 (auth refactor)
Author: @juan

[View Dashboard] [View PR]
```

---

## 6. Roadmap de Implementación

### Timeline General

```
2026
│
├── MARZO ──────────────────────────────────────────
│   └── Fase 1: Core++ (v4.0 OSS)
│       • CLI UX mejorado
│       • Reports JSON/Markdown
│       • GitHub Action
│
├── ABRIL - JUNIO ──────────────────────────────────
│   └── Fase 2: Pro (v1.0 Pro)
│       • Métricas avanzadas
│       • Security analysis
│       • HTML reports
│       • Sistema de licencias
│
├── JULIO - OCTUBRE ────────────────────────────────
│   └── Fase 3: Enterprise (v1.0 Enterprise)
│       • Dashboard web
│       • SSO/Auth
│       • Team features
│       • Alertas
│
└── NOVIEMBRE+ ─────────────────────────────────────
    └── Iteración basada en feedback
```

### Fase 1: Core++ (1-2 meses) - Open Source

| Semana | Tarea | Entregable |
|--------|-------|------------|
| 1-2 | CLI UX redesign | Output mejorado con score |
| 3-4 | Reportes básicos | `--report json` y `--report markdown` |
| 5-6 | GitHub Action | Action oficial para CI/CD |
| 7-8 | Pre-commit mejorado | Hook con `--staged` |

### Fase 2: Pro (2-3 meses) - Repo Privado

| Semana | Tarea | Entregable |
|--------|-------|------------|
| 1-3 | Motor de métricas | Ciclómatica, acoplamiento, cohesión |
| 4-5 | Code smells | Long methods, large classes, dead code |
| 6-7 | Security analysis | Secrets detection, data flow |
| 8-9 | HTML Reports | Reportes visuales |
| 10-11 | License system | Validación de licencias |
| 12 | Packaging | Binarios para distribuir |

### Fase 3: Enterprise (3-4 meses) - Dashboard

| Semana | Tarea | Entregable |
|--------|-------|------------|
| 1-3 | Backend API | API REST para métricas |
| 4-6 | Dashboard MVP | Overview, repos list |
| 7-8 | Auth + SSO | NextAuth, GitHub OAuth, SAML |
| 9-10 | Team features | Leaderboard, member management |
| 11-12 | Alertas | Slack, Email integrations |
| 13-14 | Trends | Gráficos históricos |
| 15-16 | Polish | UX, docs, onboarding |

### Prerequisitos por Fase

**Fase 1:**
- [x] Repo público existente
- [ ] Tests suite robusta
- [ ] CI/CD configurado

**Fase 2:**
- [ ] Repo privado creado
- [ ] Sistema de pagos (Stripe)
- [ ] License server básico
- [ ] Build pipeline para binarios

**Fase 3:**
- [ ] Infra cloud (Vercel, Railway)
- [ ] Dominio (architect-linter.com)
- [ ] OAuth apps (GitHub, Google)
- [ ] Slack app para alertas

---

## 7. Decisions Log

| Decisión | Opción elegida | Alternativas descartadas |
|----------|----------------|--------------------------|
| Modelo de negocio | Híbrido (Open Core) | 100% open source, 100% privado |
| Estructura repos | Dual (público + privado) | Single repo con feature flags |
| IDE integration | NO hacer extensión | VS Code extension, LSP server |
| Posicionamiento | Guardián/Gatekeeper | Linter tradicional |
| Dashboard | Web app | Solo CLI |

---

## 8. Next Steps

1. [ ] Crear repo privado `architect-linter-pro`
2. [ ] Configurar Stripe para pagos
3. [ ] Diseñar sistema de licencias
4. [ ] Comenzar Fase 1: CLI UX redesign
5. [ ] Configurar CI/CD con tests

---

*Documento generado el 2026-02-11*
