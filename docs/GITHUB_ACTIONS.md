# GitHub Actions Integration

## Setup

Copy `.github/workflows/architect-lint.yml` to your repository.

## What it does

- Runs on every push and pull request
- Checks architecture violations
- Fails CI if violations found
- Uploads report as artifact

## Customization

Edit the workflow file to:
- Change branches monitored
- Adjust severity level
- Add additional checks
- Generate different report format

## Viewing Results

1. Go to Actions tab in GitHub
2. Click on run to see logs
3. Download lint-report.json artifact
