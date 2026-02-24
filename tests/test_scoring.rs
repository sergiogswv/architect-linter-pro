/// Comprehensive unit tests for scoring engine
///
/// Tests cover:
/// - Health grade calculation (A-F)
/// - Component score calculations
/// - Score component weighting
/// - Edge cases and boundary conditions
use architect_linter_pro::analysis_result::{
    AnalysisResult, CategorizedViolation, ViolationCategory,
};
use architect_linter_pro::autofix::Violation;
use architect_linter_pro::circular::CircularDependency;
use architect_linter_pro::config::{ArchPattern, ForbiddenRule};
use architect_linter_pro::metrics::{
    ComplexityStats, ComponentStatus, HealthGrade, HealthScore, LayerStats, ScoreComponents,
};
use architect_linter_pro::scoring;
use chrono::Utc;
use std::path::PathBuf;

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
// Grade Boundary Tests (Phase 1)
// ============================================================================

#[test]
fn test_grade_boundaries_ab() {
    // A/B boundary: 90
    assert_eq!(HealthGrade::from_score(90), HealthGrade::A);
    assert_eq!(HealthGrade::from_score(89), HealthGrade::B);
    assert_eq!(HealthGrade::from_score(91), HealthGrade::A);
}

#[test]
fn test_grade_boundaries_bc() {
    // B/C boundary: 80
    assert_eq!(HealthGrade::from_score(80), HealthGrade::B);
    assert_eq!(HealthGrade::from_score(79), HealthGrade::C);
    assert_eq!(HealthGrade::from_score(81), HealthGrade::B);
}

#[test]
fn test_grade_boundaries_cd() {
    // C/D boundary: 70
    assert_eq!(HealthGrade::from_score(70), HealthGrade::C);
    assert_eq!(HealthGrade::from_score(69), HealthGrade::D);
    assert_eq!(HealthGrade::from_score(71), HealthGrade::C);
}

#[test]
fn test_grade_boundaries_df() {
    // D/F boundary: 60
    assert_eq!(HealthGrade::from_score(60), HealthGrade::D);
    assert_eq!(HealthGrade::from_score(59), HealthGrade::F);
    assert_eq!(HealthGrade::from_score(61), HealthGrade::D);
}

// ============================================================================
// Division by Zero Protection Tests
// ============================================================================

#[test]
fn test_complexity_with_zero_functions() {
    let mut result = create_test_result();
    result.complexity_stats.total_functions = 0;
    result.complexity_stats.long_functions = 0;

    let score = scoring::calculate(&result);

    assert!(score.total <= 100);
    // Complexity component should not be 0 due to division by zero
    assert!(score.components.complexity > 0);
}

#[test]
fn test_layer_isolation_with_zero_imports() {
    let mut result = create_test_result();
    result.layer_stats.total_imports = 0;
    result.layer_stats.blocked_violations = 0;

    let score = scoring::calculate(&result);

    assert!(score.total <= 100);
    assert!(score.components.layer_isolation > 0);
}

// ============================================================================
// Empty Project Edge Case
// ============================================================================

#[test]
fn test_empty_project_scoring() {
    let mut result = create_test_result();
    result.files_analyzed = 0;
    result.violations = vec![];
    result.circular_dependencies = vec![];
    result.long_functions = vec![];
    result.layer_stats.total_imports = 0;
    result.layer_stats.blocked_violations = 0;
    result.complexity_stats.total_functions = 0;
    result.complexity_stats.long_functions = 0;

    let score = scoring::calculate(&result);

    assert!(score.total <= 100);
    // Should not be F just because it's empty
    assert_ne!(score.grade, HealthGrade::F);
}

// ============================================================================
// Component Isolation Tests (Phase 2)
// ============================================================================

// Layer Isolation Component Tests

#[test]
fn test_layer_isolation_component_perfect() {
    let mut result = create_test_result();
    result.layer_stats.total_imports = 100;
    result.layer_stats.blocked_violations = 0;

    let score = scoring::calculate(&result);
    let layer_component = score.components.layer_isolation;

    assert_eq!(layer_component, 100);
}

#[test]
fn test_layer_isolation_component_warning() {
    let mut result = create_test_result();
    result.layer_stats.total_imports = 100;
    result.layer_stats.blocked_violations = 10; // 10% violations

    let score = scoring::calculate(&result);
    let layer_component = score.components.layer_isolation;

    // 10 violations out of 100 imports = 90% clean
    assert_eq!(layer_component, 90);
}

#[test]
fn test_layer_isolation_component_fail() {
    let mut result = create_test_result();
    result.layer_stats.total_imports = 100;
    result.layer_stats.blocked_violations = 30; // 30% violations

    let score = scoring::calculate(&result);
    let layer_component = score.components.layer_isolation;

    // 30 violations = 70% clean
    assert_eq!(layer_component, 70);
}

// Circular Dependencies Component Tests

#[test]
fn test_circular_deps_component_clean() {
    let mut result = create_test_result();
    result.circular_dependencies = vec![];

    let score = scoring::calculate(&result);
    let circular_component = score.components.circular_deps;

    assert_eq!(circular_component, 100);
}

#[test]
fn test_circular_deps_component_detected() {
    let mut result = create_test_result();
    result.circular_dependencies = vec![CircularDependency {
        cycle: vec!["a".to_string(), "b".to_string(), "a".to_string()],
        description: "Cycle detected: a -> b -> a".to_string(),
    }];

    let score = scoring::calculate(&result);
    let circular_component = score.components.circular_deps;

    // Any circular dependency = 0 score (binary)
    assert_eq!(circular_component, 0);
}

