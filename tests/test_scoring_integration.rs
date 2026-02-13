/// Integration tests for scoring engine with real project fixtures
///
/// Tests validate scoring behavior with realistic codebases:
/// - perfect_mvc_project: Perfect architecture, should score A
/// - failing_hexagonal: Layer violations, should score C or lower
/// - mixed_clean_arch: Minor violations, should score B or C
/// - circular_deps: Circular dependencies, should score <75

use std::path::PathBuf;
use architect_linter_pro::analysis_result::AnalysisResult;
use architect_linter_pro::metrics::HealthGrade;
use architect_linter_pro::scoring;

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

#[test]
fn test_perfect_mvc_project() {
    let fixture_path = fixture_path("perfect_mvc_project");
    let result = analyze_fixture(&fixture_path);

    let score = scoring::calculate(&result);

    // Perfect MVC should score A (90-100)
    assert_eq!(score.grade, HealthGrade::A);
    assert!(score.total >= 90);

    // No violations
    assert!(result.violations.is_empty());

    // No circular dependencies
    assert!(result.circular_dependencies.is_empty());

    println!("✓ Perfect MVC project scored: {} ({:?})", score.total, score.grade);
}

#[test]
fn test_failing_hexagonal_with_violations() {
    let fixture_path = fixture_path("failing_hexagonal");
    let result = analyze_fixture(&fixture_path);

    let score = scoring::calculate(&result);

    // Should fail due to layer violations
    assert!(score.total < 80);
    assert!(matches!(score.grade, HealthGrade::C | HealthGrade::D | HealthGrade::F));

    // Should have violations
    assert!(!result.violations.is_empty());

    // Verify specific violation: domain → infrastructure
    let has_domain_violation = result.violations.iter().any(|v| {
        v.violation.rule.from == "domain" && v.violation.rule.to == "infrastructure"
    });
    assert!(has_domain_violation, "Should detect domain → infrastructure violation");

    println!("✓ Failing hexagonal scored: {} ({:?})", score.total, score.grade);
    println!("  Violations: {}", result.violations.len());
}

#[test]
fn test_mixed_clean_arch_partial_score() {
    let fixture_path = fixture_path("mixed_clean_arch");
    let result = analyze_fixture(&fixture_path);

    let score = scoring::calculate(&result);

    // Mixed quality should give B or C (70-90)
    assert!(matches!(score.grade, HealthGrade::B | HealthGrade::C));
    assert!(score.total >= 70 && score.total < 90);

    println!("✓ Mixed clean arch scored: {} ({:?})", score.total, score.grade);
    println!("  Long functions: {}", result.complexity_stats.long_functions);
}

#[test]
fn test_scoring_with_circular_dependencies() {
    let fixture_path = fixture_path("circular_deps");
    let result = analyze_fixture(&fixture_path);

    let score = scoring::calculate(&result);

    // Circular deps should significantly reduce score
    assert!(!result.circular_dependencies.is_empty(), "Should detect circular dependencies");
    assert!(score.total < 75, "Score should be low due to circular deps");

    // Circular component should be 0
    assert_eq!(score.components.circular_deps, 0);

    println!("✓ Circular deps project scored: {} ({:?})", score.total, score.grade);
    println!("  Cycles detected: {}", result.circular_dependencies.len());
}

/// Helper function to analyze a project fixture
fn analyze_fixture(path: &PathBuf) -> AnalysisResult {
    use swc_common::sync::Lrc;
    use swc_common::SourceMap;
    use architect_linter_pro::config::load_config;
    use architect_linter_pro::analyzer::analyze_all_files;
    use walkdir::WalkDir;

    let config = load_config(&path.join("architect.json")).expect("Failed to load config");

    // Collect all TypeScript/JavaScript files
    let files: Vec<PathBuf> = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().extension()
                .and_then(|s| s.to_str())
                .map(|s| s == "ts" || s == "js" || s == "tsx" || s == "jsx")
                .unwrap_or(false)
        })
        .map(|e| e.path().to_path_buf())
        .collect();

    let cm = Lrc::new(SourceMap::default());

    let linter_context: architect_linter_pro::config::LinterContext = config.into();

    analyze_all_files(&files, path, linter_context.pattern.clone(), &linter_context, &cm, None)
        .expect("Failed to analyze files")
}
