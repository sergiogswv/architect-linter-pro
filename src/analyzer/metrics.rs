//! Metrics utilities for counting imports and functions

use crate::analysis_result::LongFunction;
use miette::{IntoDiagnostic, Result};
use std::fs;
use std::path::Path;

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

/// Count functions/methods in a file using Tree-sitter.
pub fn count_functions(path: &Path) -> Result<usize> {
    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

    // Only analyze TypeScript/JavaScript for now
    if !matches!(extension, "ts" | "tsx" | "js" | "jsx") {
        return Ok(0);
    }

    let content = fs::read_to_string(path).into_diagnostic()?;

    use tree_sitter::Parser;
    let mut parser = Parser::new();
    let lang = if matches!(extension, "tsx") {
        tree_sitter_typescript::LANGUAGE_TSX.into()
    } else if matches!(extension, "ts") {
        tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()
    } else {
        // js/jsx: use TypeScript parser as fallback (tree-sitter-javascript not in deps)
        tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()
    };
    parser
        .set_language(&lang)
        .map_err(|e| miette::miette!("Tree-sitter init error: {}", e))?;

    let tree = match parser.parse(&content, None) {
        Some(t) => t,
        None => return Ok(0),
    };

    let count = count_nodes_recursive(tree.root_node(), content.as_bytes());
    Ok(count)
}

fn count_nodes_recursive(node: tree_sitter::Node, source: &[u8]) -> usize {
    let mut count = 0usize;
    match node.kind() {
        // Count class methods and standalone function declarations.
        // Arrow functions and anonymous function expressions are excluded to
        // match the original SWC behaviour (ClassMember::Method +
        // Decl::Fn only; nested arrow functions were not counted).
        "method_definition" | "function_declaration" => {
            // Exclude constructors — match the original SWC behaviour where
            // ClassMember::Constructor was not counted.
            if !is_constructor(node, source) {
                count += 1;
            }
        }
        _ => {}
    }
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            count += count_nodes_recursive(child, source);
        }
    }
    count
}

/// Returns true if the node is a constructor method definition.
fn is_constructor(node: tree_sitter::Node, source: &[u8]) -> bool {
    if node.kind() != "method_definition" {
        return false;
    }
    // The first named child is the property name; check if its text is "constructor"
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            if child.kind() == "property_identifier" {
                if let Ok(text) = child.utf8_text(source) {
                    return text == "constructor";
                }
            }
        }
    }
    false
}

/// Find functions that exceed the max lines threshold using Tree-sitter.
pub fn find_long_functions(path: &Path, max_lines: usize) -> Result<Vec<LongFunction>> {
    let mut long_functions = Vec::new();

    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

    // Only analyze TypeScript/JavaScript for now
    if !matches!(extension, "ts" | "tsx" | "js" | "jsx") {
        return Ok(long_functions);
    }

    let content = fs::read_to_string(path).into_diagnostic()?;

    use tree_sitter::Parser;
    let mut parser = Parser::new();
    let lang = if matches!(extension, "tsx") {
        tree_sitter_typescript::LANGUAGE_TSX.into()
    } else {
        tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()
    };
    parser
        .set_language(&lang)
        .map_err(|e| miette::miette!("Tree-sitter init error: {}", e))?;

    let tree = match parser.parse(&content, None) {
        Some(t) => t,
        None => return Ok(long_functions),
    };

    collect_long_functions_recursive(
        tree.root_node(),
        path,
        max_lines,
        content.as_bytes(),
        &mut long_functions,
    );
    Ok(long_functions)
}

fn collect_long_functions_recursive(
    node: tree_sitter::Node,
    path: &Path,
    max_lines: usize,
    source: &[u8],
    out: &mut Vec<LongFunction>,
) {
    let kind = node.kind();
    if matches!(kind, "method_definition" | "function_declaration") {
        // Exclude constructors to match original SWC behaviour
        if !is_constructor(node, source) {
            let start_line = node.start_position().row + 1; // 1-based
            let end_line = node.end_position().row + 1;
            let lines = end_line.saturating_sub(start_line);

            if lines > max_lines {
                let name =
                    extract_function_name(node, source).unwrap_or_else(|| "anonymous".to_string());
                out.push(LongFunction {
                    file_path: path.to_path_buf(),
                    name,
                    line_start: start_line,
                    lines,
                    threshold: max_lines,
                });
            }
        }
    }

    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            collect_long_functions_recursive(child, path, max_lines, source, out);
        }
    }
}

/// Try to extract the name of a function/method node from source bytes.
fn extract_function_name(node: tree_sitter::Node, source: &[u8]) -> Option<String> {
    // For method_definition: the first child with kind "property_identifier"
    // For function_declaration: child with kind "identifier"
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            match child.kind() {
                "identifier" | "property_identifier" => {
                    if let Ok(text) = child.utf8_text(source) {
                        return Some(text.to_string());
                    }
                }
                _ => {}
            }
        }
    }
    None
}
