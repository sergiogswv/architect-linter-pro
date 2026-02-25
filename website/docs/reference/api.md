---
title: API & CLI Reference
sidebar_position: 1
---

# API & CLI Reference

Architect Linter Pro is designed to be used both as an interactive tool and as part of an automated pipeline.

## CLI Usage

The primary way to interact with the tool is via the Command Line Interface.

| Flag | Shorthand | Description |
|------|-----------|-------------|
| `--version` | `-v` | Show version information |
| `--help` | `-h` | Show help message |
| `--watch` | `-w` | Enable watch mode (real-time analysis) |
| `--daemon` | `-d` | Run as a background process |
| `--fix` | `-f` | AI-powered automatic fixing |
| `--staged` | `-s` | Analyze only staged files |
| `--incremental`| | Use Git-based incremental analysis |
| `--debug` | | Enable verbose logging |
| `--report <FORMAT>` | `-r` | Generate report (`json` or `markdown`) |
| `--output <FILE>` | `-o` | Specify output file for the report |

## Programmatic Use (JSON API)

When running with `--report json`, the tool generates a structured JSON output that can be consumed by other tools or custom scripts.

### JSON Schema

The output follows this general structure:

```json
{
  "version": "4.3.0",
  "timestamp": "2026-02-18T10:00:00Z",
  "project_root": "/path/to/project",
  "health_score": 92.5,
  "grade": "A",
  "summary": {
    "total_files": 150,
    "violations_count": 2,
    "circular_deps_count": 0,
    "long_functions_count": 5
  },
  "violations": [
    {
      "type": "ForbiddenImport",
      "file": "src/domain/user.ts",
      "line": 3,
      "message": "Files in '/domain/' cannot import from '/infrastructure/'",
      "context": "import { Repo } from '../infrastructure/repo';"
    }
  ],
  "circular_dependencies": [],
  "metrics": {
    "layer_isolation": 98.0,
    "complexity": 85.0,
    "coupling": 95.0
  }
}
```

## Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Success (No violations or score above threshold) |
| `1` | Architecture violations found |
| `2` | Configuration error |
| `127`| Panic / Critical internal error |

## Configuration Schema

You can validate your `architect.json` against the official JSON Schema to get IDE autocompletion.

```json
{
  "$schema": "https://raw.githubusercontent.com/sergiogswv/architect-linter-pro/main/schemas/architect.schema.json"
}
```
