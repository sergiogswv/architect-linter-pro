//! Pure functions for TypeScript parser
//!
//! This module contains pure functions that can be unit tested independently
//! of the Tree-sitter parser and file I/O operations.

use crate::autofix::Violation;
use crate::config::{ForbiddenRule, LinterContext};
use miette::IntoDiagnostic;
use std::path::Path;
use tree_sitter::{Query, QueryCursor, Tree};

/// Represents an import statement extracted from source code
#[derive(Debug, Clone, PartialEq)]
pub struct Import {
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
pub fn extract_imports_from_tree(tree: &Tree, source_code: &str) -> miette::Result<Vec<Import>> {
    let mut imports = Vec::new();

    // Query for import declarations
    let query_source = r#"
        (import_statement
          source: (string (string_fragment) @import_path))
    "#;

    let query = Query::new(&tree_sitter_typescript::language_typescript(), query_source)
        .into_diagnostic()?;

    let mut cursor = QueryCursor::new();
    let matches = cursor.matches(&query, tree.root_node(), source_code.as_bytes());

    for match_ in matches {
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

            imports.push(Import {
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
/// use architect_linter_pro::parsers::typescript_pure::normalize_path;
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
/// use architect_linter_pro::parsers::typescript_pure::normalize_pattern;
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
/// use architect_linter_pro::parsers::typescript_pure::extract_folder_from_pattern;
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
/// use architect_linter_pro::parsers::typescript_pure::generate_import_patterns;
///
/// let patterns = generate_import_patterns("services/");
/// assert!(patterns.contains(&"/services/".to_string()));
/// assert!(patterns.contains(&"../services/".to_string()));
/// assert!(patterns.contains(&"@/services/".to_string()));
/// ```
pub fn generate_import_patterns(folder: &str) -> Vec<String> {
    vec![
        format!("/{}", folder),           // Absolute path pattern
        format!("../{}", folder),         // Relative path pattern
        format!("@/{}", folder),          // Alias pattern
        folder.to_string(),               // Direct pattern
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
/// use architect_linter_pro::parsers::typescript_pure::matches_pattern;
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
/// use architect_linter_pro::parsers::typescript_pure::matches_forbidden_rule;
/// use architect_linter_pro::config::ForbiddenRule;
///
/// let rule = ForbiddenRule {
///     from: "src/controller/".to_string(),
///     to: "src/repository/".to_string(),
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
pub fn matches_forbidden_rule(
    file_path: &str,
    import_source: &str,
    rule: &ForbiddenRule,
) -> bool {
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
/// use architect_linter_pro::parsers::typescript_pure::is_controller_to_repository_violation;
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
    let is_repository = normalized_import.contains("repository")
        || normalized_import.contains(".repository");

    is_controller && is_repository
}

/// Check all forbidden rules for a single import
///
/// # Arguments
/// * `file_path` - The path of the file containing the import
/// * `import` - The import to check
/// * `forbidden_rules` - List of forbidden rules to check against
///
/// # Returns
/// A vector of matching rule indices


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
    import: &Import,
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
    imports: &[Import],
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
                },
            ));
        }
    }

    violations
}

/// Filter imports to find those that match a pattern
///
/// # Arguments
/// * `imports` - List of imports to filter
/// * `pattern` - Pattern to match against
///
/// # Returns
/// Imports whose source matches the pattern


/// Check if any import matches a pattern
///
/// # Arguments
/// * `imports` - List of imports to check
/// * `pattern` - Pattern to match against
///
/// # Returns
/// true if any import matches the pattern


/// Count imports matching a pattern
///
/// # Arguments
/// * `imports` - List of imports to count
/// * `pattern` - Pattern to match against
///
/// # Returns
/// Number of imports matching the pattern


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
        assert_eq!(normalize_path("SRC/Components/Button"), "src/components/button");
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
        assert!(matches_pattern("src/components/button.tsx", "src/components/"));
        assert!(matches_pattern("src/components/nested/Button.tsx", "src/components/"));
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
            Import {
                source: "../repository/user".to_string(),
                line_number: 1,
                raw_statement: "import { User } from '../repository/user';".to_string(),
            },
            Import {
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
            }],
            ignored_paths: vec![],
            ai_configs: vec![],
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
        let imports = vec![Import {
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
            }],
            ignored_paths: vec![],
            ai_configs: vec![],
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
