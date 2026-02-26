---
title: architect.json Schema
sidebar_label: architect.json
---

# architect.json Configuration Schema

Reference documentation for the architect.json configuration file.

## Required Fields

### max_lines_per_function

Type: number
Min: 1
Max: 1000
Default: 50

Maximum lines allowed per function.

### architecture_pattern

Type: string
Valid: "Hexagonal" | "Clean" | "MVC" | "Ninguno"
Default: "Ninguno"

Your project's architectural pattern.

### forbidden_imports

Type: array
Items: ImportRule

Array of forbidden import rules.

## ImportRule Object

### from (required)

Type: string
Glob pattern for source module.

Example: `src/components/**`

### to (required)

Type: string
Glob pattern for target module.

Example: `src/services/**`

### reason (optional)

Type: string

Human-readable reason for the restriction.

## Complete Schema Example

```json
{
  "max_lines_per_function": 50,
  "architecture_pattern": "MVC",
  "forbidden_imports": [
    {
      "from": "src/components/**",
      "to": "src/api/**",
      "reason": "Components must request data through services"
    }
  ]
}
```

## Framework Examples

See [Templates](/docs/templates) for framework-specific configurations.

## Validation Rules

- `max_lines_per_function` must be > 0
- `architecture_pattern` must be one of the valid values
- Each import rule must have both `from` and `to`
- No duplicate rules allowed

For validation errors, see [Configuration Errors](/docs/troubleshooting/config-errors).
