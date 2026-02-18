//! Integration tests for v4.3.0 features: Build Integration, Schema Validation, and Logging.

mod common;
use common::TestProject;
use std::fs;

#[test]
fn test_config_schema_validation_success() {
    let project = TestProject::new();
    let config = r#"{
        "$schema": "./schemas/architect.schema.json",
        "max_lines_per_function": 50,
        "architecture_pattern": "MVC",
        "forbidden_imports": [],
        "build_command": "npm run build",
        "ai_fix_retries": 5
    }"#;
    project.create_file("architect.json", config);

    let result = architect_linter_pro::config::load_config(project.path());
    assert!(
        result.is_ok(),
        "Config with build_command and ai_fix_retries should be valid"
    );

    let ctx = result.unwrap();
    assert_eq!(ctx.build_command, Some("npm run build".to_string()));
    assert_eq!(ctx.ai_fix_retries, 5);
}

#[test]
fn test_config_schema_validation_failure_invalid_field() {
    let project = TestProject::new();
    let config = r#"{
        "max_lines_per_function": 50,
        "architecture_pattern": "MVC",
        "forbidden_imports": [],
        "invalid_field": "should fail"
    }"#;
    project.create_file("architect.json", config);

    let result = architect_linter_pro::config::load_config(project.path());
    assert!(
        result.is_err(),
        "Config with invalid_field should fail schema validation"
    );
}

#[test]
fn test_config_schema_validation_failure_wrong_type() {
    let project = TestProject::new();
    let config = r#"{
        "max_lines_per_function": 50,
        "architecture_pattern": "MVC",
        "forbidden_imports": [],
        "ai_fix_retries": "five"
    }"#;
    project.create_file("architect.json", config);

    let result = architect_linter_pro::config::load_config(project.path());
    assert!(
        result.is_err(),
        "Config with wrong type for ai_fix_retries should fail"
    );
}

#[test]
fn test_circular_dependency_skips_python() {
    let project = TestProject::new();
    // Create a python file that would cause a parse error if treated as JS/TS
    let py_content = "def hello():\n    print('world')\n";
    project.create_file("script.py", py_content);

    let files = vec![project.path().join("script.py")];
    let cm = std::sync::Arc::new(swc_common::SourceMap::default());

    // This should not panic or return error
    let result =
        architect_linter_pro::circular::analyze_circular_dependencies(&files, project.path(), &cm);
    assert!(result.is_ok(), "Should skip non-JS/TS files without error");
    let cycles = result.unwrap();
    assert!(cycles.is_empty());
}

#[test]
fn test_build_command_execution() {
    let project = TestProject::new();

    // Create a simple success command
    let cmd = if cfg!(target_os = "windows") {
        "echo success"
    } else {
        "true"
    };

    let result = architect_linter_pro::autofix::run_build_command(cmd, project.path());
    assert!(result.is_ok(), "Build command should succeed");

    // Create a failure command
    let cmd_fail = if cfg!(target_os = "windows") {
        "exit 1"
    } else {
        "false"
    };
    let result_fail = architect_linter_pro::autofix::run_build_command(cmd_fail, project.path());
    assert!(result_fail.is_err(), "Build command should fail");
}

#[test]
fn test_logging_initialization() {
    // This just tests that init doesn't panic
    architect_linter_pro::logging::init(true);
    architect_linter_pro::logging::init(false);
}

#[test]
fn test_rollback_on_syntax_error() {
    use architect_linter_pro::autofix::{apply_fix, FixSuggestion, FixType, Violation};
    use architect_linter_pro::config::ForbiddenRule;

    let project = TestProject::new();
    let file_path = project.create_file("test.ts", "import { a } from 'b';");
    let original_content = "import { a } from 'b';";

    let violation = Violation {
        file_path: file_path.clone(),
        file_content: original_content.to_string(),
        offensive_import: "import { a } from 'b';".to_string(),
        rule: ForbiddenRule {
            from: "test.ts".into(),
            to: "b".into(),
        },
        line_number: 1,
    };

    let suggestion = FixSuggestion {
        fix_type: FixType::Refactor {
            old_code: "import { a } from 'b';".to_string(),
            new_code: "import { a } from 'b' !!! invalid syntax !!!".to_string(),
        },
        explanation: "Breaking it on purpose".to_string(),
        confidence: "high".to_string(),
    };

    // applying this should fail syntax validation and revert
    let result = apply_fix(&suggestion, &violation, project.path());
    assert!(result.is_err(), "Applying invalid syntax should fail");

    let current_content = std::fs::read_to_string(file_path).unwrap();
    assert_eq!(
        current_content, original_content,
        "Content should be reverted to original"
    );
}
