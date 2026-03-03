//! Smart configuration generator that produces 80-90% complete architect.json files.
//!
//! The ConfigGenerator analyzes a project and automatically detects:
//! - Frameworks used (NestJS, Express, Django, Laravel, etc.)
//! - Architecture patterns (Clean, Hexagonal, MVC)
//! - Recommended forbidden import rules
//!
//! This generates a configuration that requires minimal manual adjustment.

use super::types::{ArchPattern, Framework, Severity};
use super::ForbiddenRule;
use super::ConfigFile;
use std::path::Path;

/// Smart configuration generator
///
/// Automatically detects frameworks and architecture patterns to generate
/// an architect.json with 80-90% of the rules already configured.
pub struct ConfigGenerator;

impl ConfigGenerator {
    /// Create a new ConfigGenerator instance
    pub fn new() -> Self {
        Self
    }

    /// Generate a configuration file from a project directory
    ///
    /// # Arguments
    /// - `project_path`: Path to the project root directory
    ///
    /// # Returns
    /// A ConfigFile with detected frameworks and generated rules
    pub fn generate(&self, project_path: &Path) -> Result<ConfigFile, String> {
        // Detect the primary framework
        let primary_framework = crate::detector::detect_framework(project_path);
        if primary_framework == Framework::Unknown {
            return Err("Could not detect any frameworks in the project.".to_string());
        }

        // Detect pattern
        let pattern = self.detect_pattern(project_path).unwrap_or(ArchPattern::MVC);

        // Create a single framework vec for consistency
        let frameworks = vec![primary_framework.clone()];

        // Generate rules based on detected frameworks and pattern
        let forbidden_imports = self.generate_rules_for_frameworks(&frameworks, &pattern);

        // Get suggestions based on the detected framework
        let max_lines = crate::detector::get_loc_suggestion(&primary_framework);

        // Get build command suggestion
        let build_command = crate::detector::get_build_command_suggestion(&primary_framework);

        Ok(ConfigFile {
            max_lines_per_function: max_lines,
            architecture_pattern: pattern,
            forbidden_imports,
            ignored_paths: crate::config::default_ignored_paths(),
            build_command,
            ai_fix_retries: 3,
        })
    }

    /// Detect the architecture pattern from the project structure
    ///
    /// Looks for common directories indicating Clean or Hexagonal patterns.
    /// Defaults to MVC if no clear pattern is found.
    fn detect_pattern(&self, project_path: &Path) -> Option<ArchPattern> {
        // Check for Hexagonal pattern markers
        if project_path.join("domain").exists() && project_path.join("application").exists() {
            return Some(ArchPattern::Hexagonal);
        }

        // Check for Clean architecture pattern markers
        if project_path.join("entities").exists() && project_path.join("use-cases").exists() {
            return Some(ArchPattern::Clean);
        }

        // Check for MVC-style patterns
        if project_path.join("controllers").exists() || project_path.join("models").exists() {
            return Some(ArchPattern::MVC);
        }

        // Default to MVC if no clear pattern detected
        Some(ArchPattern::MVC)
    }

    /// Generate framework-specific and pattern-specific rules
    fn generate_rules_for_frameworks(
        &self,
        frameworks: &[Framework],
        pattern: &ArchPattern,
    ) -> Vec<ForbiddenRule> {
        let mut rules = Vec::new();

        // Get rules from templates if available
        for framework in frameworks {
            // Determine the pattern string to use for template lookup
            let pattern_str = match pattern {
                ArchPattern::Hexagonal => "hexagonal",
                ArchPattern::Clean => "clean",
                ArchPattern::MVC => "mvc",
                ArchPattern::Ninguno => "mvc", // Default to MVC for unknown
                ArchPattern::Custom(s) => s.as_str(),
            };

            // Try to get a template for this framework + pattern combination
            use crate::init::templates::get_template;
            if let Some(config) = get_template(framework, pattern_str) {
                rules.extend(config.forbidden_imports);
            } else {
                // Fallback: generate basic rules if no template exists
                let fallback_rules = self.generate_fallback_rules(framework, pattern);
                rules.extend(fallback_rules);
            }
        }

        // Dedup rules by (from, to) pair
        rules.sort_by(|a, b| {
            let a_key = (&a.from, &a.to);
            let b_key = (&b.from, &b.to);
            a_key.cmp(&b_key)
        });
        rules.dedup_by(|a, b| a.from == b.from && a.to == b.to);

        rules
    }

