---
title: Writing Your First Rules
sidebar_label: First Rules
---

# Writing Your First Architecture Rules

This guide shows you how to create meaningful architectural rules for your project.

## Understanding Rules

An architectural rule prevents unwanted imports between different parts of your codebase.

### Simple Rule Example

Let's say you want to prevent UI components from importing directly from the API layer:

```json
{
  "from": "src/components/**",
  "to": "src/api/**",
  "reason": "Components should request data through services, not directly from API"
}
```

This means:
- Components cannot import from api (rule is enforced)
- api can import from components (rule only goes one direction)

## Real-World Examples

### Rule 1: Service Layer Isolation

For a layered architecture, isolate your service layer:

```json
{
  "from": "src/components/**",
  "to": "src/services/**"
}
```

**Explanation:** Components can only interact with services through controllers or models.

### Rule 2: Feature-Based Architecture

For feature-based architecture, prevent features from knowing about each other:

```json
[
  {
    "from": "src/features/auth/**",
    "to": "src/features/dashboard/**"
  },
  {
    "from": "src/features/dashboard/**",
    "to": "src/features/auth/**"
  }
]
```

### Rule 3: Hexagonal Architecture

For hexagonal architecture in NestJS:

```json
[
  {
    "from": "src/domain/**",
    "to": "src/infrastructure/**",
    "reason": "Domain logic should not depend on infrastructure implementation"
  },
  {
    "from": "src/application/**",
    "to": "src/infrastructure/**",
    "reason": "Application services should not know about infrastructure details"
  }
]
```

## Glob Pattern Guide

Rules use glob patterns for flexibility:

| Pattern | Matches |
|---------|---------|
| `src/components/**` | All files in components and subdirectories |
| `src/api/*.ts` | Only .ts files directly in api/ (not subdirectories) |
| `src/**/utils` | A utils folder at any depth |
| `src/services/user/**` | All files under user service |

## Writing Effective Rules

### Good Rules

- **Specific and focused**: Target exact layers/features
- **Have clear reasons**: Document why the rule exists
- **Prevent unwanted coupling**: Keep separate concerns separate
- **Framework-aligned**: Match your chosen architecture pattern

### Rules to Avoid

- **Too restrictive**: Don't prevent necessary imports
- **Overlapping**: Don't create contradictory rules
- **Unclear intent**: Always explain why the rule exists
- **Fragile patterns**: Don't rely on complex glob patterns

## Complete Example

Here's a realistic `architect.json` for a React + NestJS monorepo:

```json
{
  "max_lines_per_function": 40,
  "architecture_pattern": "Hexagonal",
  "forbidden_imports": [
    {
      "from": "src/components/**",
      "to": "src/api/**",
      "reason": "Components must go through services"
    },
    {
      "from": "src/hooks/**",
      "to": "src/pages/**",
      "reason": "Hooks should be reusable, not page-specific"
    },
    {
      "from": "src/pages/**",
      "to": "src/styles/**",
      "reason": "Use CSS modules per component"
    },
    {
      "from": "src/utils/**",
      "to": "src/api/**",
      "reason": "Utils are domain-agnostic"
    }
  ]
}
```

## Testing Your Rules

After writing rules, run the linter to test them:

```bash
architect-linter-pro --check
```

If violations are found:
1. Review the listed violations
2. Decide if the rule is correct or too strict
3. Either fix your code or adjust the rule
4. Run again to verify

## Next Steps

- Explore [Advanced Guides](/docs/guides)
- Learn about [AI Features](/docs/guides/ai-features) for auto-fix
- See [Framework Templates](/docs/templates) for best practices
