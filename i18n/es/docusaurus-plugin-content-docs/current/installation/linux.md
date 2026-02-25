---
title: Linux Installation
sidebar_label: Linux
---

# Installation on Linux

## Quick Install using Cargo

### Step 1: Ensure Rust is installed
If you haven't installed Rust yet:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Step 2: Install from crates.io
```bash
cargo install architect-linter-pro
```

### Step 3: Verify Installation
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

This installs the binary to ~/.cargo/bin, which should already be in your PATH.

### Step 3: Verify
```bash
architect-linter-pro --help
```

## Uninstall
```bash
cargo uninstall architect-linter-pro
```
