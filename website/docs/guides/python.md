---
title: Python (Django & FastAPI) Guide
sidebar_position: 2
---

# Python Integration Guide

Architect Linter Pro provides native support for Python using Tree-sitter, allowing you to enforce architecture in Django, FastAPI, and Flask projects.

## Django Architecture

Django follows a **Model-Template-View (MTV)** pattern. A common goal is to prevent business logic from leaking into templates or views, often by using a "Services" or "Domain" layer.

**Recommended rules for Django:**

```json
{
  "forbidden_imports": [
    {
      "from": "/models.py",
      "to": "/views.py",
      "reason": "Models should not depend on views."
    },
    {
      "from": "/services/",
      "to": "/views.py",
      "reason": "Business logic should be independent of the delivery mechanism."
    }
  ]
}
```

## FastAPI & Clean Architecture

FastAPI is often used with Clean Architecture. You can easily define rules to protect your entities and use cases.

**Rules for FastAPI Clean Arch:**

```json
{
  "forbidden_imports": [
    {
      "from": "/domain/",
      "to": "/api/",
      "reason": "Domain entities should not know about API routes or schemas."
    },
    {
      "from": "/domain/",
      "to": "/infrastructure/",
      "reason": "Domain should not depend on DB drivers or external clients."
    }
  ]
}
```

## Handling Python Imports

Architect Linter Pro supports both types of Python imports:
- `import module.path`
- `from module.path import something`

The engine resolves these paths accurately to ensure your rules are respected.

## CI/CD with GitHub Actions

You can use the official GitHub Action to validate your Python architecture:

```yaml
- name: Architect Linter
  uses: sergiogswv/architect-linter-pro@v4
  with:
    path: '.'
```

## Performance Tips

- **Virtual Environments**: The linter automatically ignores common folders like `venv`, `.venv`, and `__pycache__`.
- **Incremental Analysis**: Since Python projects can grow large, always use the `--incremental` flag in your local development or CI for faster runs.
