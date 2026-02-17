//! TypeScript/JavaScript parser using Tree-sitter

use super::{ArchitectParser, Import};
use crate::autofix::Violation;
use crate::config::LinterContext;
use miette::Result;
use std::path::Path;
use std::sync::Mutex;
use tree_sitter::Parser;

// Import pure functions for unit testing
use super::typescript_pure;

pub struct TypeScriptParser {
    parser: Mutex<Parser>,
}

impl TypeScriptParser {
    pub fn new() -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_typescript::language_typescript())
            .expect("Failed to load TypeScript grammar");

        Self {
            parser: Mutex::new(parser),
        }
    }
}

impl ArchitectParser for TypeScriptParser {
    fn extract_imports(&self, source_code: &str, _file_path: &Path) -> Result<Vec<Import>> {
        // Parse the source code
        let tree = self
            .parser
            .lock()
            .unwrap()
            .parse(source_code, None)
            .ok_or_else(|| miette::miette!("Failed to parse TypeScript"))?;

        // Use the pure function to extract imports from the AST
        let pure_imports = typescript_pure::extract_imports_from_tree(
            &tree,
            source_code,
        )?;

        // Convert pure module Import type to parser Import type
        let imports = pure_imports
            .into_iter()
            .map(|pure_import| Import {
                source: pure_import.source,
                line_number: pure_import.line_number,
                raw_statement: pure_import.raw_statement,
            })
            .collect();

        Ok(imports)
    }

    fn find_violations(
        &self,
        source_code: &str,
        file_path: &Path,
        context: &LinterContext,
    ) -> Result<Vec<Violation>> {
        // Extract imports using the parser
        let imports = self.extract_imports(source_code, file_path)?;

        // Convert parser Import type to pure module Import type
        let pure_imports: Vec<typescript_pure::Import> = imports
            .iter()
            .map(|import| typescript_pure::Import {
                source: import.source.clone(),
                line_number: import.line_number,
                raw_statement: import.raw_statement.clone(),
            })
            .collect();

        // Use the pure function to find violations
        let violations = typescript_pure::find_violations_in_imports(
            file_path,
            source_code,
            &pure_imports,
            context,
        );

        Ok(violations)
    }
}
