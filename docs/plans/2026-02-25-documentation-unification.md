# Documentation Unification with Docusaurus - Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Migrate scattered root markdown files into a professional Docusaurus site with automatic deployments, versioning, and i18n support.

**Architecture:**
- Docusaurus v3+ for static site generation with versioning and i18n
- GitHub Pages for hosting (free, integrated with GitHub Actions)
- Automated CI/CD for changelog generation, versioning, and deployment
- Single source of truth with README.md in root as landing page linking to full docs

**Tech Stack:** Docusaurus 3, Node.js/npm, GitHub Actions, GitHub Pages

---

## Phase 1: Setup Docusaurus (Basic Structure)

### Task 1: Initialize Docusaurus project

**Files:**
- Create: `package.json` (root - Docusaurus config)
- Create: `docusaurus.config.js`
- Create: `sidebars.js`
- Create: `docs/intro.md`

**Step 1: Install Docusaurus locally**

Run: `npx create-docusaurus@latest . classic`

Expected: Creates docusaurus.config.js, package.json, docs/, sidebars.js structure

**Step 2: Update docusaurus.config.js with project config**

Replace `baseUrl` and site metadata:

```javascript
module.exports = {
  title: 'Architect Linter Pro',
  tagline: 'Multi-language software architecture linter written in Rust',
  url: 'https://architect-linter-pro.dev',
  baseUrl: '/',
  organizationName: 'sergio-linter',
  projectName: 'architect-linter-pro',
  deploymentBranch: 'gh-pages',
  onBrokenLinks: 'throw',
  i18n: {
    defaultLocale: 'en',
    locales: ['en', 'es'],
    localeConfigs: {
      en: { label: 'English' },
      es: { label: 'Español' }
    }
  },
  presets: [
    [
      '@docusaurus/preset-classic',
      {
        docs: { sidebarPath: require.resolve('./sidebars.js') },
        blog: { showReadingTime: true },
        theme: { customCss: require.resolve('./src/css/custom.css') }
      }
    ]
  ]
};
```

**Step 3: Create initial docs/intro.md**

```markdown
# Welcome to Architect Linter Pro

A multi-language software architecture linter written in Rust.

## What is it?

Architect Linter Pro validates architectural rules through a dynamic rule engine. Supports TypeScript, JavaScript, Python, and PHP.

## Key Features

- 🌐 Multi-Language Support
- 🔧 Dynamic Rule Engine
- 🔍 Circular Dependency Detection
- 📦 Import Validation
- ⚡ Parallel Processing
- 🏆 Health Score System

[See full features →](/docs/features)
```

**Step 4: Create sidebars.js with basic structure**

```javascript
module.exports = {
  tutorialSidebar: [
    'intro',
    {
      label: 'Installation',
      items: ['installation/windows', 'installation/linux', 'installation/macos']
    },
    {
      label: 'Getting Started',
      items: ['getting-started/basic-usage', 'getting-started/configuration', 'getting-started/first-rules']
    },
    {
      label: 'Guides',
      items: ['guides/architecture-design', 'guides/rule-engine', 'guides/ai-features', 'guides/watch-mode', 'guides/daemon-mode']
    },
    {
      label: 'Templates',
      items: ['templates/nestjs', 'templates/express', 'templates/react', 'templates/nextjs', 'templates/django']
    },
    {
      label: 'API Reference',
      items: ['api-reference/cli-commands', 'api-reference/architect-json', 'api-reference/ai-config']
    },
    {
      label: 'Troubleshooting',
      items: ['troubleshooting/config-errors', 'troubleshooting/common-issues']
    }
  ]
};
```

**Step 5: Test Docusaurus locally**

Run: `npm run start`

Expected: Browser opens to http://localhost:3000, shows Docusaurus homepage

**Step 6: Commit**

```bash
git add package.json docusaurus.config.js sidebars.js docs/intro.md
git commit -m "feat(docs): initialize docusaurus with basic structure"
```

---

### Task 2: Create directory structure for all doc sections

**Files:**
- Create: All subdirectories under `docs/`

