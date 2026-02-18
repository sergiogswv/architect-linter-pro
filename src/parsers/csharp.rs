//! C# parser using Tree-sitter
use super::{ArchitectParser, Import};
use crate::autofix::Violation;
use crate::config::{ForbiddenRule, LinterContext};
use miette::{IntoDiagnostic, Result};
use std::path::Path;
use std::sync::Mutex;
use tree_sitter::{Parser, Query, QueryCursor};
use streaming_iterator::StreamingIterator;

pub struct CSharpParser {
    parser: Mutex<Parser>,
}

impl CSharpParser {
    pub fn new() -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_c_sharp::LANGUAGE.into())
            .expect("Failed to load C# grammar");

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

impl ArchitectParser for CSharpParser {
    fn extract_imports(&self, source_code: &str, _file_path: &Path) -> Result<Vec<Import>> {
        let mut imports = Vec::new();

        let tree = self
            .parser
            .lock()
            .unwrap()
            .parse(source_code, None)
            .ok_or_else(|| miette::miette!("Failed to parse C#"))?;

        // Query for using directives
        let query_source = r#"
            (using_directive
                [(qualified_name) (identifier)] @import_path)
        "#;

        let query = Query::new(&tree_sitter_c_sharp::LANGUAGE.into(), query_source).into_diagnostic()?;

        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(&query, tree.root_node(), source_code.as_bytes());

        while let Some(match_) = matches.next() {
            for capture in match_.captures {
                let node = capture.node;
                let import_path = node.utf8_text(source_code.as_bytes()).into_diagnostic()?;
                let line_number = node.start_position().row + 1;

                // Get the full using statement
                let mut parent = node.parent();
                while let Some(p) = parent {
                    if p.kind() == "using_directive" {
                        break;
                    }
                    parent = p.parent();
                }

                let raw_statement = if let Some(p) = parent {
                    p.utf8_text(source_code.as_bytes())
                        .unwrap_or(import_path)
                        .to_string()
                } else {
                    format!("using {};", import_path)
                };

                imports.push(Import {
                    source: import_path.to_string(),
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
                        rule: ForbiddenRule {
                            from: rule.from.clone(),
                            to: rule.to.clone(),
                        },
                        line_number: import.line_number,
                    });
                }
            }
        }

        Ok(violations)
    }
}
