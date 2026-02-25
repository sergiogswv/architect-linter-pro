# Getting Started with Architect Linter Pro

## 5-Minute Quick Start

### Step 1: Initialize Your Project

Run the interactive setup wizard:

```bash
cd your-project
architect init
```

You'll be prompted to:
1. Select your framework (NestJS, Express, React, Next.js, Django)
2. Choose an architecture pattern (Hexagonal, Clean, Layered, etc.)

This generates `architect.json` with rules for your project.

### Step 2: Run Analysis

Lint your project:

```bash
architect lint .
```

Output shows:
- Architecture violations
- Circular dependencies
- Metrics and health score

### Step 3: Fix Issues

Auto-fix simple violations:

```bash
architect lint . --fix
```

For complex issues, use the violation details to manually refactor.

---

## Configuration

### architect.json Structure

```json
{
  "version": "1.0",
  "rules": [
    {
      "from": "src/controllers",
      "to": "src/models",
      "message": "Controllers should not import from models"
    }
  ]
}
```

**Rule Fields:**
- `from` - Source directory/layer
- `to` - Target directory/layer (can be array)
- `message` - Custom violation message

### Configuration Options

```bash
architect lint . --config ./architect.json
architect lint . --fix                      # Auto-fix violations
architect lint . --json                     # Output JSON format
architect lint . --severity error           # Only show errors
```

---

## Common Workflows

### Validate Architecture

```bash
architect lint . --severity error
```

### Generate Report

```bash
architect lint . --json > report.json
```

### Watch Mode

Monitor changes in real-time:

```bash
architect watch .
```

---

## Supported Languages & Frameworks

### TypeScript/JavaScript
- **NestJS** - Enterprise backend framework
- **Express** - Minimal web framework
- **React** - Frontend library
- **Next.js** - Full-stack React

### Python
- **Django** - Full-featured web framework

### PHP
- Standard PHP applications

---

## Next Steps

- See [Architecture](./ARCHITECTURE.md) for project structure
- See [Frameworks](./FRAMEWORKS.md) for detailed pattern documentation
- See [Contributing](./CONTRIBUTING.md) if you want to contribute