#[test]
fn test_circular_deps_component_multiple() {
    let mut result = create_test_result();
    result.circular_dependencies = vec![
        CircularDependency {
            cycle: vec!["a".to_string(), "b".to_string(), "a".to_string()],
            description: "Cycle detected: a -> b -> a".to_string(),
        },
        CircularDependency {
            cycle: vec![
                "x".to_string(),
                "y".to_string(),
                "z".to_string(),
                "x".to_string(),
            ],
            description: "Cycle detected: x -> y -> z -> x".to_string(),
        },
    ];

    let score = scoring::calculate(&result);
    let circular_component = score.components.circular_deps;

    // Multiple cycles = still 0 score
    assert_eq!(circular_component, 0);
}

// Complexity Component Tests

#[test]
fn test_complexity_component_low() {
    let mut result = create_test_result();
    result.complexity_stats.total_functions = 100;
    result.complexity_stats.long_functions = 5; // 5% long functions

    let score = scoring::calculate(&result);
    let complexity_component = score.components.complexity;

    assert!(complexity_component > 90);
}

#[test]
fn test_complexity_component_medium() {
    let mut result = create_test_result();
    result.complexity_stats.total_functions = 100;
    result.complexity_stats.long_functions = 20; // 20% long functions

    let score = scoring::calculate(&result);
    let complexity_component = score.components.complexity;

    assert!(complexity_component >= 70 && complexity_component <= 85);
}

#[test]
fn test_complexity_component_high() {
    let mut result = create_test_result();
    result.complexity_stats.total_functions = 100;
    result.complexity_stats.long_functions = 40; // 40% long functions

    let score = scoring::calculate(&result);
    let complexity_component = score.components.complexity;

    assert!(complexity_component < 70);
}

// Violations Component Tests

#[test]
fn test_violations_component_none() {
    let mut result = create_test_result();
    result.violations = vec![];

    let score = scoring::calculate(&result);
    let violations_component = score.components.violations;

    assert_eq!(violations_component, 100);
}

#[test]
fn test_violations_component_few() {
    let mut result = create_test_result();

    let violation = Violation {
        file_path: PathBuf::from("test.ts"),
        file_content: String::new(),
        offensive_import: "import { something } from 'forbidden/path'".to_string(),
        rule: ForbiddenRule {
            from: "domain".to_string(),
            to: "infrastructure".to_string(),
            severity: None,
            reason: None,
        },
        line_number: 10,
    };

    result.violations = vec![CategorizedViolation::new(
        violation,
        ViolationCategory::Blocked,
    )];

    let score = scoring::calculate(&result);
    let violations_component = score.components.violations;

    assert!(violations_component > 80);
}

#[test]
fn test_violations_component_many() {
    let mut result = create_test_result();

    // Add 10 violations
    for i in 0..10 {
        let violation = Violation {
            file_path: PathBuf::from(format!("test{}.ts", i)),
            file_content: String::new(),
            offensive_import: format!("import {{ something }} from 'forbidden/path/{}'", i),
            rule: ForbiddenRule {
                from: "domain".to_string(),
                to: "infrastructure".to_string(),
                severity: None,
                reason: None,
            },
            line_number: i * 10,
        };

        result.violations.push(CategorizedViolation::new(
            violation,
            ViolationCategory::Blocked,
        ));
    }

    let score = scoring::calculate(&result);
    let violations_component = score.components.violations;

    assert!(violations_component < 80);
}

// ============================================================================
// Consistency & Repeatability Tests (Phase 4)
// ============================================================================

#[test]
fn test_scoring_idempotency_same_input() {
    // Same input should always produce same output
    let mut result1 = create_test_result();
    let mut result2 = create_test_result();

    // Add same violations to both
    result1.layer_stats.blocked_violations = 5;
    result1.complexity_stats.long_functions = 3;

    result2.layer_stats.blocked_violations = 5;
    result2.complexity_stats.long_functions = 3;

    let score1 = scoring::calculate(&result1);
    let score2 = scoring::calculate(&result2);

    assert_eq!(score1.total, score2.total);
    assert_eq!(score1.grade, score2.grade);

    // All components should be identical
    assert_eq!(
        score1.components.layer_isolation,
        score2.components.layer_isolation
    );
    assert_eq!(
        score1.components.circular_deps,
        score2.components.circular_deps
    );
    assert_eq!(score1.components.complexity, score2.components.complexity);
    assert_eq!(score1.components.violations, score2.components.violations);
}

#[test]
fn test_scoring_determinism_100_runs() {
    // Run scoring 100 times on same input
    let mut scores = Vec::new();

    for _ in 0..100 {
        let mut result = create_test_result();
        result.layer_stats.blocked_violations = 5;
        result.complexity_stats.long_functions = 3;

        let score = scoring::calculate(&result);
        scores.push(score.total);
    }

    // All scores should be identical
    let first = scores[0];
    let all_same = scores.iter().all(|&s| s == first);

    assert!(all_same, "All 100 runs should produce identical scores");
    println!("✓ 100 runs produced consistent score: {}", first);
}

#[test]
fn test_scoring_with_identical_projects() {
    // Two identical projects should get same score
    let result1 = create_test_result();
    let result2 = create_test_result();

    let score1 = scoring::calculate(&result1);
    let score2 = scoring::calculate(&result2);

    assert_eq!(score1.total, score2.total);
    assert_eq!(score1.grade, score2.grade);

    println!(
        "✓ Identical projects scored: {} ({:?})",
        score1.total, score1.grade
    );
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
