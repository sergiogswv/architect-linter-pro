//! TypeScript/JavaScript parser using Tree-sitter

use super::{ArchitectParser, Import};
use crate::autofix::Violation;
use crate::config::{ForbiddenRule, LinterContext};
use miette::Result;
use std::path::Path;
use std::sync::Mutex;
use tree_sitter::Parser;
use miette::IntoDiagnostic;
use streaming_iterator::StreamingIterator;
use tree_sitter::{Query, QueryCursor, Tree};
use crate::security::cfg::{CFG, NodeType};

pub struct TypeScriptParser {
    parser: Mutex<Parser>,
}

impl TypeScriptParser {
    pub fn new() -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
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
        let pure_imports = extract_imports_from_tree(
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
        let pure_imports: Vec<PureImport> = imports
            .iter()
            .map(|import| PureImport {
                source: import.source.clone(),
                line_number: import.line_number,
                raw_statement: import.raw_statement.clone(),
            })
            .collect();

        // Use the pure function to find violations
        let violations = find_violations_in_imports(
            file_path,
            source_code,
            &pure_imports,
            context,
        );

        Ok(violations)
    }

    fn audit_security(
        &self,
        _source_code: &str,
        _file_path: &Path,
        _context: &LinterContext,
    ) -> Result<Vec<Violation>> {
        // TEMP: Taint analysis disabled due to high false positive rate
        // The TaintEngine uses overly broad substring matching:
        // - Any function with "execute", "query", "eval" triggers as sink
        // - Any parameter is treated as potential source
        // - Generates false positives for normal code patterns like executeWithErrorHandling()
        //
        // TODO: Rewrite TaintEngine with:
        // 1. More precise pattern matching (exact function names, not substrings)
        // 2. Context-aware source detection (only actual user inputs)
        // 3. Proper data flow analysis rather than string matching

        Ok(Vec::new())
    }
}

// ============================================================================
// Pure Functions (Consolidated from typescript_pure.rs)
// ============================================================================

/// Represents an import statement extracted from source code
#[derive(Debug, Clone, PartialEq)]
pub struct PureImport {
    /// The import source/path (e.g., "../services/user", "apps/user/models")
    pub source: String,
    /// Line number where the import appears
    pub line_number: usize,
    /// Full import statement text
    pub raw_statement: String,
}

/// Extract imports from a parsed Tree-sitter tree
///
/// This is a pure function that operates on the already-parsed tree,
/// making it easier to test without needing to set up the full parser.
///
/// # Arguments
/// * `tree` - The parsed Tree-sitter tree
/// * `source_code` - The source code text (needed for extracting text content)
///
/// # Returns
/// A vector of Import objects extracted from the tree
///
/// # Examples
/// ```
/// // This would be tested indirectly through the main parser tests
/// // as it requires a valid Tree-sitter tree
/// ```
pub fn extract_imports_from_tree(tree: &Tree, source_code: &str) -> miette::Result<Vec<PureImport>> {
    let mut imports = Vec::new();

    // Query for import declarations
    let query_source = r#"
        (import_statement
          source: (string (string_fragment) @import_path))
    "#;

    let query = Query::new(
        &tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
        query_source,
    )
    .into_diagnostic()?;

    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, tree.root_node(), source_code.as_bytes());

    while let Some(match_) = matches.next() {
        for capture in match_.captures {
            let node = capture.node;
            let import_path = node.utf8_text(source_code.as_bytes()).into_diagnostic()?;
            let line_number = node.start_position().row + 1;

            // Get the full import statement
            let parent = node.parent();
            let raw_statement = if let Some(p) = parent {
                p.utf8_text(source_code.as_bytes())
                    .unwrap_or(import_path)
                    .to_string()
            } else {
                format!("import ... from '{}'", import_path)
            };

            imports.push(PureImport {
                source: import_path.to_string(),
                line_number,
                raw_statement,
            });
        }
    }

    Ok(imports)
}

