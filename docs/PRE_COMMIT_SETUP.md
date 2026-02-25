# Pre-commit Hook Setup

## Prerequisites

- `architect-linter-pro` must be installed: `cargo install architect-linter-pro`
- Must be in a git repository with `.git` directory
- `architect.yaml` configuration file should exist in project root (or architecture rules defined)

## Installation

```bash
./scripts/install-hook.sh
```

## What it does

- Runs architect lint before each commit
- Blocks commits with architecture violations
- Ensures code follows defined architecture rules

## Configuration

Edit `.architect-pre-commit.sh` to customize:
- Severity level (error, warning, info)
- Target directories
- Ignore patterns

## Disable temporarily

```bash
git commit --no-verify
```

## Uninstall

```bash
rm .git/hooks/pre-commit
```

## Troubleshooting

### "architect: command not found"
The `architect-linter-pro` CLI is not installed. Install it:
```bash
cargo install architect-linter-pro
```

### "Failed to copy .architect-pre-commit.sh"
Make sure:
1. You're in the project root directory
2. `.architect-pre-commit.sh` exists in the root
3. You have write permissions to `.git/hooks/`

### Hook not running on commit
Verify the hook is installed:
```bash
ls -la .git/hooks/pre-commit
# Should show: -rwxr-xr-x ... pre-commit
```

If missing, reinstall:
```bash
./scripts/install-hook.sh
```

### "Architecture violations found" but my code is fine
Check your architecture configuration:
```bash
architect lint . --severity error
# Review violations directly to understand rules
```