    /// Generate basic fallback rules for a framework when no template is available
    fn generate_fallback_rules(&self, framework: &Framework, _pattern: &ArchPattern) -> Vec<ForbiddenRule> {
        match framework {
            // NestJS generic rules
            Framework::NestJS => vec![
                ForbiddenRule {
                    from: "/modules/".to_string(),
                    to: "/main.ts".to_string(),
                    severity: Some(Severity::Error),
                    reason: Some("Modules should not depend on the bootstrap file".to_string()),
                },
            ],

            // Express generic rules
            Framework::Express => vec![
                ForbiddenRule {
                    from: "/models/".to_string(),
                    to: "/routes/".to_string(),
                    severity: Some(Severity::Error),
                    reason: Some("Models should not depend on route definitions".to_string()),
                },
            ],

            // React generic rules
            Framework::React => vec![
                ForbiddenRule {
                    from: "/utils/".to_string(),
                    to: "/components/".to_string(),
                    severity: Some(Severity::Warning),
                    reason: Some("Utils should not depend on React components".to_string()),
                },
            ],

            // Django generic rules
            Framework::Django => vec![
                ForbiddenRule {
                    from: "/models.py".to_string(),
                    to: "/views.py".to_string(),
                    severity: Some(Severity::Warning),
                    reason: Some("Models should use relationships, not import views".to_string()),
                },
            ],

            // Laravel generic rules
            Framework::Laravel => vec![
                ForbiddenRule {
                    from: "/Models/".to_string(),
                    to: "/Http/".to_string(),
                    severity: Some(Severity::Warning),
                    reason: Some("Models should not depend on HTTP layer".to_string()),
                },
            ],

            // NextJS, Vue, Svelte, etc. - minimal rules
            _ => vec![],
        }
    }
}

impl Default for ConfigGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_config_generator_creates_valid_config() {
        let temp_dir = TempDir::new().unwrap();

        // Create a NestJS project marker
        let package_json = r#"{"name": "test", "dependencies": {"@nestjs/core": "^10.0.0"}}"#;
        fs::write(temp_dir.path().join("package.json"), package_json).unwrap();

        let generator = ConfigGenerator::new();
        let config = generator.generate(temp_dir.path());

        assert!(config.is_ok(), "Generator should create a valid config");
        let config = config.unwrap();

        // Verify config has required fields
        assert!(config.max_lines_per_function > 0, "max_lines should be > 0");
        assert!(
            !config.forbidden_imports.is_empty() || !config.forbidden_imports.is_empty(),
            "Config should have rules or frameworks"
        );
    }

    #[test]
    fn test_config_generator_respects_detected_frameworks() {
        let temp_dir = TempDir::new().unwrap();

        // Create a NestJS project marker
        let package_json = r#"{"name": "test", "dependencies": {"@nestjs/core": "^10.0.0"}}"#;
        fs::write(temp_dir.path().join("package.json"), package_json).unwrap();

        let generator = ConfigGenerator::new();
        let config = generator.generate(temp_dir.path()).unwrap();

        // NestJS max_lines should be 40
        assert_eq!(
            config.max_lines_per_function, 40,
            "NestJS should have max_lines of 40"
        );

        // Should have forbidden_imports generated from NestJS template
        assert!(
            !config.forbidden_imports.is_empty(),
            "NestJS config should have forbidden_imports"
        );
    }
}
