---
title: Architecture Design Principles
sidebar_label: Design Principles
---

# Designing Your Project Architecture

Best practices for defining layers, boundaries, and rules for your codebase.

## Layered Architecture

Organize code into clear layers:

- **Presentation/UI** - Components, pages, controllers
- **Business Logic** - Services, use cases, domain logic
- **Data Access** - Repositories, ORM, database queries
- **Infrastructure** - External services, APIs, file system

## Enforcing Boundaries

Use forbidden_imports to prevent layer crossing:

```json
{
  "forbidden_imports": [
    {
      "from": "src/presentation/**",
      "to": "src/data/**",
      "reason": "Presentation should go through services"
    }
  ]
}
```

## Domain-Driven Design

Organize by business domains:

```
src/
├── features/
│   ├── auth/
│   ├── users/
│   ├── products/
│   └── orders/
```

## Feature-Based Architecture

Group related code together:

```
src/
├── components/  (UI)
├── services/    (Business logic)
├── hooks/       (React specific)
├── utils/       (Shared utilities)
└── styles/      (CSS/theming)
```
