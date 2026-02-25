---
title: Laravel (PHP) Guide
sidebar_position: 3
---

# Laravel Integration Guide

Architect Linter Pro supports PHP and is perfectly suited for Laravel projects, helping you maintain a clean separation between Controllers, Models, and Services.

## Common Laravel Patterns

Laravel projects often suffer from "Fat Models" or "Fat Controllers". Architect Linter Pro helps you decouple these layers by enforcing strict import rules.

### Preventing Direct DB Access in Controllers

Many teams prefer that Controllers only interact with a Service layer instead of using Eloquent models directly for complex queries.

**Rules for Laravel Decoupling:**

```json
{
  "forbidden_imports": [
    {
      "from": "App/Http/Controllers",
      "to": "App/Models",
      "reason": "Controllers should use Services or Repositories, not Models directly."
    }
  ]
}
```

### Protecting Domain Logic

If you are implementing Domain-Driven Design (DDD) in Laravel, you likely have a `Domain` folder that should be protected.

**Rules for Laravel DDD:**

```json
{
  "forbidden_imports": [
    {
      "from": "App/Domain",
      "to": "App/Infrastructure",
      "reason": "Domain logic must not depend on Infrastructure details."
    }
  ]
}
```

## Tree-sitter for PHP

The linter uses a high-performance Tree-sitter parser for PHP that supports:
- `use Namespace\Class;`
- `require`, `include`, `require_once`, `include_once`
- PSR-4 namespace resolution

## Watch Mode for PHP Developers

Keep your terminal open with:
```bash
architect-linter-pro --watch .
```
The linter will notify you instantly (via OS notifications) if you introduce a violation while refactoring your Laravel app.

## Summary

By using **Architect Linter Pro** with Laravel, you ensure that your "Ready-to-Code" framework doesn't turn into a "Big Ball of Mud" as the project grows.