/// Normalize a path string for pattern matching
///
/// Converts Windows backslashes to forward slashes and converts to lowercase
/// for case-insensitive matching.
///
/// # Arguments
/// * `path` - The path to normalize
///
/// # Returns
/// A normalized string with consistent separators and case
///
/// # Examples
/// ```
/// use architect_linter_pro::parsers::typescript::normalize_path;
///
/// assert_eq!(normalize_path("C:\\Project\\Src\\File.ts"), "c:/project/src/file.ts");
/// assert_eq!(normalize_path("src/Components/Button.tsx"), "src/components/button.tsx");
/// ```
pub fn normalize_path(path: &str) -> String {
    path.to_lowercase().replace('\\', "/")
}

/// Normalize a glob pattern for pattern matching
///
/// Removes glob wildcards (**, *) to create a simpler pattern string.
///
/// # Arguments
/// * `pattern` - The glob pattern to normalize
///
/// # Returns
/// A normalized pattern string with wildcards removed
///
/// # Examples
/// ```
/// use architect_linter_pro::parsers::typescript::normalize_pattern;
///
/// assert_eq!(normalize_pattern("src/components/**"), "src/components/");
/// assert_eq!(normalize_pattern("**/*.tsx"), "/.tsx");
/// ```
pub fn normalize_pattern(pattern: &str) -> String {
    pattern
        .to_lowercase()
        .replace('\\', "/")
        .replace("**", "")
        .replace('*', "")
}

/// Extract the folder component from a pattern containing "src/"
///
/// This is used to match imports against file path patterns.
/// For example, "src/services/" becomes "services/" for flexible matching.
///
/// # Arguments
/// * `pattern` - The pattern to extract from
///
/// # Returns
/// The folder component after "src/", or None if not found
///
/// # Examples
/// ```
/// use architect_linter_pro::parsers::typescript::extract_folder_from_pattern;
///
/// assert_eq!(extract_folder_from_pattern("src/services/"), Some("services/".to_string()));
/// assert_eq!(extract_folder_from_pattern("src/components/"), Some("components/".to_string()));
/// assert_eq!(extract_folder_from_pattern("lib/utils/"), None);
/// ```
pub fn extract_folder_from_pattern(pattern: &str) -> Option<String> {
    let normalized = normalize_pattern(pattern);
    if normalized.contains("src/") {
        normalized.strip_prefix("src/").map(|s| s.to_string())
    } else {
        None
    }
}

/// Generate alternative import patterns to check against a folder pattern
///
/// When matching imports against file path patterns, we need to check multiple
/// representations because imports can be relative, absolute, or use aliases.
///
/// # Arguments
/// * `folder` - The folder name to generate patterns for
///
/// # Returns
/// A vector of pattern variations to check
///
/// # Examples
/// ```
/// use architect_linter_pro::parsers::typescript::generate_import_patterns;
///
/// let patterns = generate_import_patterns("services/");
/// assert!(patterns.contains(&"/services/".to_string()));
/// assert!(patterns.contains(&"../services/".to_string()));
/// assert!(patterns.contains(&"@/services/".to_string()));
/// ```
pub fn generate_import_patterns(folder: &str) -> Vec<String> {
    vec![
        format!("/{}", folder),   // Absolute path pattern
        format!("../{}", folder), // Relative path pattern
        format!("@/{}", folder),  // Alias pattern
        folder.to_string(),       // Direct pattern
    ]
}

