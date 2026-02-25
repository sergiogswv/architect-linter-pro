# GitHub Actions Integration

## Setup

Copy `.github/workflows/architect-lint.yml` to your repository.

## What it does

- Runs on every push and pull request
- Checks architecture violations
- Fails CI if violations found
- Uploads report as artifact

## Prerequisites

- Tool must be installed from crates.io: `cargo install architect-linter-pro`
- architect.json file should be present in project root

## Viewing Results

1. Go to **Actions** tab in GitHub
2. Click on the workflow run to see logs and status
3. Download **architect-lint-report** artifact to view the JSON report locally

Example viewing report:
```bash
cat architect-lint-report.json | jq '.violations | length'
```

## Customization

Edit `.github/workflows/architect-lint.yml` to:
- Change branches monitored: Edit the `on.push.branches` and `on.pull_request.branches` arrays
- Add additional checks: Extend the "Check for violations" step
- Generate different report format: Modify the `architect lint` command flags

## Troubleshooting

### Workflow not running
Check that:
- Workflow file is in `.github/workflows/architect-lint.yml`
- Branches match your repository (main/master/develop)
- Repository is public or Actions are enabled for this repository

### "architect-linter-pro: command not found" in workflow
The tool failed to install. Check:
- Tool is published to crates.io
- Rust toolchain installed successfully
- Check the "Install architect-linter-pro" step logs

### "Failed to parse lint report"
The linter may not have run successfully. Check:
- architect.json exists in project root
- No syntax errors in architecture configuration
- Review the "Run Architecture Lint" step output
