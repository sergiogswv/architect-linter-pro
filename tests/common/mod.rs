use std::fs;
/// Common test utilities and helpers
///
/// This module provides shared functionality for all test suites:
/// - Test fixture creation and management
/// - Temporary directory handling
/// - Config file generation
/// - Assertion helpers
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Helper to create a temporary test project
pub struct TestProject {
    pub root: TempDir,
}

impl TestProject {
    /// Create a new temporary test project
    pub fn new() -> Self {
        Self {
            root: TempDir::new().expect("Failed to create temp dir"),
        }
    }

    /// Get the root path of the test project
    pub fn path(&self) -> &Path {
        self.root.path()
    }

    /// Create a file in the test project with given content
    pub fn create_file(&self, rel_path: &str, content: &str) -> PathBuf {
        let file_path = self.root.path().join(rel_path);

        // Create parent directories if they don't exist
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create parent dirs");
        }

        fs::write(&file_path, content).expect("Failed to write file");
        file_path
    }

    /// Create architect.json config file
    pub fn create_config(&self, pattern: &str, max_lines: u32, rules: &str) -> PathBuf {
        let config = format!(
            r#"{{
  "max_lines_per_function": {},
  "architecture_pattern": "{}",
  "forbidden_imports": [
    {}
  ]
}}"#,
            max_lines, pattern, rules
        );
        self.create_file("architect.json", &config)
    }

    /// Create a minimal valid config
    pub fn create_minimal_config(&self) -> PathBuf {
        self.create_config("MVC", 100, "")
    }
}

/// Helper to create a forbidden import rule string
pub fn forbidden_rule(from: &str, to: &str) -> String {
    format!(r#"{{"from": "{}", "to": "{}"}}"#, from, to)
}

/// Helper to join multiple rules with commas
pub fn join_rules(rules: &[String]) -> String {
    rules.join(",\n    ")
}

/// Create a temporary TypeScript project with a single file
pub fn create_temp_ts_project(content: &str) -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.ts");
    std::fs::write(&file_path, content).unwrap();
    temp_dir
}

/// Create a temporary Python project with a single file
pub fn create_temp_py_project(content: &str) -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.py");
    std::fs::write(&file_path, content).unwrap();
    temp_dir
}

/// Create architect.json config in temp directory
pub fn create_architect_config(temp_dir: &Path, config: &str) {
    let config_path = temp_dir.join("architect.json");
    std::fs::write(&config_path, config).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_creation() {
        let project = TestProject::new();
        assert!(project.path().exists());
    }

    #[test]
    fn test_file_creation() {
        let project = TestProject::new();
        let file = project.create_file("test.txt", "hello");
        assert!(file.exists());
        assert_eq!(fs::read_to_string(&file).unwrap(), "hello");
    }

    #[test]
    fn test_nested_file_creation() {
        let project = TestProject::new();
        let file = project.create_file("src/domain/user.ts", "export class User {}");
        assert!(file.exists());
    }

    #[test]
    fn test_config_creation() {
        let project = TestProject::new();
        let config = project.create_minimal_config();
        assert!(config.exists());

        let content = fs::read_to_string(&config).unwrap();
        assert!(content.contains("architecture_pattern"));
        assert!(content.contains("max_lines_per_function"));
    }

    #[test]
    fn test_forbidden_rule_helper() {
        let rule = forbidden_rule("/domain/", "/infrastructure/");
        assert!(rule.contains("from"));
        assert!(rule.contains("to"));
        assert!(rule.contains("/domain/"));
    }
}
