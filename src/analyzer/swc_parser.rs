//! SWC-based parser for TypeScript/JavaScript files

use crate::autofix::Violation;
use crate::config::{ArchError, LinterContext};
use crate::parsers;
use miette::{IntoDiagnostic, Result, SourceSpan};
use std::fs;
use std::path::PathBuf;
use swc_common::SourceMap;
use swc_ecma_parser::{lexer::Lexer, EsConfig, Parser, StringInput, Syntax, TsConfig};

use super::pattern_matcher::{matches_pattern, normalize_pattern};

/// Analyze a single file for architecture violations
pub fn analyze_file(cm: &SourceMap, path: &PathBuf, ctx: &LinterContext) -> Result<()> {
    // Try to use multi-language parser first
    if let Some(parser) = parsers::get_parser_for_file(path) {
        let source_code = fs::read_to_string(path).into_diagnostic()?;
        let violations = parser.find_violations(&source_code, path, ctx)?;

        if let Some(first_violation) = violations.first() {
            // Return error for the first violation
            let fm = cm.load_file(path).into_diagnostic()?;
            return Err(create_error_from_violation(&fm, first_violation));
        }

        // Validate method length for TypeScript/JavaScript files
        let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if matches!(extension, "ts" | "tsx" | "js" | "jsx") {
            validate_method_length(cm, path, ctx)?;
        }

        return Ok(());
    }

    // Fallback to old swc parser for unsupported files
    let fm = cm.load_file(path).into_diagnostic()?;

    // Detectar si es TypeScript o JavaScript según la extensión
    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let syntax = match extension {
        "ts" | "tsx" => Syntax::Typescript(TsConfig {
            decorators: true,
            tsx: extension == "tsx",
            ..Default::default()
        }),
        "js" | "jsx" => Syntax::Es(EsConfig {
            decorators: true,
            jsx: extension == "jsx",
            ..Default::default()
        }),
        _ => Syntax::Typescript(TsConfig::default()),
    };

    // Create lexer and parser - these will be dropped when they go out of scope
    let lexer = Lexer::new(syntax, Default::default(), StringInput::from(&*fm), None);
    let mut parser = Parser::new_from(lexer);

    // Parse module immediately and process to avoid keeping AST in memory
    let module = match parser.parse_module() {
        Ok(m) => m,
        Err(e) => {
            // Clean up parser and lexer before returning error
            drop(parser);
            return Err(miette::miette!("Syntax Error: {:?}", e));
        }
    };

    let file_path_str = path.to_string_lossy().to_lowercase();

    // Process all imports and class definitions, then drop AST
    let result = process_module_items(cm, &fm, &module, &file_path_str, ctx);

    // Explicitly drop AST objects to free memory
    drop(module);
    drop(parser);

    result
}

/// Helper function to process module items and extract analysis data
fn process_module_items(
    _cm: &SourceMap,
    fm: &swc_common::SourceFile,
    module: &swc_ecma_ast::Module,
    file_path_str: &str,
    ctx: &LinterContext,
) -> Result<()> {
    for item in &module.body {
        // --- VALIDACIÓN DE IMPORTACIONES DINÁMICAS ---
        if let swc_ecma_ast::ModuleItem::ModuleDecl(swc_ecma_ast::ModuleDecl::Import(import)) = item
        {
            let source = import.src.value.to_string().to_lowercase();

            // 1. Validamos las reglas dinámicas del JSON
            for rule in &ctx.forbidden_imports {
                // Normalizar patrones: quitar '**', '*', y '/' al final para matching flexible
                let from_pattern = normalize_pattern(&rule.from);
                let to_pattern = normalize_pattern(&rule.to);

                // Verificar si el archivo coincide con el patrón 'from'
                let file_matches = matches_pattern(&file_path_str, &from_pattern);

                // Verificar si el import coincide con el patrón 'to'
                let import_matches = matches_pattern(&source, &to_pattern);

                if file_matches && import_matches {
                    return Err(create_error(
                        &fm,
                        import.span,
                        &format!(
                            "Restricción: Archivos en '{}' no pueden importar de '{}'.",
                            rule.from, rule.to
                        ),
                    ));
                }
            }

            // 2. Regla extra: Siempre prohibir Repository en Controller (Standard NestJS)
            if file_path_str.contains("controller") && source.contains(".repository") {
                return Err(create_error(
                    &fm,
                    import.span,
                    "MVC: Prohibido importar Repositorios en Controladores.",
                ));
            }
        }

        // --- VALIDACIÓN DE LÍNEAS POR MÉTODO ---
        if let swc_ecma_ast::ModuleItem::Stmt(swc_ecma_ast::Stmt::Decl(
            swc_ecma_ast::Decl::Class(c),
        )) = item
        {
            for member in &c.class.body {
                if let swc_ecma_ast::ClassMember::Method(m) = member {
                    let lo = fm.start_pos.0 as usize + m.span.lo.0 as usize;
                    let hi = fm.start_pos.0 as usize + m.span.hi.0 as usize;
                    let lines = (hi / 80) - (lo / 80); // Approximate line count

                    if lines > ctx.max_lines {
                        return Err(create_error(
                            &fm,
                            m.span,
                            &format!(
                                "Método demasiado largo ({} líneas). Máximo: {}.",
                                lines, ctx.max_lines
                            ),
                        ));
                    }
                }
            }
        }
    }
    Ok(())
}

