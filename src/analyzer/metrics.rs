//! Metrics utilities for counting imports and functions

use crate::analysis_result::LongFunction;
use miette::{IntoDiagnostic, Result};
use std::fs;
use std::path::PathBuf;
use swc_common::SourceMap;
use swc_ecma_parser::{lexer::Lexer, EsConfig, Parser, StringInput, Syntax, TsConfig};

/// Count imports in a file
pub fn count_imports(path: &PathBuf) -> Result<usize> {
    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

    // Only count for supported file types
    if !matches!(
        extension,
        "ts" | "tsx" | "js" | "jsx" | "py" | "go" | "php" | "java"
    ) {
        return Ok(0);
    }

    let content = fs::read_to_string(path).into_diagnostic()?;
    let mut count = 0usize;

    for line in content.lines() {
        let trimmed = line.trim();
        // TypeScript/JavaScript
        if trimmed.starts_with("import ") || trimmed.starts_with("import{") {
            count += 1;
        }
        // Python
        else if trimmed.starts_with("import ") || trimmed.starts_with("from ") {
            count += 1;
        }
        // Go
        else if trimmed.starts_with("import ") || trimmed == "import (" {
            count += 1;
        }
        // PHP
        else if trimmed.starts_with("use ")
            || trimmed.starts_with("require ")
            || trimmed.starts_with("include ")
        {
            count += 1;
        }
        // Java
        else if trimmed.starts_with("import ") {
            count += 1;
        }
    }

    Ok(count)
}

/// Count functions in a file
pub fn count_functions(cm: &SourceMap, path: &PathBuf) -> Result<usize> {
    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

    // Only analyze TypeScript/JavaScript for now
    if !matches!(extension, "ts" | "tsx" | "js" | "jsx") {
        return Ok(0);
    }

    let fm = cm.load_file(path).into_diagnostic()?;

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
        _ => return Ok(0),
    };

    let lexer = Lexer::new(syntax, Default::default(), StringInput::from(&*fm), None);
    let mut parser = Parser::new_from(lexer);

    let module = match parser.parse_module() {
        Ok(m) => m,
        Err(_) => return Ok(0),
    };

    let mut count = 0usize;

    for item in &module.body {
        if let swc_ecma_ast::ModuleItem::Stmt(swc_ecma_ast::Stmt::Decl(
            swc_ecma_ast::Decl::Class(c),
        )) = item
        {
            for member in &c.class.body {
                if let swc_ecma_ast::ClassMember::Method(_) = member {
                    count += 1;
                }
            }
        }
        // Count standalone functions
        else if let swc_ecma_ast::ModuleItem::Stmt(swc_ecma_ast::Stmt::Decl(
            swc_ecma_ast::Decl::Fn(_),
        )) = item
        {
            count += 1;
        }
    }

    Ok(count)
}

/// Find functions that exceed the max lines threshold
pub fn find_long_functions(
    cm: &SourceMap,
    path: &PathBuf,
    max_lines: usize,
) -> Result<Vec<LongFunction>> {
    let mut long_functions = Vec::new();

    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

    // Only analyze TypeScript/JavaScript for now
    if !matches!(extension, "ts" | "tsx" | "js" | "jsx") {
        return Ok(long_functions);
    }

    let fm = cm.load_file(path).into_diagnostic()?;

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
        _ => return Ok(long_functions),
    };

    let lexer = Lexer::new(syntax, Default::default(), StringInput::from(&*fm), None);
    let mut parser = Parser::new_from(lexer);

    let module = match parser.parse_module() {
        Ok(m) => m,
        Err(_) => return Ok(long_functions),
    };

    for item in &module.body {
        // Check class methods
        if let swc_ecma_ast::ModuleItem::Stmt(swc_ecma_ast::Stmt::Decl(
            swc_ecma_ast::Decl::Class(c),
        )) = item
        {
            for member in &c.class.body {
                if let swc_ecma_ast::ClassMember::Method(m) = member {
                    let lo = cm.lookup_char_pos(m.span.lo).line;
                    let hi = cm.lookup_char_pos(m.span.hi).line;
                    let lines = hi - lo;

                    if lines > max_lines {
                        // Get method name
                        let name = match &m.key {
                            swc_ecma_ast::PropName::Ident(id) => id.sym.to_string(),
                            swc_ecma_ast::PropName::Str(s) => s.value.to_string(),
                            swc_ecma_ast::PropName::Num(n) => n.value.to_string(),
                            swc_ecma_ast::PropName::BigInt(b) => b.value.to_string(),
                            _ => "anonymous".to_string(),
                        };

                        long_functions.push(LongFunction {
                            file_path: path.clone(),
                            name,
                            line_start: lo,
                            lines,
                            threshold: max_lines,
                        });
                    }
                }
            }
        }
        // Check standalone function declarations
        else if let swc_ecma_ast::ModuleItem::Stmt(swc_ecma_ast::Stmt::Decl(
            swc_ecma_ast::Decl::Fn(f),
        )) = item
        {
            let lo = cm.lookup_char_pos(f.function.span.lo).line;
            let hi = cm.lookup_char_pos(f.function.span.hi).line;
            let lines = hi - lo;

            if lines > max_lines {
                let name = f.ident.sym.to_string();

                long_functions.push(LongFunction {
                    file_path: path.clone(),
                    name,
                    line_start: lo,
                    lines,
                    threshold: max_lines,
                });
            }
        }
        // Check exported functions
        else if let swc_ecma_ast::ModuleItem::ModuleDecl(swc_ecma_ast::ModuleDecl::ExportDecl(
            e,
        )) = item
        {
            if let swc_ecma_ast::Decl::Fn(f) = &e.decl {
                let lo = cm.lookup_char_pos(f.function.span.lo).line;
                let hi = cm.lookup_char_pos(f.function.span.hi).line;
                let lines = hi - lo;

                if lines > max_lines {
                    let name = f.ident.sym.to_string();

                    long_functions.push(LongFunction {
                        file_path: path.clone(),
                        name,
                        line_start: lo,
                        lines,
                        threshold: max_lines,
                    });
                }
            }
            // Also check exported classes
            else if let swc_ecma_ast::Decl::Class(c) = &e.decl {
                for member in &c.class.body {
                    if let swc_ecma_ast::ClassMember::Method(m) = member {
                        let lo = cm.lookup_char_pos(m.span.lo).line;
                        let hi = cm.lookup_char_pos(m.span.hi).line;
                        let lines = hi - lo;

                        if lines > max_lines {
                            let name = match &m.key {
                                swc_ecma_ast::PropName::Ident(id) => id.sym.to_string(),
                                swc_ecma_ast::PropName::Str(s) => s.value.to_string(),
                                swc_ecma_ast::PropName::Num(n) => n.value.to_string(),
                                swc_ecma_ast::PropName::BigInt(b) => b.value.to_string(),
                                _ => "anonymous".to_string(),
                            };

                            long_functions.push(LongFunction {
                                file_path: path.clone(),
                                name,
                                line_start: lo,
                                lines,
                                threshold: max_lines,
                            });
                        }
                    }
                }
            }
        }
    }

    Ok(long_functions)
}
