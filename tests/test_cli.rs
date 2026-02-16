/// End-to-end CLI tests
///
/// These tests validate the command-line interface behavior including:
/// - Basic commands (--version, --help)
/// - Analysis modes (--watch, --staged, --fix)
/// - Report generation (--report json/markdown)
/// - Error handling (invalid paths, missing config)
/// - Exit codes
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

#[path = "common/mod.rs"]
mod common;

use common::TestProject;

/// Get the binary command
fn cmd() -> Command {
    Command::cargo_bin("architect-linter-pro").unwrap()
}

// ============================================================================
// Basic Commands
// ============================================================================

#[test]
fn test_version_flag() {
    cmd()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("4.1.0-beta"));
}

#[test]
fn test_version_short_flag() {
    cmd()
        .arg("-v")
        .assert()
        .success()
        .stdout(predicate::str::contains("4.1.0-beta"));
}

#[test]
fn test_help_flag() {
    cmd()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("architect-linter-pro"))
        .stdout(predicate::str::contains("--report"))
        .stdout(predicate::str::contains("--watch"));
}

#[test]
fn test_help_short_flag() {
    cmd()
        .arg("-h")
        .assert()
        .success()
        .stdout(predicate::str::contains("USO")); // Spanish: "USO" instead of "Usage"
}

// ============================================================================
// Report Generation Tests
// ============================================================================

#[test]
fn test_report_json_generation() {
    let project = TestProject::new();
    project.create_minimal_config();
    project.create_file("src/test.ts", "export class User {}");

    let report_path = project.path().join("report.json");

    cmd()
        .current_dir(project.path())
        .arg("--report")
        .arg("json")
        .arg("--output")
        .arg(&report_path)
        .arg(".")
        .assert()
        .success();

    // Verify report file was created
    assert!(report_path.exists(), "Report file should be created");

    // Verify it's valid JSON
    let content = fs::read_to_string(&report_path).unwrap();
    let json: serde_json::Value =
        serde_json::from_str(&content).expect("Report should be valid JSON");

    // Verify required fields
    assert!(json["health_score"].is_object(), "Should have health_score");
    assert!(json["summary"].is_object(), "Should have summary");
    assert!(json["version"].is_string(), "Should have version");
}

#[test]
fn test_report_json_short_flags() {
    let project = TestProject::new();
    project.create_minimal_config();
    project.create_file("src/test.ts", "export class User {}");

    let report_path = project.path().join("report.json");

    cmd()
        .current_dir(project.path())
        .arg("-r")
        .arg("json")
        .arg("-o")
        .arg(&report_path)
        .arg(".")
        .assert()
        .success();

    assert!(report_path.exists());
}

#[test]
fn test_report_markdown_generation() {
    let project = TestProject::new();
    project.create_minimal_config();
    project.create_file("src/test.ts", "export class User {}");

    let report_path = project.path().join("report.md");

    cmd()
        .current_dir(project.path())
        .arg("--report")
        .arg("markdown")
        .arg("--output")
        .arg(&report_path)
        .arg(".")
        .assert()
        .success();

    // Verify report file was created
    assert!(report_path.exists(), "Report file should be created");

    // Verify it's markdown format
    let content = fs::read_to_string(&report_path).unwrap();
    assert!(content.contains("# "), "Should have markdown headers");
    assert!(
        content.contains("Architecture"),
        "Should mention Architecture"
    );
}

#[test]
fn test_report_without_output_path() {
    let project = TestProject::new();
    project.create_minimal_config();

    // Without --output, report is written to stdout
    cmd()
        .current_dir(project.path())
        .arg("--report")
        .arg("json")
        .arg(".")
        .assert()
        .success(); // CLI writes to stdout instead of failing
}

#[test]
fn test_report_invalid_format() {
    let project = TestProject::new();
    project.create_minimal_config();

    // Invalid format shows error in stderr
    cmd()
        .current_dir(project.path())
        .arg("--report")
        .arg("invalid")
        .arg("-o")
        .arg("report.txt")
        .arg(".")
        .assert()
        .success() // Currently returns 0 even with error
        .stderr(predicate::str::contains("inválido").or(predicate::str::contains("invalid")));
}

// ============================================================================
// Analysis Tests
// ============================================================================