/// Validate method length for TypeScript/JavaScript files using swc
/// Ensures AST is properly scoped and dropped after analysis
pub fn validate_method_length(cm: &SourceMap, path: &PathBuf, ctx: &LinterContext) -> Result<()> {
    let fm = cm.load_file(path).into_diagnostic()?;

    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let syntax = match extension {
        "ts" | "tsx" => Syntax::Typescript(TsConfig {
            decorators: true,
            tsx: extension == "tsx",
            ..Default::default()
        }),
        "js" | "jsx" => Syntax::Es(EsConfig {
            decorators: true,
            jsx: extension == "jsx",
            ..Default::default()
        }),
        _ => return Ok(()),
    };

    // Create lexer and parser - these will be dropped when they go out of scope
    let lexer = Lexer::new(syntax, Default::default(), StringInput::from(&*fm), None);
    let mut parser = Parser::new_from(lexer);

    // Parse module and immediately extract required information
    let module = match parser.parse_module() {
        Ok(m) => m,
        Err(_) => return Ok(()), // Skip on syntax error
    };

    // Process the AST and immediately drop references to large structures
    let _functions_extracted = count_methods_in_module(&fm, &module, ctx);

    // Explicitly drop the module to free memory
    drop(module);

    // Additional cleanup - ensure no references remain to AST structures
    drop(parser);

    // Return result based on extracted functions
    Ok(())
}

/// Helper function to count methods in a module and extract analysis data
fn count_methods_in_module(
    fm: &swc_common::SourceFile,
    module: &swc_ecma_ast::Module,
    ctx: &LinterContext,
) -> usize {
    let mut method_count = 0;

    for item in &module.body {
        if let swc_ecma_ast::ModuleItem::Stmt(swc_ecma_ast::Stmt::Decl(
            swc_ecma_ast::Decl::Class(c),
        )) = item
        {
            for member in &c.class.body {
                if let swc_ecma_ast::ClassMember::Method(m) = member {
                    let lo = fm.start_pos.0 as usize + m.span.lo.0 as usize;
                    let hi = fm.start_pos.0 as usize + m.span.hi.0 as usize;
                    let lines = (hi / 80) - (lo / 80); // Approximate line count

                    if lines > ctx.max_lines {
                        // Just count the method, don't return error
                        println!("Method too long: {} lines", lines);
                    }
                    method_count += 1;
                }
            }
        }
    }

    method_count
}

/// Create a miette error from SWC span
pub fn create_error(
    fm: &swc_common::SourceFile,
    span: swc_common::Span,
    msg: &str,
) -> miette::Report {
    let start = (span.lo.0 - fm.start_pos.0) as usize;
    let end = (span.hi.0 - fm.start_pos.0) as usize;

    ArchError {
        src: fm.src.to_string(),
        span: SourceSpan::new(start.into(), (end - start).into()),
        message: msg.to_string(),
    }
    .into()
}

/// Create a miette error from a Violation
pub fn create_error_from_violation(
    fm: &swc_common::SourceFile,
    violation: &Violation,
) -> miette::Report {
    // Try to find the import line in the source
    let lines: Vec<&str> = fm.src.lines().collect();
    let line_idx = violation.line_number.saturating_sub(1);

    if line_idx < lines.len() {
        let line_content = lines[line_idx];
        // Find the position of the import statement in the file
        let mut char_offset = 0;
        for (idx, line) in lines.iter().enumerate() {
            if idx == line_idx {
                break;
            }
            char_offset += line.len() + 1; // +1 for newline
        }

        ArchError {
            src: fm.src.to_string(),
            span: SourceSpan::new(char_offset.into(), line_content.len().into()),
            message: format!(
                "Restricción: Archivos en '{}' no pueden importar de '{}'.",
                violation.rule.from, violation.rule.to
            ),
        }
        .into()
    } else {
        // Fallback if line number is out of bounds
        miette::miette!(
            "Restricción: Archivos en '{}' no pueden importar de '{}' (línea {}).",
            violation.rule.from,
            violation.rule.to,
            violation.line_number
        )
    }
}

/// Collect violations from a file without failing
/// Useful for --fix mode where we want to process all violations
pub fn collect_violations_from_file(
    _cm: &SourceMap,
    path: &PathBuf,
    ctx: &LinterContext,
) -> Result<Vec<Violation>> {
    // Try to use multi-language parser first
    if let Some(parser) = parsers::get_parser_for_file(path) {
        let source_code = fs::read_to_string(path).into_diagnostic()?;
        return parser.find_violations(&source_code, path, ctx);
    }

    // Fallback: return empty violations for unsupported files
    Ok(Vec::new())
}
