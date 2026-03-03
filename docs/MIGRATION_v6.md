# Migration Guide: v5 to v6

Welcome to Architect Linter Pro v6! This release brings significant improvements to security analysis, framework detection, and configuration management. This guide will help you upgrade from v5 to v6.

## What Changed in v6

### Framework Enum Cleanup

The Framework enum has been restructured for better maintainability:
- **Production Frameworks**: NestJS, Express, React, NextJS, Django
- All other frameworks have been removed (see Breaking Changes below)
- Framework detection now uses pattern-based recognition with improved accuracy

### TaintEngine v2: Control Flow Graph Analysis

The new TaintEngine leverages Control Flow Graph (CFG) analysis for more accurate security scanning:
- **Previous Approach**: Substring matching (inefficient, many false positives)
- **New Approach**: Context-aware data flow analysis with CFG
- **Benefits**:
  - Reduced false positives
  - Better accuracy on real-world code
  - Improved performance through smarter analysis
  - Production-grade security audits

### Smart Configuration System

- **Auto-Detection**: `architect init` now automatically detects your project's framework
- **Smart Defaults**: Configuration is generated based on detected framework patterns
- **Simplified Setup**: No more manual configuration for common frameworks

## Breaking Changes

### Removed Language Support

The following languages have been removed in v6. They will not be analyzed:

| Language | Removal Reason |
|----------|---|
| Go | Low adoption rate, maintenance burden |
| Java | Deprecated in favor of TypeScript/Python stack |
| C# | Not in production use cases |
| Ruby | Not in production stack |
| Kotlin | Superseded by Java deprecation |
| Rust | Complex parsing, limited use case |

**Action Required**: If your project uses any of these languages, you have two options:
1. Upgrade to a supported language (TypeScript for Go/Rust, Python/Django for Ruby)
2. Stay on v5.0.2 until language support is added back

### Taint Analysis Changes

If you were relying on security analysis features:
- TaintEngine v2 has stricter matching rules
- Some rules from v5 may not trigger in v6 (due to false positive elimination)
- Results are now more reliable and actionable

### Configuration Format Compatibility

v6 can read v5 configurations, but new features (like framework auto-detection) require regenerating your config:

```bash
# Backup old config
cp architect.json architect.json.backup

# Generate new v6 config
architect init
```

## Migration Steps

### Step 1: Update Your Installation

```bash
# Using cargo
cargo install architect-linter-pro@6.0.0

# Or if using npm/other package manager
npm install -g architect-linter-pro@6.0.0
```

### Step 2: Verify Your Project's Language Support

Check if your project uses only supported languages:
- TypeScript/JavaScript
- Python
- PHP

If you use unsupported languages, see the **Removed Language Support** section above.

### Step 3: Regenerate Configuration (Recommended)

While v6 can read v5 configs, we recommend regenerating to take advantage of new features:

```bash
# Backup your current config
cp architect.json architect.json.v5.backup

# Generate new config (auto-detects framework)
architect init

# Review the generated config
cat architect.json

# If you want to keep custom rules from v5, merge them manually
diff architect.json.v5.backup architect.json
```

### Step 4: Test Your Analysis

```bash
# Run lint on your project
architect lint .

# Compare results with v5
architect lint . > v6-results.txt

# If you have v5 still installed
# architect-linter-pro@5 lint . > v5-results.txt
# diff v5-results.txt v6-results.txt
```

### Step 5: Update CI/CD Pipelines

If you have automated linting in CI/CD, update the version constraints:

```yaml
# GitHub Actions example
- name: Architect Linter
  run: |
    cargo install architect-linter-pro@6.0.0
    architect-linter-pro lint .
```

Or update Docker image tags if you're using containers:

```dockerfile
FROM rustlang/rust:latest as builder
RUN cargo install architect-linter-pro@6.0.0
```

## Rollback Instructions

If you encounter issues with v6, you can rollback to v5:

### Using Cargo

```bash
# Uninstall v6
cargo uninstall architect-linter-pro

# Install v5
cargo install architect-linter-pro@5.0.2
```

### Using npm (if applicable)

```bash
npm uninstall -g architect-linter-pro
npm install -g architect-linter-pro@5.0.2
```

### Docker

```dockerfile
FROM rustlang/rust:latest as builder
RUN cargo install architect-linter-pro@5.0.2
```

### If You Modified architect.json for v6

If v6 generated a new `architect.json` that doesn't work with v5, restore your backup:

```bash
rm architect.json
mv architect.json.v5.backup architect.json
```

## Frequently Asked Questions

### Q: Why were these languages removed?

**A**: To focus on production use cases. The primary users of Architect Linter Pro work with:
- **Backend**: TypeScript (NestJS, Express) and Python (Django)
- **Frontend**: TypeScript (React, Next.js)
- **Full-stack**: PHP for WordPress/Laravel projects

Removing low-adoption languages allows us to:
- Improve the quality of remaining parsers
- Reduce maintenance burden
- Focus on core features like security analysis

### Q: Can I still analyze my Go/Java/Ruby projects with v6?

**A**: No, v6 does not support these languages. Your options:
1. Stay on v5.0.2
2. Rewrite unsupported code in supported languages
3. Use language-specific linters alongside architect-linter-pro

### Q: Will these languages be added back?

**A**: Potentially, yes! If there's enough demand, we can add:
- Go (if 10+ users request it)
- Ruby (if 10+ users request it)
- Other languages (based on community feedback)

Please open an issue on GitHub to register your interest.

### Q: My security checks aren't finding vulnerabilities anymore. Is v6 broken?

**A**: Likely not! TaintEngine v2 uses more accurate analysis:
- **False Positives**: Some v5 warnings were false positives; v6 eliminates these
- **True Positives**: Real vulnerabilities are still detected
- **Accuracy**: v6's results are more reliable

If you believe a real vulnerability was missed, please file a GitHub issue with:
1. Your code snippet
2. The vulnerability type
3. Why you believe it's a real issue

### Q: How do I report issues during migration?

**A**: Open an issue on GitHub with:
1. Your v5 config
2. Your v6 config
3. The command that failed
4. Error message and output
5. Steps to reproduce

## Getting Help

- **Documentation**: [Read the v6 docs](/docs)
- **GitHub Issues**: [Report issues here](https://github.com/sergiogswv/architect-linter-pro/issues)
- **Discord Community**: [Join our community](https://discord.gg/architect-linter)

## What's New in v6

Beyond the breaking changes, v6 includes many improvements:

- **CFG-Based Security Analysis**: More accurate vulnerability detection
- **Framework Auto-Detection**: `architect init` detects your stack automatically
- **Smart Config Generation**: Generated configs follow best practices
- **80%+ Test Coverage**: Production-grade code quality
- **Integration Tests**: Real-world validation

See [CHANGELOG_v6.md](../CHANGELOG_v6.md) for the full release notes.

---

**Happy upgrading! 🚀**