/// Check if a path matches a given pattern
///
/// This performs flexible matching to handle different path formats:
/// - Absolute paths: "src/components/button.tsx"
/// - Relative imports: "../services/userservice"
/// - Alias imports: "@/services/api"
///
/// # Arguments
/// * `path` - The path to check
/// * `pattern` - The pattern to match against
///
/// # Returns
/// true if the path matches the pattern, false otherwise
///
/// # Examples
/// ```
/// use architect_linter_pro::parsers::typescript::matches_pattern;
///
/// // Direct containment
/// assert!(matches_pattern("src/components/button.tsx", "src/components/"));
///
/// // Relative imports
/// assert!(matches_pattern("../services/userservice", "src/services/"));
///
/// // Alias imports
/// assert!(matches_pattern("@/services/api", "src/services/"));
///
/// // Non-matching
/// assert!(!matches_pattern("src/models/user.ts", "src/components/"));
/// ```
pub fn matches_pattern(path: &str, pattern: &str) -> bool {
    let normalized_path = normalize_path(path);
    let normalized_pattern = normalize_pattern(pattern);

    // Empty pattern matches nothing
    if normalized_pattern.is_empty() {
        return false;
    }

    // Direct containment check
    if normalized_path.contains(&normalized_pattern) {
        return true;
    }

    // For patterns with "src/", extract folder and check alternatives
    if let Some(folder_part) = extract_folder_from_pattern(pattern) {
        // Don't match if the folder_part is empty (e.g., pattern is just "src/")
        if folder_part.is_empty() || folder_part == "/" {
            return false;
        }

        let import_patterns = generate_import_patterns(&folder_part);
        for import_pattern in import_patterns {
            if normalized_path.contains(&import_pattern) {
                return true;
            }
        }
    }

    false
}

/// Check if a file path and import source match a forbidden rule
///
/// # Arguments
/// * `file_path` - The path of the file containing the import
/// * `import_source` - The import path being checked
/// * `rule` - The forbidden rule to check against
///
/// # Returns
/// true if the combination violates the rule, false otherwise
///
/// # Examples
/// ```
/// use architect_linter_pro::parsers::typescript::matches_forbidden_rule;
/// use architect_linter_pro::config::ForbiddenRule;
///
/// let rule = ForbiddenRule {
///     from: "src/controller/".to_string(),
///     to: "src/repository/".to_string(),
///     severity: None,
///     reason: None,
/// };
///
/// // Violation: controller importing from repository
/// assert!(matches_forbidden_rule(
///     "src/controller/user.controller.ts",
///     "../repository/user",
///     &rule
/// ));
///
/// // No violation: service importing from repository
/// assert!(!matches_forbidden_rule(
///     "src/service/user.service.ts",
///     "../repository/user",
///     &rule
/// ));
/// ```
pub fn matches_forbidden_rule(file_path: &str, import_source: &str, rule: &ForbiddenRule) -> bool {
    let file_matches = matches_pattern(file_path, &rule.from);
    let import_matches = matches_pattern(import_source, &rule.to);

    file_matches && import_matches
}

/// Check if a controller file imports from a repository (MVC pattern violation)
///
/// This is a special-case rule for common MVC architecture violations.
/// Checks if the import path contains "repository" (with or without a dot prefix).
///
/// # Arguments
/// * `file_path` - The path of the file to check
/// * `import_source` - The import path being checked
///
/// # Returns
/// true if this is a controller importing a repository, false otherwise
///
/// # Examples
/// ```
/// use architect_linter_pro::parsers::typescript::is_controller_to_repository_violation;
///
/// // Matches: ../repository/user
/// assert!(is_controller_to_repository_violation(
///     "src/controller/user.controller.ts",
///     "../repository/user"
/// ));
///
/// // Matches: ./user.repository
/// assert!(is_controller_to_repository_violation(
///     "src/controller/user.controller.ts",
///     "./user.repository"
/// ));
///
/// // Doesn't match: not a repository import
/// assert!(!is_controller_to_repository_violation(
///     "src/controller/user.controller.ts",
///     "../service/user"
/// ));
/// ```
pub fn is_controller_to_repository_violation(file_path: &str, import_source: &str) -> bool {
    let normalized_file_path = normalize_path(file_path);
    let normalized_import = normalize_path(import_source);

    let is_controller = normalized_file_path.contains("controller");
    let is_repository =
        normalized_import.contains("repository") || normalized_import.contains(".repository");

    is_controller && is_repository
}

