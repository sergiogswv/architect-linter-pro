//! Metrics utilities for counting imports and functions

use crate::analysis_result::LongFunction;
use miette::{IntoDiagnostic, Result};
use std::fs;
use std::path::Path;
use swc_common::SourceMap;
use swc_ecma_parser::{lexer::Lexer, EsConfig, Parser, StringInput, Syntax, TsConfig};

/// Represents a function or method call extracted from the AST
#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub line: usize,
}

/// Extract function calls from a file
/// This function parses the AST and extracts all function/method calls
pub fn extract_function_calls(cm: &SourceMap, path: &Path) -> Result<Vec<FunctionCall>> {
    let mut calls = Vec::new();

    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

    // Only analyze TypeScript/JavaScript for now
    if !matches!(extension, "ts" | "tsx" | "js" | "jsx") {
        return Ok(calls);
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
        _ => return Ok(calls),
    };

    let lexer = Lexer::new(syntax, Default::default(), StringInput::from(&*fm), None);
    let mut parser = Parser::new_from(lexer);

    let module = match parser.parse_module() {
        Ok(m) => m,
        Err(_) => return Ok(calls),
    };

    // Recursively visit all nodes in the AST to extract function calls
    for item in &module.body {
        extract_calls_from_module_item(item, &mut calls, cm);
    }

    Ok(calls)
}

/// Recursively extract function calls from a module item
fn extract_calls_from_module_item(
    item: &swc_ecma_ast::ModuleItem,
    calls: &mut Vec<FunctionCall>,
    cm: &SourceMap,
) {
    match item {
        swc_ecma_ast::ModuleItem::Stmt(stmt) => extract_calls_from_stmt(stmt, calls, cm),
        swc_ecma_ast::ModuleItem::ModuleDecl(_) => {}
    }
}

/// Extract calls from a statement
fn extract_calls_from_stmt(
    stmt: &swc_ecma_ast::Stmt,
    calls: &mut Vec<FunctionCall>,
    cm: &SourceMap,
) {
    match stmt {
        swc_ecma_ast::Stmt::Decl(decl) => extract_calls_from_decl(decl, calls, cm),
        swc_ecma_ast::Stmt::Expr(expr_stmt) => {
            extract_calls_from_expr(&expr_stmt.expr, calls, cm);
        }
        swc_ecma_ast::Stmt::Block(block) => {
            for stmt in &block.stmts {
                extract_calls_from_stmt(stmt, calls, cm);
            }
        }
        swc_ecma_ast::Stmt::If(if_stmt) => {
            extract_calls_from_expr(&if_stmt.test, calls, cm);
            extract_calls_from_stmt(&*if_stmt.cons, calls, cm);
            if let Some(alt) = &if_stmt.alt {
                extract_calls_from_stmt(alt, calls, cm);
            }
        }
        swc_ecma_ast::Stmt::While(while_stmt) => {
            extract_calls_from_expr(&while_stmt.test, calls, cm);
            extract_calls_from_stmt(&*while_stmt.body, calls, cm);
        }
        swc_ecma_ast::Stmt::For(for_stmt) => {
            if let Some(init) = &for_stmt.init {
                extract_calls_from_for_init(init, calls, cm);
            }
            if let Some(test) = &for_stmt.test {
                extract_calls_from_expr(test, calls, cm);
            }
            if let Some(update) = &for_stmt.update {
                extract_calls_from_expr(update, calls, cm);
            }
            extract_calls_from_stmt(&*for_stmt.body, calls, cm);
        }
        swc_ecma_ast::Stmt::Return(ret_stmt) => {
            if let Some(arg) = &ret_stmt.arg {
                extract_calls_from_expr(arg, calls, cm);
            }
        }
        _ => {}
    }
}

/// Extract calls from a declaration
fn extract_calls_from_decl(
    decl: &swc_ecma_ast::Decl,
    calls: &mut Vec<FunctionCall>,
    cm: &SourceMap,
) {
    match decl {
        swc_ecma_ast::Decl::Class(class) => {
            for member in &class.class.body {
                if let swc_ecma_ast::ClassMember::Method(method) = member {
                    extract_calls_from_function_body(&method.function, calls, cm);
                }
            }
        }
        swc_ecma_ast::Decl::Fn(fn_decl) => {
            extract_calls_from_function_body(&fn_decl.function, calls, cm);
        }
        swc_ecma_ast::Decl::Var(var_decl) => {
            for decl in &var_decl.decls {
                if let Some(init) = &decl.init {
                    extract_calls_from_expr(init, calls, cm);
                }
            }
        }
        _ => {}
    }
}

/// Extract calls from a function body
fn extract_calls_from_function_body(
    function: &swc_ecma_ast::Function,
    calls: &mut Vec<FunctionCall>,
    cm: &SourceMap,
) {
    if let Some(body) = &function.body {
        for stmt in &body.stmts {
            extract_calls_from_stmt(stmt, calls, cm);
        }
    }
}

/// Extract calls from a for loop init expression
fn extract_calls_from_for_init(
    init: &swc_ecma_ast::VarDeclOrExpr,
    calls: &mut Vec<FunctionCall>,
    cm: &SourceMap,
) {
    match init {
        swc_ecma_ast::VarDeclOrExpr::VarDecl(var_decl) => {
            for decl in &var_decl.decls {
                if let Some(init) = &decl.init {
                    extract_calls_from_expr(init, calls, cm);
                }
            }
        }
        swc_ecma_ast::VarDeclOrExpr::Expr(expr) => {
            extract_calls_from_expr(expr, calls, cm);
        }
    }
}