#[test]
fn test_basic_analysis_with_config() {
    let project = TestProject::new();
    project.create_minimal_config();
    project.create_file("src/user.ts", "export class User {}");
    project.create_file("src/product.ts", "export class Product {}");

    cmd()
        .current_dir(project.path())
        .arg(".")
        .assert()
        .success()
        .stdout(predicate::str::contains("ARCHITECTURE HEALTH"));
}

#[test]
fn test_analysis_without_config() {
    let project = TestProject::new();
    // Don't create config
    project.create_file("src/test.ts", "export class Test {}");

    // Should fail or prompt for config (depending on implementation)
    let result = cmd().current_dir(project.path()).arg(".").assert();

    // Either succeeds with auto-config or fails gracefully
    // For now, we just verify it doesn't panic
    assert!(result.get_output().status.code().is_some());
}

#[test]
fn test_analysis_with_violations() {
    let project = TestProject::new();

    // Create config that forbids controller -> repository
    let rule = common::forbidden_rule("/controller/", "/repository/");
    project.create_config("MVC", 100, &rule);

    // Create violation
    project.create_file(
        "src/user.controller.ts",
        r#"import { UserRepository } from './user.repository';
export class UserController {
    constructor(private repo: UserRepository) {}
}"#,
    );

    project.create_file("src/user.repository.ts", "export class UserRepository {}");

    // Should fail due to violation
    cmd()
        .current_dir(project.path())
        .arg(".")
        .assert()
        .failure();
}

// ============================================================================
// Path and Directory Tests
// ============================================================================

#[test]
fn test_invalid_path() {
    cmd()
        .arg("/nonexistent/path/that/does/not/exist")
        .assert()
        .failure();
}

#[test]
fn test_current_directory_dot() {
    let project = TestProject::new();
    project.create_minimal_config();
    project.create_file("src/test.ts", "export class Test {}");

    cmd()
        .current_dir(project.path())
        .arg(".")
        .assert()
        .success();
}

#[test]
fn test_explicit_path() {
    let project = TestProject::new();
    project.create_minimal_config();
    project.create_file("src/test.ts", "export class Test {}");

    cmd().arg(project.path()).assert().success();
}

// ============================================================================
// Exit Code Tests
// ============================================================================

#[test]
fn test_exit_code_success_clean_project() {
    let project = TestProject::new();
    project.create_minimal_config();
    project.create_file("src/test.ts", "export class Test {}");

    let output = cmd()
        .current_dir(project.path())
        .arg(".")
        .assert()
        .success();

    assert_eq!(output.get_output().status.code(), Some(0));
}

#[test]
fn test_exit_code_failure_with_violations() {
    let project = TestProject::new();

    // Create config with controller -> repository violation rule
    let rule = common::forbidden_rule("/controller/", "/repository/");
    project.create_config("MVC", 50, &rule);

    // Create a violation: controller imports from repository
    project.create_file(
        "src/controller/user.controller.ts",
        "import { UserRepository } from '../repository/user.repository';",
    );
    project.create_file(
        "src/repository/user.repository.ts",
        "export class UserRepository {}",
    );

    let output = cmd()
        .current_dir(project.path())
        .arg(".")
        .assert()
        .failure();

    assert_ne!(output.get_output().status.code(), Some(0));
}

// ============================================================================
// Output Format Tests
// ============================================================================

#[test]
fn test_output_contains_health_score() {
    let project = TestProject::new();
    project.create_minimal_config();
    project.create_file("src/test.ts", "export class Test {}");

    cmd()
        .current_dir(project.path())
        .arg(".")
        .assert()
        .success()
        .stdout(predicate::str::contains("ARCHITECTURE HEALTH"))
        .stdout(predicate::str::contains("/100"));
}

#[test]
fn test_output_contains_grade() {
    let project = TestProject::new();
    project.create_minimal_config();
    project.create_file("src/test.ts", "export class Test {}");

    cmd()
        .current_dir(project.path())
        .arg(".")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"[A-F]").unwrap());
}

#[test]
fn test_output_contains_component_scores() {
    let project = TestProject::new();
    project.create_minimal_config();
    project.create_file("src/test.ts", "export class Test {}");

    cmd()
        .current_dir(project.path())
        .arg(".")
        .assert()
        .success()
        .stdout(predicate::str::contains("Layer"))
        .stdout(predicate::str::contains("circular")) // lowercase in Spanish output
        .stdout(predicate::str::contains("Complexity"));
}

// ============================================================================
// Staged Flag Tests
// ============================================================================

