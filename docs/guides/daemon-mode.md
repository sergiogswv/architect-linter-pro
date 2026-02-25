---
title: Daemon Mode
sidebar_label: Daemon Mode
---

# Background Monitoring with Daemon Mode

Run the linter continuously in the background.

## Enable Daemon Mode

```bash
architect --daemon
```

Starts a background process that:
- Continuously monitors your codebase
- Analyzes changes without blocking
- Keeps your terminal free

## Benefits

- Run alongside your dev server
- No terminal window needed
- Persistent monitoring
- Low resource usage with smart debouncing

## Stopping the Daemon

```bash
# Stop the background process
architect --daemon stop
```

## Integration with CI/CD

Use in your CI/CD pipeline:

```yaml
# GitHub Actions example
- name: Check Architecture
  run: architect --check --staged
```
