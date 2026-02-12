/// Tests for common test utilities

#[path = "common/mod.rs"]
mod common;

use common::*;

#[test]
fn test_project_creation() {
    let project = TestProject::new();
    assert!(project.path().exists());
}

#[test]
fn test_file_creation() {
    let project = TestProject::new();
    let file = project.create_file("test.txt", "hello world");
    assert!(file.exists());

    let content = std::fs::read_to_string(&file).unwrap();
    assert_eq!(content, "hello world");
}

#[test]
fn test_nested_file_creation() {
    let project = TestProject::new();
    let file = project.create_file("src/domain/user.ts", "export class User {}");
    assert!(file.exists());
    assert!(file.parent().unwrap().exists());
}

#[test]
fn test_minimal_config_creation() {
    let project = TestProject::new();
    let config = project.create_minimal_config();
    assert!(config.exists());

    let content = std::fs::read_to_string(&config).unwrap();
    assert!(content.contains("architecture_pattern"));
    assert!(content.contains("max_lines_per_function"));
    assert!(content.contains("forbidden_imports"));
}

#[test]
fn test_custom_config_creation() {
    let project = TestProject::new();

    let rule = forbidden_rule("/domain/", "/infrastructure/");
    let config = project.create_config("Hexagonal", 50, &rule);

    assert!(config.exists());
    let content = std::fs::read_to_string(&config).unwrap();
    assert!(content.contains("Hexagonal"));
    assert!(content.contains("50"));
    assert!(content.contains("/domain/"));
}

#[test]
fn test_multiple_rules() {
    let rules = vec![
        forbidden_rule("/controllers/", "/repositories/"),
        forbidden_rule("/domain/", "/infrastructure/"),
    ];
    let rules_str = join_rules(&rules);

    assert!(rules_str.contains("/controllers/"));
    assert!(rules_str.contains("/repositories/"));
    assert!(rules_str.contains("/domain/"));
    assert!(rules_str.contains(","));
}