**Step 1: Create all necessary directories**

```bash
mkdir -p docs/installation
mkdir -p docs/getting-started
mkdir -p docs/guides
mkdir -p docs/templates
mkdir -p docs/api-reference
mkdir -p docs/troubleshooting
mkdir -p docs/architecture
```

**Step 2: Create placeholder files (empty markdown)**

Run:
```bash
touch docs/installation/{windows,linux,macos}.md
touch docs/getting-started/{basic-usage,configuration,first-rules}.md
touch docs/guides/{architecture-design,rule-engine,ai-features,watch-mode,daemon-mode}.md
touch docs/templates/{nestjs,express,react,nextjs,django}.md
touch docs/api-reference/{cli-commands,architect-json,ai-config}.md
touch docs/troubleshooting/{config-errors,common-issues}.md
touch docs/architecture/{design,development}.md
```

**Step 3: Verify structure**

Run: `tree docs/` or `find docs -type f -name "*.md"`

Expected: All .md files created with empty content

**Step 4: Commit**

```bash
git add docs/
git commit -m "docs: create directory structure for all sections"
```

---

## Phase 2: Migrate Content (By Priority)

### Task 3: Migrate Installation docs (HIGH PRIORITY)

**Files:**
- Source: `INSTALL_WINDOWS.md`
- Target: `docs/installation/windows.md`
- Target: `docs/installation/linux.md` (new)
- Target: `docs/installation/macos.md` (new)

**Step 1: Copy and adapt INSTALL_WINDOWS.md**

Read current `INSTALL_WINDOWS.md` and copy content to `docs/installation/windows.md` with Docusaurus markdown format:
- Remove standalone title (Docusaurus uses sidebar)
- Add frontmatter: `---\ntitle: Windows Installation\n---\n`
- Fix any broken links to point to Docusaurus doc structure

```markdown
---
title: Windows Installation
sidebar_label: Windows
---

# Installation on Windows

[Content from INSTALL_WINDOWS.md with link fixes]
```

**Step 2: Create linux.md and macos.md stubs**

```markdown
---
title: Linux Installation
sidebar_label: Linux
---

# Installation on Linux

[Basic instructions - can expand later]
```

```markdown
---
title: macOS Installation
sidebar_label: macOS
---

# Installation on macOS

[Basic instructions - can expand later]
```

**Step 3: Test local build**

Run: `npm run start` and navigate to Installation section

Expected: All 3 installation pages render without errors

**Step 4: Commit**

```bash
git add docs/installation/
git commit -m "docs: migrate installation guides"
```

---

### Task 4: Migrate Troubleshooting docs (HIGH PRIORITY)

**Files:**
- Source: `CONFIG_ERRORS.md`, `CONFIG_ERRORS_ES.md`
- Target: `docs/troubleshooting/config-errors.md`
- Target: `docs/troubleshooting/common-issues.md`

**Step 1: Copy CONFIG_ERRORS.md**

Read `CONFIG_ERRORS.md`, copy to `docs/troubleshooting/config-errors.md` with frontmatter:

```markdown
---
title: Configuration Errors
---

# Configuration Errors

[Content from CONFIG_ERRORS.md, adjust heading levels]
```

**Step 2: Create common-issues.md**

Create a new file that consolidates common issues from across current docs:

```markdown
---
title: Common Issues & Solutions
---

# Common Issues & Solutions

[Gather from NEXT_STEPS.md, README.md, other error references]
```

**Step 3: Test build**

Run: `npm run build` to ensure no broken references

Expected: Build completes without errors

**Step 4: Commit**

```bash
git add docs/troubleshooting/
git commit -m "docs: migrate troubleshooting guides"
```

---

### Task 5: Migrate Getting Started docs (HIGH PRIORITY)

**Files:**
- Source: `README.md`, `NEXT_STEPS.md`
- Target: `docs/getting-started/basic-usage.md`
- Target: `docs/getting-started/configuration.md`
- Target: `docs/getting-started/first-rules.md`

**Step 1: Extract "Quick Start" from README**

