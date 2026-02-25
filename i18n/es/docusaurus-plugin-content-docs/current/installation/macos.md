---
title: macOS Installation
sidebar_label: macOS
---

# Installation on macOS

## Quick Install using Homebrew

### Step 1: Install using Homebrew
```bash
brew install architect-linter-pro
```

### Step 2: Verify Installation
```bash
architect-linter-pro --version
```

## Alternative: Install from Cargo

If you prefer using Cargo (Rust package manager):

### Step 1: Ensure Rust is installed
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Step 2: Install from crates.io
```bash
cargo install architect-linter-pro
```

### Step 3: Verify
```bash
architect-linter-pro --version
```

## Building from Source

### Step 1: Clone the repository
```bash
git clone https://github.com/sergio/architect-linter-pro.git
cd architect-linter-pro
```

### Step 2: Build and install
```bash
cargo install --path .
```

### Step 3: Verify
```bash
architect-linter-pro --help
```

## Uninstall

If you installed via Homebrew:
```bash
brew uninstall architect-linter-pro
```

If you installed via Cargo:
```bash
cargo uninstall architect-linter-pro
```
