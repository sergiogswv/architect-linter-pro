# Installation Guide

## Installation Methods

### 1. Using Cargo (Recommended)

Install from crates.io:

```bash
cargo install architect-linter-pro
```

Verify installation:

```bash
architect --version
```

### 2. From Source

Clone and build:

```bash
git clone https://github.com/sergiogswv/architect-linter.git
cd architect-linter-pro
cargo build --release
```

Binary will be at: `target/release/architect`

### 3. macOS with Homebrew

```bash
brew install architect-linter-pro
```

### 4. Docker

```bash
docker run --rm -v $(pwd):/workspace \
  sergiogswv/architect-linter-pro:latest \
  lint /workspace
```

## Platform Support

| Platform | Support | Method |
|----------|---------|--------|
| Linux    | ✅ Full | Cargo, Source |
| macOS    | ✅ Full | Cargo, Homebrew, Source |
| Windows  | ✅ Full | Cargo, Source |

## Requirements

- Rust 1.56+ (for building from source)
- No additional system dependencies

## Troubleshooting

### "architect: command not found"

Make sure Cargo's bin directory is in your PATH:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

### Build fails with "tree-sitter not found"

Try updating your Rust toolchain:

```bash
rustup update
```

## Next Steps

- See [Getting Started](./GETTING_STARTED.md) for quick-start guide
- See [Frameworks](./FRAMEWORKS.md) for framework-specific patterns
- See [Architecture](./ARCHITECTURE.md) for codebase overview
