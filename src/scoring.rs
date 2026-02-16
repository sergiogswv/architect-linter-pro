//! Scoring engine for Architecture Health Score
//!
//! This module implements the scoring algorithm that converts analysis results
//! into a 0-100 health score with letter grades.

use crate::analysis_result::AnalysisResult;
use crate::metrics::{HealthGrade, HealthScore, ScoreComponents};

/// Severity level of a violation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViolationType {
    /// Critical violation that severely impacts architecture
    Critical,
    /// Warning that should be reviewed
    Warning,
    /// Informational notice
    Info,
}

/// A simple violation for unit testing
#[derive(Debug, Clone)]
pub struct Violation {
    /// Severity of the violation
    pub severity: ViolationType,
    /// File path where the violation occurred
    pub file_path: String,
    /// Line number where the violation occurred
    pub line: usize,
    /// Violation message
    pub message: String,
}

/// Calculate the health score from an analysis result
pub fn calculate(result: &AnalysisResult) -> HealthScore {
    let components = ScoreComponents {
        layer_isolation: calculate_layer_isolation_score(result),
        circular_deps: calculate_circular_deps_score(result),
        complexity: calculate_complexity_score(result),
        violations: calculate_violations_score(result),
    };

    HealthScore::new(components)
}

/// Calculate layer isolation score
/// Formula: 100 - (blocked_violations / total_imports * 100)
/// Minimum score is 0
fn calculate_layer_isolation_score(result: &AnalysisResult) -> u8 {
    let blocked = result.layer_stats.blocked_violations;
    let total = result.layer_stats.total_imports.max(1); // Avoid division by zero

    if blocked == 0 {
        return 100;
    }

    let ratio = blocked as f64 / total as f64;
    let score = 100.0 - (ratio * 100.0);
    score.max(0.0).min(100.0) as u8
}

/// Calculate circular dependencies score
/// Binary: 100 if no cycles, 0 if cycles exist
fn calculate_circular_deps_score(result: &AnalysisResult) -> u8 {
    if result.circular_dependencies.is_empty() {
        100
    } else {
        0
    }
}

/// Calculate complexity score
/// Formula: 100 - (long_functions / total_functions * 100)
fn calculate_complexity_score(result: &AnalysisResult) -> u8 {
    let long_funcs = result.complexity_stats.long_functions;
    let total_funcs = result.complexity_stats.total_functions.max(1);

    if long_funcs == 0 {
        return 100;
    }

    let ratio = long_funcs as f64 / total_funcs as f64;
    let score = 100.0 - (ratio * 100.0);
    score.max(0.0).min(100.0) as u8
}

/// Calculate violations score
/// Formula: 100 - ((blocked * 2 + warnings) / total_checks * 100)
/// Where total_checks is estimated based on files analyzed
fn calculate_violations_score(result: &AnalysisResult) -> u8 {
    let blocked = result.blocked_count();
    let warnings = result.warning_count();

    if blocked == 0 && warnings == 0 {
        return 100;
    }

    // Estimate total checks as files * 5 (rough average of imports per file)
    let total_checks = (result.files_analyzed * 5).max(1);

    let penalty = (blocked * 2) as f64 + warnings as f64;
    let score = 100.0 - (penalty / total_checks as f64 * 100.0);
    score.max(0.0).min(100.0) as u8
}

/// Get a visual progress bar for a score
pub fn get_progress_bar(score: u8, width: usize) -> String {
    let filled = (score as usize * width) / 100;
    let empty = width - filled;

    let filled_char = "=";
    let empty_char = " ";

    format!(
        "[{}{}]",
        filled_char.repeat(filled),
        empty_char.repeat(empty)
    )
}

/// Get grade color code for terminal (ANSI)
pub fn get_grade_color(grade: HealthGrade) -> &'static str {
    match grade {
        HealthGrade::A => "\x1b[32m", // Green
        HealthGrade::B => "\x1b[36m", // Cyan
        HealthGrade::C => "\x1b[33m", // Yellow
        HealthGrade::D => "\x1b[35m", // Magenta
        HealthGrade::F => "\x1b[31m", // Red
    }
}

/// Reset ANSI color
pub fn reset_color() -> &'static str {
    "\x1b[0m"
}