Create `docs/getting-started/basic-usage.md`:

```markdown
---
title: Basic Usage
---

# Basic Usage

## Installation

[Copy quick install from README]

## Your First Analysis

architect --init
architect --check

## Understanding the output

[Explain default output format]
```

**Step 2: Create configuration.md**

Extract configuration section from README and NEXT_STEPS.md:

```markdown
---
title: Configuration
---

# Configuring Architect Linter Pro

## architect.json

[Explain config structure]

## architect.ai.json

[Explain AI config]
```

**Step 3: Create first-rules.md**

From README and examples:

```markdown
---
title: Writing Your First Rules
---

# Writing Your First Architecture Rules

## Simple Rule

[Basic example]

## Rule with Violations

[More complex example]
```

**Step 4: Test**

Run: `npm run start` and go through Getting Started flow

Expected: Can navigate through 3 pages in order, links work

**Step 5: Commit**

```bash
git add docs/getting-started/
git commit -m "docs: migrate getting started guides"
```

---

### Task 6: Migrate API Reference docs (MEDIUM PRIORITY)

**Files:**
- Source: README.md (Commands section)
- Target: `docs/api-reference/cli-commands.md`
- Target: `docs/api-reference/architect-json.md`
- Target: `docs/api-reference/ai-config.md`

**Step 1: Create cli-commands.md**

Extract all CLI commands from README:

```markdown
---
title: CLI Commands
---

# CLI Reference

## architect --init

[Description and examples]

## architect --check

[Description and examples]

## architect --fix

[Description and examples]

[... etc for all commands]
```

**Step 2: Create architect-json.md**

Document the architect.json schema:

```markdown
---
title: architect.json Schema
---

# architect.json Configuration

## File Structure

[Schema documentation]

## Examples

[Real examples from templates]
```

**Step 3: Create ai-config.md**

Document .architect.ai.json:

```markdown
---
title: AI Configuration
---

# AI Configuration (.architect.ai.json)

## Supported Providers

[List Claude, OpenAI, Gemini, etc.]

## Setup Instructions

[Per-provider setup]
```

**Step 4: Test**

Run: `npm run start`

Expected: API Reference section complete, no broken links

**Step 5: Commit**

```bash
git add docs/api-reference/
git commit -m "docs: migrate api reference documentation"
```

---

### Task 7: Migrate Guides docs (MEDIUM PRIORITY)

**Files:**
- Source: README.md (features), various sections
- Target: `docs/guides/*.md`

**Step 1: Create architecture-design.md**

Extract architecture explanation:

```markdown
---
title: Architecture Design Principles
---

# Designing Your Architecture

[Best practices for defining layers and rules]
```

**Step 2: Create rule-engine.md**

Detailed guide on how rule engine works:

```markdown
---
title: Understanding the Rule Engine
---

# Rule Engine Deep Dive

[How violations are detected, constraint types, etc.]
```

**Step 3: Create ai-features.md**

From README AI section:

```markdown
---
title: AI-Powered Features
---

# AI Features

## Auto-Fix

[How --fix works, fallback support]

## Architecture Assistant

[How Claude integration works]
```

**Step 4: Create watch-mode.md and daemon-mode.md**

From README Watch Mode and Daemon Mode sections:

```markdown
---
title: Watch Mode
---

# Watch Mode

[Real-time monitoring setup and usage]
```

```markdown
---
title: Daemon Mode
---

# Daemon Mode

[Background process setup and usage]
```

**Step 5: Commit**

```bash
git add docs/guides/
git commit -m "docs: migrate guides documentation"
```

---

### Task 8: Migrate Templates docs (LOW PRIORITY)

**Files:**
- Source: README.md (Templates section)
- Target: `docs/templates/*.md`

**Step 1: Create template files**

For each framework (NestJS, Express, React, NextJS, Django), create:

```markdown
---
title: NestJS Template
---

# NestJS Architecture Template

[Pre-configured rules for NestJS projects]

## Quick Start

architect --template nestjs

## What's included

[Layer structure, rules, examples]
```