/// Extract calls from an expression
fn extract_calls_from_expr(
    expr: &swc_ecma_ast::Expr,
    calls: &mut Vec<FunctionCall>,
    cm: &SourceMap,
) {
    match expr {
        swc_ecma_ast::Expr::Call(call_expr) => {
            let line = cm.lookup_char_pos(call_expr.span.lo).line;

            // Extract the function name
            let name = extract_call_name(&call_expr.callee);

            calls.push(FunctionCall { name, line });

            // Also extract calls from arguments
            for arg in &call_expr.args {
                extract_calls_from_expr_or_spread(arg, calls, cm);
            }
        }
        swc_ecma_ast::Expr::Member(member_expr) => {
            extract_calls_from_expr(&*member_expr.obj, calls, cm);
            if let swc_ecma_ast::MemberProp::Computed(prop) = &member_expr.prop {
                extract_calls_from_expr(&*prop.expr, calls, cm);
            }
        }
        swc_ecma_ast::Expr::Ident(_) => {}
        swc_ecma_ast::Expr::Lit(_) => {}
        swc_ecma_ast::Expr::This(_) => {}
        swc_ecma_ast::Expr::Array(array_lit) => {
            for elem in &array_lit.elems {
                if let Some(elem_expr) = elem {
                    extract_calls_from_expr_or_spread(elem_expr, calls, cm);
                }
            }
        }
        swc_ecma_ast::Expr::Unary(unary_expr) => {
            extract_calls_from_expr(&*unary_expr.arg, calls, cm);
        }
        swc_ecma_ast::Expr::Bin(bin_expr) => {
            extract_calls_from_expr(&*bin_expr.left, calls, cm);
            extract_calls_from_expr(&*bin_expr.right, calls, cm);
        }
        swc_ecma_ast::Expr::Assign(assign_expr) => {
            // Handle PatOrExpr
            match &assign_expr.left {
                swc_ecma_ast::PatOrExpr::Pat(pat) => {
                    // For patterns, we don't extract calls
                    let _ = pat;
                }
                swc_ecma_ast::PatOrExpr::Expr(expr) => {
                    extract_calls_from_expr(expr, calls, cm);
                }
            }
            extract_calls_from_expr(&*assign_expr.right, calls, cm);
        }
        swc_ecma_ast::Expr::Update(update_expr) => {
            extract_calls_from_expr(&*update_expr.arg, calls, cm);
        }
        swc_ecma_ast::Expr::Cond(cond_expr) => {
            extract_calls_from_expr(&*cond_expr.test, calls, cm);
            extract_calls_from_expr(&*cond_expr.cons, calls, cm);
            extract_calls_from_expr(&*cond_expr.alt, calls, cm);
        }
        swc_ecma_ast::Expr::New(new_expr) => {
            extract_calls_from_expr(&*new_expr.callee, calls, cm);
        }
        swc_ecma_ast::Expr::Seq(seq_expr) => {
            for expr in &seq_expr.exprs {
                extract_calls_from_expr(expr, calls, cm);
            }
        }
        _ => {}
    }
}

/// Extract calls from ExprOrSpread (used in function arguments and array elements)
fn extract_calls_from_expr_or_spread(
    expr_or_spread: &swc_ecma_ast::ExprOrSpread,
    calls: &mut Vec<FunctionCall>,
    cm: &SourceMap,
) {
    extract_calls_from_expr(&expr_or_spread.expr, calls, cm);
}

/// Extract the name from a call expression callee
fn extract_call_name(callee: &swc_ecma_ast::Callee) -> String {
    match callee {
        swc_ecma_ast::Callee::Expr(callee_expr) => {
            match &**callee_expr {
                swc_ecma_ast::Expr::Ident(ident) => ident.sym.to_string(),
                swc_ecma_ast::Expr::Member(member_expr) => {
                    // Handle methods like this.helper() or console.log()
                    match &*member_expr.obj {
                        swc_ecma_ast::Expr::Ident(obj_ident) => {
                            match &member_expr.prop {
                                swc_ecma_ast::MemberProp::Ident(prop_ident) => {
                                    format!("{}.{}", obj_ident.sym, prop_ident.sym)
                                }
                                _ => "member_call".to_string(),
                            }
                        }
                        swc_ecma_ast::Expr::This(_) => {
                            match &member_expr.prop {
                                swc_ecma_ast::MemberProp::Ident(prop_ident) => {
                                    format!("this.{}", prop_ident.sym)
                                }
                                _ => "this.method".to_string(),
                            }
                        }
                        _ => "unknown_call".to_string(),
                    }
                }
                _ => "anonymous_call".to_string(),
            }
        }
        swc_ecma_ast::Callee::Super(_) => "super_call".to_string(),
        swc_ecma_ast::Callee::Import(_) => "import_call".to_string(),
    }
}

/// Count imports in a file
pub fn count_imports(path: &Path) -> Result<usize> {
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
pub fn count_functions(cm: &SourceMap, path: &Path) -> Result<usize> {
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
    path: &Path,
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
                            file_path: path.to_path_buf(),
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
                    file_path: path.to_path_buf(),
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
                        file_path: path.to_path_buf(),
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
                                file_path: path.to_path_buf(),
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
