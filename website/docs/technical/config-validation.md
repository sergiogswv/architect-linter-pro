---
title: Configuration Validation
sidebar_position: 4
---
# Configuration Schema Validation Implementation

**Date:** 2026-02-17
**Version:** v4.3.0

## Overview

This document summarizes the implementation of full JSON Schema validation for the `architect.json` configuration file, improving developer experience (DX) and preventing misconfigurations.

## Components Implemented

### 1. JSON Schema (`schemas/architect.schema.json`)
- Created a comprehensive draft-07 JSON Schema.
- Defines types, descriptions, and constraints for all configuration fields:
    - `$schema`: URL/path to the schema itself.
    - `max_lines_per_function`: Integer limit.
    - `architecture_pattern`: Enum of supported patterns.
    - `forbidden_imports`: Array of rule objects with `from` and `to` strings.
    - `ignored_paths`: Array of glob patterns.
    - `performance`: Performance tuning settings.
    - `ai`: AI fallback configuration.

### 2. Validation Engine Integration (`src/config/loader.rs`)
- integrated the `jsonschema` crate.
- Added a validation step before deserialization.
- Provides clear, actionable error messages when the configuration violates the schema.

### 3. Configuration Migration (`src/config/migration.rs`)
- Added a migration layer to handle legacy configuration formats.
- Ensures that even if the internal structure changes, old user configurations are gracefully upgraded before validation.

### 4. IDE Support (`.vscode/settings.json`)
- Configured VS Code to automatically associate `architect.json` and `architect.json.example` with the new schema.
- Enables:
    - Real-time validation in the editor.
    - Property autocompletion.
    - Hover information for configuration fields.

### 5. CLI Check Mode (`src/cli.rs`, `src/main.rs`)
- Added a `--check` flag to the CLI.
- Responds by only validating the configuration and exiting with a success/failure code.
- Optimized for CI/CD and pre-commit hooks.

### 6. Husky Integration (`src/config/husky.rs`)
- Updated the pre-commit hook generator to include an explicit `--check` phase.
- Prevents commits if the architecture configuration file is malformed.

## Usage

### Validate configuration only
```bash
architect-linter-pro --check .
```

### Enable autocompletion in your project
Update your `architect.json`:
```json
{
  "$schema": "./schemas/architect.schema.json",
  "max_lines_per_function": 40,
  ...
}
```

## Testing
- Verified with `cargo test --test test_config`.
- Manual verification of VS Code autocompletion.
- Manual verification of the `--check` flag.