Repeat for: express.md, react.md, nextjs.md, django.md

**Step 2: Test build**

Run: `npm run build`

Expected: No errors

**Step 3: Commit**

```bash
git add docs/templates/
git commit -m "docs: migrate template documentation"
```

---

### Task 9: Update README.md in root (NEW LANDING PAGE)

**Files:**
- Modify: `README.md`

**Step 1: Replace README.md with landing page version**

```markdown
# Architect Linter Pro

<p align="center">
  <img src="./public/architect-linter-pro-banner.png" alt="Architect Linter Pro Banner" width="100%">
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-4.3.0-blue.svg" alt="Version">
  <img src="https://img.shields.io/badge/rust-2021-orange.svg" alt="Rust Edition">
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="License">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey.svg" alt="Platform">
</p>

A multi-language software architecture linter written in Rust that validates architectural rules through a dynamic rule engine. Supports **4 production languages: TypeScript, JavaScript, Python, and PHP** using Tree-sitter for fast and accurate parsing.

## 📚 Full Documentation

👉 **[Read the complete documentation](https://architect-linter-pro.dev)**

### Quick Navigation
- [Installation Guide](https://architect-linter-pro.dev/docs/installation)
- [Getting Started](https://architect-linter-pro.dev/docs/getting-started)
- [API Reference](https://architect-linter-pro.dev/docs/api-reference)
- [Architecture Templates](https://architect-linter-pro.dev/docs/templates)
- [Troubleshooting](https://architect-linter-pro.dev/docs/troubleshooting)
- [Blog & Changelog](https://architect-linter-pro.dev/blog)

## 🚀 Quick Install

```bash
cargo install architect-linter-pro
architect --init
architect --check
```

## ✨ Key Features

- 🌐 **Multi-Language Support**: TypeScript, JavaScript, Python, PHP
- 🔧 **Dynamic Rule Engine**: Define custom constraints via architect.json
- 🔍 **Circular Dependency Detection**: Automatic cycle detection
- 📦 **Import Validation**: Block architecture violations
- ⚡ **Parallel Processing**: Ultra-fast analysis with Rayon
- 🏆 **Health Score System**: Comprehensive quality metrics (A-F grading)
- 🤖 **AI-Powered Auto-Fix**: Automatic suggestions and fixes
- 👁️ **Watch Mode**: Real-time monitoring with OS notifications
- 👻 **Daemon Mode**: Background process for continuous monitoring

## 🤝 Contributing

Contributions are welcome! See our [contributing guide](https://architect-linter-pro.dev/docs/contributing) for details.

## 📄 License

MIT

---

**Languages:** English | [Español](README_ES.md)
```

**Step 2: Create README_ES.md**

Spanish version of the new README:

```markdown
# Architect Linter Pro

[Spanish translation of landing page]

**Idiomas:** [Español](README_ES.md) | [English](README.md)
```

**Step 3: Test that links work**

Run: `npm run start` and verify README links point to https://architect-linter-pro.dev

Expected: Links are correct (domain will be live after deploy)

**Step 4: Commit**

```bash
git add README.md README_ES.md
git commit -m "docs: convert README to landing page with doc links"
```

---

## Phase 3: Add Internationalization (i18n)

### Task 10: Setup Spanish translations

**Files:**
- Create: `i18n/es/docusaurus-plugin-content-docs/current.json`
- Create: `i18n/es/docusaurus-plugin-content-blog/current.json`

**Step 1: Copy docs to Spanish folder**

```bash
mkdir -p i18n/es/docusaurus-plugin-content-docs/current
cp -r docs/* i18n/es/docusaurus-plugin-content-docs/current/
```

**Step 2: Translate key intro.md to Spanish**

Edit `i18n/es/docusaurus-plugin-content-docs/current/intro.md`:

```markdown
---
title: Bienvenido a Architect Linter Pro
---

# Bienvenido a Architect Linter Pro

Un linter de arquitectura de software multi-lenguaje escrito en Rust.

[Spanish translation of English intro]
```

**Step 3: Update sidebars.js with Spanish versions**

