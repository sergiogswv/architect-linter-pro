/// Comprehensive unit tests for scoring engine
///
/// Tests cover:
/// - Health grade calculation (A-F)
/// - Component score calculations
/// - Score component weighting
/// - Edge cases and boundary conditions
use architect_linter_pro::analysis_result::AnalysisResult;
use architect_linter_pro::circular::CircularDependency;
use architect_linter_pro::config::ArchPattern;
use architect_linter_pro::metrics::{
    ComplexityStats, ComponentStatus, HealthGrade, HealthScore, LayerStats, ScoreComponents,
};
use architect_linter_pro::scoring;
use chrono::Utc;

// Helper to create a minimal AnalysisResult for testing
fn create_test_result() -> AnalysisResult {
    AnalysisResult {
        project_name: "test-project".to_string(),
        pattern: ArchPattern::MVC,
        files_analyzed: 10,
        violations: vec![],
        circular_dependencies: vec![],
        long_functions: vec![],
        layer_stats: LayerStats {
            total_imports: 100,
            blocked_violations: 0,
        },
        complexity_stats: ComplexityStats {
            total_functions: 50,
            long_functions: 0,
            max_lines_threshold: 100,
        },
        health_score: None,
        timestamp: Utc::now(),
    }
}

// ============================================================================
// HealthGrade Tests
// ============================================================================

#[test]
fn test_health_grade_a_range() {
    assert_eq!(HealthGrade::from_score(100), HealthGrade::A);
    assert_eq!(HealthGrade::from_score(95), HealthGrade::A);
    assert_eq!(HealthGrade::from_score(90), HealthGrade::A);
}

#[test]
fn test_health_grade_b_range() {
    assert_eq!(HealthGrade::from_score(89), HealthGrade::B);
    assert_eq!(HealthGrade::from_score(85), HealthGrade::B);
    assert_eq!(HealthGrade::from_score(80), HealthGrade::B);
}

#[test]
fn test_health_grade_c_range() {
    assert_eq!(HealthGrade::from_score(79), HealthGrade::C);
    assert_eq!(HealthGrade::from_score(75), HealthGrade::C);
    assert_eq!(HealthGrade::from_score(70), HealthGrade::C);
}

#[test]
fn test_health_grade_d_range() {
    assert_eq!(HealthGrade::from_score(69), HealthGrade::D);
    assert_eq!(HealthGrade::from_score(65), HealthGrade::D);
    assert_eq!(HealthGrade::from_score(60), HealthGrade::D);
}

#[test]
fn test_health_grade_f_range() {
    assert_eq!(HealthGrade::from_score(59), HealthGrade::F);
    assert_eq!(HealthGrade::from_score(30), HealthGrade::F);
    assert_eq!(HealthGrade::from_score(0), HealthGrade::F);
}

#[test]
fn test_health_grade_as_str() {
    assert_eq!(HealthGrade::A.as_str(), "A");
    assert_eq!(HealthGrade::B.as_str(), "B");
    assert_eq!(HealthGrade::C.as_str(), "C");
    assert_eq!(HealthGrade::D.as_str(), "D");
    assert_eq!(HealthGrade::F.as_str(), "F");
}

#[test]
fn test_health_grade_emoji() {
    assert_eq!(HealthGrade::A.emoji(), "🏆");
    assert_eq!(HealthGrade::B.emoji(), "✨");
    assert_eq!(HealthGrade::C.emoji(), "👍");
    assert_eq!(HealthGrade::D.emoji(), "⚠️");
    assert_eq!(HealthGrade::F.emoji(), "❌");
}

// ============================================================================
// ComponentStatus Tests
// ============================================================================

#[test]
fn test_component_status_ok() {
    assert_eq!(ComponentStatus::from_score(100), ComponentStatus::Ok);
    assert_eq!(ComponentStatus::from_score(95), ComponentStatus::Ok);
    assert_eq!(ComponentStatus::from_score(90), ComponentStatus::Ok);
}

#[test]
fn test_component_status_warning() {
    assert_eq!(ComponentStatus::from_score(89), ComponentStatus::Warning);
    assert_eq!(ComponentStatus::from_score(75), ComponentStatus::Warning);
    assert_eq!(ComponentStatus::from_score(60), ComponentStatus::Warning);
}

#[test]
fn test_component_status_blocked() {
    assert_eq!(ComponentStatus::from_score(59), ComponentStatus::Blocked);
    assert_eq!(ComponentStatus::from_score(30), ComponentStatus::Blocked);
    assert_eq!(ComponentStatus::from_score(0), ComponentStatus::Blocked);
}

#[test]
fn test_component_status_emoji() {
    assert_eq!(ComponentStatus::Ok.emoji(), "[OK]");
    assert_eq!(ComponentStatus::Warning.emoji(), "[!]");
    assert_eq!(ComponentStatus::Blocked.emoji(), "[X]");
}

