---
title: Go (Standard & Frameworks) Guide
sidebar_position: 4
---

# Go Integration Guide

Architect Linter Pro supports Go codebases, focusing on package organization and dependency management.

## Package-Based Architecture

In Go, architecture is often defined by how packages interact. A common pattern is to follow a **"Standard Package Layout"** or **Hexagonal Architecture**.

### 1. Protecting the Internal Layer

A common goal in Go is to prevent entry points (like `cmd/` or `api/` handlers) from bypassing the business logic.

**Forbidden Rules for Go:**

```json
{
  "forbidden_imports": [
    {
      "from": "/cmd/",
      "to": "/internal/db",
      "reason": "Direct database access from entry points is forbidden. Use the service layer."
    },
    {
      "from": "/pkg/api",
      "to": "/internal/repository",
      "reason": "API handlers should depend on services, not repositories."
    }
  ]
}
```

### 2. Gin / Fiber / Echo Frameworks

Whether you use a framework or the standard library, the rules remain the same.

```json
{
  "forbidden_imports": [
    {
      "from": "handler",
      "to": "store",
      "reason": "Handlers must use a service or use-case layer."
    }
  ]
}
```

## Tree-sitter for Go

Our Go parser is built on Tree-sitter, providing accurate analysis of:
- `import "package/path"`
- Grouped imports: `import ( "a"; "b" )`
- Aliased imports: `import f "fmt"`

## Circular Dependencies in Go

Go's compiler already forbids circular dependencies between packages. However, **Architect Linter Pro** provides:
- **Visual Cycle Detection**: Better error messages and visualization than the standard compiler.
- **Cross-Layer Analysis**: Detect cycles that might not be purely package-level but break architectural layers.

## CI/CD for Go Projects

Integrate with your Go pipeline:

```bash
# In your CI script
architect-linter-pro . --incremental
```

## Best Practices

- **Internal Folder**: Use Go's `internal` directory convention to hide implementation details.
- **Dependency Inversion**: Use interfaces in your domain packages to avoid importing infrastructure packages.
- **Small Functions**: Go encourages simple, readable code. Set your `max_lines_per_function` to **30-40** for idiomatic Go.
