---
title: NestJS Architecture Template
sidebar_label: NestJS
---

# NestJS Architecture Template

Pre-configured architectural rules for NestJS projects.

## Quick Start

```bash
architect --template nestjs
```

## Layer Structure

NestJS follows a modular architecture with clear separation:

- **Controllers** - HTTP request handling
- **Services** - Business logic and data operations
- **Modules** - Feature grouping and dependency injection
- **Guards/Interceptors** - Cross-cutting concerns
- **Database** - ORM and repository patterns

```
src/
├── modules/
│   ├── auth/
│   │   ├── auth.module.ts
│   │   ├── auth.service.ts
│   │   ├── auth.controller.ts
│   │   └── guards/
│   ├── users/
│   │   ├── users.module.ts
│   │   ├── users.service.ts
│   │   └── users.controller.ts
│   └── products/
├── common/
│   ├── filters/
│   ├── interceptors/
│   └── decorators/
└── app.module.ts
```

## Pre-configured Rules

```json
{
  "max_lines_per_function": 50,
  "architecture_pattern": "Hexagonal",
  "forbidden_imports": [
    {
      "from": "src/modules/**/controllers/**",
      "to": "src/modules/**/database/**",
      "reason": "Controllers must use services, not access database directly"
    },
    {
      "from": "src/modules/**/services/**",
      "to": "src/modules/*/controllers/**",
      "reason": "Services should not depend on controllers"
    },
    {
      "from": "src/common/**",
      "to": "src/modules/**",
      "reason": "Common utilities should not import from modules"
    }
  ]
}
```

## Best Practices

- Use Dependency Injection for all services
- Keep controllers thin (routing only)
- Put business logic in services
- Organize by feature modules
- Use guards for authentication/authorization
- Create custom decorators for cross-cutting concerns

## Common Issues

### Circular Dependencies

NestJS modules can create circular dependencies. Use `forwardRef()`:

```typescript
@Module({
  imports: [forwardRef(() => OtherModule)]
})
```

### Service Injection

Always inject services through the constructor:

```typescript
constructor(private userService: UserService) {}
```
