---
title: AI-Powered Features
sidebar_label: AI Features
---

# Using AI Features

Leverage artificial intelligence to improve your architecture.

## Auto-Fix

Automatically fix violations:

```bash
architect --fix
```

Features:
- Generates refactoring suggestions
- Applies fixes automatically
- Validates with rebuild
- Rolls back if build fails
- Retries with different approaches

## Architecture Assistant

Get AI-powered suggestions when setting up:

```bash
architect --init
```

The AI:
- Analyzes your codebase structure
- Suggests appropriate architectural patterns
- Recommends layer boundaries
- Creates initial rules

## Multi-Model Support

Fallback to alternative models if primary fails:

1. Try primary provider
2. If fails, try next in fallback list
3. If all fail, show manual suggestions

## Configuring Providers

See [AI Configuration](/docs/api-reference/ai-config) for setup.

## Privacy Considerations

- Code is sent to the AI provider
- Self-hosted options available (Ollama)
- Configure for your security needs
