---
title: Frontend (React & Next.js) Guide
sidebar_position: 6
---

# Frontend Integration Guide

Architect Linter Pro is highly effective for modern frontend applications (React, Next.js, Vue, Angular) where component coupling can quickly become unmanageable.

## Common Frontend Patterns

Modern frontend architecture often revolves around **Feature-based organization** or **Layered Atomic Design**.

### 1. Feature Isolation

A common goal is to prevent a feature from reaching into the internals of another feature.

**Forbidden Rules for Features:**

```json
{
  "forbidden_imports": [
    {
      "from": "features/auth",
      "to": "features/shop",
      "reason": "Features should be isolated and only communicate through shared hooks or context."
    }
  ]
}
```

### 2. Protecting the UI Layer (Clean Architecture)

In a Clean Architecture setup, your UI components should not import business logic or API clients directly.

**Forbidden Rules for Clean UI:**

```json
{
  "forbidden_imports": [
    {
      "from": "components/ui",
      "to": "api/",
      "reason": "Presentational components should focus only on UI, not data fetching."
    },
    {
      "from": "components/common",
      "to": "store/",
      "reason": "Common components should be pure and independent of the global state."
    }
  ]
}
```

## Next.js (App Router)

With Next.js, it's crucial to manage the boundary between **Client Components** and **Server Components**.

```json
{
  "forbidden_imports": [
    {
      "from": ".server.tsx",
      "to": "framer-motion",
      "reason": "Server components cannot import libraries that require browser APIs."
    }
  ]
}
```

## Tree-sitter for JS/TS/JSX

The linter fully supports:
- ES Modules (`import/export`)
- Dynamic imports (`import()`)
- JSX/TSX syntax
- Alias resolution (e.g., `@/components/*`)

## Watch Mode & Hot Reloading

Run the linter alongside your dev server:
```bash
# Terminal 1
npm run dev

# Terminal 2
architect-linter-pro --watch .
```
Get instant **OS notifications** when a code change violates your frontend architecture patterns.

## Summary

Frontend architecture is often overlooked until it becomes a maintenance nightmare. **Architect Linter Pro** helps you enforce patterns like "Bulletproof React" or "Feature-Sliced Design" automatically.
