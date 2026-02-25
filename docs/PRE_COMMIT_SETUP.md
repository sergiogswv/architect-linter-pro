# Pre-commit Hook Setup

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