/// Apply severity multipliers to a base score
///
/// This function applies various multipliers and bonuses to a base health score:
/// - Empty project bonus: 10% bonus for projects with 0 files and 0 violations
/// - Architecture pattern bonus: 5% bonus for detected patterns (e.g., MVC)
/// - Historical trend factor: adjustment based on improvement/degradation over time
///
/// # Arguments
/// * `violations` - List of violations found in the project
/// * `base_score` - The base health score (0-100)
/// * `total_files` - Total number of files in the project
///
/// # Returns
/// Adjusted health score with multipliers applied
pub fn apply_severity_multiplier(violations: &[Violation], base_score: f64, total_files: usize) -> f64 {
    let mut score = base_score;

    // Empty project bonus: 10% bonus for projects with no files and no violations
    if total_files == 0 && violations.is_empty() {
        score = base_score * 1.1; // 10% bonus
    }

    // Architecture pattern bonus: 5% bonus for detected patterns
    // TODO: This is a placeholder for future implementation. In the full version, this should:
    // - Detect actual architectural patterns (MVC, Layered, Hexagonal, etc.)
    // - Use static analysis to identify pattern-specific structures
    // - Apply bonus based on pattern adherence quality
    // For now, we apply a simple heuristic based on project size as a temporary approximation
    if total_files >= 100 && violations.is_empty() {
        score = score * 1.05; // 5% bonus for well-structured large projects
    }

    // Historical trend factor would be applied here
    // TODO: This is a placeholder for future functionality. In the full version, this should:
    // - Accept historical trend data as a parameter
    // - Compare current results with previous runs
    // - Apply multiplier based on improvement/degradation trends
    // For now, we just return the score with applied bonuses

    score.min(100.0) // Cap scores at 100.0 to prevent overflow from multipliers
}

/// Calculate health score from a list of violations
///
/// This is a simplified scoring function for unit testing that takes
/// a list of violations and returns a 0-100 score.
///
/// # Arguments
/// * `violations` - List of violations found in the project
/// * `total_files` - Total number of files in the project
///
/// # Returns
/// Health score from 0.0 to 100.0
///
/// # Scoring Formula
/// - Critical violations: -10 points each
/// - Warning violations: -2 points each
/// - Info violations: -0.5 points each
/// - Base score: 100.0
/// - Final score is clamped to 0.0-100.0
pub fn calculate_health_score(violations: &[Violation], total_files: f64) -> f64 {
    let mut score = 100.0;

    for violation in violations {
        match violation.severity {
            ViolationType::Critical => score -= 10.0,
            ViolationType::Warning => score -= 2.0,
            ViolationType::Info => score -= 0.5,
        }
    }

    // Normalize by project size (more violations in larger projects have less impact)
    // This prevents very large projects from being overly penalized
    let size_factor = (total_files / 100.0).min(5.0); // Cap the reduction
    let normalized_score = score + (size_factor * 2.0);

    // Clamp to 0-100
    normalized_score.max(0.0).min(100.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_grade_from_score() {
        assert_eq!(HealthGrade::from_score(95), HealthGrade::A);
        assert_eq!(HealthGrade::from_score(85), HealthGrade::B);
        assert_eq!(HealthGrade::from_score(75), HealthGrade::C);
        assert_eq!(HealthGrade::from_score(65), HealthGrade::D);
        assert_eq!(HealthGrade::from_score(55), HealthGrade::F);
    }

    #[test]
    fn test_score_components_calculate_total() {
        let components = ScoreComponents {
            layer_isolation: 100,
            circular_deps: 100,
            complexity: 100,
            violations: 100,
        };
        assert_eq!(components.calculate_total(), 100);

        let components = ScoreComponents {
            layer_isolation: 80,
            circular_deps: 100,
            complexity: 90,
            violations: 70,
        };
        // (80 * 0.30) + (100 * 0.25) + (90 * 0.20) + (70 * 0.25)
        // = 24 + 25 + 18 + 17.5 = 84.5 ≈ 85
        assert_eq!(components.calculate_total(), 85);
    }

    #[test]
    fn test_progress_bar() {
        let bar = get_progress_bar(100, 10);
        assert_eq!(bar, "[==========]");

        let bar = get_progress_bar(50, 10);
        assert_eq!(bar, "[=====     ]");

        let bar = get_progress_bar(0, 10);
        assert_eq!(bar, "[          ]");
    }

    // Unit tests for calculate_health_score
    #[test]
    fn test_calculate_health_score_with_zero_violations() {
        let violations = vec![];
        let score = calculate_health_score(&violations, 100.0);
        assert_eq!(score, 100.0, "Perfect project should score 100");
    }

    #[test]
    fn test_calculate_health_score_with_critical_violations() {
        let mut violations = Vec::new();
        for i in 0..10 {
            violations.push(Violation {
                severity: ViolationType::Critical,
                file_path: format!("src/bad{}.rs", i),
                line: i,
                message: "Critical violation".to_string(),
            });
        }

        let score = calculate_health_score(&violations, 100.0);
        assert!(score < 50.0, "Score should be <50 with 10 critical violations");
    }

    #[test]
    fn test_calculate_health_score_boundary_values() {
        let violations = vec![];
        let score_min = calculate_health_score(&violations, 1.0);
        assert_eq!(score_min, 100.0, "1 file project should score 100 with no violations");

        let score_max = calculate_health_score(&violations, 1000.0);
        assert_eq!(score_max, 100.0, "Large project should score 100 with no violations");
    }

    #[test]
    fn test_calculate_health_score_with_many_files() {
        let mut violations = Vec::new();
        for i in 0..1000 {
            violations.push(Violation {
                severity: ViolationType::Warning,
                file_path: format!("src/file{}.rs", i),
                line: 1,
                message: "Warning violation".to_string(),
            });
        }

        let score = calculate_health_score(&violations, 1000.0);
        assert!(score < 80.0, "1000 files with warnings should score <80");
    }
}