Edit `sidebars.js` to include Spanish sidebar paths (Docusaurus handles automatically)

**Step 4: Test i18n**

Run: `npm run start` and check language switcher

Expected: Can switch between EN and ES, pages render correctly

**Step 5: Commit**

```bash
git add i18n/
git commit -m "docs: add Spanish translations for core docs"
```

---

## Phase 4: Setup CI/CD & Deployment

### Task 11: Create GitHub Actions workflow for deployment

**Files:**
- Create: `.github/workflows/deploy-docs.yml`

**Step 1: Create deploy workflow**

```yaml
# .github/workflows/deploy-docs.yml
name: Deploy Docs

on:
  push:
    branches:
      - master
  pull_request:
    branches:
      - master

jobs:
  build-and-deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - uses: actions/setup-node@v3
        with:
          node-version: '18'
          cache: 'npm'

      - name: Install dependencies
        run: npm ci

      - name: Validate markdown
        run: npm run build

      - name: Deploy to GitHub Pages
        if: github.ref == 'refs/heads/master'
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./build
```

**Step 2: Enable GitHub Pages in repository settings**

Manual step in GitHub repo:
- Go to Settings → Pages
- Set source to: Deploy from a branch
- Branch: gh-pages, folder: / (root)

**Step 3: Test locally first**

Run: `npm run build`

Expected: `build/` folder created with static site

**Step 4: Commit workflow**

```bash
git add .github/workflows/deploy-docs.yml
git commit -m "ci: add github pages deployment workflow"
```

---

### Task 12: Add build validation step

**Files:**
- Modify: `package.json` (add lint script)
- Create: `.docusaurus-lintrc.js` (optional validation config)

**Step 1: Add npm scripts for validation**

Edit `package.json`, ensure these scripts exist:

```json
{
  "scripts": {
    "start": "docusaurus start",
    "build": "docusaurus build",
    "validate": "docusaurus build --out-dir /tmp/test-build",
    "write-translations": "docusaurus write-translations --locale es"
  }
}
```

**Step 2: Test validation**

Run: `npm run validate`

Expected: Build completes without warnings

**Step 3: Commit**

```bash
git add package.json
git commit -m "ci: add validation scripts"
```

---

### Task 13: Setup automatic changelog from git tags

**Files:**
- Create: `.github/workflows/changelog.yml`
- Create: `scripts/generate-changelog.js`

**Step 1: Create changelog generation script**

```javascript
// scripts/generate-changelog.js
const { execSync } = require('child_process');
const fs = require('fs');

try {
  const tags = execSync('git tag --sort=-version:refname | head -20')
    .toString()
    .split('\n')
    .filter(Boolean);

  let changelog = '# Changelog\n\n';

  for (let i = 0; i < tags.length - 1; i++) {
    const currentTag = tags[i];
    const previousTag = tags[i + 1];
    const commits = execSync(`git log ${previousTag}..${currentTag} --oneline`)
      .toString()
      .trim();

    if (commits) {
      changelog += `## [${currentTag}]\n\n${commits}\n\n`;
    }
  }

  fs.writeFileSync('docs/changelog.md',
    '---\ntitle: Changelog\n---\n\n' + changelog);
  console.log('✅ Changelog generated');
} catch (error) {
  console.error('❌ Changelog generation failed:', error.message);
  process.exit(1);
}
```

**Step 2: Create changelog workflow**

```yaml
# .github/workflows/changelog.yml
name: Update Changelog

on:
  push:
    tags:
      - 'v*'

jobs:
  changelog:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
        with:
          fetch-depth: 0

      - uses: actions/setup-node@v3
        with:
          node-version: '18'

      - name: Generate changelog
        run: node scripts/generate-changelog.js

      - name: Commit changelog
        run: |
          git config user.name "GitHub Actions"
          git config user.email "actions@github.com"
          git add docs/changelog.md
          git commit -m "docs: update changelog for $(git describe --tags)"
          git push
