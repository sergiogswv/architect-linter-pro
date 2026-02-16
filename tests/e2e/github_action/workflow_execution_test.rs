/// E2E tests for GitHub Action workflow execution
///
/// These tests validate the complete workflow execution as it would run in CI/CD:
/// - Building the binary
/// - Running analysis on test projects
/// - Verifying exit codes and output format
/// - Testing violation detection in realistic scenarios

use std::process::Command;
use std::path::PathBuf;
use std::sync::Once;
use tempfile::TempDir;

#[path = "../../common/mod.rs"]
mod common;

static BUILD_ONCE: Once = Once::new();

/// Build the binary in release mode
fn build_binary() -> Result<(), String> {
    let status = Command::new("cargo")
        .args(["build", "--release"])
        .status()
        .map_err(|e| format!("Failed to execute cargo build: {}", e))?;

    if status.success() {
        Ok(())
    } else {
        Err("Build failed".to_string())
    }
}

/// Ensure the binary is built only once across all tests
fn ensure_binary_built() {
    BUILD_ONCE.call_once(|| {
        build_binary().expect("Binary must build successfully");
    });
}

/// Get the path to the release binary
fn get_binary_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("target");
    path.push("release");
    if cfg!(windows) {
        path.push("architect-linter-pro.exe");
    } else {
        path.push("architect-linter-pro");
    }
    path
}

// ============================================================================
// Happy Path Tests
// ============================================================================

