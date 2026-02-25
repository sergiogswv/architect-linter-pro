---
title: Next.js Architecture Template
sidebar_label: Next.js
---

# Next.js Architecture Template

Pre-configured architectural rules for Next.js projects.

## Quick Start

```bash
architect --template nextjs
```

## Layer Structure

Next.js combines frontend and backend in a single codebase:

- **Pages/App** - Route definitions and layouts
- **API Routes** - Backend endpoints
- **Components** - UI components
- **Services** - Business logic
- **Utils** - Helper functions
- **Lib** - Shared libraries

```
src/
├── pages/
│   ├── api/
│   │   ├── auth/
│   │   ├── users/
│   │   └── products/
│   ├── _app.tsx
│   ├── _document.tsx
│   ├── index.tsx
│   ├── auth/
│   └── dashboard/
├── components/
│   ├── common/
│   ├── features/
│   └── layout/
├── services/
│   ├── api.ts
│   └── auth.ts
├── lib/
│   ├── prisma.ts
│   └── utils.ts
└── styles/
```

## Pre-configured Rules

```json
{
  "max_lines_per_function": 50,
  "architecture_pattern": "Ninguno",
  "forbidden_imports": [
    {
      "from": "src/pages/**",
      "to": "src/pages/api/**",
      "reason": "Pages should use services, not import API routes directly"
    },
    {
      "from": "src/pages/api/**",
      "to": "src/components/**",
      "reason": "API routes must not import React components"
    },
    {
      "from": "src/components/**",
      "to": "src/pages/**",
      "reason": "Components should not import specific pages"
    },
    {
      "from": "src/lib/**",
      "to": "src/pages/**",
      "reason": "Shared libraries should not depend on page implementations"
    }
  ]
}
```

## Best Practices

- Keep pages thin, move logic to services
- Use API routes for backend logic
- Separate frontend and backend concerns
- Use lib/ for shared utilities
- Keep components reusable
- Use getServerSideProps/getStaticProps for data

## Common Issues

### Circular Dependencies

Next.js can have circular dependencies between pages and components:

```typescript
// Break the cycle with lazy loading
const PageComponent = dynamic(() => import('./component'));
```

### Server vs Client Code

Use 'use client' directive and server components appropriately:

```typescript
'use client' // This component runs on client

// API routes run on server
// pages/api/endpoint.ts
```
