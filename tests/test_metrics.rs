//! Unit tests for metrics module
//!
//! This test suite verifies health score calculation, grade conversion,
//! and performance metrics tracking.

use architect_linter_pro::metrics::{ComponentStatus, HealthGrade, HealthScore, ScoreComponents};

// ============================================================================
// Tests for PerformanceMetrics (existing)
// ============================================================================

// ============================================================================
// Tests for HealthGrade
// ============================================================================

#[test]
fn test_health_grade_from_score_a() {
    assert_eq!(HealthGrade::from_score(100), HealthGrade::A);
    assert_eq!(HealthGrade::from_score(95), HealthGrade::A);
    assert_eq!(HealthGrade::from_score(90), HealthGrade::A);
}

#[test]
fn test_health_grade_from_score_b() {
    assert_eq!(HealthGrade::from_score(89), HealthGrade::B);
    assert_eq!(HealthGrade::from_score(85), HealthGrade::B);
    assert_eq!(HealthGrade::from_score(80), HealthGrade::B);
}

#[test]
fn test_health_grade_from_score_c() {
    assert_eq!(HealthGrade::from_score(79), HealthGrade::C);
    assert_eq!(HealthGrade::from_score(75), HealthGrade::C);
    assert_eq!(HealthGrade::from_score(70), HealthGrade::C);
}

#[test]
fn test_health_grade_from_score_d() {
    assert_eq!(HealthGrade::from_score(69), HealthGrade::D);
    assert_eq!(HealthGrade::from_score(65), HealthGrade::D);
    assert_eq!(HealthGrade::from_score(60), HealthGrade::D);
}

#[test]
fn test_health_grade_from_score_f() {
    assert_eq!(HealthGrade::from_score(59), HealthGrade::F);
    assert_eq!(HealthGrade::from_score(30), HealthGrade::F);
    assert_eq!(HealthGrade::from_score(0), HealthGrade::F);
}

#[test]
fn test_health_grade_boundary_values() {
    // Test boundary values
    assert_eq!(HealthGrade::from_score(90), HealthGrade::A);
    assert_eq!(HealthGrade::from_score(89), HealthGrade::B);
    assert_eq!(HealthGrade::from_score(80), HealthGrade::B);
    assert_eq!(HealthGrade::from_score(79), HealthGrade::C);
    assert_eq!(HealthGrade::from_score(70), HealthGrade::C);
    assert_eq!(HealthGrade::from_score(69), HealthGrade::D);
    assert_eq!(HealthGrade::from_score(60), HealthGrade::D);
    assert_eq!(HealthGrade::from_score(59), HealthGrade::F);
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
// Tests for ScoreComponents
// ============================================================================

#[test]
fn test_score_components_perfect_scores() {
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
fn test_score_components_zero_scores() {
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
fn test_score_components_weighted_calculation() {
    // Test that weights are applied correctly
    // layer_isolation: 30%, circular_deps: 25%, complexity: 20%, violations: 25%
    let components = ScoreComponents {
        layer_isolation: 100,
        circular_deps: 0,
        complexity: 0,
        violations: 0,
    };

    let total = components.calculate_total();
    assert_eq!(total, 30); // 100 * 0.30 = 30
}

// ============================================================================
// Tests for HealthScore
// ============================================================================

#[test]
fn test_health_score_perfect() {
    let components = ScoreComponents {
        layer_isolation: 100,
        circular_deps: 100,
        complexity: 100,
        violations: 100,
    };

    let score = HealthScore::new(components);

    assert_eq!(score.total, 100);
    assert_eq!(score.grade, HealthGrade::A);
    assert_eq!(score.components.layer_isolation, 100);
}

#[test]
fn test_health_score_failing() {
    let components = ScoreComponents {
        layer_isolation: 0,
        circular_deps: 0,
        complexity: 0,
        violations: 0,
    };

    let score = HealthScore::new(components);

    assert_eq!(score.total, 0);
    assert_eq!(score.grade, HealthGrade::F);
}

#[test]
fn test_health_score_status_indicator() {
    let components_a = ScoreComponents {
        layer_isolation: 100,
        circular_deps: 100,
        complexity: 100,
        violations: 100,
    };
    let score_a = HealthScore::new(components_a);
    assert_eq!(score_a.status_indicator(), "✅ Excellent");

    let components_f = ScoreComponents {
        layer_isolation: 0,
        circular_deps: 0,
        complexity: 0,
        violations: 0,
    };
    let score_f = HealthScore::new(components_f);
    assert_eq!(score_f.status_indicator(), "✗ Critical");
}

// ============================================================================
// Tests for ComponentStatus
// ============================================================================

#[test]
fn test_component_status_from_score_ok() {
    assert_eq!(ComponentStatus::from_score(100), ComponentStatus::Ok);
    assert_eq!(ComponentStatus::from_score(95), ComponentStatus::Ok);
    assert_eq!(ComponentStatus::from_score(90), ComponentStatus::Ok);
}

#[test]
fn test_component_status_from_score_warning() {
    assert_eq!(ComponentStatus::from_score(89), ComponentStatus::Warning);
    assert_eq!(ComponentStatus::from_score(75), ComponentStatus::Warning);
    assert_eq!(ComponentStatus::from_score(60), ComponentStatus::Warning);
}

#[test]
fn test_component_status_from_score_blocked() {
    assert_eq!(ComponentStatus::from_score(59), ComponentStatus::Blocked);
    assert_eq!(ComponentStatus::from_score(30), ComponentStatus::Blocked);
    assert_eq!(ComponentStatus::from_score(0), ComponentStatus::Blocked);
}

#[test]
fn test_component_status_boundary_values() {
    assert_eq!(ComponentStatus::from_score(90), ComponentStatus::Ok);
    assert_eq!(ComponentStatus::from_score(89), ComponentStatus::Warning);
    assert_eq!(ComponentStatus::from_score(60), ComponentStatus::Warning);
    assert_eq!(ComponentStatus::from_score(59), ComponentStatus::Blocked);
}

#[test]
fn test_component_status_emoji() {
    assert_eq!(ComponentStatus::Ok.emoji(), "[OK]");
    assert_eq!(ComponentStatus::Warning.emoji(), "[!]");
    assert_eq!(ComponentStatus::Blocked.emoji(), "[X]");
}
