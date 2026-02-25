# Contributing to Architect Linter Pro

We love contributions! Here's how to get started.

## Development Setup

### Prerequisites
- Rust 1.56+
- Git
- Cargo

### Clone and Setup

```bash
git clone https://github.com/sergiogswv/architect-linter.git
cd architect-linter-pro
cargo build
cargo test
```

## Making Changes

### 1. Create a Feature Branch

```bash
git checkout -b feature/your-feature-name
```

### 2. Follow TDD

1. Write a failing test
2. Implement the feature
3. Make the test pass
4. Refactor if needed

```bash
cargo test --lib
```

### 3. Follow Conventions

- Commit messages: `feat:`, `fix:`, `test:`, `docs:`, `refactor:`
- Rust style: `cargo fmt` and `cargo clippy`

### 4. Test Coverage

Ensure tests cover happy path, error cases, and edge cases.

Run tests:

```bash
cargo test --lib --no-fail-fast
cargo test --test "*" --no-fail-fast
```

## Code Quality

### Format Code

```bash
cargo fmt
```

### Check for Issues

```bash
cargo clippy --all-targets --all-features
```

### Run Full Test Suite

```bash
cargo test --all
```

## Submitting Changes

### Pull Request Process

1. Ensure tests pass
2. Update documentation
3. Create PR with clear description
4. Request review

## Support for New Languages

To add a new language:

1. Create parser in `src/parsers/language_name.rs`
2. Implement `ArchitectParser` trait
3. Add to `Language` enum in `src/parsers/mod.rs`
4. Add tree-sitter crate to `Cargo.toml`
5. Add tests in `tests/test_parsers.rs`

## Support for New Frameworks

To add a new framework:

1. Create template in `src/init/templates/framework.rs`
2. Add to `Framework` enum
3. Add to `patterns_for_framework()`
4. Add tests
5. Update FRAMEWORKS.md

## Questions?

- Open an issue on GitHub
- Check existing issues/discussions
- See ARCHITECTURE.md for codebase overview

Thank you for contributing!
