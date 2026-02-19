//! Ruby parser using Tree-sitter
use super::{ArchitectParser, Import};
use crate::autofix::Violation;
use crate::config::LinterContext;
use miette::{IntoDiagnostic, Result};
use std::path::Path;
use std::sync::Mutex;
use streaming_iterator::StreamingIterator;
use tree_sitter::{Parser, Query, QueryCursor};

pub struct RubyParser {
    parser: Mutex<Parser>,
}

impl RubyParser {
    pub fn new() -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_ruby::LANGUAGE.into())
            .expect("Failed to load Ruby grammar");

        Self {
            parser: Mutex::new(parser),
        }
    }

    fn matches_pattern(path: &str, pattern: &str) -> bool {
        let normalized_path = path.to_lowercase().replace('\\', "/");
        let normalized_pattern = pattern
            .to_lowercase()
            .replace('\\', "/")
            .replace("**", "")
            .replace('*', "");

        normalized_path.contains(&normalized_pattern)
    }
}

impl ArchitectParser for RubyParser {
    fn extract_imports(&self, source_code: &str, _file_path: &Path) -> Result<Vec<Import>> {
        let mut imports = Vec::new();

        let tree = self
            .parser
            .lock()
            .unwrap()
            .parse(source_code, None)
            .ok_or_else(|| miette::miette!("Failed to parse Ruby"))?;

        // Query for require, require_relative, and load
        let query_source = r#"
            (call
              method: (identifier) @method (#match? @method "^(require|require_relative|load)$")
              arguments: (argument_list (string (string_content) @import_path)))
        "#;

        let query =
            Query::new(&tree_sitter_ruby::LANGUAGE.into(), query_source).into_diagnostic()?;

        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(&query, tree.root_node(), source_code.as_bytes());

        while let Some(match_) = matches.next() {
            let mut import_path = String::new();
            let mut line_number = 0;
            let mut node_to_use = None;

            for capture in match_.captures {
                let node = capture.node;
                if capture.index == 1 {
                    // @import_path
                    import_path = node
                        .utf8_text(source_code.as_bytes())
                        .into_diagnostic()?
                        .to_string();
                    line_number = node.start_position().row + 1;
                    node_to_use = Some(node);
                }
            }

            if let Some(node) = node_to_use {
                // Get the full statement
                let mut parent = node.parent();
                while let Some(p) = parent {
                    if p.kind() == "call" || p.kind() == "command" {
                        break;
                    }
                    parent = p.parent();
                }

                let raw_statement = if let Some(p) = parent {
                    p.utf8_text(source_code.as_bytes())
                        .unwrap_or(&import_path)
                        .to_string()
                } else {
                    format!("require '{}'", import_path)
                };

                imports.push(Import {
                    source: import_path,
                    line_number,
                    raw_statement: raw_statement.trim().to_string(),
                });
            }
        }

        Ok(imports)
    }

    fn find_violations(
        &self,
        source_code: &str,
        file_path: &Path,
        context: &LinterContext,
    ) -> Result<Vec<Violation>> {
        let mut violations = Vec::new();
        let imports = self.extract_imports(source_code, file_path)?;
        let file_path_str = file_path.to_string_lossy().to_lowercase();

        for import in imports {
            for rule in &context.forbidden_imports {
                let file_matches = Self::matches_pattern(&file_path_str, &rule.from);
                let import_matches = Self::matches_pattern(&import.source, &rule.to);

                if file_matches && import_matches {
                    violations.push(Violation {
                        file_path: file_path.to_path_buf(),
                        file_content: source_code.to_string(),
                        offensive_import: import.raw_statement.clone(),
                        rule: rule.clone(),
                        line_number: import.line_number,
                    });
                }
            }
        }

        Ok(violations)
    }
}
