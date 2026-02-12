/// Integration tests using test fixtures
///
/// These tests run the full CLI on pre-created test projects (fixtures)
/// and validate that the scoring system works correctly end-to-end.
///
/// Fixtures are located in tests/fixtures/
use assert_cmd::Command;
use predicates::prelude::*;
use std::path::PathBuf;
use tempfile::TempDir;

/// Helper to get fixture path
fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

/// Helper to run analysis on a fixture and capture JSON output
fn analyze_fixture_json(name: &str) -> serde_json::Value {
    let fixture_dir = fixture_path(name);
    let temp_dir = TempDir::new().unwrap();
    let report_path = temp_dir.path().join("report.json");

    // Run analysis with JSON report
    Command::cargo_bin("architect-linter-pro")
        .unwrap()
        .current_dir(&fixture_dir)
        .arg("--report")
        .arg("json")
        .arg("--output")
        .arg(&report_path)
        .arg(".")
        .assert();

    // Read the JSON report
    let report_content = std::fs::read_to_string(&report_path).expect("Failed to read report file");

    serde_json::from_str(&report_content).expect("Failed to parse JSON report")
}

// ============================================================================
// Fixture #1: perfect_project
// ============================================================================

#[test]
fn test_fixture_perfect_project() {
    let report = analyze_fixture_json("perfect_project");

    // Verify project name
    assert_eq!(report["project"]["name"], "perfect_project");

    // Perfect project should have no violations
    assert_eq!(report["summary"]["total_violations"], 0);
    assert_eq!(report["summary"]["circular_dependencies"], 0);
    assert_eq!(report["summary"]["long_functions"], 0);

    // Check health score
    let score = &report["health_score"];
    assert_eq!(score["total"], 100, "Perfect project should score 100");
    assert_eq!(score["grade"], "A", "Perfect project should be grade A");

    // All components should be 100
    assert_eq!(score["components"]["layer_isolation"], 100);
    assert_eq!(score["components"]["circular_deps"], 100);
    assert_eq!(score["components"]["complexity"], 100);
    assert_eq!(score["components"]["violations"], 100);
}

#[test]
fn test_fixture_perfect_project_exit_code() {
    let fixture_dir = fixture_path("perfect_project");

    // Perfect project should exit with success (0)
    Command::cargo_bin("architect-linter-pro")
        .unwrap()
        .current_dir(&fixture_dir)
        .arg(".")
        .assert()
        .success();
}

// ============================================================================
// Fixture #2: circular_deps
// ============================================================================

#[test]
fn test_fixture_circular_deps() {
    let report = analyze_fixture_json("circular_deps");

    // Should detect circular dependencies
    let cycles = report["summary"]["circular_dependencies"].as_u64().unwrap();
    assert!(
        cycles >= 2,
        "Should detect at least 2 cycles, found {}",
        cycles
    );

    // Check health score
    let score = &report["health_score"];

    // Circular deps component should be 0 (binary: has cycles = 0)
    assert_eq!(
        score["components"]["circular_deps"], 0,
        "Circular deps score should be 0 when cycles exist"
    );

    // Total score should be low (< 80)
    let total = score["total"].as_u64().unwrap();
    assert!(
        total < 80,
        "Project with cycles should score < 80, got {}",
        total
    );

    // Should be C, D or F grade
    let grade = score["grade"].as_str().unwrap();
    assert!(
        matches!(grade, "C" | "D" | "F"),
        "Expected C/D/F grade, got {}",
        grade
    );
}

#[test]
fn test_fixture_circular_deps_exit_code() {
    let fixture_dir = fixture_path("circular_deps");

    // Project with cycles should exit with failure (non-zero)
    Command::cargo_bin("architect-linter-pro")
        .unwrap()
        .current_dir(&fixture_dir)
        .arg(".")
        .assert()
        .failure();
}

// ============================================================================
// Fixture #3: long_functions
// ============================================================================

#[test]
fn test_fixture_long_functions() {
    let report = analyze_fixture_json("long_functions");

    // Should detect long functions
    let long_funcs = report["summary"]["long_functions"].as_u64().unwrap();
    assert!(
        long_funcs >= 2,
        "Should detect at least 2 long functions, found {}",
        long_funcs
    );

    // Check health score
    let score = &report["health_score"];

    // Complexity score should be penalized
    let complexity = score["components"]["complexity"].as_u64().unwrap();
    assert!(
        complexity < 100,
        "Complexity score should be penalized, got {}",
        complexity
    );

    // Total score should be B or lower (< 90)
    let total = score["total"].as_u64().unwrap();
    assert!(
        total < 90,
        "Project with long functions should score < 90, got {}",
        total
    );
}

#[test]
fn test_fixture_long_functions_list() {
    let report = analyze_fixture_json("long_functions");

    // Verify long functions are listed
    let long_funcs = report["long_functions"].as_array().unwrap();
    assert!(
        long_funcs.len() >= 2,
        "Should list at least 2 long functions"
    );

    // Verify each long function has required fields
    for func in long_funcs {
        assert!(func["name"].is_string(), "Should have name");
        assert!(func["file"].is_string(), "Should have file");
        assert!(func["lines"].is_number(), "Should have line count");
        assert!(func["threshold"].is_number(), "Should have threshold");
    }
}

// ============================================================================
// Fixture #4: forbidden_imports
// ============================================================================

