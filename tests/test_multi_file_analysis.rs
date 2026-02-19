//! Integration test for multi-file analysis combining parsing with scoring
//!
//! This test validates that:
//! - Multiple TypeScript files can be parsed and analyzed together
//! - Scoring works correctly with real parsed violations
//! - The integration between parser and scoring engine is seamless

use swc_common::sync::Lrc;
use swc_common::SourceMap;
use tempfile::TempDir;

use architect_linter_pro::analyzer::analyze_all_files;
use architect_linter_pro::config::load_config;

/// Test multi-file scoring with real parsing
///
/// This test creates multiple TypeScript files, analyzes them with the real parser,
/// and verifies that scoring works correctly with the results.
#[test]
fn test_multi_file_scoring() {
    let temp_dir = TempDir::new().unwrap();

    // Create multiple TypeScript files with different roles
    let file1 = temp_dir.path().join("UserController.ts");
    std::fs::write(
        &file1,
        r#"
export class UserController {
    constructor() {}

    index() {
        return {};
    }

    show() {
        return {};
    }

    create() {
        return {};
    }
}
"#,
    )
    .unwrap();

    let file2 = temp_dir.path().join("UserService.ts");
    std::fs::write(
        &file2,
        r#"
export class UserService {
    constructor() {}

    get() {
        return {};
    }

    save() {
        return {};
    }

    delete() {
        return {};
    }
}
"#,
    )
    .unwrap();

    let file3 = temp_dir.path().join("Model.ts");
    std::fs::write(
        &file3,
        r#"
export class Model {
    constructor() {}

    save() {
        return true;
    }

    validate() {
        return true;
    }

    delete() {
        return false;
    }
}
"#,
    )
    .unwrap();

    // Create architect config
    let config_path = temp_dir.path().join("architect.json");
    std::fs::write(
        &config_path,
        r#"
{
  "max_lines_per_function": 30,
  "architecture_pattern": "MVC",
  "forbidden_imports": []
}
"#,
    )
    .unwrap();

    // Load config
    let config = load_config(temp_dir.path()).expect("Failed to load config");
    let linter_context: architect_linter_pro::config::LinterContext = config.into();

    // Collect all files
    let files = vec![file1, file2, file3];

    // Run real analysis with the parser
    let cm = Lrc::new(SourceMap::default());
    let analysis_result = analyze_all_files(
        &files,
        temp_dir.path(),
        linter_context.pattern.clone(),
        &linter_context,
        &cm,
        None,
    )
    .expect("Failed to analyze files");

    // Verify analysis was successful
    assert_eq!(analysis_result.files_analyzed, 3, "Should analyze 3 files");

    // Verify the full scoring engine also works
    use architect_linter_pro::scoring;
    let health_score = scoring::calculate(&analysis_result);

    // The full scoring should give a good grade for clean code
    assert!(
        health_score.total >= 70,
        "Clean code should score at least 70, got {}",
        health_score.total
    );

    println!(
        "✓ Multi-file analysis test passed with score: {} ({:?})",
        health_score.total, health_score.grade
    );
}