/// Create violations for a matching rule
///
/// # Arguments
/// * `file_path` - The path of the file
/// * `source_code` - The source code content
/// * `import` - The import that violates the rule
/// * `rule` - The rule being violated
///
/// # Returns
/// A Violation object representing the violation
pub fn create_violation(
    file_path: &Path,
    source_code: &str,
    import: &PureImport,
    rule: ForbiddenRule,
) -> Violation {
    Violation {
        file_path: file_path.to_path_buf(),
        file_content: source_code.to_string(),
        offensive_import: import.raw_statement.clone(),
        rule,
        line_number: import.line_number,
    }
}

/// Find all violations in a set of imports
///
/// # Arguments
/// * `file_path` - The path of the file being checked
/// * `source_code` - The source code content
/// * `imports` - List of imports extracted from the file
/// * `context` - The linter context containing rules
///
/// # Returns
/// A vector of violations found
pub fn find_violations_in_imports(
    file_path: &Path,
    source_code: &str,
    imports: &[PureImport],
    context: &LinterContext,
) -> Vec<Violation> {
    let mut violations = Vec::new();
    let file_path_str = file_path.to_string_lossy().to_string();

    for import in imports {
        // Check against configured forbidden rules
        for rule in &context.forbidden_imports {
            if matches_forbidden_rule(&file_path_str, &import.source, rule) {
                violations.push(create_violation(
                    file_path,
                    source_code,
                    import,
                    rule.clone(),
                ));
            }
        }

        // Check for controller-to-repository violations
        if is_controller_to_repository_violation(&file_path_str, &import.source) {
            violations.push(create_violation(
                file_path,
                source_code,
                import,
                ForbiddenRule {
                    from: "controller".to_string(),
                    to: ".repository".to_string(),
                    severity: Some(crate::config::Severity::Error),
                    reason: None,
                },
            ));
        }
    }

    violations
}

