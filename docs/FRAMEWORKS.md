# Framework-Specific Patterns and Configuration

## TypeScript/JavaScript Frameworks

### NestJS

**Recommended Pattern:** Hexagonal Architecture

```
src/
├── domain/           # Business logic (entities, value objects)
├── application/      # Use cases and orchestration
├── infrastructure/   # External adapters (DB, API, etc)
└── presentation/     # Controllers and HTTP layer
```

**architect.json:**

```json
{
  "version": "1.0",
  "rules": [
    {
      "from": "src/domain",
      "to": ["src/application", "src/infrastructure"],
      "message": "Domain must not depend on other layers"
    }
  ]
}
```

### Express

**Recommended Pattern:** MVC

```
src/
├── routes/          # Route definitions
├── controllers/     # Request handlers
├── services/        # Business logic
├── models/          # Data models
└── middleware/      # Middleware functions
```

### React

**Recommended Pattern:** Feature-based with Layering

```
src/
├── features/        # Feature modules (isolated)
├── common/          # Shared components
└── core/           # Core services
```

## Python Frameworks

### Django

**Recommended Pattern:** MVT with Service Layer

```
myapp/
├── models/          # Database models
├── views/           # View functions/classes
├── services/        # Business logic
└── templates/       # HTML templates
```

---

## PHP

**Recommended Pattern:** MVC or Layered

```
app/
├── models/         # Database models
├── controllers/    # Request handlers
├── services/       # Business logic
└── views/          # Templates
```

---

## Best Practices

1. Define clear boundaries between layers
2. Avoid circular dependencies
3. Use consistent naming conventions
4. Document your architecture patterns
5. Run architect lint in your CI/CD pipeline

---

## See Also

- [Getting Started](./GETTING_STARTED.md)
- [Architecture](./ARCHITECTURE.md)
- [Contributing](./CONTRIBUTING.md)
