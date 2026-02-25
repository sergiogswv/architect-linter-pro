//! Parsers module - Multi-language parsing abstraction using Tree-sitter
//!
//! This module provides a unified interface for parsing source code across
//! different programming languages using Tree-sitter.

use crate::autofix::Violation;
use crate::config::LinterContext;
use miette::Result;
use std::path::Path;

pub mod php;
pub mod python;
pub mod typescript;

/// Represents an import statement extracted from source code
#[derive(Debug, Clone)]
pub struct Import {
    /// The import source/path (e.g., "../services/user", "apps.user.models")
    pub source: String,
    /// Line number where the import appears
    pub line_number: usize,
    /// Full import statement text
    pub raw_statement: String,
}

/// Language identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    TypeScript,
    JavaScript,
    Python,
    Php,
}

impl Language {
    /// Get language from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext {
            "ts" | "tsx" => Some(Language::TypeScript),
            "js" | "jsx" => Some(Language::JavaScript),
            "py" => Some(Language::Python),
            "php" => Some(Language::Php),
            _ => None,
        }
    }

    /// Get file extensions for this language
    pub fn extensions(&self) -> &'static [&'static str] {
        match self {
            Language::TypeScript => &["ts", "tsx"],
            Language::JavaScript => &["js", "jsx"],
            Language::Python => &["py"],
            Language::Php => &["php"],
        }
    }
}

/// Main trait for language parsers
///
/// Implementations of this trait provide language-specific parsing logic
/// using Tree-sitter to extract imports and detect architectural violations.
///
/// # Design Decision (2026-02-25)
///
/// The ArchitectParser trait is intentionally minimal with three core methods:
///
/// 1. **extract_imports** (required): Extracts all import statements from source code.
///    This is essential for all language implementations to enable dependency analysis.
///
/// 2. **find_violations** (required): Detects architectural violations based on configured
///    rules. All parsers must implement language-specific pattern matching for rule enforcement.
///
/// 3. **audit_security** (optional with default): Audits files for security vulnerabilities
///    (Pro feature). Defaults to no-op (returns empty Vec) to keep Community edition lightweight.
///    Pro parsers can override this method to provide security scanning.
///
/// This trait achieves excellent separation of concerns:
/// - Each parser handles its language-specific syntax via Tree-sitter
/// - Common logic (rule matching, violation reporting) is abstracted away
/// - The factory pattern via `get_parser_for_file()` provides transparent language detection
/// - The optional `audit_security` method allows Pro-specific features without burdening
///   Community implementations
///
/// **No refactoring needed.** All 3 parser implementations (TypeScript, Python, PHP)
/// correctly implement the trait. No dead code. The design successfully abstracts
/// multi-language parsing while maintaining clear boundaries.
pub trait ArchitectParser: Send + Sync {
    /// Extract all imports from source code
    fn extract_imports(&self, source_code: &str, file_path: &Path) -> Result<Vec<Import>>;

    /// Find architectural violations in a file
    fn find_violations(
        &self,
        source_code: &str,
        file_path: &Path,
        context: &LinterContext,
    ) -> Result<Vec<Violation>>;

    /// Audit file for security vulnerabilities (Pro feature)
    fn audit_security(
        &self,
        _source_code: &str,
        _file_path: &Path,
        _context: &LinterContext,
    ) -> Result<Vec<Violation>> {
        Ok(Vec::new())
    }
}

/// Factory function to get appropriate parser for a file
pub fn get_parser_for_file(file_path: &Path) -> Option<Box<dyn ArchitectParser>> {
    let ext = file_path.extension()?.to_str()?;
    let lang = Language::from_extension(ext)?;

    match lang {
        Language::TypeScript | Language::JavaScript => {
            Some(Box::new(typescript::TypeScriptParser::new()))
        }
        Language::Python => Some(Box::new(python::PythonParser::new())),
        Language::Php => Some(Box::new(php::PhpParser::new())),
    }
}

/// Get all supported languages
pub fn supported_languages() -> Vec<Language> {
    vec![
        Language::TypeScript,
        Language::JavaScript,
        Language::Python,
        Language::Php,
    ]
}

/// Get all supported file extensions
pub fn supported_extensions() -> Vec<&'static str> {
    let mut extensions = Vec::new();
    for lang in supported_languages() {
        extensions.extend_from_slice(lang.extensions());
    }
    extensions
}
