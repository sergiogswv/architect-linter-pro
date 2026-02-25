---
title: Watch Mode
sidebar_label: Watch Mode
---

# Real-Time Monitoring with Watch Mode

Continuously analyze your code as you work.

## Enable Watch Mode

```bash
architect --watch
```

Features:
- Real-time analysis of file changes
- Fast incremental updates (only changed files)
- 300ms debounce to avoid spam
- Native OS notifications

## Notifications

Get desktop alerts when violations are detected:

- **Windows**: Windows Notification Center
- **macOS**: Notification Center
- **Linux**: libnotify (D-Bus)

## Workflow

1. Start watch mode: `architect --watch`
2. Edit your code
3. Linter automatically re-analyzes on save
4. Get notified of violations
5. Fix violations in real-time

## Performance

Watch mode is optimized for speed:
- Only re-analyzes changed files
- Caches previous results
- Debounces rapid changes
- Won't block your editing
