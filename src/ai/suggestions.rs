//! Smart violation suggestions module
//! Provides intelligent, context-aware suggestions for fixing architecture violations

/// Generates intelligent suggestions for architecture violations
pub struct SmartSuggestions;

impl SmartSuggestions {
    /// Generate suggestion for common violations based on violation type and context
    ///
    /// # Arguments
    /// * `violation_type` - The type of violation detected
    /// * `context` - Contextual information about the violation (e.g., file names, patterns)
    ///
    /// # Returns
    /// A string containing the suggested fix
    pub fn suggest_fix(violation_type: &str, context: &str) -> String {
        match violation_type {
            "import_violation" => {
                if context.contains("controller") && context.contains("model") {
                    "Move model logic to service layer, import service from controller instead"
                        .to_string()
                } else if context.contains("utils") {
                    "Check if this import violates layer boundaries in architect.json"
                        .to_string()
                } else {
                    "Check layer dependencies in architect.json".to_string()
                }
            }
            "circular_dependency" => {
                "Break circular dependency by extracting shared logic to new module".to_string()
            }
            "complex_method" => {
                "Consider breaking method into smaller, focused methods".to_string()
            }
            "too_many_parameters" => {
                "Consider using a parameter object or builder pattern to reduce parameters"
                    .to_string()
            }
            "deep_nesting" => {
                "Reduce nesting depth by extracting nested logic into separate functions"
                    .to_string()
            }
            _ => "Review architecture pattern for this layer".to_string(),
        }
    }

    /// Suggest refactoring approaches based on file characteristics
    ///
    /// # Arguments
    /// * `file_path` - Path to the file being analyzed
    /// * `violations_count` - Number of violations found in the file
    ///
    /// # Returns
    /// A vector of refactoring suggestions
    pub fn suggest_refactoring(file_path: &str, violations_count: usize) -> Vec<String> {
        let mut suggestions = Vec::new();

        if violations_count > 10 {
            suggestions.push(format!(
                "File '{}' has {} violations. Consider breaking into multiple files",
                file_path, violations_count
            ));
        } else if violations_count > 5 {
            suggestions.push(format!(
                "File '{}' has {} violations. Review and refactor to reduce complexity",
                file_path, violations_count
            ));
        }

        if file_path.contains("utils") || file_path.contains("helpers") {
            suggestions.push(
                "Consider categorizing utilities into specific modules (validation, formatting, etc)"
                    .to_string(),
            );
        }

        if file_path.contains("service") {
            suggestions.push(
                "Service files should be focused. If violating multiple rules, consider splitting by domain"
                    .to_string(),
            );
        }

        if file_path.contains("controller") || file_path.contains("handler") {
            suggestions.push(
                "Controllers/handlers should be thin. Move business logic to services"
                    .to_string(),
            );
        }

        suggestions
    }