// ============================================================================
// ScoreComponents Tests
// ============================================================================

#[test]
fn test_score_components_perfect_score() {
    let components = ScoreComponents {
        layer_isolation: 100,
        circular_deps: 100,
        complexity: 100,
        violations: 100,
    };

    let total = components.calculate_total();
    assert_eq!(total, 100);
}

#[test]
fn test_score_components_zero_score() {
    let components = ScoreComponents {
        layer_isolation: 0,
        circular_deps: 0,
        complexity: 0,
        violations: 0,
    };

    let total = components.calculate_total();
    assert_eq!(total, 0);
}

#[test]
fn test_score_components_weighting() {
    // Test with known values to verify weights: 30%, 25%, 20%, 25%
    let components = ScoreComponents {
        layer_isolation: 80, // 80 * 0.30 = 24
        circular_deps: 100,  // 100 * 0.25 = 25
        complexity: 60,      // 60 * 0.20 = 12
        violations: 40,      // 40 * 0.25 = 10
    };
    // Total = 24 + 25 + 12 + 10 = 71

    let total = components.calculate_total();
    assert_eq!(total, 71);
}

#[test]
fn test_score_components_mixed_scores() {
    let components = ScoreComponents {
        layer_isolation: 90,
        circular_deps: 0, // Has cycles
        complexity: 85,
        violations: 75,
    };

    let total = components.calculate_total();
    // 90*0.3 + 0*0.25 + 85*0.2 + 75*0.25 = 27 + 0 + 17 + 18.75 = 62.75 ≈ 63
    assert!(total >= 62 && total <= 63);
}

// ============================================================================
// Layer Isolation Score Tests
// ============================================================================

#[test]
fn test_layer_isolation_perfect() {
    let mut result = create_test_result();
    result.layer_stats.blocked_violations = 0;
    result.layer_stats.total_imports = 100;

    let score = scoring::calculate(&result);
    assert_eq!(score.components.layer_isolation, 100);
}

#[test]
fn test_layer_isolation_with_violations() {
    let mut result = create_test_result();
    result.layer_stats.blocked_violations = 10;
    result.layer_stats.total_imports = 100;
    // 100 - (10/100 * 100) = 100 - 10 = 90

    let score = scoring::calculate(&result);
    assert_eq!(score.components.layer_isolation, 90);
}

#[test]
fn test_layer_isolation_all_violations() {
    let mut result = create_test_result();
    result.layer_stats.blocked_violations = 100;
    result.layer_stats.total_imports = 100;

    let score = scoring::calculate(&result);
    assert_eq!(score.components.layer_isolation, 0);
}

#[test]
fn test_layer_isolation_more_violations_than_imports() {
    let mut result = create_test_result();
    result.layer_stats.blocked_violations = 150;
    result.layer_stats.total_imports = 100;

    let score = scoring::calculate(&result);
    // Should be clamped to 0
    assert_eq!(score.components.layer_isolation, 0);
}

#[test]
fn test_layer_isolation_zero_imports() {
    let mut result = create_test_result();
    result.layer_stats.blocked_violations = 0;
    result.layer_stats.total_imports = 0;

    let score = scoring::calculate(&result);
    // Should avoid division by zero and return 100
    assert_eq!(score.components.layer_isolation, 100);
}

// ============================================================================
// Circular Dependencies Score Tests
// ============================================================================

#[test]
fn test_circular_deps_none() {
    let mut result = create_test_result();
    result.circular_dependencies = vec![];

    let score = scoring::calculate(&result);
    assert_eq!(score.components.circular_deps, 100);
}

#[test]
fn test_circular_deps_has_cycles() {
    let mut result = create_test_result();
    result.circular_dependencies = vec![CircularDependency {
        cycle: vec!["a.ts".to_string(), "b.ts".to_string(), "a.ts".to_string()],
        description: "Test cycle".to_string(),
    }];

    let score = scoring::calculate(&result);
    assert_eq!(score.components.circular_deps, 0);
}

#[test]
fn test_circular_deps_multiple_cycles() {
    let mut result = create_test_result();
    result.circular_dependencies = vec![
        CircularDependency {
            cycle: vec!["a.ts".to_string(), "b.ts".to_string()],
            description: "Cycle 1".to_string(),
        },
        CircularDependency {
            cycle: vec!["c.ts".to_string(), "d.ts".to_string()],
            description: "Cycle 2".to_string(),
        },
    ];

    let score = scoring::calculate(&result);
    // Still 0 if any cycles exist
    assert_eq!(score.components.circular_deps, 0);
}

// ============================================================================
// Complexity Score Tests
// ============================================================================

#[test]
fn test_complexity_no_long_functions() {
    let mut result = create_test_result();
    result.complexity_stats.long_functions = 0;
    result.complexity_stats.total_functions = 50;

    let score = scoring::calculate(&result);
    assert_eq!(score.components.complexity, 100);
}