#[test]
fn test_fixture_forbidden_imports() {
    let report = analyze_fixture_json("forbidden_imports");

    // Should detect violations
    let total_violations = report["summary"]["total_violations"].as_u64().unwrap();
    assert!(
        total_violations >= 4,
        "Should detect at least 4 violations, found {}",
        total_violations
    );

    let blocked = report["summary"]["blocked_violations"].as_u64().unwrap();
    assert!(blocked >= 4, "Should have at least 4 blocked violations");

    // Check health score
    let score = &report["health_score"];

    // Layer isolation should be penalized
    let layer_isolation = score["components"]["layer_isolation"].as_u64().unwrap();
    assert!(
        layer_isolation < 100,
        "Layer isolation should be penalized, got {}",
        layer_isolation
    );

    // Total score should be D or lower (< 70)
    let total = score["total"].as_u64().unwrap();
    assert!(
        total < 80,
        "Project with violations should score < 80, got {}",
        total
    );
}

#[test]
fn test_fixture_forbidden_imports_exit_code() {
    let fixture_dir = fixture_path("forbidden_imports");

    // Project with violations should exit with failure
    Command::cargo_bin("architect-linter-pro")
        .unwrap()
        .current_dir(&fixture_dir)
        .arg(".")
        .assert()
        .failure();
}

// ============================================================================
// Fixture #5: mixed_issues
// ============================================================================

#[test]
fn test_fixture_mixed_issues() {
    let report = analyze_fixture_json("mixed_issues");

    // Should have multiple types of issues
    let violations = report["summary"]["total_violations"].as_u64().unwrap();
    let cycles = report["summary"]["circular_dependencies"].as_u64().unwrap();
    let long_funcs = report["summary"]["long_functions"].as_u64().unwrap();

    // Should have at least 2 types of issues
    let issue_count = [violations > 0, cycles > 0, long_funcs > 0]
        .iter()
        .filter(|&&x| x)
        .count();

    assert!(
        issue_count >= 2,
        "Mixed issues fixture should have at least 2 types of issues"
    );

    // Check health score
    let score = &report["health_score"];

    // Total score should be very low (< 70)
    let total = score["total"].as_u64().unwrap();
    assert!(
        total < 70,
        "Project with mixed issues should score < 70, got {}",
        total
    );

    // Should be D or F grade
    let grade = score["grade"].as_str().unwrap();
    assert!(
        matches!(grade, "D" | "F"),
        "Expected D/F grade, got {}",
        grade
    );
}

#[test]
fn test_fixture_mixed_issues_comprehensive() {
    let report = analyze_fixture_json("mixed_issues");

    println!("Mixed Issues Analysis:");
    println!("  Violations: {}", report["summary"]["total_violations"]);
    println!("  Cycles: {}", report["summary"]["circular_dependencies"]);
    println!("  Long Functions: {}", report["summary"]["long_functions"]);

    let score = &report["health_score"];
    println!("  Total Score: {} ({})", score["total"], score["grade"]);
    println!(
        "  Layer Isolation: {}",
        score["components"]["layer_isolation"]
    );
    println!("  Circular Deps: {}", score["components"]["circular_deps"]);
    println!("  Complexity: {}", score["components"]["complexity"]);
    println!("  Violations: {}", score["components"]["violations"]);

    // Multiple component scores should be penalized
    let components = &score["components"];
    let penalized_components = [
        components["layer_isolation"].as_u64().unwrap(),
        components["circular_deps"].as_u64().unwrap(),
        components["complexity"].as_u64().unwrap(),
        components["violations"].as_u64().unwrap(),
    ]
    .iter()
    .filter(|&&x| x < 100)
    .count();

    assert!(
        penalized_components >= 2,
        "At least 2 components should be penalized"
    );
}

// ============================================================================
// Comparative Tests
// ============================================================================

#[test]
fn test_fixtures_score_ordering() {
    // Analyze all fixtures
    let perfect = analyze_fixture_json("perfect_project");
    let long_fns = analyze_fixture_json("long_functions");
    let forbidden = analyze_fixture_json("forbidden_imports");
    let circular = analyze_fixture_json("circular_deps");
    let mixed = analyze_fixture_json("mixed_issues");

    // Extract scores
    let perfect_score = perfect["health_score"]["total"].as_u64().unwrap();
    let long_fns_score = long_fns["health_score"]["total"].as_u64().unwrap();
    let forbidden_score = forbidden["health_score"]["total"].as_u64().unwrap();
    let circular_score = circular["health_score"]["total"].as_u64().unwrap();
    let mixed_score = mixed["health_score"]["total"].as_u64().unwrap();

    println!("Fixture Scores:");
    println!("  perfect_project: {}", perfect_score);
    println!("  long_functions: {}", long_fns_score);
    println!("  forbidden_imports: {}", forbidden_score);
    println!("  circular_deps: {}", circular_score);
    println!("  mixed_issues: {}", mixed_score);

    // Perfect should be highest
    assert_eq!(perfect_score, 100, "Perfect project should score 100");

    // Mixed issues should be lowest (has all problems)
    assert!(
        mixed_score < long_fns_score,
        "Mixed issues should score lower than single-issue projects"
    );
    assert!(
        mixed_score < forbidden_score,
        "Mixed issues should score lower than single-issue projects"
    );
}

// ============================================================================
// Snapshot Tests
// ============================================================================

#[test]
#[ignore] // TODO: Enable when insta json snapshots are configured
fn test_fixture_snapshots() {
    // Generate snapshots for all fixtures
    let fixtures = vec![
        "perfect_project",
        "circular_deps",
        "long_functions",
        "forbidden_imports",
        "mixed_issues",
    ];

    for fixture in fixtures {
        let report = analyze_fixture_json(fixture);
        let score = &report["health_score"];

        // Snapshot test for each fixture's score
        // insta::assert_json_snapshot!(format!("{}_score", fixture), score);
        println!("Snapshot for {}: {:?}", fixture, score);
    }
}
