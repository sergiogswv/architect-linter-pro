//! Scoring engine for Architecture Health Score
//!
//! This module implements the scoring algorithm that converts analysis results
//! into a 0-100 health score with letter grades.

use crate::analysis_result::AnalysisResult;
use crate::metrics::{HealthGrade, HealthScore, ScoreComponents};

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
}