    /// Provides context-aware fix guidance for specific violation patterns
    ///
    /// # Arguments
    /// * `from_layer` - The source layer of the import
    /// * `to_layer` - The target layer being imported from
    ///
    /// # Returns
    /// A string with specific guidance for this violation pattern
    pub fn suggest_layer_fix(from_layer: &str, to_layer: &str) -> String {
        match (from_layer.to_lowercase().as_str(), to_layer.to_lowercase().as_str()) {
            (from, to) if from.contains("controller") && to.contains("model") => {
                format!(
                    "Controllers should not directly import models. Use services instead: {} -> service -> {}",
                    from, to
                )
            }
            (from, to) if from.contains("model") && to.contains("controller") => {
                format!(
                    "Models should not import controllers. Extract shared logic to a service or utility: {} should not depend on {}",
                    from, to
                )
            }
            (from, to) if from.contains("view") && to.contains("service") => {
                "Views should communicate with controllers, not services directly".to_string()
            }
            (from, to) => {
                format!(
                    "Layer dependency violation: {} -> {}. Check architect.json for allowed dependencies",
                    from, to
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suggest_fix_import_violation_with_controller_model() {
        let suggestion = SmartSuggestions::suggest_fix(
            "import_violation",
            "controller imports model",
        );
        assert!(suggestion.contains("service layer"));
    }

    #[test]
    fn test_suggest_fix_import_violation_with_utils() {
        let suggestion = SmartSuggestions::suggest_fix(
            "import_violation",
            "utils helper function",
        );
        assert!(suggestion.contains("layer boundaries"));
    }

    #[test]
    fn test_suggest_fix_import_violation_generic() {
        let suggestion = SmartSuggestions::suggest_fix(
            "import_violation",
            "some other context",
        );
        assert!(suggestion.contains("architect.json"));
    }

    #[test]
    fn test_suggest_fix_circular_dependency() {
        let suggestion = SmartSuggestions::suggest_fix(
            "circular_dependency",
            "context",
        );
        assert!(suggestion.contains("extracting shared logic"));
    }

    #[test]
    fn test_suggest_fix_complex_method() {
        let suggestion = SmartSuggestions::suggest_fix(
            "complex_method",
            "context",
        );
        assert!(suggestion.contains("smaller, focused methods"));
    }

    #[test]
    fn test_suggest_fix_too_many_parameters() {
        let suggestion = SmartSuggestions::suggest_fix(
            "too_many_parameters",
            "context",
        );
        assert!(suggestion.contains("parameter object") || suggestion.contains("builder"));
    }

    #[test]
    fn test_suggest_fix_deep_nesting() {
        let suggestion = SmartSuggestions::suggest_fix(
            "deep_nesting",
            "context",
        );
        assert!(suggestion.contains("nesting depth"));
    }

    #[test]
    fn test_suggest_fix_unknown_violation() {
        let suggestion = SmartSuggestions::suggest_fix(
            "unknown_violation_type",
            "context",
        );
        assert!(suggestion.contains("Review architecture pattern"));
    }

    #[test]
    fn test_suggest_refactoring_many_violations() {
        let suggestions = SmartSuggestions::suggest_refactoring(
            "src/auth/auth_service.rs",
            15,
        );
        assert!(!suggestions.is_empty());
        assert!(suggestions.iter().any(|s| s.contains("15 violations")));
    }

    #[test]
    fn test_suggest_refactoring_moderate_violations() {
        let suggestions = SmartSuggestions::suggest_refactoring(
            "src/auth/auth_service.rs",
            7,
        );
        assert!(!suggestions.is_empty());
        assert!(suggestions.iter().any(|s| s.contains("refactor")));
    }

    #[test]
    fn test_suggest_refactoring_utils_file() {
        let suggestions = SmartSuggestions::suggest_refactoring(
            "src/utils/helpers.ts",
            3,
        );
        assert!(suggestions.iter().any(|s| s.contains("categorizing")));
    }

    #[test]
    fn test_suggest_refactoring_service_file() {
        let suggestions = SmartSuggestions::suggest_refactoring(
            "src/services/user_service.ts",
            8,
        );
        assert!(suggestions.iter().any(|s| s.contains("Service files")));
    }

    #[test]
    fn test_suggest_refactoring_controller_file() {
        let suggestions = SmartSuggestions::suggest_refactoring(
            "src/controllers/user_controller.ts",
            5,
        );
        assert!(suggestions.iter().any(|s| s.contains("business logic")));
    }

    #[test]
    fn test_suggest_refactoring_handler_file() {
        let suggestions = SmartSuggestions::suggest_refactoring(
            "src/handlers/request_handler.ts",
            4,
        );
        assert!(suggestions.iter().any(|s| s.contains("business logic")));
    }

    #[test]
    fn test_suggest_layer_fix_controller_to_model() {
        let suggestion = SmartSuggestions::suggest_layer_fix(
            "controller",
            "model",
        );
        assert!(suggestion.contains("service"));
    }

    #[test]
    fn test_suggest_layer_fix_model_to_controller() {
        let suggestion = SmartSuggestions::suggest_layer_fix(
            "model",
            "controller",
        );
        assert!(suggestion.contains("Extract shared logic"));
    }

    #[test]
    fn test_suggest_layer_fix_view_to_service() {
        let suggestion = SmartSuggestions::suggest_layer_fix(
            "view",
            "service",
        );
        assert!(suggestion.contains("controllers"));
    }

    #[test]
    fn test_suggest_layer_fix_generic() {
        let suggestion = SmartSuggestions::suggest_layer_fix(
            "domain",
            "application",
        );
        assert!(suggestion.contains("architect.json"));
    }
}
