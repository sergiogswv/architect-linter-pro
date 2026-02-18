//! Kotlin parser using Tree-sitter
use super::{ArchitectParser, Import};
use crate::autofix::Violation;
use crate::config::{ForbiddenRule, LinterContext};
use miette::{IntoDiagnostic, Result};
use std::path::Path;
use std::sync::Mutex;
use tree_sitter::{Parser, Query, QueryCursor};
use streaming_iterator::StreamingIterator;

pub struct KotlinParser {
    parser: Mutex<Parser>,
}

impl KotlinParser {
    pub fn new() -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_kotlin::LANGUAGE.into())
            .expect("Failed to load Kotlin grammar");

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

        if normalized_path.contains(&normalized_pattern) {
            return true;
        }

        // Kotlin uses dot notation for packages
        let path_with_dots = normalized_path.replace('/', ".");
        let pattern_with_dots = normalized_pattern.replace('/', ".");

        path_with_dots.contains(&pattern_with_dots)
    }
}

impl ArchitectParser for KotlinParser {
    fn extract_imports(&self, source_code: &str, _file_path: &Path) -> Result<Vec<Import>> {
        let mut imports = Vec::new();

        let tree = self
            .parser
            .lock()
            .unwrap()
            .parse(source_code, None)
            .ok_or_else(|| miette::miette!("Failed to parse Kotlin"))?;

        // Query for imports
        let query_source = r#"
            (import) @import_node
        "#;

        let query = Query::new(&tree_sitter_kotlin::LANGUAGE.into(), query_source).into_diagnostic()?;

        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(&query, tree.root_node(), source_code.as_bytes());

        while let Some(match_) = matches.next() {
            for capture in match_.captures {
                let node = capture.node;
                let full_text = node.utf8_text(source_code.as_bytes()).into_diagnostic()?;
                let line_number = node.start_position().row + 1;

                // Strip "import " prefix
                let import_path = full_text.trim_start_matches("import").trim().to_string();

                imports.push(Import {
                    source: import_path,
                    line_number,
                    raw_statement: full_text.trim().to_string(),
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