```

**Step 3: Add script to package.json**

```json
{
  "scripts": {
    "changelog": "node scripts/generate-changelog.js"
  }
}
```

**Step 4: Test locally**

Run: `npm run changelog`

Expected: `docs/changelog.md` is created with commit history

**Step 5: Commit**

```bash
git add scripts/generate-changelog.js .github/workflows/changelog.yml
git commit -m "ci: add automated changelog generation"
```

---

### Task 14: Add doc versioning support

**Files:**
- Modify: `docusaurus.config.js`
- Create: `versioned_docs/version-4.3.0/intro.md` (when ready to version)

**Step 1: Update docusaurus.config.js for versioning**

Add to presets config:

```javascript
docs: {
  sidebarPath: require.resolve('./sidebars.js'),
  lastVersion: 'current',
  versions: {
    current: {
      label: 'Next',
      path: 'next'
    }
  }
}
```

**Step 2: Create version for current stable (v4.3.0)**

When ready (after deployment), run:

```bash
npm run docusaurus docs:version 4.3.0
```

This creates `versioned_docs/version-4.3.0/` automatically.

**Step 3: Update sidebar for docs**

Docusaurus creates `versioned_sidebars/version-4.3.0-sidebars.json` automatically.

**Step 4: Commit**

```bash
git add docusaurus.config.js versioned_docs/ versioned_sidebars/
git commit -m "docs: add versioning support for v4.3.0"
```

---

## Phase 5: Polish & Finalization

### Task 15: Add search functionality

**Files:**
- Modify: `docusaurus.config.js`

**Step 1: Add Algolia search config** (or local search)

For local search (no external service):

```javascript
// In docusaurus.config.js presets
{
  '@docusaurus/preset-classic': {
    docs: { ... },
    theme: {
      customCss: require.resolve('./src/css/custom.css'),
    }
  }
}
```

Docusaurus includes local search by default in v3+.

**Step 2: Test search**

Run: `npm run start` and use search box in header

Expected: Can search docs and get results

**Step 3: Commit (if any config changes)**

```bash
git add docusaurus.config.js
git commit -m "docs: configure search functionality"
```

---

### Task 16: Add SEO and metadata

**Files:**
- Modify: `docusaurus.config.js`
- Create: `static/sitemap.xml` (auto-generated)

**Step 1: Update docusaurus.config.js with SEO**

```javascript
module.exports = {
  title: 'Architect Linter Pro',
  tagline: 'Multi-language software architecture linter written in Rust',
  url: 'https://architect-linter-pro.dev',
  baseUrl: '/',
  favicon: 'img/favicon.ico',
  organizationName: 'sergio-linter',
  projectName: 'architect-linter-pro',
  themeConfig: {
    image: 'img/architect-linter-pro-og.png',
    metadata: [
      {
        name: 'description',
        content: 'Architect Linter Pro - Multi-language software architecture linter'
      }
    ]
  }
}
```

**Step 2: Verify sitemap is generated**

Run: `npm run build` and check `build/sitemap.xml`

Expected: sitemap.xml exists with all doc URLs

**Step 3: Test**

Run: `npm run start`

Expected: Meta tags show in browser dev tools

**Step 4: Commit**

```bash
git add docusaurus.config.js
git commit -m "docs: add seo metadata and sitemap support"
```

---

### Task 17: Delete old markdown files from root

**Files:**
- Delete: `CHANGELOG.md`, `CONFIG_ERRORS.md`, `CONFIG_ERRORS_ES.md`, `INSTALL_WINDOWS.md`, `NEXT_STEPS.md`, `PENDING_TASKS.md`, `ROADMAP.md`, `ROADMAP_ES.md`

**Step 1: Verify all content is migrated**

Checklist:
- ✅ INSTALL_WINDOWS.md → docs/installation/windows.md
- ✅ CONFIG_ERRORS.md → docs/troubleshooting/config-errors.md
- ✅ NEXT_STEPS.md → docs/getting-started/
- ✅ ROADMAP.md → docs/ or blog/ (decide final location)
- ✅ CHANGELOG.md → automated via workflow

**Step 2: Delete old files**

```bash
rm CHANGELOG.md CONFIG_ERRORS.md CONFIG_ERRORS_ES.md INSTALL_WINDOWS.md NEXT_STEPS.md PENDING_TASKS.md ROADMAP.md ROADMAP_ES.md
```

**Step 3: Verify git status**

Run: `git status`

Expected: Only deletions shown

**Step 4: Commit**

```bash
git add -A
git commit -m "docs: remove old root markdown files (migrated to docusaurus)"
```

---

### Task 18: Test full deployment locally

**Files:**
- No changes, testing only

**Step 1: Build and verify production build**

```bash
npm run build
npm run serve
```

Expected: Site serves correctly at http://localhost:3000

**Step 2: Test all sections navigate correctly**

- ✅ Home → all navigation links work
- ✅ Installation → Windows/Linux/macOS pages accessible
- ✅ Getting Started → flow makes sense
- ✅ API Reference → commands are searchable
- ✅ Templates → all 5 templates present
- ✅ Troubleshooting → config errors accessible
- ✅ Spanish version works (language switch)
- ✅ Search box works

**Step 3: Run build validation**

```bash
npm run validate
```

Expected: No warnings or errors

**Step 4: No commit needed** (testing only)

---

### Task 19: Deploy to GitHub Pages

**Files:**
- No code changes

**Step 1: Ensure .gitignore excludes build output**

Check `.gitignore` contains:

```
build/
.docusaurus/
node_modules/
```

**Step 2: Push to master**

```bash
git push origin master
```

**Step 3: Monitor GitHub Actions**

Go to repository → Actions tab, watch deploy-docs.yml workflow

Expected: Workflow runs and completes successfully in 2-3 min

**Step 4: Verify live site**

Visit: https://architect-linter-pro.dev (or gh-pages URL if custom domain not set)

Expected: Site is live and matches local version

**Step 5: No commit needed** (deployment only)

---

### Task 20: Final verification and documentation

**Files:**
- Create: `DOCUMENTATION.md` (guide for maintaining docs)

**Step 1: Create maintenance guide**

```markdown
# Documentation Maintenance Guide