#[test]
fn test_complexity_some_long_functions() {
    let mut result = create_test_result();
    result.complexity_stats.long_functions = 5;
    result.complexity_stats.total_functions = 50;
    // 100 - (5/50 * 100) = 100 - 10 = 90

    let score = scoring::calculate(&result);
    assert_eq!(score.components.complexity, 90);
}

#[test]
fn test_complexity_all_long_functions() {
    let mut result = create_test_result();
    result.complexity_stats.long_functions = 50;
    result.complexity_stats.total_functions = 50;

    let score = scoring::calculate(&result);
    assert_eq!(score.components.complexity, 0);
}

#[test]
fn test_complexity_half_long() {
    let mut result = create_test_result();
    result.complexity_stats.long_functions = 25;
    result.complexity_stats.total_functions = 50;

    let score = scoring::calculate(&result);
    assert_eq!(score.components.complexity, 50);
}

#[test]
fn test_complexity_zero_functions() {
    let mut result = create_test_result();
    result.complexity_stats.long_functions = 0;
    result.complexity_stats.total_functions = 0;

    let score = scoring::calculate(&result);
    // Should avoid division by zero and return 100
    assert_eq!(score.components.complexity, 100);
}

// ============================================================================
// HealthScore Creation Tests
// ============================================================================

#[test]
fn test_health_score_perfect_project() {
    let result = create_test_result();
    let score = scoring::calculate(&result);

    assert_eq!(score.total, 100);
    assert_eq!(score.grade, HealthGrade::A);
    assert_eq!(score.layer_isolation_status, ComponentStatus::Ok);
    assert_eq!(score.circular_deps_status, ComponentStatus::Ok);
    assert_eq!(score.complexity_status, ComponentStatus::Ok);
    assert_eq!(score.violations_status, ComponentStatus::Ok);
}

#[test]
fn test_health_score_with_all_issues() {
    let mut result = create_test_result();
    result.layer_stats.blocked_violations = 50;
    result.circular_dependencies = vec![CircularDependency {
        cycle: vec!["a.ts".to_string()],
        description: "test".to_string(),
    }];
    result.complexity_stats.long_functions = 25;

    let score = scoring::calculate(&result);

    // Should be a failing score (F grade < 60)
    assert!(score.total < 60, "Expected score < 60, got {}", score.total);
    assert_eq!(score.grade, HealthGrade::F);
}

#[test]
fn test_health_score_status_indicator() {
    let components = ScoreComponents {
        layer_isolation: 100,
        circular_deps: 100,
        complexity: 100,
        violations: 100,
    };

    let score = HealthScore::new(components);
    assert_eq!(score.status_indicator(), "✅ Excellent");
}

// ============================================================================
// Progress Bar Tests
// ============================================================================

#[test]
fn test_progress_bar_full() {
    let bar = scoring::get_progress_bar(100, 10);
    assert_eq!(bar, "[==========]");
}

#[test]
fn test_progress_bar_empty() {
    let bar = scoring::get_progress_bar(0, 10);
    assert_eq!(bar, "[          ]");
}

#[test]
fn test_progress_bar_half() {
    let bar = scoring::get_progress_bar(50, 10);
    assert_eq!(bar, "[=====     ]");
}

#[test]
fn test_progress_bar_different_widths() {
    assert_eq!(scoring::get_progress_bar(100, 5), "[=====]");
    assert_eq!(scoring::get_progress_bar(50, 20), "[==========          ]");
    assert_eq!(scoring::get_progress_bar(75, 8), "[======  ]");
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_realistic_good_project() {
    let mut result = create_test_result();
    result.files_analyzed = 100;
    result.layer_stats.total_imports = 500;
    result.layer_stats.blocked_violations = 5; // 99% clean
    result.circular_dependencies = vec![];
    result.complexity_stats.total_functions = 200;
    result.complexity_stats.long_functions = 10; // 5% long

    let score = scoring::calculate(&result);

    // Should get a B or A grade
    assert!(
        score.total >= 80,
        "Expected score >= 80, got {}",
        score.total
    );
    assert!(matches!(score.grade, HealthGrade::A | HealthGrade::B));
}

#[test]
fn test_realistic_poor_project() {
    let mut result = create_test_result();
    result.files_analyzed = 50;
    result.layer_stats.total_imports = 200;
    result.layer_stats.blocked_violations = 50; // 25% violations
    result.circular_dependencies = vec![CircularDependency {
        cycle: vec!["a.ts".to_string(), "b.ts".to_string()],
        description: "test".to_string(),
    }];
    result.complexity_stats.total_functions = 100;
    result.complexity_stats.long_functions = 40; // 40% long

    let score = scoring::calculate(&result);

    // Should get D or F grade (< 70)
    assert!(
        score.total <= 60,
        "Expected score <= 60, got {}",
        score.total
    );
    assert!(matches!(score.grade, HealthGrade::D | HealthGrade::F));
}
