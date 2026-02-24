//! Analysis using Tree-sitter for TypeScript/JavaScript files.
//! SWC has been removed — all parsing now uses Tree-sitter.

use crate::autofix::Violation;
use crate::config::{ArchError, LinterContext};
use crate::parsers;
use crate::source_span;
use miette::{IntoDiagnostic, Result, SourceSpan};
use std::fs;
use std::path::PathBuf;

use super::pattern_matcher::{matches_pattern, normalize_pattern};

/// Analyze a single file for architecture violations.
///
/// The `_cm` parameter is kept for API compatibility with callers in main.rs
/// that still pass a `swc_common::SourceMap`. It is not used; removal is
/// deferred to Task 2.5.
pub fn analyze_file<C>(_cm: &C, path: &PathBuf, ctx: &LinterContext) -> Result<()> {
    let source_code = fs::read_to_string(path).into_diagnostic()?;

    // Try to use multi-language parser first
    if let Some(parser) = parsers::get_parser_for_file(path) {
        let violations = parser.find_violations(&source_code, path, ctx)?;

        if let Some(first_violation) = violations.first() {
            return Err(create_error_from_source(
                &source_code,
                first_violation,
            ));
        }

        // Validate method length for TypeScript/JavaScript files
        let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if matches!(extension, "ts" | "tsx" | "js" | "jsx") {
            validate_method_length_ts(&source_code, ctx.max_lines)?;
        }

        return Ok(());
    }

    // Fallback: non-TS/JS files with no dedicated parser — check forbidden import
    // patterns using line-based scanning.
    let file_path_str = path.to_string_lossy().to_lowercase();

    for (line_number, line) in source_code.lines().enumerate() {
        let trimmed = line.trim();

        // Only process import lines
        if !trimmed.starts_with("import ") && !trimmed.starts_with("import{") {
            continue;
        }

        // Extract the module path from the import statement heuristically
        let source_lower = trimmed.to_lowercase();

        for rule in &ctx.forbidden_imports {
            let from_pattern = normalize_pattern(&rule.from);
            let to_pattern = normalize_pattern(&rule.to);

            let file_matches = matches_pattern(&file_path_str, &from_pattern);
            let import_matches = matches_pattern(&source_lower, &to_pattern);

            if file_matches && import_matches {
                let span = source_span::span_for_line(&source_code, line_number + 1);
                return Err(create_arch_error(
                    &source_code,
                    span,
                    &format!(
                        "Restricción: Archivos en '{}' no pueden importar de '{}'.",
                        rule.from, rule.to
                    ),
                ));
            }
        }

        // Standard NestJS rule: no Repository in Controller
        if file_path_str.contains("controller") && source_lower.contains(".repository") {
            let span = source_span::span_for_line(&source_code, line_number + 1);
            return Err(create_arch_error(
                &source_code,
                span,
                "MVC: Prohibido importar Repositorios en Controladores.",
            ));
        }
    }

    Ok(())
}

/// Validate that no class method exceeds `max_lines` using Tree-sitter.
/// Only applies to TypeScript content.
pub fn validate_method_length_ts(content: &str, max_lines: usize) -> Result<()> {
    use tree_sitter::Parser;
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
        .map_err(|e| miette::miette!("Tree-sitter init error: {}", e))?;

    let tree = parser
        .parse(content, None)
        .ok_or_else(|| miette::miette!("Failed to parse TypeScript content"))?;

    check_methods_recursive(tree.root_node(), max_lines)
}

fn check_methods_recursive(
    node: tree_sitter::Node,
    max_lines: usize,
) -> Result<()> {
    if node.kind() == "method_definition" || node.kind() == "function_declaration" {
        let start_line = node.start_position().row + 1; // 1-based
        let end_line = node.end_position().row + 1;
        let line_count = end_line.saturating_sub(start_line);
        if line_count > max_lines {
            return Err(miette::miette!(
                "Method too long ({} lines, starts at line {}). Maximum: {}.",
                line_count,
                start_line,
                max_lines
            ));
        }
    }
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            check_methods_recursive(child, max_lines)?;
        }
    }
    Ok(())
}

/// Create a miette ArchError from source content and a span.
fn create_arch_error(src: &str, span: SourceSpan, msg: &str) -> miette::Report {
    ArchError {
        src: src.to_string(),
        span,
        message: msg.to_string(),
    }
    .into()
}

/// Create a miette error from a Violation, using file source content directly.
pub fn create_error_from_source(src: &str, violation: &Violation) -> miette::Report {
    let span = source_span::span_for_line(src, violation.line_number);
    ArchError {
        src: src.to_string(),
        span,
        message: format!(
            "Restricción: Archivos en '{}' no pueden importar de '{}'.",
            violation.rule.from, violation.rule.to
        ),
    }
    .into()
}

/// Validate method length for a file, using Tree-sitter internally.
///
/// This wrapper accepts the generic `_cm` parameter for API compatibility
/// with existing callers (e.g., `test_memory_optimization.rs`) that pass a
/// `swc_common::SourceMap`. The parameter is ignored; all parsing is done
/// with Tree-sitter. Removal is deferred to Task 2.5.
pub fn validate_method_length<C>(
    _cm: &C,
    path: &PathBuf,
    ctx: &LinterContext,
) -> Result<()> {
    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    if !matches!(extension, "ts" | "tsx" | "js" | "jsx") {
        return Ok(());
    }
    let content = fs::read_to_string(path).into_diagnostic()?;
    validate_method_length_ts(&content, ctx.max_lines)
}

/// Collect violations from a file without failing.
/// Useful for --fix mode where we want to process all violations.
///
/// The `_cm` parameter is kept for API compatibility with callers in main.rs
/// that still pass a `swc_common::SourceMap`. It is not used; removal is
/// deferred to Task 2.5.
pub fn collect_violations_from_file<C>(
    _cm: &C,
    path: &PathBuf,
    ctx: &LinterContext,
) -> Result<Vec<Violation>> {
    // Try to use multi-language parser first
    if let Some(parser) = parsers::get_parser_for_file(path) {
        let source_code = fs::read_to_string(path).into_diagnostic()?;
        let mut violations = parser.find_violations(&source_code, path, ctx)?;

        // Security audit (Tier Pro)
        if let Ok(mut security_violations) = parser.audit_security(&source_code, path, ctx) {
            violations.append(&mut security_violations);
        }

        return Ok(violations);
    }

    // Fallback: return empty violations for unsupported files
    Ok(Vec::new())
}