## Adding New Pages

1. Create .md file in appropriate `docs/` folder
2. Add entry to `sidebars.js`
3. Push to master → auto-deploys

## Updating Existing Pages

Edit the .md file directly → auto-deploys on push

## Adding Spanish Translations

Edit corresponding file in `i18n/es/docusaurus-plugin-content-docs/current/`

## Creating a New Version (Release)

When releasing v4.4.0:

```bash
npm run docusaurus docs:version 4.4.0
git add versioned_docs/ versioned_sidebars/
git commit -m "docs: version for v4.4.0"
git tag v4.4.0
git push --tags
```

## Testing Locally

```bash
npm install
npm run start
```

## Building for Production

```bash
npm run build
npm run serve
```

## Troubleshooting

- **Broken links:** Run `npm run build` - shows broken refs
- **Sidebar not updating:** Restart `npm run start`
- **Language switcher missing:** Check i18n config in docusaurus.config.js
```

**Step 2: Add to documentation**

Save as `DOCUMENTATION.md` in project root

**Step 3: Commit**

```bash
git add DOCUMENTATION.md
git commit -m "docs: add documentation maintenance guide"
```

---

## Summary

**Total Tasks:** 20
**Phase 1 (Setup):** 2 tasks
**Phase 2 (Migration):** 7 tasks
**Phase 3 (i18n):** 1 task
**Phase 4 (CI/CD):** 4 tasks
**Phase 5 (Polish):** 6 tasks

**Expected Timeline:**
- If executing 2-3 tasks per session: ~7-10 sessions
- Each task: 5-15 minutes depending on content volume

**Deliverables:**
- ✅ Professional Docusaurus site with full content
- ✅ Automatic GitHub Pages deployment
- ✅ Spanish translation support
- ✅ Changelog auto-generation from git tags
- ✅ Versioning support for releases
- ✅ SEO-optimized with search
- ✅ README.md as landing page linking to full docs
- ✅ CI/CD fully integrated