/// Construye un CFG básico para TypeScript buscando fuentes de entrada y sumideros
pub fn build_cfg_from_tree(tree: &Tree, source_code: &str) -> CFG {
    let mut cfg = CFG::new();
    let root = tree.root_node();
    let start_node = cfg.add_node(NodeType::Entry, "START".to_string(), 1);

    // Query para detectar llamadas a funciones (posibles sinks) y accesos a objetos (posibles sources)
    let query_source = r#"
        (call_expression
          function: (identifier) @func_name)
        (call_expression
          function: (member_expression
            property: (property_identifier) @method_name))
        (member_expression
          object: (identifier) @obj_name
          property: (property_identifier) @prop_name)
    "#;

    let query = Query::new(
        &tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
        query_source,
    ).expect("Failed to create TS Security Query");

    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, root, source_code.as_bytes());

    let mut last_node_id = start_node;

    while let Some(match_) = matches.next() {
        for capture in match_.captures {
            let node = capture.node;
            let name = node.utf8_text(source_code.as_bytes()).unwrap_or("unknown");
            let line = node.start_position().row + 1;

            let node_type = match name {
                "query" | "execute" | "eval" | "exec" | "spawn" => NodeType::Sink,
                "body" | "params" | "query_params" => NodeType::Source,
                _ => NodeType::Call,
            };

            let current_node_id = cfg.add_node(node_type, name.to_string(), line);
            cfg.add_edge(last_node_id, current_node_id);
            last_node_id = current_node_id;
        }
    }

    cfg.add_node(NodeType::Exit, "END".to_string(), root.end_position().row + 1);
    cfg
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // normalize_path tests
    // =========================================================================

    #[test]
    fn test_normalize_path_windows_to_unix() {
        assert_eq!(
            normalize_path("C:\\Project\\Src\\File.ts"),
            "c:/project/src/file.ts"
        );
        assert_eq!(
            normalize_path("src\\components\\Button.tsx"),
            "src/components/button.tsx"
        );
    }

    #[test]
    fn test_normalize_path_lowercase() {
        assert_eq!(
            normalize_path("SRC/Components/Button"),
            "src/components/button"
        );
        assert_eq!(normalize_path("MyFile.TS"), "myfile.ts");
    }

    #[test]
    fn test_normalize_path_mixed_separators() {
        assert_eq!(
            normalize_path("src\\components/Button.tsx"),
            "src/components/button.tsx"
        );
    }

    // =========================================================================
    // normalize_pattern tests
    // =========================================================================

    #[test]
    fn test_normalize_pattern_wildcards() {
        assert_eq!(normalize_pattern("src/components/**"), "src/components/");
        assert_eq!(normalize_pattern("**/*.tsx"), "/.tsx");
        assert_eq!(normalize_pattern("src/services/*"), "src/services/");
    }

    #[test]
    fn test_normalize_pattern_no_wildcards() {
        assert_eq!(normalize_pattern("src/components/"), "src/components/");
        assert_eq!(normalize_pattern("repository"), "repository");
    }

    // =========================================================================
    // extract_folder_from_pattern tests
    // =========================================================================

    #[test]
    fn test_extract_folder_from_pattern_with_src() {
        assert_eq!(
            extract_folder_from_pattern("src/services/"),
            Some("services/".to_string())
        );
        assert_eq!(
            extract_folder_from_pattern("src/components/**"),
            Some("components/".to_string())
        );
    }

    #[test]
    fn test_extract_folder_from_pattern_without_src() {
        assert_eq!(extract_folder_from_pattern("lib/utils/"), None);
        assert_eq!(extract_folder_from_pattern("repository"), None);
    }

    // =========================================================================
    // generate_import_patterns tests
    // =========================================================================

    #[test]
    fn test_generate_import_patterns() {
        let patterns = generate_import_patterns("services/");
        assert!(patterns.contains(&"/services/".to_string()));
        assert!(patterns.contains(&"../services/".to_string()));
        assert!(patterns.contains(&"@/services/".to_string()));
        assert!(patterns.contains(&"services/".to_string()));
        assert_eq!(patterns.len(), 4);
    }

    // =========================================================================
    // matches_pattern tests
    // =========================================================================

    #[test]
    fn test_matches_pattern_direct_containment() {
        assert!(matches_pattern(
            "src/components/button.tsx",
            "src/components/"
        ));
        assert!(matches_pattern(
            "src/components/nested/Button.tsx",
            "src/components/"
        ));
    }

    #[test]
    fn test_matches_pattern_relative_imports() {
        assert!(matches_pattern("../services/userservice", "src/services/"));
        assert!(matches_pattern("../repository/user", "src/repository/"));
    }

    #[test]
    fn test_matches_pattern_alias_imports() {
        assert!(matches_pattern("@/services/api", "src/services/"));
        assert!(matches_pattern("@/models/user", "src/models/"));
    }

    #[test]
    fn test_matches_pattern_case_insensitive() {
        assert!(matches_pattern("SRC/Components/Button", "src/components/"));
        assert!(matches_pattern("src/COMPONENTS/Button", "src/components/"));
    }

    #[test]
    fn test_matches_pattern_no_match() {
        assert!(!matches_pattern("src/models/user.ts", "src/components/"));
        assert!(!matches_pattern("test.ts", "src/"));
    }

    #[test]
    fn test_matches_pattern_empty_pattern() {
        assert!(!matches_pattern("src/components/Button.tsx", ""));
        assert!(!matches_pattern("test.ts", ""));
    }

    #[test]
    fn test_matches_pattern_windows_paths() {
        assert!(matches_pattern(
            "C:\\Project\\Src\\Components\\Button.tsx",
            "src/components/"
        ));
    }

    // =========================================================================
    // matches_forbidden_rule tests
    // =========================================================================

    #[test]
    fn test_matches_forbidden_rule_violation() {
        let rule = ForbiddenRule {
            from: "src/controller/".to_string(),
            to: "src/repository/".to_string(),
            severity: None,
            reason: None,
        };

        assert!(matches_forbidden_rule(
            "src/controller/user.controller.ts",
            "../repository/user",
            &rule
        ));

        assert!(matches_forbidden_rule(
            "SRC/CONTROLLER/User.Controller.ts",
            "../repository/user",
            &rule
        ));
    }

    #[test]
    fn test_matches_forbidden_rule_no_violation_different_file() {
        let rule = ForbiddenRule {
            from: "src/controller/".to_string(),
            to: "src/repository/".to_string(),
            severity: None,
            reason: None,
        };

        assert!(!matches_forbidden_rule(
            "src/service/user.service.ts",
            "../repository/user",
            &rule
        ));
    }

    #[test]
    fn test_matches_forbidden_rule_no_violation_different_import() {
        let rule = ForbiddenRule {
            from: "src/controller/".to_string(),
            to: "src/repository/".to_string(),
            severity: None,
            reason: None,
        };

        assert!(!matches_forbidden_rule(
            "src/controller/user.controller.ts",
            "../service/user",
            &rule
        ));
    }

    // =========================================================================
    // is_controller_to_repository_violation tests
    // =========================================================================

    #[test]
    fn test_is_controller_to_repository_violation_true() {
        assert!(is_controller_to_repository_violation(
            "src/controller/user.controller.ts",
            "../repository/user"
        ));

        assert!(is_controller_to_repository_violation(
            "CONTROLLER/User.Controller.ts",
            "./user.repository"
        ));
    }

    #[test]
    fn test_is_controller_to_repository_violation_false_not_controller() {
        assert!(!is_controller_to_repository_violation(
            "src/service/user.service.ts",
            "../repository/user"
        ));
    }

    #[test]
    fn test_is_controller_to_repository_violation_false_not_repository() {
        assert!(!is_controller_to_repository_violation(
            "src/controller/user.controller.ts",
            "../service/user"
        ));
    }

    // =========================================================================
    // Integration-style tests
    // =========================================================================

    #[test]
    fn test_find_violations_in_imports_multiple_violations() {
        let imports = vec![
            PureImport {
                source: "../repository/user".to_string(),
                line_number: 1,
                raw_statement: "import { User } from '../repository/user';".to_string(),
            },
            PureImport {
                source: "../repository/product".to_string(),
                line_number: 2,
                raw_statement: "import { Product } from '../repository/product';".to_string(),
            },
        ];

        let context = LinterContext {
            max_lines: 100,
            framework: crate::config::Framework::NestJS,
            pattern: crate::config::ArchPattern::MVC,
            forbidden_imports: vec![ForbiddenRule {
                from: "src/controller/".to_string(),
                to: "src/repository/".to_string(),
                severity: None,
                reason: None,
            }],
            ignored_paths: vec![],
            ai_configs: vec![],
            ..Default::default()
        };

        let violations = find_violations_in_imports(
            Path::new("src/controller/user.controller.ts"),
            "source code",
            &imports,
            &context,
        );

        // Should have 4 violations:
        // - 2 from the configured forbidden rule (user.repository and product.repository)
        // - 2 from the controller-to-repository rule (same imports)
        assert_eq!(violations.len(), 4);
    }

    #[test]
    fn test_find_violations_in_imports_no_violations() {
        let imports = vec![PureImport {
            source: "../service/user".to_string(),
            line_number: 1,
            raw_statement: "import { UserService } from '../service/user';".to_string(),
        }];

        let context = LinterContext {
            max_lines: 100,
            framework: crate::config::Framework::NestJS,
            pattern: crate::config::ArchPattern::MVC,
            forbidden_imports: vec![ForbiddenRule {
                from: "src/controller/".to_string(),
                to: "src/repository/".to_string(),
                severity: None,
                reason: None,
            }],
            ignored_paths: vec![],
            ai_configs: vec![],
            ..Default::default()
        };

        let violations = find_violations_in_imports(
            Path::new("src/controller/user.controller.ts"),
            "source code",
            &imports,
            &context,
        );

        assert_eq!(violations.len(), 0);
    }
}
