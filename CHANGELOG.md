# Changelog

All notable changes to this project will be documented in this file.

## [5.0.0] - 2026-02-25

### BREAKING CHANGES
- **Scope Reduction**: Removed support for Go, Java, C#, Ruby, Kotlin, Rust. Focus now on 4 production languages: TypeScript, JavaScript, Python, PHP.
- **Deprecated Frameworks**: Spring Framework template removed (Java deprecated)

### Changed
- Reduced parser count from 10 to 4 languages
- Removed 6 tree-sitter dependencies (40% reduction)
- Simplified Language enum and parser factory
- Updated all documentation to reflect production languages

### Added
- Architecture documentation (ARCHITECTURE.md)
- Comprehensive test coverage for core languages
- Developer runbook and maintenance guide
- Framework-specific templates and patterns

### Fixed
- Consolidated scope to reduce maintenance burden

### Removed
- Parsers: Go, Java, C#, Ruby, Kotlin, Rust
- Spring Framework template
- Beta language indicators
- Unused tree-sitter dependencies

## [4.3.0] - Previous release

Previous changelog entries would go here if available.
