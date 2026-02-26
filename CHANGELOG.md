# Changelog

All notable changes to Architect Linter Pro will be documented in this file.

## v5.0.2 - 2026-02-26

### 🐛 Bug Fixes
- **Watch Mode Debounce Fix**: Fixed issue where watch mode would report multiple files as changed when only one file was modified
  - Problem: After processing changes, the debounce timer wasn't reset, causing file events detected during analysis to accumulate
  - Solution: Reset `last_event_time` after clearing changed_files to ensure proper debounce behavior
  - Files: `src/watch/mod.rs:278`

### 📦 Changes
- Updated installer to handle version checking and atomic binary replacement
- Added automatic backup system for previous versions in `~/.cargo/bin/.architect-backups/`

---

## v5.0.1 - 2026-02-26

### 🐛 Bug Fixes
- **Taint Analysis Disabled**: Disabled the TaintEngine security audit due to high false positive rate
  - Problem: Used overly broad substring matching (any function with "execute", "query", "eval" was marked as sink)
  - Impact: Eliminated false positives like "params cannot import from executeWithErrorHandling"
  - Status: Module disabled until rewrite with proper data flow analysis
  - Files: `src/parsers/typescript.rs:91`

### 🔧 Improvements
- Improved violation reporting to show actual flow details instead of hardcoded "SecurityModule → InsecureSink"
- Files: `src/security/data_flow.rs:41-42`

---

## v5.0.0 - 2026-02-20

### ✨ Features
- Initial release of Architect Linter Pro v5
- Support for TypeScript/JavaScript, Python, and PHP
- Dynamic rule engine
- Watch mode for continuous analysis
- AI-powered auto-fix suggestions
- Interactive command mode

### 📊 Supported Languages
- TypeScript/JavaScript (NestJS, Express, React, NextJS)
- Python (Django)
- PHP

---

## Installation & Updates

### Quick Install
\`\`\`bash
cd /path/to/architect-linter-pro
./install.sh
\`\`\`

### Check for Updates
\`\`\`bash
./install.sh --check-only
\`\`\`

### Force Reinstall
\`\`\`bash
./install.sh --force
\`\`\`

### Verbose Output
\`\`\`bash
./install.sh --verbose
\`\`\`

### Automatic Backups
Previous versions are automatically backed up to:
\`\`\`
~/.cargo/bin/.architect-backups/
\`\`\`
(Keeps last 3 versions)
