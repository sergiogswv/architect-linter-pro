---
title: CLI Commands
sidebar_label: Commands
---

# CLI Reference

## architect --init

Initialize a new architect.json configuration.

```bash
architect --init
```

Options:
- Interactive wizard guides you through setup
- Auto-detects framework
- Suggests rules based on project structure

## architect --check

Validate your architect.json without running analysis.

```bash
architect --check
```

## architect --fix

Automatically fix detected violations.

```bash
architect --fix
```

Features:
- AI-powered suggestions (if configured)
- Auto-rebuild verification
- Intelligent rollback on build failure

## architect --watch

Run in watch mode for real-time monitoring.

```bash
architect --watch
```

Features:
- Incremental analysis (faster)
- 300ms debounce to avoid spam
- Native OS notifications on violations
- Automatic restart on config changes

## architect --daemon

Run in background daemon mode.

```bash
architect --daemon
```

Keeps linter running without keeping terminal open.

## architect --report

Export analysis results.

```bash
architect --report json > report.json
architect --report markdown > report.md
```

Formats: json, markdown

## architect --staged

Analyze only staged files (git integration).

```bash
architect --staged
```

## architect --debug

Enable debug logging.

```bash
architect --debug
```

## architect --version

Show version.

```bash
architect --version
```

## architect --help

Show all available commands.

```bash
architect --help
```