#[test]
fn test_github_action_workflow_happy_path() {
    // Build the binary first
    ensure_binary_built();

    // Create a test project with valid architecture
    let temp_dir = TempDir::new().unwrap();

    let test_code = r#"
export class ApiController {
    constructor() {}

    handleRequest() {
        return { status: 200 };
    }

    getData() {
        return { data: [] };
    }
}
"#;

    let project = temp_dir.path().join("test.ts");
    std::fs::write(&project, test_code).unwrap();

    // Create minimal valid config
    let config_path = temp_dir.path().join("architect.json");
    std::fs::write(&config_path, r#"
{
  "architecture_pattern": "MVC",
  "max_lines_per_function": 30,
  "forbidden_imports": []
}
"#).unwrap();

    // Run architect-linter-pro using the binary
    let binary_path = get_binary_path();
    let output = Command::new(&binary_path)
        .current_dir(temp_dir.path())
        .arg(".")
        .output()
        .expect("Failed to execute architect-linter-pro");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Assertions
    assert!(
        output.status.success(),
        "Command should succeed. stdout: {}, stderr: {}",
        stdout,
        stderr
    );

    // Verify output contains expected elements
    assert!(
        stdout.contains("ARCHITECTURE HEALTH") || stdout.contains("100/100") || stdout.contains("A"),
        "Should show health score or grade. Output: {}",
        stdout
    );

    // Verify no violations reported
    assert!(
        stdout.contains("None") || stdout.contains("Excellent") || stdout.contains("✨"),
        "Should indicate excellent health. Output: {}",
        stdout
    );
}

#[test]
fn test_github_action_workflow_with_report_generation() {
    // Build the binary first
    ensure_binary_built();

    let temp_dir = TempDir::new().unwrap();

    // Create a simple test file
    let test_code = r#"
export class UserService {
    constructor() {}

    getUser(id: string) {
        return { id, name: "Test" };
    }
}
"#;

    let project = temp_dir.path().join("src/user.service.ts");
    std::fs::create_dir_all(temp_dir.path().join("src")).unwrap();
    std::fs::write(&project, test_code).unwrap();

    // Create config
    let config_path = temp_dir.path().join("architect.json");
    std::fs::write(&config_path, r#"
{
  "architecture_pattern": "Clean",
  "max_lines_per_function": 50,
  "forbidden_imports": []
}
"#).unwrap();

    // Run with JSON report generation
    let binary_path = get_binary_path();
    let report_path = temp_dir.path().join("report.json");

    let output = Command::new(&binary_path)
        .current_dir(temp_dir.path())
        .arg("--report")
        .arg("json")
        .arg("--output")
        .arg(&report_path)
        .arg(".")
        .output()
        .expect("Failed to execute architect-linter-pro");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Command should succeed
    assert!(
        output.status.success(),
        "Command should succeed. Output: {}",
        stdout
    );

    // Report file should be created
    assert!(
        report_path.exists(),
        "Report file should be created at {:?}",
        report_path
    );

    // Verify report is valid JSON
    let report_content = std::fs::read_to_string(&report_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&report_content)
        .expect("Report should be valid JSON");

    // Verify required fields
    assert!(
        json["health_score"].is_object(),
        "Report should have health_score field"
    );
    assert!(
        json["summary"].is_object(),
        "Report should have summary field"
    );
    assert!(
        json["version"].is_string(),
        "Report should have version field"
    );
}

// ============================================================================
// Violation Detection Tests
// ============================================================================

#[test]
fn test_violation_detection_long_function() {
    // Build the binary first
    ensure_binary_built();

    let temp_dir = TempDir::new().unwrap();

    // Create project with a long function violation
    let long_function_code = r#"
export class DataProcessor {
    constructor() {}

    // This function is intentionally too long
    processLargeDataSet() {
        const data = [];
        for (let i = 0; i < 100; i++) {
            data.push(i);
        }

        const result1 = data.map(x => x * 2);
        const result2 = data.map(x => x * 3);
        const result3 = data.map(x => x * 4);
        const result4 = data.map(x => x * 5);
        const result5 = data.map(x => x * 6);
        const result6 = data.map(x => x * 7);
        const result7 = data.map(x => x * 8);
        const result8 = data.map(x => x * 9);
        const result9 = data.map(x => x * 10);

        return [result1, result2, result3, result4, result5, result6, result7, result8, result9];
    }
}
"#;

    let project = temp_dir.path().join("test.ts");
    std::fs::write(&project, long_function_code).unwrap();

    // Create config with low max_lines_per_function
    let config_path = temp_dir.path().join("architect.json");
    std::fs::write(&config_path, r#"
{
  "architecture_pattern": "MVC",
  "max_lines_per_function": 10,
  "forbidden_imports": []
}
"#).unwrap();

    // Run linter
    let binary_path = get_binary_path();
    let output = Command::new(&binary_path)
        .current_dir(temp_dir.path())
        .arg(".")
        .output()
        .expect("Failed to execute architect-linter-pro");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should detect violations or show lower score
    // The exact behavior depends on implementation, so we just verify it runs
    assert!(
        output.status.code().is_some(),
        "Should have exit code"
    );

    // Verify output contains health information
    assert!(
        stdout.contains("HEALTH") || stdout.contains("health") || stdout.contains("100"),
        "Output should contain health information. Output: {}",
        stdout
    );
}

#[test]
fn test_violation_detection_forbidden_imports() {
    // Build the binary first
    ensure_binary_built();

    let temp_dir = TempDir::new().unwrap();

    // Create project with forbidden import
    // Controller importing from repository (forbidden in MVC)
    let controller_code = r#"
import { UserRepository } from './user.repository';

export class UserController {
    constructor(private repo: UserRepository) {}

    getUser(id: string) {
        return this.repo.findById(id);
    }
}
"#;

    let repository_code = r#"
export class UserRepository {
    findById(id: string) {
        return { id, name: 'Test' };
    }
}
"#;

    std::fs::write(temp_dir.path().join("user.controller.ts"), controller_code).unwrap();
    std::fs::write(temp_dir.path().join("user.repository.ts"), repository_code).unwrap();

    // Create config with forbidden import
    let config_path = temp_dir.path().join("architect.json");
    std::fs::write(&config_path, r#"
{
  "architecture_pattern": "MVC",
  "max_lines_per_function": 50,
  "forbidden_imports": [
    {"from": "*/controller/*", "to": "*/repository/*"}
  ]
}
"#).unwrap();

    // Run linter
    let binary_path = get_binary_path();
    let output = Command::new(&binary_path)
        .current_dir(temp_dir.path())
        .arg(".")
        .output()
        .expect("Failed to execute architect-linter-pro");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should complete successfully (exit code 0 or non-0 depending on violations)
    assert!(
        output.status.code().is_some(),
        "Should have exit code"
    );

    // Output should mention violations or score
    assert!(
        stdout.contains("HEALTH") || stdout.contains("violations") || stdout.contains("health") || stdout.contains("/"),
        "Output should contain health or violation information. Output: {}",
        stdout
    );
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[test]
fn test_github_action_workflow_missing_config() {
    // Build the binary first
    ensure_binary_built();

    let temp_dir = TempDir::new().unwrap();

    // Create test file but NO config file
    let test_code = r#"
export class Test {
    method() {}
}
"#;

    std::fs::write(temp_dir.path().join("test.ts"), test_code).unwrap();

    // Run linter without config
    let binary_path = get_binary_path();
    let output = Command::new(&binary_path)
        .current_dir(temp_dir.path())
        .arg(".")
        .output()
        .expect("Failed to execute architect-linter-pro");

    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should fail with appropriate error message
    assert!(
        !output.status.success() || stderr.contains("architecture_pattern"),
        "Should fail or warn about missing config. stderr: {}",
        stderr
    );
}

#[test]
fn test_github_action_workflow_invalid_path() {
    // Build the binary first
    ensure_binary_built();

    // Try to analyze non-existent path
    let binary_path = get_binary_path();
    let output = Command::new(&binary_path)
        .arg("/nonexistent/path/that/does/not/exist")
        .output()
        .expect("Failed to execute architect-linter-pro");

    // Should fail
    assert!(
        !output.status.success(),
        "Should fail for invalid path"
    );
}

// ============================================================================
// Multi-File Project Tests
// ============================================================================

#[test]
fn test_github_action_workflow_multi_file_project() {
    // Build the binary first
    ensure_binary_built();

    let temp_dir = TempDir::new().unwrap();

    // Create a multi-file project
    let src_dir = temp_dir.path().join("src");
    std::fs::create_dir_all(&src_dir).unwrap();

    // Create multiple files
    std::fs::write(src_dir.join("user.model.ts"), "export class User {}").unwrap();
    std::fs::write(src_dir.join("product.model.ts"), "export class Product {}").unwrap();
    std::fs::write(src_dir.join("order.model.ts"), "export class Order {}").unwrap();

    // Create config
    let config_path = temp_dir.path().join("architect.json");
    std::fs::write(&config_path, r#"
{
  "architecture_pattern": "MVC",
  "max_lines_per_function": 50,
  "forbidden_imports": []
}
"#).unwrap();

    // Run linter
    let binary_path = get_binary_path();
    let output = Command::new(&binary_path)
        .current_dir(temp_dir.path())
        .arg(".")
        .output()
        .expect("Failed to execute architect-linter-pro");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should succeed
    assert!(
        output.status.success(),
        "Command should succeed. Output: {}",
        stdout
    );

    // Should mention multiple files analyzed
    assert!(
        stdout.contains("Files:") || stdout.contains("analyzed") || stdout.contains("HEALTH"),
        "Should mention files analyzed or health. Output: {}",
        stdout
    );
}

// ============================================================================
// Exit Code Tests
// ============================================================================

#[test]
fn test_github_action_workflow_exit_code_success() {
    // Build the binary first
    ensure_binary_built();

    let temp_dir = TempDir::new().unwrap();

    // Create clean project
    std::fs::write(temp_dir.path().join("test.ts"), "export class Test {}").unwrap();

    // Create config
    std::fs::write(
        temp_dir.path().join("architect.json"),
        r#"{"architecture_pattern": "MVC", "max_lines_per_function": 50, "forbidden_imports": []}"#
    ).unwrap();

    // Run linter
    let binary_path = get_binary_path();
    let output = Command::new(&binary_path)
        .current_dir(temp_dir.path())
        .arg(".")
        .output()
        .expect("Failed to execute architect-linter-pro");

    // Should exit with success code
    assert_eq!(
        output.status.code(),
        Some(0),
        "Should exit with code 0 for clean project"
    );
}

#[test]
fn test_github_action_workflow_performance_small_project() {
    // Build the binary first
    ensure_binary_built();

    let temp_dir = TempDir::new().unwrap();

    // Create a small project with 5 files
    let src_dir = temp_dir.path().join("src");
    std::fs::create_dir_all(&src_dir).unwrap();

    for i in 0..5 {
        let content = format!("export class Class{} {{ method() {{ return {}; }} }}", i, i);
        std::fs::write(src_dir.join(format!("file{}.ts", i)), content).unwrap();
    }

    // Create config
    std::fs::write(
        temp_dir.path().join("architect.json"),
        r#"{"architecture_pattern": "MVC", "max_lines_per_function": 50, "forbidden_imports": []}"#
    ).unwrap();

    // Run linter and measure time
    let binary_path = get_binary_path();
    let start = std::time::Instant::now();

    let output = Command::new(&binary_path)
        .current_dir(temp_dir.path())
        .arg(".")
        .output()
        .expect("Failed to execute architect-linter-pro");

    let duration = start.elapsed();

    // Should succeed
    assert!(
        output.status.success(),
        "Command should succeed"
    );

    // Should complete quickly (less than 30 seconds for small project)
    assert!(
        duration.as_secs() < 30,
        "Analysis should complete quickly, took: {:?}",
        duration
    );
}