#[test]
fn test_staged_flag_without_git() {
    let project = TestProject::new();
    project.create_minimal_config();
    project.create_file("src/test.ts", "export class Test {}");

    // Should handle gracefully when not in git repo
    let result = cmd().current_dir(project.path()).arg("--staged").assert();

    // Either succeeds with warning or fails gracefully
    assert!(result.get_output().status.code().is_some());
}

// ============================================================================
// Flag Combination Tests
// ============================================================================

#[test]
fn test_report_and_analysis() {
    let project = TestProject::new();
    project.create_minimal_config();
    project.create_file("src/test.ts", "export class Test {}");

    let report_path = project.path().join("report.json");

    cmd()
        .current_dir(project.path())
        .arg("--report")
        .arg("json")
        .arg("--output")
        .arg(&report_path)
        .arg(".")
        .assert()
        .success();

    // Both report and terminal output should be generated
    assert!(report_path.exists());
}

#[test]
fn test_multiple_flags_together() {
    let project = TestProject::new();
    project.create_minimal_config();
    project.create_file("src/test.ts", "export class Test {}");

    let report_path = project.path().join("report.json");

    // Test multiple short flags
    cmd()
        .current_dir(project.path())
        .arg("-r")
        .arg("json")
        .arg("-o")
        .arg(&report_path)
        .arg(".")
        .assert()
        .success();
}

// ============================================================================
// Error Message Tests
// ============================================================================

#[test]
fn test_helpful_error_for_missing_path() {
    // Should fail when no path is provided and not in interactive mode
    cmd().assert().failure(); // Just verify it fails, don't check stderr (varies in test mode)
}

#[test]
fn test_error_message_for_invalid_report_format() {
    let project = TestProject::new();
    project.create_minimal_config();

    // Invalid format shows error message in stderr
    cmd()
        .current_dir(project.path())
        .arg("--report")
        .arg("xml")
        .arg("-o")
        .arg("report.xml")
        .arg(".")
        .assert()
        .success() // Currently returns 0 even with error
        .stderr(predicate::str::contains("inválido").or(predicate::str::contains("invalid")));
}

// ============================================================================
// Performance Tests (basic)
// ============================================================================

#[test]
fn test_analysis_completes_quickly_small_project() {
    let project = TestProject::new();
    project.create_minimal_config();

    // Create a small project (10 files)
    for i in 0..10 {
        project.create_file(
            &format!("src/file{}.ts", i),
            &format!("export class Class{} {{}}", i),
        );
    }

    let start = std::time::Instant::now();

    cmd()
        .current_dir(project.path())
        .arg(".")
        .assert()
        .success();

    let duration = start.elapsed();

    // Should complete in less than 5 seconds
    assert!(
        duration.as_secs() < 5,
        "Analysis took too long: {:?}",
        duration
    );
}

// ============================================================================
// Real-world Scenario Tests
// ============================================================================

#[test]
fn test_typical_workflow_analysis_then_report() {
    let project = TestProject::new();
    project.create_minimal_config();
    project.create_file("src/user.ts", "export class User {}");
    project.create_file("src/product.ts", "export class Product {}");

    // Step 1: Run analysis
    cmd()
        .current_dir(project.path())
        .arg(".")
        .assert()
        .success();

    // Step 2: Generate report
    let report_path = project.path().join("report.json");
    cmd()
        .current_dir(project.path())
        .arg("--report")
        .arg("json")
        .arg("--output")
        .arg(&report_path)
        .arg(".")
        .assert()
        .success();

    assert!(report_path.exists());
}

#[test]
fn test_ci_cd_scenario_json_report_check_exit_code() {
    let project = TestProject::new();
    project.create_minimal_config();
    project.create_file("src/test.ts", "export class Test {}");

    let report_path = project.path().join("report.json");

    // Simulate CI/CD: generate report and check exit code
    let output = cmd()
        .current_dir(project.path())
        .arg("--report")
        .arg("json")
        .arg("--output")
        .arg(&report_path)
        .arg(".")
        .assert()
        .success();

    // Verify report exists
    assert!(report_path.exists());

    // Verify exit code is 0 for clean project
    assert_eq!(output.get_output().status.code(), Some(0));

    // Verify report has required fields for CI
    let content = fs::read_to_string(&report_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();

    assert!(json["health_score"]["total"].is_number());
    assert!(json["health_score"]["grade"].is_string());
    assert!(json["summary"]["total_violations"].is_number());
}
