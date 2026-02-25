---
title: Basic Usage
sidebar_label: Basic Usage
---

# Basic Usage

This guide will help you get started with Architect Linter Pro.

## Installation

For detailed installation instructions for your platform, see the [Installation Guide](/docs/installation).

### Quick Install

Choose one of these quick installation methods:

```bash
# Option 1: Using Cargo (fastest, requires Rust)
cargo install architect-linter-pro

# Option 2: Using Homebrew (macOS/Linux)
brew tap sergiogswv/architect-linter-pro
brew install architect-linter-pro

# Option 3: Using npm
npm install -g @architect-linter/cli
```

After installation, verify it works:
```bash
architect-linter-pro --version
```

## Your First Analysis

### Step 1: Navigate to your project

```bash
cd /path/to/your/project
```

### Step 2: Run Architect Linter

```bash
architect-linter-pro
```

Or run it interactively:
```bash
# Interactive mode (shows you available projects)
architect-linter-pro
```

### Step 3: Let the wizard guide you

On first run, the linter will:
1. Display a visual welcome banner
2. Detect your framework automatically
3. Use AI to suggest architectural rules
4. Guide you through an interactive wizard
5. Create `architect.json` with suggested rules

## Understanding the Output

After analysis, you'll see:

### Health Score

A visual dashboard showing your project's health (0-100) with:
- **A-F Grading**: Overall project quality assessment
- **Four Key Metrics**: Layer Isolation, Circular Dependencies, Code Complexity, Rule Violations
- **Actionable Insights**: Detailed recommendations for improvement

### Violations

List of architectural rule violations found in your code:
- File location and line number
- Description of the violation
- Suggested fix (if available with AI features)

## Next Steps

- Learn about [Configuration](/docs/getting-started/configuration)
- Write your own [Architecture Rules](/docs/getting-started/first-rules)
- Explore [Advanced Guides](/docs/guides)
