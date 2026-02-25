---
title: Understanding the Rule Engine
sidebar_label: Rule Engine
---

# The Rule Engine Deep Dive

How Architect Linter detects and reports architectural violations.

## How Rules Work

Each rule is a forbidden import path:

```json
{
  "from": "src/components/**",
  "to": "src/api/**"
}
```

This means: **No file in src/components/ can import from src/api/**

## Detection

The linter:
1. Parses all source files with Tree-sitter
2. Extracts all import statements
3. Matches imports against rules
4. Reports violations with file and line number

## Glob Patterns

- `src/components/**` - All files recursively
- `src/api/*.ts` - Only .ts in api/ (not subdirectories)
- `src/**/utils` - utils folder at any depth

## Circular Dependency Detection

The linter automatically detects circular dependencies:

```
A imports B
B imports A
```

Reports as separate violations.

## Performance

Uses parallel processing with Rayon for fast analysis on large codebases.
