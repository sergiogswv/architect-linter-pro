# Scoop Bucket for Architect Linter Pro

This directory contains the Scoop manifest for Windows distribution.

## About the Hash Placeholder

The `hash` field in `architect-linter-pro.json` is initialized to all zeros
(`0000000000000000000000000000000000000000000000000000000000000000`) and is
automatically updated by the `update-scoop.yml` workflow when a GitHub Release
is published. Do not attempt to install from the manifest before the first
release tag is pushed — the zero hash will cause Scoop to reject the download
as expected, signalling that the release artifact does not yet exist.

## Setup Instructions

### 1. Create the Scoop bucket repository
Create a new GitHub repository named `scoop-architect-linter-pro`.

### 2. Initialize the bucket structure
```bash
git clone https://github.com/sergiogswv/scoop-architect-linter-pro
cd scoop-architect-linter-pro
mkdir bucket
cp /path/to/architect-linter-pro/scoop/architect-linter-pro.json bucket/
git add . && git commit -m "init: add architect-linter-pro manifest"
git push
```

### 3. Add the TAP_REPO_TOKEN secret
In the **main architect-linter-pro repo** settings, add a secret:
- Name: `TAP_REPO_TOKEN`
- Value: Personal Access Token with `contents:write` scope on `scoop-architect-linter-pro`

### 4. User installation
```powershell
scoop bucket add architect https://github.com/sergiogswv/scoop-architect-linter-pro
scoop install architect-linter-pro
```
