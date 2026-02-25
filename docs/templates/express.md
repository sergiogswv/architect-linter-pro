---
title: Express Architecture Template
sidebar_label: Express
---

# Express Architecture Template

Pre-configured architectural rules for Express.js projects.

## Quick Start

```bash
architect --template express
```

## Layer Structure

Express projects typically follow MVC or layered architecture:

- **Routes** - URL endpoint definitions
- **Controllers** - Request handlers
- **Services** - Business logic
- **Models** - Data schemas
- **Middleware** - Request processing

```
src/
├── routes/
│   ├── auth.routes.ts
│   ├── users.routes.ts
│   └── products.routes.ts
├── controllers/
│   ├── auth.controller.ts
│   ├── users.controller.ts
│   └── products.controller.ts
├── services/
│   ├── auth.service.ts
│   ├── users.service.ts
│   └── products.service.ts
├── models/
│   └── user.model.ts
├── middleware/
│   ├── auth.middleware.ts
│   └── errorHandler.ts
└── app.ts
```

## Pre-configured Rules

```json
{
  "max_lines_per_function": 50,
  "architecture_pattern": "MVC",
  "forbidden_imports": [
    {
      "from": "src/routes/**",
      "to": "src/models/**",
      "reason": "Routes must use controllers, not access models directly"
    },
    {
      "from": "src/controllers/**",
      "to": "src/models/**",
      "reason": "Controllers must use services for data access"
    },
    {
      "from": "src/services/**",
      "to": "src/routes/**",
      "reason": "Services should not depend on routes"
    },
    {
      "from": "src/middleware/**",
      "to": "src/services/**",
      "reason": "Middleware should only handle request/response, not business logic"
    }
  ]
}
```

## Best Practices

- Use middleware for cross-cutting concerns
- Keep route handlers thin
- Put business logic in services
- Use models for data validation
- Separate authentication/authorization middleware
- Error handling at the middleware level

## Common Issues

### Circular Dependencies

Use lazy loading or refactor shared logic:

```typescript
// Lazy load to break circular dependency
const userService = require('./services/user.service');
```

### Middleware Order

Middleware order matters in Express:

```typescript
app.use(authMiddleware);
app.use(errorHandler); // Should be last
```
